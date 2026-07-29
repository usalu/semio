---
name: Flow Undo Redo Catalogue Restore
overview: Restore the real FlowHost-backed undo/redo and operator catalogue that regressed/were never wired in flow/plugin/rs/lib.rs, matching what commit 465eed3c1 already achieved and what the pre-migration flow/react implementation had.
todos:
 - id: core-history-api
   content: "flow/core/rs/lib.rs: make begin_change pub; add set_fixture_preserving_history; add flow_operator_catalogue_json()"
   status: completed
 - id: plugin-host-restore
   content: "flow/plugin/rs/lib.rs: restore persistent FlowPlayApp{host} + host_for caching; wire set_host_catalogue_json on construction"
   status: completed
 - id: plugin-undo-redo
   content: Route undo/redo commands through host.undo()/redo(); remove undo_fixtures/redo_fixtures/snapshot_fixture
   status: completed
 - id: plugin-snapshot-cleanup
   content: Remove redundant snapshot_fixture calls; add host.begin_change() for moveMediaNode, patchFlowWidgets, renameFlowWidget, setFixture (with set_fixture_preserving_history)
   status: completed
 - id: tests-and-verify
   content: Update FlowPlayApp constructors in tests; cargo test, wasm rebuild, flow E2E (react+wgpu) verification
   status: completed
isProject: false
---

# Flow: Restore Premigration Completeness (Undo/Redo + Catalogue)

## Evidence (from git, as requested)

Comparing the uncommitted working tree of [flow/plugin/rs/lib.rs](flow/plugin/rs/lib.rs) against `HEAD` (`465eed3c1`, whose message is literally "audit full 25-app premigration parity gaps... restoration"):

