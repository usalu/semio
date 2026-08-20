# 📓️ terra-sdk-helpers

## done

Converted every genuinely-node-typed internal helper `sdk-flip` deliberately left broken in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (the guest SDK, `pub mod app`) from the
old `ui_wgpu::wgpu` vocabulary (`UiNode`/`UiPresence`/`UiControlNode`/`ActionDescriptor`/…) to
`semio_framework_ui_contract`'s `ui::*` builder DSL / `BuiltNode`, plus the `semio_framework_ui_runtime`
boundary (`ComponentTree`/`TreeNode`) that `Present`/`ArtifactApp::render`/`PluginApp::render`/
`ArtifactEditor::render`/`ArtifactViewer::render` all require. Also converted the fully-owned
declaration-inventory doc comment work and every SDK-internal test that exercised these helpers.

**New shared helpers added** (region `🔖️TreeConvert`, right after the crate's import block):
- `built_to_tree(BuiltNode) -> TreeNode` and `built_to_component_tree(BuiltNode) -> ComponentTree` —
  the recipe's §5 fold, written once and reused at every `render()`/`Present` boundary in this file
  instead of duplicated per call site.
- `dsl_value_to_ui_value(DslValue) -> UiValue` (region `🔖️ActionFactory`) — a recursive structural
  conversion the recipe doesn't mention at all: `ActionBinding.args` is `Option<UiValue>`, but several
  SDK helpers still receive action args as the kernel's `DslValue` (`Keybinding`-adjacent code,
  `ActionFactory`, `ui_history_panel`'s revert action). Both enums are structurally identical closed
  JSON-value shapes; this is a straight field-for-field fold.

**Functions/types converted** (all now return `BuiltNode` unless noted, matching recipe §7's own
worked example — the runtime boundary conversion happens once, at the actual `render()`/`Present::
present` return site, never inside a helper):
`tree_item`, `tree_item_desc`, `tree_item_with_action`, `tree_item_with_action_draggable`,
`PanelTreeBuilder` (all methods), `FormPanelBuilder` (all methods, renamed field types), `entity_detail`
(follows recipe §7 verbatim), `ActionFactory::action` (now returns `(ActionId, Option<UiValue>)`),
`ui_history_panel`, `stamp_and_cache_interaction_ui` (now takes `&mut ComponentTree`, delegates to a
new `stamp_and_cache_interaction_ui_node(&mut TreeNode, ..)` for the uniform-recursion walk),
`ui_tree_domain_topology`/its inner `visit` (now walks `&[TreeNode]`), the `VcsArtifactApp`-wrapper
`PluginApp::render` impl (`Result<UiNode, Fault>` → `Result<ComponentTree, Fault>`), `ArtifactEditor::
render`/`ArtifactViewer::render` trait declarations AND their generic delegating-wrapper impls (both
were still `-> UiNode`, sdk-flip's report only mentions flipping `ArtifactApp`/`PluginApp`), plus every
`#[cfg(test)]` `render()` fixture across the file (`DummySnapshot`/`TxnSnapshot`/`SurfaceSnapshot`
(×2)/`TestSnapshot`, the `fixture_channel!` macro's two generated `render()`s).

**`WindowKit` family — deliberately NOT converted, see "ambiguities" below.**

**Registrar-adjacent bug found and fixed, not requiring a registrar**: `pub mod plugin_runtime` (a
sibling of `pub mod app`, not nested inside it) calls `plugin_render`/`plugin_render_with_document`
which sdk-flip's own report claims were flipped to `Result<ComponentTree, Fault>` — true for the
signatures, but `ComponentTree` was never actually imported into that module's own `use` block, so the
name was unresolved. Added `use semio_framework_ui_runtime::ComponentTree;` there. Also dropped that
module's now-dead `UiNode` import (unused after the flip; the only remaining reference was a comment).

## acceptance: UNRUN (per U4 — the coordinator runs cargo)

Cheap checks only, no `rustc`/`cargo` invoked:
- Every edit re-read from disk immediately before editing (collision rule) via fresh `grep -n`/`sed -n`
  right before each `Edit`/splice — line numbers were re-derived every time, never cached across edits,
  because the peer async-fixup pass was live-editing this same file throughout (confirmed: `git log
  --oneline -3` for this file went `6cf8d6c858 545` → `bd1ce10b9b 546` mid-session, i.e. the auto-commit
  bot swept up a peer's pending work plus mine partway through; working-tree edits survived intact).
- Whole-file brace/paren balance check (`{`/`}` and `(`/`)` counts, python) after all edits: both
  balanced (5279/5279 braces, 17201/17201 parens).
- **Self-check grep, exactly as U4/the packet brief require** — every old symbol, negative-lookbehind
  excluding legitimate `ui_wgpu::wgpu::`-qualified survivors, run against the CURRENT file:
  ```
  UiNode: 7 (all `///`/`//` prose)          UiTreeItemNode: 2 (prose)
  UiPresence: 1 (prose)                     ActionDescriptor: 13 (prose + the world3d module's own
  UiTreeSectionNode: 1 (prose)                explicit `use ui_wgpu::wgpu::{.., ActionDescriptor, ..}`,
  UiPeerMark: 0                               untouched — see "deviations")
  UiKeyValueEntry: 0                        UiControlNode: 2 (prose)
  UiButtonNode: 0                           UiSectionNode: 1 (prose)
  ui_text: 0                                UiTreeNode: 0
  UiSelectItem: 0                           UiFieldNode: 0
  UiInputNode: 0                            UiKeyValueNode: 0
  UiSelectNode: 0                           UiTreeItemAction: 0
  UiTreeActionPlacement: 0                  ui_stack_vertical: 0
  ui_tree_stamp_presence: 3 (prose)         ui_control_to_node: 0
  UiState: 0                                UiMenuRef: 3 (all legitimately kept — see below)
  ```
  Every surviving hit is inside a `///`/`//` comment or a place I deliberately kept the old type on
  purpose (documented per-site below) — zero remaining PRODUCTION references to a removed symbol.
- Manually traced every builder-chain call site against `🦀️builder.rs`'s actual signatures (`HasBase`/
  `HasChildren`/`HasStackLayout`/`Buildable` trait methods, each builder's own inherent methods) rather
  than assuming the recipe's prose — e.g. confirmed `ui::tree_section`/`ui::field`/`ui::section` always
  wrap their label in `Some(..)` (no optional-label constructor exists), which drove the "empty Label
  for a `None` label" decisions below.
- Traced `TreeNode`/`ComponentTree` field-for-field against `🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️present.rs`
  directly (not from memory) before writing `built_to_tree`.

## peer-coexistence

Collision rule followed throughout: re-read from disk before every edit, region-scoped edits only,
never a wholesale rewrite. No peer work was found overlapping any region I edited — every hunk I
produced traces to my own edits. The file is under heavy concurrent churn from the sibling
MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME program's async/await-fixup pass (confirmed via `git diff
--stat` showing 500-1000+ line diffs at multiple points during this session, `git log` advancing by one
commit mid-session) — I did not touch, read into, or need to absorb any of it; my regions (imports,
`PanelKit`/`FormKit`, `ActionFactory`, `HistoryPanel`, `stamp_and_cache_interaction_ui`, `WindowKits`,
the `ArtifactEditor`/`ArtifactViewer` trait declarations + their `#[cfg(test)]` fixtures, `plugin_runtime`'s
import block) never coincided with theirs.

## the presence decisions (`UiPresence`, 18 sites budgeted)

Every site fell into one of two buckets — I did not find a genuine `Hidden`/`Introducing`/`Celebrating`
site among them (the fleet's own `UiPresence::state(UiState::Disabled)` calls were the only non-default
uses):

1. **`state: Disabled` → `record.disabled`.** `ui_history_panel`'s `action_item` closure used to stamp
   `UiPresence::state(UiState::Disabled)` on BOTH the tree item and its inline button control when
   `!enabled`. Both destinations exist directly on the new record (`HasBase::disabled(bool)`) — I set
   `.disabled(!enabled)` on both the `tree_item` and its `button` child, faithfully preserving the old
   dual-stamping.
2. **`hover`/`selected`/`color`/`peers` → the `PresenceUpdate` channel, which this SDK layer cannot
   publish this wave.** This is the load-bearing case, and it is a REAL regression, not a silent
   approximation:
   - **`PanelTreeBuilder::selected()`/`.highlighted()`** (§6 of the recipe names this exact helper by
     name as the case with no safe mechanical translation). I kept both methods on the builder — they
     still record `selected_ids`/`highlighted_ids` — but `.build()` no longer stamps anything with them,
     because there is nowhere left to stamp. Documented in the struct's own doc comment; a future packet
     wiring a real per-app `Present`/`HandleIntent` + `transact()` can read these straight off the
     builder to publish the equivalent `PresenceUpdate`.
   - **`stamp_and_cache_interaction_ui`** (the framework-owned override of the above, for
     `interaction_domain`-bound trees) — same story. It used to compute `selected`/`hovered` sets from
     `protocol::InteractionState` plus merge `peers_selecting`/`peers_hovering` into `UiPeerMark`s and
     stamp all of it onto the tree via `ui_tree_stamp_presence`. I kept the OTHER half of this function
     (topology caching into `interaction_ui_topology`, which is pure document-shape reading, not
     presence) working, on `TreeNode`'s uniform `children` recursion — much simpler than the old
     per-variant `Stack`/`Section`/`Group`/`Field` match arms, since every component nests the same way
     now. The presence-stamping half is gone; `state: &protocol::InteractionState` is still threaded
     through the signature (a future packet needs it right there) but nothing currently reads it.
   - `InteractionView::peers_selecting`/`peers_hovering` themselves are untouched and still directly
     tested — only their one caller (the removed `marks_for` closure) is gone; updated that method's
     stale doc comment accordingly.

## `UiPeerMark` (4 sites)

All 4 were inside the same `stamp_and_cache_interaction_ui` presence-stamping block covered above (the
`marks_for` closure that built `UiPeerMark`s from `peers_selecting`/`peers_hovering`) plus one test
(`ui_tree_stamping_replaces_app_supplied_presence_from_interaction_state`, renamed
`..._caches_interaction_topology_from_a_domain_bound_tree`) asserting on the stamped result. Same
destination as above: `PeerMark` (contract's presence-channel type, `🦀️presence.rs`) is real and
correctly shaped, but publishing one needs the same `transact()`/`PresenceHub` infrastructure this SDK
layer doesn't have. The rewritten test keeps the peer-roster fixture setup (documenting the shape a
future `PresenceUpdate`-publishing test will need) but no longer asserts on stamped peer marks — it now
proves topology caching only.

## ambiguities I could not resolve mechanically (flagged, not guessed silently)

**`WindowKit` (7 kit impls sharing one trait method) — deliberately left on `ui_wgpu::wgpu::UiNode`,
fully qualified rather than converted.** Three of the seven kits (`TextWindowKit`, `TableWindowKit`,
`MeshWindowKit`) build a `ComponentScene`-shaped payload (`ui_wgpu::wgpu::build_text_editor_scene`/
`build_table_scene`/`build_world_3d_scene`). The contract's own `Component::Surface(SurfaceProps)` needs
a pack-encoded `SurfaceDoc.bytes` payload whose product-specific scene structs (`TextEditorScene`/
`TableScene`/`World3dScene`, …) "move to `🖱️ui/🎬️scene/🦀️component.rs` in a later packet" per
`🦦️contract/🦀️surface.rs`'s own header — that crate doesn't exist yet, and `🦀️surface.rs` itself is
still an unfinished SCAFFOLD (its own file header says so). Recipe §2 names exactly this case as out of
its depth. Since `WindowKit::render` is ONE trait method shared by all seven impls, it needs ONE return
type — the four kits that individually COULD convert today (`TreeWindowKit`, `ImageWindowKit`,
`DocumentWindowKit`, `MediaWindowKit`) are held back by the other three rather than half-migrating the
trait. I fully-qualified every bare old-vocabulary reference in this ~600-line region (`UiNode`,
`UiTreeItemNode`, `UiTreeSectionNode`, `UiTreeItemAction`, `UiTreeActionPlacement`, `ActionDescriptor`,
`Label`) with `ui_wgpu::wgpu::` rather than leave the local import shims that would silently un-flip the
module's own glob if copy-pasted elsewhere, and added a doc comment on the trait explaining the gap.
**Flagging to the coordinator: worth its own follow-up packet once `contract-layout`'s `SurfaceProps`
and the scene crate exist** — `TreeWindowKit`/`ImageWindowKit`/`DocumentWindowKit`/`MediaWindowKit`
could convert today in isolation if the trait were split.

**`ActionFactory::action`'s return shape.** The old signature bundled a controller-scoped id and
optional args into one `ActionDescriptor` a caller assigned straight to a node's sole action field. The
new model splits "what to call" (`ActionId`) from "when" (`Trigger`, chosen wherever `.on`/`.on_with` is
actually called) — a factory scoped to one controller cannot know the trigger. I chose `(ActionId,
Option<UiValue>)` over silently dropping `args` capability or inventing a `Trigger` parameter this
factory has no way to source correctly. Documented in its own doc comment as the affected packets'
"~30 hand-rolled wrapper" call sites (in the 33 fleet crates) will all need the same reshaping.

**`tree_item_with_action`/`tree_item_with_action_draggable`/`FormPanelBuilder::submit` — narrowed from
`ActionDescriptor` (id + optional args) to a bare `ActionId` (id only, bound `Trigger::Activate`, no
args).** None of this file's own call sites needed args on these three helpers. A fleet caller that does
need args can build the node directly via `ui::tree_item(label).id(id).on_with(Trigger::Activate,
action, args)` instead of the convenience wrapper. Documented at each site.

## recipe amendments made (`📓️recipe-plugin.md` NOT touched — see below)

I did **not** amend `📓️recipe-plugin.md` itself (edits there are within my OWNS per the packet brief,
but I judged the risk of colliding with a fleet-migration packet already reading it mid-flight higher
than the value of a same-session edit — recorded here instead for whoever next touches it):
1. **Missing: `DslValue` → `UiValue` conversion.** The recipe's §1 covers the `Trigger`/`ActionId`
   decomposition in detail but never mentions that a plugin's OLD `ActionDescriptor.args` was
   `Option<DslValue>` (the os-kernel's value type) while the new `ActionBinding.args` is
   `Option<UiValue>` (the contract's neutral value type) — two structurally-identical but NAME-different
   enums. Every plugin that ever passed real args through an action will hit this. Worth a short §1.5
   with the fold I wrote (`dsl_value_to_ui_value`, this file's `🔖️ActionFactory` region) as the
   reference implementation.
2. **Missing: optional-label constructors don't exist.** `ui::section`/`ui::field`/`ui::tree_section`
   all require a label (wrap it in `Some(..)` unconditionally) — there is no "labelless section" builder
   path even though `ContainerProps.label`/`TreeSectionProps.label` both stay `Option<Label>` on the
   record. Every old `UiSectionNode { label: None, .. }`/`UiTreeSectionNode { label: None, .. }` site
   needs an explicit decision (empty `Label` vs. a different `ContainerRole`/component entirely — I used
   `ui::column()` (`Plain`) for an unlabeled `Section`-shaped wrapper and empty-`Label` for an unlabeled
   `TreeSection`, see `FormPanelBuilder::build`/`PanelTreeBuilder::section`'s doc comments for the
   reasoning in each case) — worth a short note under recipe §2's `Stack`/`Section`/`Group`/`Field` row.
3. **`WindowKit`-shaped "one trait, many impls, some scene-blocked" is a real pattern, not a one-off.**
   Worth a short addendum to §2's `ComponentScene` row warning that a shared trait method forces
   ALL its impls onto the slowest-to-convert one, so an agent should check for this before starting a
   partial conversion.

## registrar-requests

None beyond the one `sdk-flip` already filed (swap `ui_wgpu` for `ui_contract`/`ui_runtime` in
`🔌️plugin/📦️packages/🦀️rust/Cargo.toml`) — still outstanding, still correct, not re-filed here.

## deviations

- Did not touch `📓️recipe-plugin.md` — see "recipe amendments" above for why, and what it should say.
- Did not convert `WindowKit`/its 7 impls — see "ambiguities" above; deliberately fully-qualified to the
  old vocabulary instead, flagged to the coordinator.
- Did not touch the world3d module's own `use ui_wgpu::wgpu::{.., ActionDescriptor, MeasureSelectItem,
  WindowMeasure, World3dScene};` (line ~19891) or its two `world3d_sun_measures`/
  `world3d_projection_measures` functions — these build `WindowMeasure`s (a kept non-node type) via
  closures typed `impl Fn(&str, Option<Value>) -> ActionDescriptor`; that `ActionDescriptor` is the
  SAME kept-old type `MeasureSelectItem` (also from `ui_wgpu::wgpu`, untouched) itself still requires
  internally — not a node-type migration site, no change needed.
- Added `ui_wgpu::wgpu::ActionDescriptor as KeybindingActionDescriptor` and bare `ui_wgpu::wgpu::
  UiMenuRef` to the crate's top-level non-node explicit-import block (previously only
  `LocalizedLabel`/`WindowMeasure`/`Locale`/`Terminology`/`ContextMenu*`/`WindowEngagement`/
  `NamedLayout`) — both are forced old-typed by OTHER kept-old or FORBIDDEN-file types
  (`manifest::Keybinding.action`, `ContextMenuRequest.menu`) that this packet's `OWNS` cannot touch, not
  genuine node-type migration sites. Documented in-code at the import site in both cases.
- Did not embed the full `ui_runtime::UiRuntime`/wire real `PresenceUpdate` publishing anywhere — out of
  this packet's `OWNS` for the same reason `sdk-flip` didn't (needs per-app `Present`/`HandleIntent`,
  fleet-domain design work).

## files touched

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` (surgical, region-scoped — import
  block, `🔖️TreeConvert` (new), `🔖️InteractionArgs`, `🔖️PanelKit`, `🔖️FormKit`, `🔖️ActionFactory`,
  `🔖️HistoryPanel`, `🔖️InteractionDispatch`/`stamp_and_cache_interaction_ui`, the `VcsArtifactApp`
  wrapper's `render`, `ArtifactEditor`/`ArtifactViewer` trait declarations + generic delegating impls,
  `🔖️WindowKits` (fully-qualified, not converted — see ambiguities), the `fixture_channel!` macro and
  its `declarations::fixture` import block, `plugin_runtime`'s import block, and every `#[cfg(test)]`
  module that exercised any of the above)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY/📓️terra-sdk-helpers-report.md`
  (new, this file)
