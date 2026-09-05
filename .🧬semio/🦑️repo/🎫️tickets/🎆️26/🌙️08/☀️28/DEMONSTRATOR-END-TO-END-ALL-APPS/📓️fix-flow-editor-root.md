# 📓️ Fix compile errors — `semio-s-plugin-flow`, editor root (`✏️editor/🦀️.rs`)

Scope: exactly one file —
`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`.
No `cargo` was run (per instructions); every fix below was verified by reading the definitions it
depends on and cross-checking against clean-compiling sibling code (`cad`, `puzzle/2d`). 39 errors
were attributed to this file by a block-split of the coordinator's log
(`scratchpad/flow-process.txt`); 36 are fixed, 3 remain — genuinely blocked by files outside this
ticket's ownership (see §4).

## 1. `FlowMutation`/`Widget::id` families — NOT actually in this file

The task brief's families 1 and 3 (`FlowMutation::Widgets/Synapses/SetLayout/SetFixture` variants,
`Widget::id()`) describe errors that live in
`🧬️schema/🧬️mutations/🦀️.rs`, `➕️create-widget/🦀️.rs`, etc. — a different crate file, owned by
another agent, mid-edit throughout this session (confirmed via the tool's own "changed on disk"
notice). A precise `grep -n "✳️any/✏️editor/🦀️.rs:[0-9]"` against the log shows **none** of those
diagnostics point at my file — this file already used `crate::artifacts::flow::schema::mutations::…`
(its OWN flat `FlowMutation` enum: `CreateWidget`/`DeleteWidget`/…/`DuplicateWidget`), never the
framework's `flow::FlowMutation`. Family 2 (`Label` duality) likewise lives only in
`✏️editor/📌️panels/🛍️catalogue/🦀️.rs` and `.../📄️artifact/🦀️.rs` — both under the excluded
`✏️editor/📌️panels/` tree. Nothing to do here; no changes made outside my file.

## 2. Real errors in this file, and the fix for each (36/39)

**a. `::mutation::X` import paths (9× E0432, lines 13–21).** The mutation-leaf files
(`🔗️connect-widgets/🦀️.rs` etc.) no longer nest a `mod mutation` — `ConnectWidgets` etc. sit
directly in `connect_widgets`. Confirmed by reading `🔗️connect-widgets/🦀️.rs:1-20` (struct is
top-level) and `find`-listing the `🧬️mutations/` tree (no `mutation` subdirectory anywhere).
Fix: dropped `::mutation` from all 9 `use` lines.

**b. Missing `Arc` (3× E0425/E0433, lines 1270/1361/1409).** `Arc<FlowHostEffectPayload>` /
`Arc::ptr_eq` / `Arc::new` used with no import. Added `use std::sync::Arc;` to the import block.

**c. `usize`→`u64` at `ToolExecutionContract::resumable` (E0308, was 1188:93).**
`semio_framework::ToolExecutionContract::resumable`'s 3rd param (`max_work_units_per_step`) is `u64`
(`🎯️action-bus/🦀️.rs:248`); `FLOW_STORE_MAX_MUTATION_ITEMS` stays `usize` (compared against
`.len()` everywhere else in the file — cannot widen the const). Fixed with
`FLOW_STORE_MAX_MUTATION_ITEMS as u64` at the one call site that needed it.

**d. `flow_fixture_operations(...)` now fallible (E0631 + E0599 `collect`, was 1798).**
`flow::flow_fixture_operations` (`🌿️vcs/🦀️.rs:188`) returns
`MutationApplyResult<Vec<FlowMutation>>` now (was bare `Vec<…>`), so `.into_iter()` iterated the
`Result` (0-or-1 `Vec` items) instead of the mutations. `host_operations`'s own doc already commits
to "returns an empty vec" on a no-op; extended that to the diff-failure case too:
`.unwrap_or_default().into_iter().filter_map(…).collect()`.

