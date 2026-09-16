# Explore — artifact-tree + inspector panels for a plugin editor, and what 🔋️energy already has

Scope: read-only. Precedents read: fem2d (`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor`,
finished today), cad (`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`), the
framework SDK (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, 39211 lines) and the UI contract
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/…`). Target: `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

## 0. Headline finding

**🔋️energy has zero of the panel/interaction machinery today** — no `📌️panels/` directory (only
empty `.empty.md` stubs in `🎭️modes/✏️edit/🎮️commands` and `🎭️modes/✏️edit/🎚️config`), no
`InteractionDefinition`, no `ActionFactory`/controller id, no `ui_label`/`ui_node_list` helpers, no
`app_labels!` terminology module, and `Editor::handle`'s `_interaction: &InteractionView<'_>` parameter
is unused (underscore-prefixed) — confirmed at
`✏️s/…/✏️editor/🦀️.rs:1246-1255`. It has two framework-kit **windows** (`structure` = `TreeWindowKit`,
`zones` = `TableWindowKit`) but no panel tabs at all.

**The good news:** the *semantic mutations* the inspector would need already exist for every field the
task asks about — `ChangeSurfaceClass`, `ChangeSurfaceBoundaryCondition`,
`ChangeFenestrationUValue/Shgc/Vlt`, `ChangeMaterialThickness/Conductivity`, zone name/volume, and
`Site` — at `🧬️schema/🧬️mutations/`. Only the **editor-level command + dispatch wiring + panel UI** is
missing, not the domain mutations. One real gap: `model_edit`'s whole-model diff (§4) has no
`diff_fenestrations` step, so a fenestration property edit would currently be a **silent no-op**.

---

## 1. Tree panel — how fem2d builds it

### 1.1 Declaration and mounting

A panel tab is a `PanelTabDefinition` returned by a `definition()` function and registered on the
manifest with `.panel_tab_def(...)`:

```rust
// ✏️…/fem2d/✏️editor/📌️panels/🗿️artifact/🦀️.rs:38-40
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"), group: PanelGroup::Workbench, body_key: Some(BODY_KEY.into()), children: Vec::new() }
}
```

`BODY_KEY: &str = "fem2d.play.artifact"` (line 26). Registration is one line each in the manifest
builder chain:

```rust
// ✏️…/fem2d/✏️editor/🦀️.rs:1439-1441
.panel_tab_def(artifact_panel::definition())
.panel_tab_def(inspection_panel::definition())
.panel_tab_def(results_panel::definition())
```

Render dispatch is a `match body_key` inside `Editor::render_body`
(`✏️…/fem2d/✏️editor/🦀️.rs:1107-1134`):

```rust
match body_key {
    model_window::BODY_KEY => { ... }
    results_window::BODY_KEY => { ... }
    artifact_panel::BODY_KEY => artifact_panel::render(doc.snapshot, &interaction, labels),
    inspection_panel::BODY_KEY => inspection_panel::render(doc.snapshot, &interaction, labels),
    results_panel::BODY_KEY => results_panel::render(doc.snapshot, results_window::config::captured(cfg).as_ref(), &results_window_instance_id(view_state).unwrap_or_default(), labels),
    _ => built_text_node(Label::data(format!("Unknown body: {body_key}")))...,
}
```

`render_body` is called from both the *typed* `render()` (empty interaction snapshot) and the
*interaction-aware* variant which builds `Fem2dInteractionSnapshot::from_interaction(interaction)`
(lines 1083-1105) — i.e. every panel/window render receives one shared interaction snapshot built once
per render.

energy currently has an analogous `match body_key` at `✏️…/energy/✏️editor/🦀️.rs:1258-1264`, but only
three arms (`structure::BODY_KEY`, `zones::BODY_KEY`, `simulation::BODY_KEY`) — no interaction snapshot
is threaded because none exists.

### 1.2 Row construction and the `"fem2d"` domain

`build_artifact_tree` (`📌️panels/🗿️artifact/🦀️.rs:375-404`) assembles 9 sections in document order
(nodes, elements, regions, supports, load cases [+nested loads], combinations [+nested terms],
materials, sections, analysis). Every selectable row is built by `entity_row`
(lines 195-205):

```rust
fn entity_row(id: &str, granularity: &str, label: &str, description: &LabelText, icon: &str, dimmed: bool) -> UiAssemblyResult<BuiltNode> {
    let mut item = tree_item_with_action(id, UiLabel(UiText::clipped(label)), None, pick_action(granularity, id)?)?;
    ...
}
```

The row key is the **raw entity id** (not namespaced) — this is what makes a tree row, a viewport pick
and the inspector selection agree on one vocabulary. `pick_action` (lines 181-186) mints the pick:

```rust
fn pick_action(granularity: &str, id: &str) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
    let target = protocol::InteractionTarget { granularity: granularity.into(), id: id.into() };
    let targets = protocol::json::to_json_string(&protocol::DslValue::Array(vec![protocol::ToValue::to_value(&target)]));
    let args = ui_value_map([("domainId", ui_value_text(FEM2D_INTERACTION_DOMAIN)?), ("merge", ui_value_text("replace")?), ("method", ui_value_text("pick")?), ("targets", ui_value_text(targets)?)])?;
    fem2d_action(INTERACTION_SELECT_ACTION_ID, Some(args))
}
```

`INTERACTION_SELECT_ACTION_ID = "interactionSelect"` and `INTERACTION_HOVER_ACTION_ID =
"interactionHover"` are framework-reserved constants
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1076,1080`). `fem2d_action` (`✏️…/fem2d/✏️editor/🦀️.rs:1162-1164`)
is `ActionFactory::new(FEM2D_PLAY_CONTROLLER_ID).action(action, args)` — `FEM2D_PLAY_CONTROLLER_ID =
FEM2D_APP_ID = "fem2d-play"` (lines 39, 42). Deliberately **fem2d's own tree carries no row actions**
beyond the one pick — focus/delete moved to the inspector after the argument-arena starvation incident
(§2 traps); the row's own `tree_item_with_action` binds `Trigger::Activate` to the pick, nothing else.