- `git diff HEAD -- flow/plugin/rs/lib.rs` shows the working tree **reverted** the persistent `FlowPlayApp { host: Option<FlowHost> }` + `host_for()` caching pattern back to a stateless `struct FlowPlayApp;` that rebuilds a throwaway `FlowHost` on every command. This silently switched `"undo"`/`"redo"` from real `FlowHost::undo()/redo()` (backed by `flow/core/rs/lib.rs`'s `FlowHistory` stack, added by the completed [`.cursor/plans/flow_undo_redo_6e12141c.plan.md`](.cursor/plans/flow_undo_redo_6e12141c.plan.md)) back to an ad-hoc `envelope.runtime.undo_fixtures`/`redo_fixtures` `Vec<FlowFixture>` snapshot list — a duplicate, weaker mechanism that also bloats the persisted document JSON with full fixture clones.
- This exact gap is independently documented in [`.cursor/plans/full_25-app_premigration_parity_2bee1ddf.plan.md`](.cursor/plans/full_25-app_premigration_parity_2bee1ddf.plan.md) wave 7: _"catalogue hardcoded ... instead of `host.catalogue_json()`; undo/redo bypasses `FlowHost::undo/redo`"_.
- Separately, comparing to the pre-migration `flow/react` implementation (deleted in commit `40b5caaba`, recovered via `git show 40b5caaba~1:flow/worker-client.ts`), the old app wired `session.setCatalogueJson(...)` (→ `FlowHost::set_host_catalogue_json`) from the registry's module-grouped operators (see completed [`.cursor/plans/flow_catalogue_dnd_2ffa4bf9.plan.md`](.cursor/plans/flow_catalogue_dnd_2ffa4bf9.plan.md)). The current program **never calls `set_host_catalogue_json`**, so `FlowHost::catalogue_json()` only ever returns the 3 built-in static sections (Inputs/Outputs/Contract) — every math/text/logic/dictionary/list/brep/draw/bim operator is missing from the drag-and-drop catalogue today, a real functional loss versus premigration.
- A related, previously-latent bug found while tracing this: `FlowHost::replace_fixture` unconditionally resets `self.history = FlowHistory::default()`. The plugin's `patchFlowWidgets`, `renameFlowWidget`, and `nodeGraphEdit`'s `"setFixture"` operation all call `replace_fixture`, so once undo/redo is routed through real history, every slider drag from the inspector or DSL-driven fixture replace would silently **wipe the entire undo stack**. This must be fixed as part of restoring history, not left as a new trap.

## Scope boundary (explicitly excluded)

The same audit plan also flags an "async eval-bridge/worker path" gap (`set_eval_bridge`, `apply_eval_outputs_json`, `set_computing_progress` unused). Investigation shows `neural_engine`'s `Registry::dispatch` already evaluates all current operators natively and synchronously in Rust, and the entire plugin runtime ([framework/plugin/rs/plugin_runtime.rs](framework/plugin/rs/plugin_runtime.rs)) is synchronous end-to-end for all 25 apps — there is no worker-thread invocation path anywhere in the new architecture to wire into. Re-introducing genuine off-main-thread evaluation would be a cross-cutting plugin-runtime change, not a flow-specific fix, so it is **not** part of this plan.

`.document(["semio", "flow"])` on the `App::builder` call (added in the working tree, not in HEAD) belongs to the separate in-flight "Hierarchical App Identity" ticket and is left untouched.

## Changes

### [flow/core/rs/lib.rs](flow/core/rs/lib.rs)

- Make `begin_change` `pub fn begin_change(&mut self)` (currently private) so the plugin can snapshot single-shot mutations that don't already call it internally (`move_widget`).
- Split `replace_fixture`'s body into a private helper taking a `reset_history: bool` flag; keep `pub fn replace_fixture(...)` (full reset, used for `setDocument`/initial load) and add `pub fn set_fixture_preserving_history(&mut self, fixture: FlowFixture)` (same effect minus the history wipe) for patch-style mutations.
- Add `pub fn flow_operator_catalogue_json() -> String`: builds `CatalogueSection`s (already `pub`, see `#region 🔖Catalogue`) grouped by `OperatorInfo.module` from the same `flow_registry()` used by `evaluate_internal` (core/math/text/logic/dictionary/list/brep/draw/bim), mapping each operator to a `CatalogueItem { kind: "neuron", neuronKind: Some(info.id), name: info.name, abbreviation: info.abbreviation, icon: info.icon, summary: info.summary, .. }`.

### [flow/plugin/rs/lib.rs](flow/plugin/rs/lib.rs)

- Restore `struct FlowPlayApp { host: Option<FlowHost> }` + `host_for(&mut self, envelope) -> &mut FlowHost` (replaces host only when `host.fixture != envelope.fixture`), and have both `host_for` and `host_from_envelope` call `host.set_host_catalogue_json(&flow_core::flow_operator_catalogue_json())` right after construction.
- Remove `undo_fixtures`/`redo_fixtures` from `FlowPlayRuntime` and delete the `snapshot_fixture` helper.
- `"undo"`/`"redo"`: call `host.undo()` / `host.redo()`, sync `envelope.fixture = host.fixture.clone()`, refresh `last_eval_json` via `host.evaluate()` on success.
- Drop the now-redundant `snapshot_fixture(...)` calls before `addWidget`, `removeWidget`, `deleteSelection`, `disconnect`, `connectMediaPorts`, `reorganize`, and inside `nodeGraphEdit`'s `"deleteSelection"`/`"connect"` operations — `FlowHost::add_widget/remove_widget/connect_ports/disconnect/reorganize/delete_selection` already call `begin_change()` internally.
- `moveMediaNode`: replace the manual snapshot with `host.begin_change()` before `host.move_widget(...)` (on the now-persistent host), so drags are undoable through real history.
- `patchFlowWidgets`, `renameFlowWidget`, and `nodeGraphEdit`'s `"setFixture"` operation: call `host.begin_change()` before mutating, and swap `host.replace_fixture(...)` for `host.set_fixture_preserving_history(...)` so these no longer wipe history.
- Update `bundle()` to `Box::new(FlowPlayApp { host: None })` and the 5 test constructors in `mod tests` to match.

## Verification

- `cargo test -p flow_core -p flow-plugin`.
- Rebuild the flow plugin WASM bundle (existing `bun ./script.ts wasm` flow used earlier in this session).
- Re-run the flow entry in `.repo/🎫/26/07/04/WGPU-PLAYGROUND-E2E/verify-wgpu-playgrounds-e2e.ts` and its react counterpart in `.repo/🎫/26/07/05/SUPPORT-REACT-AND-WGPU-RENDERERS-IN-PLAYGROUNDS/verify-react-playgrounds-e2e.ts`, confirming: catalogue panel shows module-grouped operator sections (not just Inputs/Outputs/Contract), Ctrl+Z/Ctrl+Shift+Z undoes/redoes add/connect/delete/move/rename/patch actions, and repeated undo across several distinct actions restores each prior state in order.
