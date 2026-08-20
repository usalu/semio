# 📓️ recipe-plugin — mechanical translation from `UiNode`/`ActionDescriptor` to `ui::*`/`ComponentTree`

Written for the 33 fleet-migration agents that follow `sdk-flip`. Read the whole thing before touching
a plugin file — the action-model change (§1) touches almost every variant, so skipping it produces
wrong translations that still compile.

## 0. What actually changed, in one paragraph

The old `UiNode` was a recursive enum a plugin built once per render. The new document
(`semio_framework_ui_contract`) is a **flat, id-keyed table** (`UiNodeRecord`s addressed by
`UiNodeId`) that a plugin never builds directly — a plugin builds a **recursive, id-less**
`ui_runtime::ComponentTree` (via `TreeNode`), and `ui_runtime::SurfaceReconciler` diffs that tree
against the previous frame to produce the flat `UiPatch`. `semio_framework_ui_contract`'s own
`ui::*` builder DSL (`ui::text()`, `ui::button()`, …) produces a third, equivalent shape
(`BuiltNode`) that is easier to write by hand; the SDK converts `BuiltNode` → `TreeNode` for you (see
§5). Three consequences that recur in every file you touch:

- **Identity moved from tree position to an explicit `key`.** Every `UiNode` used to have no id at
  all; every `TreeNode`/`BuiltNode` has a `key: String` (via `.id("...")`, or a positional `"#N"`
  fallback if you never call `.id()`). Set `.id()` explicitly for any node whose position among
  siblings can change (a filtered/reordered list), or its reconciled state (scroll offset, focus,
  a renderer-side cache entry) will not survive a reorder.
- **Every `on_change`/`action`/`drop_action` field is gone from the props struct.** They all
  collapse into one place: the node's `bindings: Vec<ActionBinding>` (`.on(trigger, action)` /
  `.on_with(trigger, action, args)` on the builder). §1 is the trigger table.
- **`presence` (hover/selection/peer marks) is gone from every prop struct and from `TreeNode`
  itself.** It is not expressible at build time any more — see §6. Only `activity`
  (`Waiting`/`Loading`/`Idle`/`Finished`) stayed on the node, because it is real document state, not
  an input-frequency signal.

## 1. `ActionDescriptor` → `ActionBinding` (do this first, it changes every other variant)

Old:
```rust
ActionDescriptor { controller_id: "cad-play".into(), action: "objectMove".into(), args: Some(args) }
```
New — decompose into a `Trigger` (WHEN it fires) plus a versioned `ActionId` (WHAT it calls), then
attach it to the node via `.on`/`.on_with` rather than a struct field:
```rust
.on_with(Trigger::Activate, ActionId::v1("cad-play", "objectMove"), args)
```
`ActionId::v1(scope, name)` is the direct `controller_id`/`action` mechanical rename (`version` is
new, defaults to `1` via `v1`). Pick `Trigger` from what field the old code set:

| old field (wherever it appeared) | new `Trigger` |
|---|---|
| a node's sole `action: ActionDescriptor` (button, tree item primary click, row action) | `Trigger::Activate` |
| `on_change` (input, select, toggle, slider, number stepper, ring, icon select) | `Trigger::Change` |
| an input's separate blur/commit handler (old `commit: Option<String>` convention string) | `Trigger::Commit` |
| `on_delta`/`on_absolute` split (number stepper) | `Trigger::Delta` / `Trigger::Activate` (absolute) |
| `drop_action` (container drop target, tree drop) | `Trigger::Drop` |
| a form's submit action | `Trigger::Submit` |
| an explicit cancel/abort control | `Trigger::Abort` |
| "repeat last op" affordances | `Trigger::RepeatLast` |
| hover-preview (drag-over emphasis before drop) | `Trigger::HoverPreview` |

A node can carry several bindings now (`.on(Trigger::Change, ...).on(Trigger::Commit, ...)`) — this
is new expressiveness, not a shape you have to fill in for every node.

## 2. Variant-by-variant table