The whole tree is bound to the domain at build time:

```rust
// build_artifact_tree, 🦀️.rs:400-402
.interaction_domain(FEM2D_INTERACTION_DOMAIN)?
.selected(marked_ids(&interaction.selected_ids))?
.highlighted(marked_ids(&interaction.hovered_ids))?
```

`marked_ids` (line 323-325) caps at `MARKED_IDS_LIMIT = UI_FIXED_LIST_ITEMS = 32`
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:21`), so a wider selection marks only its first
page instead of refusing the render.

### 1.3 Paging — `panel_page_rows`, `PanelRowBudget`, max-min-fair quotas

Framework helpers (`🧰️framework/…/🔌️plugin/🦀️.rs`):

- `panel_page_rows()` (line 5888-5890) = `UI_VALUE_PAGE_ROWS.min(ui_value_headroom().rows())` — the UI
  contract's per-render row ceiling, clamped by whatever the process-wide `UiValue` argument arena
  still admits right now (`UI_VALUE_PAGE_ROWS = UI_BUILT_CHILDREN_MAX - 1`,
  `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:54`; the arena's per-row cost is
  `UI_VALUE_ROW_COLLECTIONS`/`UI_VALUE_ROW_ITEMS`, same file lines 39-42).
- `struct PanelRowBudget(usize)` (line 5893) — `.spend()` decrements-or-refuses, `.remaining()`,
  `.nested(reserved, build)` (lines 5918-5926) runs a nested build against `total - reserved` and settles
  back what it under-spent.
- `paged_panel_section<T>(section_id, entries, section_rows, budget, row)` (lines 5952-5985) — places
  rows while both the section quota and the shared budget last, catches a `ui.fixed-capacity` refusal
  mid-section (another panel may be taking arena credit concurrently) and closes with
  `panel_continuation_row(section_id, omitted)` (`+N`, lines 5940-5949) instead of failing the render.
- `PanelTreeBuilder` (lines 5784-5866) — namespaced ids, `.section()` / `.section_or_placeholder()`,
  `.selected()`/`.highlighted()`, `.interaction_domain()`, terminal `.build() -> BuiltNode`
  (a `Component::Tree`). Its own doc comment flags a real limitation: `.selected()`/`.highlighted()`
  are recorded on the builder but **do not currently reach the rendered tree** — selection/hover is a
  separate `PresenceUpdate` channel this SDK layer has no `transact()` for yet. (fem2d still calls
  `.selected()`/`.highlighted()` for the future wiring; don't rely on it painting anything today.)
- `UI_DOCUMENT_NODES = 128` (`🧰️framework/…/🧬️contract/📃️document/🦀️.rs:95`) — total built-node budget
  a document may spend; fem2d's widest section (60-node synthetic doc) measured 47/128.

fem2d's own `section_quotas` (`📌️panels/🗿️artifact/🦀️.rs:349-364`) is a **deliberate improvement over
cad's pattern**: cad's artifact panel reserves `SECTIONS - k` verbatim per pane
(`✏️…/cad/✏️editor/📌️panels/🗿️artifact/🦀️.rs:252-262`, `budget.nested(SECTIONS - 1, ...)` etc.) — a
"reserve what my siblings still need" scheme that, applied to fem2d's asymmetric 9 sections, gave the
first section everything and starved load cases to one row. fem2d's `section_quotas(demands, page)`
instead computes a max-min-fair per-section ceiling (raise the ceiling while every section-under-it
fits) then hands leftover rows to the widest sections — every small section stays whole, only the wide
ones truncate.

### 1.4 Row pick → selection

`INTERACTION_SELECT_ACTION_ID`/`INTERACTION_HOVER_ACTION_ID` are framework-**reserved** action ids —
the framework auto-injects `interactionSelect`/`interactionHover`/`clearSelection`/`selectAll` handling
for any domain declared via `.interaction(...)` + `.window_kind_interactions(...)`
(`✏️…/fem2d/✏️editor/🦀️.rs:1437-1438`):

```rust
.interaction(fem2d_interaction_definition())
.window_kind_interactions(model_window::WINDOW_KIND_ID, vec![InteractionRef::new(FEM2D_INTERACTION_DOMAIN)])
.window_kind_interactions(results_window::WINDOW_KIND_ID, vec![InteractionRef::new(FEM2D_INTERACTION_DOMAIN)])
```

`fem2d_interaction_definition()` (`✏️…/fem2d/✏️editor/🕹️interaction/🦀️.rs:30-56`) declares
`InteractionDefinition { id, label, granularities: Vec<GranularityDefinition>, hierarchy:
HierarchyProvider::Flat, hover: HoverSpec::default(), selection: SelectionSpec { modes: [Multiple,
Single], methods: [Pick, Rectangle, Lasso], merges: [Replace, Additive, Subtractive, Invertive],
transitive: false, broadcast: true } }`. Selection/hover state itself is framework-owned — the app
never stores it; it reads it back through `InteractionView::selection(domain)`/`.hover(domain,
channel)` (`Fem2dInteractionSnapshot::from_interaction`, `🕹️interaction/🦀️.rs` ~line 66) and a viewport
pick command (`canvasPointerDown`) emits the *same* `interactionSelect` effect a tree row dispatches:

```rust
// 🕹️interaction/🦀️.rs (§ Effects, per w-a report)
interaction_select_effect(targets, merge) -> Effect::ReplayShellCommand {
    action_id: semio_framework::INTERACTION_SELECT_ACTION_ID,
    args: { domainId: "fem2d", targets: <json Vec<InteractionTarget>>, merge, method: "pick" }
}
```

So there is exactly **one selection channel**: tree row, inspector pick-row (a load inside a load
case) and viewport pick all dispatch `interactionSelect` with the same `domainId`.

### 1.5 Tree shows current selection/hover

`Fem2dInteractionSnapshot { selected_ids: Vec<String>, hovered_ids: Vec<String> }`
(`🕹️interaction/🦀️.rs`, `from_interaction` reads `interaction.selection(DOMAIN).ids` /
`interaction.hover(DOMAIN, POINTER_CHANNEL).ids`) is built once per render and threaded into both
panels' `render()`. The tree passes it to `.selected()`/`.highlighted()` on `PanelTreeBuilder` (capped
at 32 ids, see §1.3's caveat about `PanelTreeBuilder` not yet painting these onto the built tree). The
canvas highlight (not a panel concern, but the third leg of the same snapshot) is painted by
`fem2d_structure_layers_with(doc, ..., interaction)` = base layers + `fem2d_highlight_layers`, emitting
`sel-*`/`hov-*` overlay layers (colors `#facc15`/`#fde68a`) — see §3.