**e. `Emit`/`Result<Emit,_>` "not a future" + wrong-shaped ref args (7× E0277/E0308, was
1318–1320).** `evaluate::evaluate_result`, `flow_eval_tick::handle`, `flow_eval_resolve::handle` (all
three under `✏️editor/🎮️commands/`, which I own transitively via read-but-not-exclude) are now plain
sync `fn`s returning `Emit<…>` / `Result<Emit<…>, Fault>` directly — no longer futures. The old code
wrapped every one in `semio_framework_plugin::resolve_ready(...)`, which now demands a `Future` arg.
Fixed by calling them directly: `Ok(evaluate::evaluate_result(...))`,
`flow_eval_tick::handle(&command, ...)`, `flow_eval_resolve::handle(&command, ...)`. The `&command`
was rustc's own suggested fix for the arg-type mismatch (`expected &FlowEvalTick, found FlowEvalTick`)
— applied verbatim since `&&FlowEvalTick` deref-coerces to `&FlowEvalTick` regardless, so it's safe
even if my own match-ergonomics reasoning about why the un-referenced form fails is incomplete.

**f. `.action_with(...)` given a `Future` instead of `ActionDefinition` (6× E0308, was
1861/1865/1877/1885/1886/1887).** `ActionDefinition::with_category`/`.category` are `async fn` now
(`🛂️manifest/🦀️.rs:1050/1056`), but `EditorBuilder::action_with` is a **sync** macro-generated
wrapper (`🔌️plugin/🦀️.rs:26913` `surface_builder_forward!`) that itself does
`resolve_ready(self.inner.action_with(...))` internally — it wants a plain `ActionDefinition`, not a
future. `cad`'s `create_cad_app()` (its own render/menu code already compiles) shows the fix:
wrap the `.with_category(...)` tail in `semio_framework_plugin::resolve_ready(...)` before handing it
to `.action_with(...)`. Applied to all 6 sites (`deleteSelection`, `reorganize`, `focusSelection`,
`setPreviewOff`, `openSpotlight`, `replaceImage`).

**g. Context-menu `Menu` builder now fully async (8× E0599/E0277, was 244–296, plus the
`selection_count_phrase` follow-on).** `semio_framework_plugin::Menu::of/.action/.item/.group/.build`
are all `async fn` now (`🔌️plugin/🦀️.rs:12203-12341`), and `selection_count_phrase` too
(`🔌️plugin/🦀️.rs:12339` area). Rewrote `flow_context_menu_items` to mirror cad's
`context_menu` (`✏️s/📐️cad/…/✏️editor/🦀️.rs:1972`, `resolve_ready(async { Menu::of(...).await… })`):
the whole body now runs inside `semio_framework_plugin::resolve_ready(async { … })`, with `.await`
after every `Menu::of`/`.item`/`.action`/`.group`/`.build` call and after `selection_count_phrase`.
The `.group(...)` closures were left untouched — `group`'s signature
(`F: FnOnce(Menu<'a>) -> Fut, Fut: Future<Output = Menu<'a>>`) is satisfied by the closures' existing
un-awaited tail call (`m.action(...)`/`m.item(...)` returning `impl Future<Output = Menu>` directly).

**h. Two arms of the `render()`/`render_with_instance_operation_owner()` dispatch (part of the
blocked family, §4, but partially fixable here).** `document_panel::render` and
`catalogue_panel::render` (`✏️editor/📌️panels/📄️artifact/🦀️.rs`,
`.../🛍️catalogue/🦀️.rs`) already return `UiAssemblyResult<BuiltNode>` (migrated). Converted both
call sites (both match blocks) with `.map(semio_framework_plugin::built_to_component_tree)`
(`🔌️plugin/🦀️.rs:334`, the same helper `cad`/`puzzle-2d` use). Replaced the `_ => ui_text(...)` arms
with `semio_framework_plugin::built_text_to_component_tree(Label::data(...))`
(`🔌️plugin/🦀️.rs:353`) — matches `cad`'s pattern exactly. `ui_text`/`ActionDescriptor`/`UiNode` are
now genuinely unused in this file (verified with `grep`, including test code) and were dropped from
the `semio_framework_plugin::{...}` import list.

## 3. `Label` duality (family 2) — confirmed not applicable

`semio_framework_plugin::Label` (imported at the top of this file, used only at the two
`built_text_to_component_tree(Label::data(...))` call sites from §2h) resolves to
`ui_wgpu::wgpu::Label` via that crate's top-level `pub use ui_wgpu::wgpu::*;`
(`🔌️plugin/🦀️.rs:37410`) — the SAME type `built_text_to_component_tree`'s parameter wants
(`ui_wgpu::wgpu::Label`, per `🔌️plugin/🦀️.rs:353`), and the same type `cad`'s working
`context_menu`/`create_cad_app` use. No conversion needed, no ambiguity in this file. The dual
`semio_framework_plugin::plugin_app_close_prelude::Label` (the contract-crate one) never appears here.