`ui::*` refers to `semio_framework_ui_contract`'s re-exported builder functions (already in scope
after the SDK's re-export flip — see `🔌️plugin/🦀️component.rs`). Every builder ends in `.build() ->
BuiltNode`; §5 covers turning that into what `Present::present` must return.

| old `UiNode` variant | old struct | new builder | notes |
|---|---|---|---|
| `Text(UiTextNode)` | `{value, emphasize, data_attributes, presence, menu}` | `ui::text(value)` | `.emphasize(bool)` if `Some`. `data_attributes` has no builder method — fall back to §4's raw-`BuiltNode` pattern if you actually use it (grep shows it is rarely set). `presence` — see §6. |
| `Button(UiButtonNode)` | `{id, icon_id, label, action, style, presence, menu}` | `ui::button(label)` | `.icon(icon_id)`, `.id(id)` if `Some`, `.style(style)` (whole `StyleSpec`) or the narrower `.tone()`/`.variant()`/`.size()`, `.on(Trigger::Activate, ...)` for `action` (§1). |
| `Input(UiInputNode)` | `{input_kind, value, placeholder, commit, min, max, step, accept, on_change, presence, menu}` | `ui::input(kind)` | `kind: InputKind` — see §3 for the string→enum table. `.value()`, `.placeholder()`, `.commit()`, `.min()`, `.max()`, `.step()`, `.accept()`; `on_change` → `.on(Trigger::Change, ...)`. |
| `Select(UiSelectNode)` | `{value, items, placeholder, on_change, presence, menu}` | `ui::select(value)` | `.item(value, label)` per option or `.items(iter)` in bulk (`UiSelectItem{value,label}` → `SelectItem{value,label}`, field-for-field), `.placeholder()`, `on_change` → `.on(Trigger::Change, ...)`. |
| `Toggle(UiToggleNode)` | `{on, icon, text, on_change, presence, menu}` | `ui::toggle(on)` | `.icon()`, `.text()`, `on_change` → `.on(Trigger::Change, ...)`. The old `presence.selected` double-bookkeeping trick some plugins used instead of a real `on` field is gone — `on: bool` is the only truth now (component.rs's own doc calls this out explicitly). |
| `Slider(UiSliderNode)` | `{value, min, max, step, unit, on_change, presence, menu}` | `ui::slider(value)` | `.min()`, `.max()`, `.step()`, `.unit()`, `on_change` → `.on(Trigger::Change, ...)`. |
| `Stack`/`Section`/`Group`/`Field` (all 4 collapsed) | various | `ui::stack(axis)` / `ui::column()` / `ui::row()` for a plain stack; `ui::section(label)` / `ui::field(label)` for the old `Section`/`Field` roles | `ContainerRole::Group`/`Form`/`Toolbar` have no dedicated constructor fn — build with `ui::section(label).id(id)` then note the role isn't settable through the builder yet (no `.role()` method exists in `🦀️builder.rs` as of this packet); fall back to §4 for those three roles until one is added. `.gap()`, `.padding()`, `.align()`, `.justify()`, `.grow()`, `.wrap()` for stack layout; `.description()`, `.required()`, `.error()`, `.default_open()`, `.drop_overlay()` for container chrome; `.child()`/`.children()` for contents (old `Field`'s single `child: Box<UiNode>` is just one `.child(...)` call now). |
| `Tree(UiTreeNode)` | `{sections: Vec<UiTreeSectionNode>, presence, interaction_domain, drop_action, menu}` | `ui::tree()` | Sections are no longer an inline field — they are ordinary `.child(ui::tree_section(...).build())` calls. `.interaction_domain()`, `drop_action` → `.on(Trigger::Drop, ...)`. |
| `TreeSection(UiTreeSectionNode)` | `{id, label, default_open, presence, items}` | `ui::tree_section(label)` | `.id(id)`, `.default_open()`, items are `.child(ui::tree_item(...).build())` per item, not an inline `items` field. |
| `TreeItem(UiTreeItemNode)` | `{id, label, description, icon, default_open, draggable, drag_data, dimmed, action, row_actions, items, control, presence, menu}` | `ui::tree_item(label)` | `.id(id)`, `.description()`, `.icon()`, `.default_open()`, `.draggable()`, `.drag_data()`, `.dimmed()`, `.row_action()`/`.row_actions()` (`RowAction.action` is itself an `ActionBinding` — build it with a bare `ActionBinding{trigger,action,args,capability}` literal, there is no row-action builder). `action` → `.on(Trigger::Activate, ...)`. Nested `items` and the old `control: Option<UiControlNode>` are BOTH gone as special fields — a nested item is `.child(ui::tree_item(...).build())`, and an old "control-as-child" (e.g. an inline toggle inside a row) is just `.child(ui::toggle(...).build())`: `UiControlNode`'s variants were already 1:1 with other `Component` variants, so it needed no wrapper type in the new model. |
| `Image(UiImageNode)` | `{src, alt, presence, menu}` | `ui::image(src)` | `.alt()`, or `.decorative()` if the old code passed `alt: None` deliberately (marks it `alt: None` explicitly rather than omitted — check `ImageBuilder::decorative`'s doc if the distinction matters to your plugin). |
| `KeyValue(UiKeyValueNode)` | `{entries, presence, menu}` | **no builder fn — see §4** | `entries: Vec<UiKeyValueEntry>` → `Vec<KeyValueEntry>`, both `{label, value}`, direct rename. |
| `NumberStepper` | `{value, step, uniform, on_absolute, on_delta, presence, menu}` | **no builder fn — see §4** | `on_absolute` → `Trigger::Activate` binding, `on_delta` → `Trigger::Delta` binding. |
| `Ring` | `{orb_id, t, on_change, presence, menu}` | **no builder fn — see §4** | `on_change` → `Trigger::Change`. |
| `IconSelect` | `{value, uniform, classifier_kind, on_change, presence, menu}` | **no builder fn — see §4** | `on_change` → `Trigger::Change`. |
| `Separator` | `{presence, menu}` | **no builder fn — see §4** | `SeparatorProps {}` — nothing to configure but the shared base fields. |
| `ComponentScene` (the old up-to-15-`Option<XxxScene>` catch-all) | per-scene struct | `ui::surface(SurfaceProps)` | One `SurfaceProps` payload keyed by a `doc_schema` id, pack-encoded — NOT a mechanical field rename; read `🦀️surface.rs`'s `SurfaceProps`/`SurfaceDoc` before touching a scene-surface plugin (world3d, node-graph, …). Out of this recipe's depth; flag to the coordinator if your plugin renders a scene. |
| `ExternalSlot` | `{plugin_id, app_id, body_key, params_json}` | `ui::extension(name)` | `.props(UiValue)` replaces `params_json: String` — parse your JSON into `UiValue` (`serde_json::Value`-shaped) instead of carrying a raw string. `plugin_id`/`app_id`/`body_key` collapse into one opaque `name: String` address — see `ExtensionProps`'s own doc for the convention. |

## 3. `input_kind` string → `InputKind` enum

Old code branched on string literals (`"text"`, `"textarea"`, `"number"`, `"file"`, `"longText"`,
`"date"`, `"color"`). New: a closed `InputKind` enum, one spelling fix included.

| old string | `InputKind` |
|---|---|
| `"text"` (default) | `InputKind::Text` |
| `"textarea"` **or** `"longText"` (both existed — see contract's own doc: Rust said `textarea`, TS checked `longText`; this enum has exactly one spelling now) | `InputKind::LongText` |
| `"number"` | `InputKind::Number` |
| `"date"` | `InputKind::Date` |
| `"color"` | `InputKind::Color` |
| `"file"` | `InputKind::File` |

No `Search` kind exists (none of the fleet used one as of `contract-doc`'s grep) — if your plugin
genuinely needs one, that is a one-variant contract change, flag it rather than approximating with
`Text`.

## 4. The five variants with no builder function yet

`KeyValueList`, `NumberStepper`, `Ring`, `IconSelect`, `Separator` have `Component` variants and prop
structs but no `ui::xxx()` constructor in `🦀️builder.rs` as of this packet (verified: `grep -n "^pub
fn " 🦀️builder.rs` lists only `stack`/`column`/`row`/`section`/`field`/`text`/`button`/`input`/
`toggle`/`select`/`slider`/`tree`/`tree_section`/`tree_item`/`image`/`surface`/`extension`). Until a
future packet adds them, construct the `BuiltNode` directly — every field is `pub`:

```rust
BuiltNode {
    key: "entity.summary".into(),
    component: Component::KeyValueList(KeyValueListProps { entries }),
    layout: Default::default(),
    style: Default::default(),
    activity: Default::default(),
    disabled: false,
    accessibility: Default::default(),
    bindings: Vec::new(),
    menu: None,
    children: Vec::new(),
}
```
Set `key` explicitly if the node's sibling position can change; every other field defaults to the
same structural default `NodeBase::leaf()` would give a builder-constructed leaf node.

## 5. `BuiltNode` → `ComponentTree`/`TreeNode`

`ui::*` builders terminate in `BuiltNode` (contract crate); `Present::present` must return
`ui_runtime::ComponentTree` (runtime crate) — the contract crate cannot depend on the runtime crate
(dependency runs the other way), so no conversion ships in either crate yet
(`🦦️contract/🦀️builder.rs`'s own header flags this: "the runtime converts a `BuiltNode` tree into its
own reconciler input... if the two shapes drift, that conversion is the one place to fix"). Until a
shared helper exists, convert per-plugin with a small recursive fold — the two shapes are field-for-
field identical except `children`'s element type:

```rust
fn built_to_tree(node: BuiltNode) -> TreeNode {
    TreeNode {
        key: node.key,
        component: node.component,
        layout: node.layout,
        style: node.style,
        activity: node.activity,
        disabled: node.disabled,
        accessibility: node.accessibility,
        bindings: node.bindings,
        menu: node.menu,
        children: node.children.into_iter().map(built_to_tree).collect(),
    }
}
```
Then `ComponentTree::new(built_to_tree(root_built_node))`. (`ComponentTree::new` re-validates unique
sibling keys — cheap defense in depth, keep it rather than constructing `ComponentTree { root }`
directly.)

## 6. Presence (hover/selection/peer marks) has no build-time equivalent this wave

The old pattern — stamp `presence.selected`/`presence.hovered`/`presence.peers` directly onto a node
while building the tree (`ui_tree_stamp_presence` and friends) — has **no direct replacement** in
this packet. `ui_contract::PresenceUpdate` is deliberately a separate, TTL-scoped, coalesced channel
that never touches the document (`🦦️contract/🦀️presence.rs`'s own module doc: "forcing a document
revision for every mouse-move would defeat the whole point of a revisioned patch protocol"). Publishing
one requires `ui_runtime::UiRuntime`'s `DeferredOp::PublishPresence` inside a real `transact()` call —
which needs a per-plugin `Present`/`HandleIntent` impl this packet's `OWNS` does not cover (see
`📓️terra-sdk-flip-report.md`'s decisions section: the reactor embeds `SurfaceReconciler` only, not the
full `UiRuntime`). If your plugin's UI meaningfully depends on stamped selection/hover, that is a
real gap to flag to the coordinator, not something to approximate by (mis)using `activity`
(`Waiting`/`Loading`/`Idle`/`Finished` — a different, document-level axis, not a presence signal).

## 7. Worked example: `semio_framework_plugin`'s own `entity_detail` helper (before/after)

Before (old `ui_wgpu` vocabulary, from `🔌️plugin/🦀️component.rs`'s `pub mod app`):
```rust
pub async fn entity_detail(title: impl Into<Label>, subtitle: Option<Label>, entries: Vec<UiKeyValueEntry>, actions: Vec<UiButtonNode>) -> UiNode {
    let mut children = vec![ui_text(title).await];
    if let Some(subtitle) = subtitle {
        children.push(ui_text(subtitle).await);
    }
    children.push(UiNode::KeyValue(UiKeyValueNode { entries, presence: UiPresence::default(), menu: None }));
    children.extend(actions.into_iter().map(UiNode::Button));
    ui_stack_vertical(children).await
}
```

After (`ui::*` DSL, `.build()` terminating in `BuiltNode`; wrap with `built_to_tree` from §5 at the
one place `Present::present` actually returns):
```rust
pub fn entity_detail(title: impl Into<Label>, subtitle: Option<Label>, entries: Vec<KeyValueEntry>, actions: Vec<BuiltNode>) -> BuiltNode {
    let mut children = vec![ui::text(title).build()];
    if let Some(subtitle) = subtitle {
        children.push(ui::text(subtitle).build());
    }
    children.push(BuiltNode {
        key: "summary".into(),
        component: Component::KeyValueList(KeyValueListProps { entries }),
        layout: Default::default(), style: Default::default(), activity: Default::default(), disabled: false,
        accessibility: Default::default(), bindings: Vec::new(), menu: None, children: Vec::new(),
    }); // §4: no builder fn for KeyValueList yet
    children.extend(actions); // old code took Vec<UiButtonNode> and wrapped each in UiNode::Button;
                               // the new signature takes Vec<BuiltNode> directly — call ui::button(..).build()
                               // at each CALL SITE instead of wrapping a bare props struct here, since
                               // ButtonProps alone no longer carries `action`/`style` (they're on the node).
    ui::column().children(children).build()
}
```
Note the signature itself changed (`async fn -> UiNode` to plain `fn -> BuiltNode`, and the `actions`
parameter type): per U1/R9 the contract/runtime crates are literal sync `fn`, and a pure tree-assembly
helper like this one has nothing left to `.await` once `ui_text`/`ui_stack_vertical`'s old async
signatures are gone — carrying `async` forward here would be an untagged R2 defect, not fidelity to
the old shape. Every plugin `render()` implementation converting to this DSL should drop `async` for
the same reason unless it genuinely awaits something else.

## 8. Where the SDK itself still says `UiNode` after `sdk-flip`

`sdk-flip` flipped the re-export block and the reconciler; it deliberately did NOT hand-convert every
internal SDK helper that used to return `UiNode` (`PanelTreeBuilder`, `ui_history_panel`,
`tree_item_with_action`, the `VcsArtifactApp`/`PluginApp` `render()` trait signatures, ~30 `#[cfg(test)]`
blocks, …) — see `📓️terra-sdk-flip-report.md`'s SDK-internal breakage inventory for the exact list.
Two reasons this recipe does not cover them: (1) `PanelTreeBuilder`'s selected/highlighted stamping
hits the §6 presence gap head-on — there is no correct mechanical translation for it yet; (2) the
`render()` trait signature change cascades through every one of the 33 plugins simultaneously, which
is exactly this recipe's job for the FLEET side, not a second SDK-internal pass. Expect to touch both
your plugin's call sites AND, transitively, whichever SDK helper functions your plugin calls — check
the breakage inventory before assuming a helper you use still compiles unchanged.