---

## 2. Inspector — how fem2d builds it (and how it got there)

### 2.1 The layout trap that must NOT be repeated

**First cut used `PanelTreeBuilder`/`Component::Tree` for the inspector body and it silently dropped
every control.** Verified defect, `📓️w-d-inspector-2026-09-16.md` §7: the React `Interpreter`'s
`uiTreeNodeToTreePanelConfig` maps a tree's sections onto `TreeDataSection`/`TreeDataItem` records and
reads only `label`/`description`/`items`/`action` per row — a non-tree child (a number input, a
select) has nowhere to go in that mapping and never reaches the DOM. Only rows built from
`tree_item_desc`/`tree_item_with_action` survived; the panel showed `ID n1` and two action buttons and
nothing else.

**The fix — and the load-bearing rule for any new inspector — is: the body must be a
`ui::column()` of `ui::section()`s whose rows are `ui::field(label) > control`, never a
`Component::Tree`.** `Component::Container(role: Section | Field)` is interpreted structurally (a
section renders `<Section>`, a field renders `<Field>` with its control inside), which the tree
renderer does not do. fem2d's results panel and puzzle3d's Settings panel always used this shape; only
the inspector had briefly used the tree builder. Three of fem2d's panel tests now assert
`!carries_a_tree(json)` to pin this down for good.

Concretely (`📌️panels/🔍️inspection/🦀️.rs`):

```rust
// Layout primitives (lines 79-100), all over semio_framework_ui_contract (`use ... as ui`)
fn section(id: &str, label: &str, rows: UiFixedList<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    let builder = ui_id(ui::section(ui_label(label)?), id)?.default_open(true);
    ui_build(builder.try_children(rows)...)
}
fn column(sections: UiFixedList<BuiltNode>) -> UiAssemblyResult<BuiltNode> {
    let builder = ui_id(ui::column(), ROOT)?;
    ui_build(builder.try_children(sections)...)
}
fn control_row(row_id: &str, label: &str, control: BuiltNode) -> UiAssemblyResult<BuiltNode> {
    let row = ui_id(ui::field(ui_label(label)?), row_id)?;
    ui_build(row.try_child(control)...)
}
```

`ui::section`/`ui::field`/`ui::column`/`ui::input`/`ui::select`/`ui::slider`/`ui::toggle`/`ui::button`/
`ui::text` are builder functions in
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs` (e.g. `section` line 952 → `ContainerBuilder::new(ContainerRole::Section, label)`;
`field` line 958 → `ContainerRole::Field`; `column` line 867 → a `Container(role: Plain)` stack; `input`
line 1085; `toggle` line 1169; `select` line 1217; `slider` line 1270). `ContainerRole` and `InputKind`
are declared in `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:70,92`.

### 2.2 Reading the current selection

```rust
// render(), 🦀️.rs:442-445
pub fn render(doc: &Fem2dSnapshot, interaction: &Fem2dInteractionSnapshot, labels: &Fem2dLabels) -> UiAssemblyResult<BuiltNode> {
    let Some((id, kind)) = interaction.selected_ids.iter().find_map(|id| fem2d_entity_kind(doc, id).map(|kind| (id.as_str(), kind))) else {
        return summary(doc, labels);
    };
    ...
```

`fem2d_entity_kind(doc, id) -> Option<&'static str>` (shared resolver, `🕹️interaction/🦀️.rs`, one of
the "names are frozen" resolvers slice A wrote for slices B/D to share) walks all nine granularities in
one fixed precedence and is the single source of truth for "what kind of thing is this id". The
inspector shows the **first** resolvable selected id's fields (a multi-selection header lists every
id as plain text rows, §2.6) and falls back to a document `summary()` when nothing resolves.

### 2.3 Binding a control: `try_on_with(Trigger::Change, action, args)`

Every editable row shares one binder:

```rust
// bind(), 🦀️.rs:152-158
fn bind<B: HasBase>(builder: B, action: &str, field_name: &str, id: &str) -> UiAssemblyResult<B> {
    let (action, args) = fem2d_action(action, Some(patch_args(field_name, id)?))?;
    match args {
        Some(args) => builder.try_on_with(Trigger::Change, action, args).map_err(...),
        None => builder.try_on(Trigger::Change, action).map_err(...),
    }
}
```