## 4. Unresolved — genuine cross-file blockers (3/39), left untouched

**i. `retained::artifact::preparation::PreparationFactory` is unreachable (E0603, line ~1589).**
`editor/🧵️retained/🗿️artifact/🦀️.rs:17` declares `pub(super) mod preparation;` — visible to
`retained` and its descendants only. `editor` (this file, the *grandparent*) is outside that reach;
Rust privacy does not grant ancestor modules access via a descendant's `pub(super)`. The struct itself
is already correctly scoped for this call site (`📬️preparation/🦀️.rs:10`:
`pub(in super::super::super) struct PreparationFactory;` = `pub(in editor)`) — only the **module
path** is too narrow. Fix needed (not mine to make): widen
`editor/🧵️retained/🗿️artifact/🦀️.rs:17` from `pub(super) mod preparation;` to
`pub(in super::super) mod preparation;`, matching the struct's own declared reach. That file is under
the excluded `✏️editor/🧵️retained/` tree.

**j. `render()`/`render_with_instance_operation_owner()` mix migrated and unmigrated body/panel
renderers (E0308 ×2, one per function).** Ground-truth-checked every renderer this dispatch calls:
- Still returning bare `UiNode` (**unmigrated**, sdk-flip ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`):
  `main::render` (`✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs:83`),
  `compiled::render` (`.../🗣️compiled/🦀️.rs:38`),
  `generations::render` (`✏️editor/🎭️modes/🧬️generate/🪟️windows/🗂️generations/🦀️.rs:35`),
  `form::render` (`.../📝️form/🦀️.rs:38`), `preview::render` (`.../👁️preview/🦀️.rs:38`),
  `inspection_panel::render` (`✏️editor/📌️panels/🔍️inspection/🦀️.rs:30`).
- Already migrated to `UiAssemblyResult<BuiltNode>`: `document_panel::render`, `catalogue_panel::render`
  (fixed at this file's call sites, §2h).

Per the sdk-flip recipe (`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY/📓️recipe-plugin.md`
§5/§8), there is **no sanctioned conversion from the old `UiNode` enum** to `ComponentTree` or
`BuiltNode` — the recipe explicitly says the old enum has to be rebuilt with the `ui::*` DSL
variant-by-variant (§2 of that doc), not bridged. I verified no such bridge exists anywhere in the
repo (`grep -rn` for a `UiNode -> ComponentTree`/`BuiltNode` conversion returned nothing). Writing one
myself would mean re-implementing that whole recipe table for six renderers I don't own, in files
under `✏️editor/🎭️modes/` and `✏️editor/📌️panels/🔍️inspection/` — both explicitly excluded from this
ticket. Left both match blocks exactly as compiler-correct as they can be from this file alone (the
already-migrated arms convert via `.map(built_to_component_tree)`); the six unmigrated arms are a real
gap that needs those six functions themselves flipped to the `ui::*`/`BuiltNode` shape by whoever owns
`✏️editor/🎭️modes/` and `✏️editor/📌️panels/🔍️inspection/`.

## 5. What remains unverified

No `cargo check`/`cargo build` was run (per instructions — a shared lock across parallel agents).
Every fix above was checked by reading the actual current definitions it depends on (framework
`🔌️plugin/🦀️.rs`, `🛂️manifest/🦀️.rs`, `🌿️vcs/🦀️.rs`, the six renderer files, the mutation-leaf
files) and cross-referencing the same shape against `cad`'s and `puzzle/2d`'s already-compiling
`✏️editor/🦀️.rs`. I did not re-run any of the peer schema/mutations edits that were landing
concurrently in `🧬️schema/🧬️mutations/🦀️.rs` during this session — my file's imports of that
module's public API (`FlowMutation`'s flat variants, `from_framework_mutation`'s signature) are
structural and should be stable across that peer's in-flight edits, but this is not compiler-confirmed.
The three blocked items in §4 will still produce compile errors until the owning agents make the
described changes.