`patch_args` (lines 145-150) builds `{field, id}` via `UiMapBuilder` (keys pushed in the ascending
order the builder requires — `UiMapBuilder::push` panics/errors on out-of-order keys). Concrete row
builders (`number_row`/`text_row`/`slider_row`/`toggle_row`/`select_row`, lines 165-197) all end in
`bind(control, action, field_name, id)`. E.g.:

```rust
push(&mut rows, number_row("node.x", labels.x.as_str(), node.x, 0.1, "patchNode", "x", &node.id))?;
```

For a `select`, options are `Vec<(String, String)>` (value, display-label) capped at `SELECT_ITEMS_MAX
= 24` (`UiFixedList` itself admits 32). Verbs (Focus/Delete, and a load's own pick row inside a load
case) use `action_button` bound `Trigger::Activate` instead (lines 104-112) — activation, not change.

### 2.4 How the value arrives at `command_from_action`

**The host merges the control's own current value under the key `value`** into whatever argument map
the control declared — so the wire payload the action handler receives is always
`{field, id, value}`, even though the panel code only ever builds `{field, id}`. This is stated
explicitly in the panel's module doc (`📌️panels/🔍️inspection/🦀️.rs:11-13`) and is *why* `patch_args`
never includes a `value` key itself — putting one there would be overwritten/duplicated. On the
dispatch side, `EnergyModelEditorCommand`'s `args_bridge::command_from_action` shows the general shape
any editor uses to turn that `DslValue` map into a typed command (see §4) — numeric fields accept
**numeric strings** as well as numbers, because a `<select>`-sourced argument arrives as a string:

```rust
fn number(args: Option<&dsl::DslValue>, key: &str) -> Option<f64> {
    let value = field(args, key)?;
    value.as_f64().or_else(|| value.as_str()?.parse().ok())
}
```

(energy's `args_bridge`, `✏️…/energy/✏️editor/🦀️.rs:187-192` — same pattern fem2d would need for its
own `command_from_action`, not shown verbatim here but implied by the "numeric strings" note in the
task and confirmed live in energy's own bridge.)

### 2.5 What a "patch" command looks like → semantic mutation

fem2d's design (`📓️w-d-inspector-2026-09-16.md` §1, confirmed against the nine `patch*` command files):
**one flat `{field, id}` payload per entity kind, matched to one existing whole-record mutation**:

- `patchNode{field: "x"|"y", id, value}` → `ReplaceNode` (whole record, one field changed).
- `patchElement` → `ReplaceElement` (handles `kind` bar↔beam variant swap too).
- `patchMaterial`/`patchSection`/`patchSupport`/`patchRegion`/`patchLoad`/`patchCombination` → their
  `Replace*` mutations, same shape.
- **Exception: `patchLoadCase`** does NOT replace the whole case (it owns a `loads` collection —
  re-sending it on a rename would lose a concurrent `add-load`). `name` → `ChangeLoadCaseName`,
  `selfWeight` → `ChangeLoadCaseSelfWeight` — two narrow mutations instead of one wide `Replace`.
- **`patchSupport`'s `fixed`** is a DOF *set*, not a scalar: each of `tx`/`ty`/`rz` is its own toggle
  (a membership edit), and the rebuilt set is always emitted in `FemDof::ALL` order so two concurrent
  toggles converge on one spelling.
- **`patchCombination` gained a second field name**, `addTerm`, alongside `term:<caseId>`: an "add a
  term" select's chosen case id rides the control's own `value`, and `field` must stay constant across
  the select's options, which `term:<caseId>` cannot express. A `term:<caseId>` factor of exactly `0`
  **removes** the term.
- Three refusal shapes, one code family: `<app>.patch.<noun>-missing` / `-field` / `-value`. A value
  equal to the current one is `Emit::default()` (no revision opened).

energy's own reduce (§4) shows a **different, whole-model-diff** strategy that is arguably an even
better fit for a generic `set*Property{id, property, value: f64}` verb — see §4/§5.

### 2.6 Refresh / dirty scope, and the "action group not per row" rule

The inspector doesn't manage its own refresh — `render()` is re-invoked by the host on every
mutation/interaction change, reading the live `doc`/`interaction` fresh each time; there is no
separate "dirty" tracking in the panel. What IS managed deliberately is **argument-arena spend**:
Focus/Delete are rendered **once per selection, in one `action_rows` group**
(`📌️panels/🔍️inspection/🦀️.rs:414-427`), not per entity row, specifically because "a page of rows
carrying two actions each... is exactly what exhausts the one-page `UiValue` argument arena and starves
every panel rendered beside the tree (the cad incident, and this ticket's first browser probe)" — the
same reasoning that pulled focus/delete out of the artifact tree (§1.2) and into the inspector.

### 2.7 Selection plumbing detail: a load inside a load case

`load_pick_row` (`📌️panels/🔍️inspection/🦀️.rs:337-345`) is a `ui::button` bound `Trigger::Activate` to
`select_action(id)` (lines 347-355), which mints the exact same `interactionSelect` payload the tree
and a viewport pick use, at `load` granularity — "so the inspector re-renders on that load instead of
inventing a second selection channel."

---

## 3. Selection plumbing between tree / inspector / viewport

One domain, one wire contract, three producers:

| Producer | Mechanism |
|---|---|
| Tree row click | `tree_item_with_action(id, label, ..., pick_action(granularity, id))` → `interactionSelect` |
| Inspector pick row (nested load) | `action_button` bound `Trigger::Activate` → same `interactionSelect` payload |
| Viewport pick (`canvasPointerDown`) | `fem2d_hit_test` resolves `(granularity, id)`, then `interaction_select_effect(targets, merge)` → `Effect::ReplayShellCommand { action_id: INTERACTION_SELECT_ACTION_ID, args: {domainId, targets, merge, method:"pick"} }` |

The framework, not the app, owns the resulting selection/hover state; every render reads it back via
`InteractionView::selection(domain)`/`.hover(domain, channel)`. The interaction **domain and its
granularities** are declared once (`fem2d_interaction_definition()`) and bound to every window kind
that should support a canvas pick (`.window_kind_interactions(WINDOW_KIND_ID, vec![InteractionRef::new(DOMAIN)])`)
— both fem2d Canvas2d windows (model, results) opt in.

**Hit-testing and `focusEntity`** (viewport side, not a panel, but the third leg): `fem2d_hit_test(doc,
camera, x, y, w, h)` runs pick tiers by granularity (node → support → load → element → region, each
nearest-candidate-wins, canvas-pixel tolerances) and `focusEntity{id}` resolves
`fem2d_entity_model_point`, projects through `screen_2d`, and re-centers the addressed window's camera
at its *existing* zoom (`focus_entity` command, `set_camera::handle_window`). Hover is emitted on every
pointer sample (`canvasPointerMove`) via `interaction_hover_effect`.

**Highlight painting** rides the same snapshot: `fem2d_structure_layers_with(doc, ..., interaction)` =
bare layers + `fem2d_highlight_layers`, emitting `sel-*`/`hov-*` overlay layers per entity kind
(bounds-circle for node/support, `segments` path for element/load/region — `segments` is the only
Canvas2d shape honoring `stroke.width`), colors `SELECTION_COLOR_2D = #facc15` /
`HOVER_COLOR_2D = #fde68a`.

**cad's variant is a hybrid**, worth knowing before copying fem2d verbatim: cad's inspection panel
(`✏️…/cad/✏️editor/📌️panels/🔍️inspection/🦀️.rs`) reads THREE different selections in priority order —
`selected_object_section` (the framework `"cad"` interaction domain, `envelope.interaction.ids`, for
objects across four panes), `selected_reference_section` (an **app-owned transient/runtime** field,
`envelope.runtime.selected_reference_id`, for a reference overlay that has no interaction-domain
granularity) and `selected_node_section` (another app-owned field, `envelope.runtime.selected_node_ids`,
for the document-tree's own node selection). Only the first goes through
`InteractionView`/`interactionSelect`; the other two are plain app state edited by a dedicated
`patchCadPlayReference` action. **cad's inspector is also entirely read-only except for that one
reference-patch path** — it has no `number`/`select`/`slider`/`toggle` control bound with
`Trigger::Change` anywhere; fem2d's inspector is the actual first "real editable form controls"
precedent, not cad's.

---

## 4. What 🔋️energy already has, and what a panel would need to add

### 4.1 The 14 retained tool ids (`ENERGY_MODEL_RETAINED_TOOL_IDS`, `🦀️.rs:72-86`)

```
set-node, set-cell, create-zone, rename-zone, delete-zone, create-surface, delete-surface,
assign-surface-construction, set-material-property, set-thermostat-setpoints, set-site,
set-run-period, setActiveExample, set-simulation-settings
```
(the first two are stamped `Migrated` by the framework's `TreeWindowKit`/`TableWindowKit` themselves;
the rest are classified explicitly in the manifest, `🦀️.rs:1347-1349`.)
`ENERGY_MODEL_DOCUMENT_TOOL_IDS` (lines 92-105) is the 12 of these that publish to the `Artifact` lane
(everything except `setActiveExample`, which is `HostOnly` — it swaps the document via
`Effect::LoadDocument`, and `set-simulation-settings` which publishes to the `Config` lane).

### 4.2 `args_bridge::command_from_action` (`🦀️.rs:170-266`)

Exactly the shape §2.4 describes generically: `field(args, key) -> Option<&DslValue>`, then `text`/
`number`/`flag` helpers that each fall back through string↔numeric coercions, feeding a `match action {
... }` that builds one `EnergyModelEditorCommand` variant per action id, with per-field fallback
defaults (e.g. `CREATE_ZONE_ACTION_ID => Command::CreateZone { name: text_or("name","Zone"),
volume_m3: f64_or("volumeM3", 100.0), ... }`).

### 4.3 `reduce()` (`🦀️.rs:580-733`) — a whole-model diff, NOT fem2d's per-field match

This is the one architectural fact that most changes the recipe: energy's `reduce()` clones the whole
`crate::model::Model`, mutates the clone in a `match command { ... }` arm (touching plain struct
fields, e.g. `zone.volume_m3 = parsed`), then calls `model_edit(kind, &base_model, &edited_model,
description)` (line 613). **`model_edit`** (lines 308-361) is a structural diff over the WHOLE model —
`diff_zones`, `diff_surfaces`, `diff_materials`, `diff_thermostats` each compare `base` vs `model`
field-by-field and push exactly the mutation that changed (`if was.class != now.class { push(
change_surface_class(...)) }`), then a `probe` re-assembles `base` with every OTHER field replaced by
`model`'s and asserts `probe == *model` — if some field changed that no diff function accounted for,
`kind_unavailable(kind, kind)` faults loudly rather than silently dropping it.

**Confirmed live gap:** `probe.fenestrations = model.fenestrations.clone();` (line ~353) means the
probe check can never catch a fenestration *field* change (it copies the whole collection verbatim
before comparing), and there is **no `diff_fenestrations` function** — `diff_surfaces` only diffs
fenestration **deletion** (`for was in &base.fenestrations { if !model.fenestrations.iter().any(...) {
push(delete_fenestration) } }`, lines 410-414), never a property change. **A `SetFenestrationProperty`
command that mutates `fenestration.u_value_w_m2k` on the model clone today would pass the probe check
(nothing flags it) and simply emit NO mutation for that edit — a silent no-op**, exactly the trap class
fem2d's `w-a` report calls out for other reasons (§ "3 refusal shapes... a value equal to the current
one returns Emit::default()" — but this is worse: the value genuinely changed and nothing reports it).

`diff_surfaces` (lines 409-464) DOES already diff every `Surface` field, including the two the task
asks about:
```rust
if was.class != now.class { steps.push(mutations::change_surface_class(now.id, now.class)); }
...
if was.outside_boundary_condition != now.outside_boundary_condition {
    steps.push(mutations::change_surface_boundary_condition(now.id, now.outside_boundary_condition.kind(), interzone_partner(now.outside_boundary_condition)));
}
```
and construction is already a dedicated command (`AssignSurfaceConstruction` → `change_surface_construction`,
lines ~466 in reduce / line ~452 in diff). So **surface class + boundary only need an editor-level
command**; the diff and the mutations already exist.

`set_material_property` (lines 753-780) is the existing generic pattern most useful to copy: one
command `SetMaterialProperty{material, property: String, value: f64}`, one `match property { "thicknessM"
=> ..., "conductivityWMK" => ..., ... }` inside the handler (not `reduce`'s outer match) that sets the
field on the clone and returns the mutation-kind string; `model_edit`'s later `diff_materials` (not
shown in full above, but same shape as `diff_surfaces`) does the actual per-field mutation emission.
Thickness (`thicknessM`) and conductivity (`conductivityWMK`) are **already wired**, end to end.

### 4.4 Publication contracts / manifest (`🦀️.rs:155-207`, `1324-1349`)

`ArtifactToolPublicationContract` rows (one per tool id, `Artifact` vs `HostOnly` vs presumably
`Config` for `set-simulation-settings` — not fully quoted above but same shape as fem2d's
`FEM2D_PUBLICATION_CONTRACTS`). The manifest builder (`create_energy_model_editor`, lines 1324-1349)
has **no** `.interaction(...)`, `.window_kind_interactions(...)` or `.panel_tab_def(...)` calls today —
those three are exactly what a new tree/inspector pair must add.

### 4.5 What's missing outright for a panel pair

- No controller-id constant / `ActionFactory` helper (fem2d's `fem2d_action`, cad's `cad_action`) —
  energy needs e.g. `pub const ENERGY_MODEL_PLAY_CONTROLLER_ID: &str = "energy-model-play";` and a
  `pub fn energy_model_action(action, args) -> UiAssemblyResult<(ActionId, Option<UiValue>)> {
  ActionFactory::new(ENERGY_MODEL_PLAY_CONTROLLER_ID).action(action, args) }`.
- No `ui_label`/`ui_node_list` admission helpers (fem2d `🦀️.rs:1148-1158`) — trivial ports.
- No `InteractionDefinition`/domain/granularities/snapshot struct (fem2d's whole `🕹️interaction/🦀️.rs`
  module) — energy's natural granularities are `zone`, `surface`, `fenestration`, `material`,
  `construction`, `thermostat` (maybe `schedule`, `space`); all entities key by `EntityId(u32)`, so a
  row/target id is the decimal string of that `u32` (`entity.id.0.to_string()`).
- `Editor::Transient`/`Presence` are both `NoTransient`/`NoPresence` (`🦀️.rs:1068-1071`) — fine, fem2d's
  selection is 100% framework-owned too and needs neither; only cad's reference-overlay/tree-node
  selections needed app-owned state, and energy has no such secondary selection concept yet.
- No `🗣️terminology/🦀️.rs` / `app_labels!` roster at all — energy's manifest currently spells every
  label inline via `LocalizedLabel::native(en, de)` (e.g. `"Load example"`/`"Beispiel laden"`,
  line ~1338). A new panel pair can keep doing that (skip the terminology apparatus) or build a proper
  `EnergyLabels` roster like fem2d/cad; the former is less work and nothing downstream requires the
  latter — call it a scope decision, not a hard requirement.

### 4.6 New commands needed for the requested inspector edits

| Requested edit | Existing mutation | Existing editor command? | New command needed |
|---|---|---|---|
| Zone name | `mutations::rename_zone` | Yes — `SetZoneCell{row, column:"name", value}` | none |
| Zone volume | `mutations::change_zone_volume` | Yes — `SetZoneCell{column:"volumeM3"}` | none |
| Surface construction | `mutations::change_surface_construction` | Yes — `AssignSurfaceConstruction` | none |
| Surface class | `mutations::change_surface_class` | **No** — diffed, not addressable | **`SetSurfaceProperty{surface, property, value}`** (string `class`) or a dedicated `SetSurfaceClass{surface, class}` |
| Surface boundary | `mutations::change_surface_boundary_condition` | **No** | same command, `property:"boundary"` (+ partner surface for `Interzone`) |
| Material thickness | `mutations::change_material_thickness` | Yes — `SetMaterialProperty{property:"thicknessM"}` | none |
| Material conductivity | `mutations::change_material_conductivity` | Yes — `SetMaterialProperty{property:"conductivityWMK"}` | none |
| Window U-value | `mutations::change_fenestration_u_value` | **No** | **`SetFenestrationProperty{fenestration, property, value}`**, property `"uValueWM2K"` |
| Window SHGC | `mutations::change_fenestration_shgc` | **No** | same command, `property:"shgc"` |
| Window VLT | `mutations::change_fenestration_vlt` | **No** | same command, `property:"vlt"` |
| Site (lat/long/elev/tz/north) | `mutations::update_site` | Yes — `SetSite{...}` | none |

**Blocking prerequisite for the two window fields and the two surface fields:** add a
`diff_fenestrations` step to `model_edit` (mirroring `diff_materials`) covering at minimum
`u_value_w_m2k`/`shgc`/`vlt` (and ideally the rest: `area_m2`, `height_m`, `sill_height_m`,
`frame_conductance_w_k`, `divider_conductance_w_k`, `overhang_depth_m`/`offset_m`, `fin_depth_m`/`offset_m`,
`glazing_construction_id`), and stop the `probe.fenestrations = model.fenestrations.clone()` shortcut
from masking it (change that line to compare rather than copy, once the new diff exists — same
"probe re-assembles every OTHER field, then asserts equality" discipline `diff_materials` already
uses for material identity). Surface class/boundary need NO new mutation or diff step — only the new
editor command + args_bridge arm + manifest classification + publication-contract row.

`SetMaterialProperty`'s exact shape (`🦀️.rs:753-780`) is the template for both new commands:
```rust
fn set_material_property(material: &mut Material, property: &str, value: f64) -> Result<&'static str, Fault> {
    let positive = |value: f64| value > 0.0;
    Ok(match property {
        "thicknessM" if positive(value) => { material.thickness_m = value; "change-material-thickness" }
        "conductivityWMK" if positive(value) => { material.conductivity_w_m_k = value; "change-material-conductivity" }
        ...
        _ => return Err(Fault::new(...)),
    })
}
```
`SetFenestrationProperty`/`SetSurfaceProperty` would follow the identical `match property { "x" if
<SI-range check> => {...}, ... }` shape, each returning the `&'static str` kind string `model_edit`
uses only for its error message on an identity-change refusal (the kind string plays no role in which
mutation is actually emitted — that's the diff functions' job).

---

## 5. Minimal recipe for 🔋️energy

Ordered, each step buildable/testable independently; files to create are under the energy editor's
`📌️panels/` (new directory) and `🎮️commands/` is NOT the right place for these — energy has no
`🎮️commands/` subdirectory pattern like fem2d (its commands are inline `match` arms in the one editor
root file, not one-file-per-command). Two paths: (A) keep energy's existing "one big `reduce()` match"
style and just add arms + diff steps (least churn, matches the file's own convention), or (B) start
splitting into fem2d's per-command-file style now. Given energy's current file is a monolith and
nothing else in it uses per-file commands, **(A) is the lower-risk, precedent-consistent choice** for
the *command* half; panels are new regardless and naturally get their own directory (fem2d's own
convention, no monolith option exists for panels).

1. **Interaction domain** — new `✏️editor/🕹️interaction/🦀️.rs` (mirrors fem2d's file):
   `ENERGY_MODEL_INTERACTION_DOMAIN`, granularity constants (`zone`, `surface`, `fenestration`,
   `material`, `construction`, `thermostat`), `energy_model_interaction_definition() ->
   InteractionDefinition`, `EnergyModelInteractionSnapshot { selected_ids, hovered_ids }` +
   `from_interaction`, a resolver `energy_entity_kind(model, id) -> Option<&'static str>` (id is
   `EntityId.0.to_string()`; walk zones/surfaces/fenestrations/materials/constructions/thermostats in
   fixed precedence, same shape as `fem2d_entity_kind`). Unit test: one law per granularity, resolver
   precedence, round-trip of the `interactionSelect` wire contract (decode with
   `serde_json::from_str::<Vec<protocol::InteractionTarget>>`, per fem2d's
   `select_and_hover_effects_carry_the_framework_wire_contract`).
2. **Controller id + action helper + ui helpers** — additions to `✏️editor/🦀️.rs` (not a new file):
   `ENERGY_MODEL_PLAY_CONTROLLER_ID`, `energy_model_action()`, `ui_label()`, `ui_node_list()` (port
   verbatim from fem2d `🦀️.rs:1148-1164`).
3. **`diff_fenestrations`** — new function in `✏️editor/🦀️.rs` beside `diff_surfaces`/`diff_materials`,
   called from `model_edit`; fix the `probe.fenestrations` line so an un-diffed field change still
   faults via `kind_unavailable` rather than vanishing. Unit tests: one per new field (value applied,
   mutation emitted, identity-change still refused), plus a regression test that a `u_value` edit
   WITHOUT this diff step (i.e. today's code) is what motivated the fix — assert `Emit.artifact_mutations`
   is non-empty after a fenestration property edit.
4. **Two new commands** — add to the existing `EnergyModelEditorCommand` enum + `reduce()` match +
   `args_bridge::command_from_action` + `ENERGY_MODEL_RETAINED_TOOL_IDS` +
   `ENERGY_MODEL_DOCUMENT_TOOL_IDS` + manifest `action_interactive_job(..., Migrated)` +
   `ArtifactToolPublicationContract` row (`Artifact` lane) + a `#[dsl(key = "...")]` action id:
   - `SET_SURFACE_PROPERTY_ACTION_ID = "set-surface-property"` →
     `SetSurfaceProperty { surface: u32, property: String, value: String }` (value is `String` because
     `class` is an enum spelling and `boundary` needs a discriminator + optional partner id — unlike
     `SetMaterialProperty`'s `f64`; parse per-property inside the handler, mirroring `patchSupport`'s
     enum-spelling parse in fem2d). Properties: `"class"` (parse to `SurfaceClass`), `"boundary"`
     (parse to `OutsideBoundaryKind` + optional `partnerSurface` arg for `Interzone`).
   - `SET_FENESTRATION_PROPERTY_ACTION_ID = "set-fenestration-property"` →
     `SetFenestrationProperty { fenestration: u32, property: String, value: f64 }`. Properties:
     `"uValueWM2K"`, `"shgc"`, `"vlt"` (fraction-bounded like `thermalAbsorptance`), extend later for
     the other seven `Fenestration` fields once the diff step covers them.
   Unit tests: four shapes per command, same discipline as fem2d's `patch*` tests (§2.5) — value
   applied end to end, mutation emitted directly, refusal triple (unknown property / unparsable value
   / missing entity), unchanged-value no-op.
5. **`📌️panels/🗿️artifact/🦀️.rs`** (new dir+file) — `BODY_KEY = "energy.model.play.artifact"`,
   `definition()`, `build_artifact_tree` over zones/surfaces/fenestrations/materials/constructions/
   thermostats (6 sections, smaller than fem2d's 9 — max-min-fair `section_quotas` is still worth
   reusing verbatim, it generalizes to any section count), rows keyed by `entity.id.0.to_string()`,
   `.interaction_domain(ENERGY_MODEL_INTERACTION_DOMAIN)`. Unit tests: one per section (counts, dangling
   refs — e.g. a surface whose `construction_id` doesn't resolve — `dimmed`), paging/continuation-row
   law, an oversized-document law, empty-document placeholders, the manifest body-key law.
6. **`📌️panels/🔍️inspection/🦀️.rs`** (new dir+file) — `BODY_KEY = "energy.model.play.inspection"`,
   `ui::column`/`ui::section`/`ui::field` body from the start (never `PanelTreeBuilder` — §2.1's trap),
   one section per selected entity kind with `number_row`/`select_row`/`slider_row`/`toggle_row`/
   `text_row` bound via `bind()`→`Trigger::Change`→`energy_model_action(...)`, action-group
   Focus?/Delete? buttons via `Trigger::Activate` if energy wants row verbs (energy's zones/surfaces
   already have `DeleteZone`/`DeleteSurface`; there's no viewport to "focus" into since energy has no
   Canvas2d/3d window with camera control today — Focus may not apply here at all, worth confirming
   with whoever owns the 3d viewer before adding it). Document summary fallback when nothing selects.
   Unit tests: per-entity-kind field-binding laws (control type + bound action + field name, per
   fem2d's `node_at`/`component_at` walker pattern), the multi-selection header, the `!carries_a_tree`
   law, the body-key route.
7. **Manifest wiring** (`create_energy_model_editor`, `🦀️.rs:1324-1349`): add
   `.interaction(energy_model_interaction_definition())`, `.window_kind_interactions(...)` for whichever
   windows should support a pick (probably none today — energy has no Canvas2d/3d window; the domain
   still needs declaring for the tree/inspector to share a selection even with zero canvas producers),
   `.panel_tab_def(artifact_panel::definition())`, `.panel_tab_def(inspection_panel::definition())`.
8. **`render_body`/`match body_key`** (`🦀️.rs:1258-1264`): add the two new arms, threading
   `EnergyModelInteractionSnapshot::from_interaction(interaction)` the way fem2d's `render_body` does —
   this requires energy's `render()`/`render_body` to stop ignoring `_interaction` and become two
   functions (typed + interaction-aware) like fem2d's, or thread it straight through if energy's
   `Editor::render` signature already receives an `InteractionView` (check the trait signature before
   assuming a split is needed — fem2d needed the split because its `Editor::render` is called from two
   call sites with/without live interaction; verify whether energy's harness has the same duality).

**Tests the precedents keep beside each piece** (apply throughout): a `#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;` at the bottom of every new file (panel or command), 4 shapes per patch-style command
(applied end-to-end / mutation-emitted-directly / refusal-triple / unchanged-no-op), one panel law per
entity-kind section plus paging/placeholder/oversized-document laws, and — new for this editor — a
regression test proving the fenestration diff gap is closed (§5 step 3) before wiring the two window
fields into the inspector, or the inspector's own "value applied end to end" test for those two fields
will pass its Emit-shape assertion but the field will never actually reach the document.

## 6. Traps to carry over explicitly

1. **Never build an inspector body from `PanelTreeBuilder`.** `ui::column`/`ui::section`/`ui::field`
   from the start (§2.1).
2. **Keep row actions off high-multiplicity tree rows.** One pick action per row; put verbs
   (focus/delete) in the inspector as one grouped action row, not per tree row (§1.2, §2.6) — the
   argument arena (`UI_VALUE_ROW_COLLECTIONS`/`UI_VALUE_ROW_ITEMS`) is process-wide and shared with
   every other panel rendered at the same time.
3. **`model_edit`'s probe-clone pattern silently swallows undiffed fields.** Any new mutable field a
   command touches on the model clone MUST have a matching diff step, or the edit disappears with no
   error (§4.3) — this is worse than fem2d's loud `kind_unavailable`/refusal-triple discipline for the
   exact same class of mistake.
4. **`UiMapBuilder`/argument maps require strictly ascending keys** on `.push()` — `field` before `id`
   before `value` alphabetically is not a style choice, it is enforced.
5. **`.selected()`/`.highlighted()` on `PanelTreeBuilder` don't currently paint onto the built tree** —
   don't expect visual selection highlighting on tree rows from this call alone; it's future wiring
   (§1.3).
6. **A viewport-pick-style `focusEntity` verb presumes camera/window addressing that energy may not
   have** — check whether energy has any camera-bearing window before porting it (§5 step 6).
