# Flow WASM API Drift

Source: `job-flow-wasm.log.2` (run wbYhU7Z). Crate: `semio-os-flow-component` (manifest `🌊️flow/🧩️component/📦️packages/🦀️rust/Cargo.toml`), target `wasm32-unknown-unknown`.

## Errors in the log
- `semio-plugin-instance-typed-ops`: E0061 `publish_mounted_owned_child_operation_unit` missing `RetainedCloneGrant`. Already fixed concurrently by another agent (caller now passes `remaining_plugin_turn_grant(turn,*spent)`, like the sibling units).
- `semio-plugin-time-travel-turns`: 7 errors (E0599 `PreviewDeferred`, E0609 `preview_due` x4, E0559 `terminal`, E0769 `OriginalProgress{..}`).

## New store semantics (semio_plugin_time_travel, working tree)
- `preview_due` and `PreviewDeferred` are gone: `Preview` never defers; an unsettled previous preview is cancelled and refused with a `WorkLimit` fault, otherwise it returns `Previewed(Pending)`.
- `CloseOriginalSources { grant }` lost `terminal`; session shutdown moved to `CloseSessionHandoff { grant }`.
- `OriginalProgress(RetainedCloneProgress)` is a tuple; `preview_closing` is replaced by `TimeTravelOwners::original_sources_close_pending`.

## Changes (both under `⏪️time-travel/🧵️instance/`)
- `🔎️preview-replay-and-session-start/🦀️.rs`: `show_time_travel_preview` handles only `Previewed(step)`; dropped the `preview_due` writes and the `PreviewDeferred` arm.
- `🏃️turn-driving-and-replay-steps/🦀️.rs`: `CloseOriginalSources{grant}` with `OriginalProgress(progress)`; `preview_closing` now derives from `owners().any(original_sources_close_pending)`; removed the `preview_due` re-issue branch (stepping by `preview_progress()` only); removed `preview_due=false` in the discard publication.

## Remaining, unrelated drift (other agents' in-flight work, not touched)
Wasm check of `semio-os-flow-component` now stops at:
- `semio-plugin-instance-commands`: `🧊️reserved-routes` (5x E0308, `&ReservedCommitMetadata` where `&ActionMeta` expected; use `.get()`), `🚦️actions` (`ActionIngressSource.original_press_selected` removed, E0609 x3; E0308 `Option<&DslValue>` vs `&mut Option<DslValue>`).
- `semio-os-infinite-world-cursors`: `🧲️cursors:139` field `engagement_session_active` missing on `WorldSelectionState`.

## Follow-up: wasm check clean
`repo cargo --store=shared --configuration=prototype -- check -p semio-os-flow-component --target wasm32-unknown-unknown --message-format=short` exits 0 (after retries on exit 75 / transient `host-capacity` import errors from concurrent edits in the repo build tooling).
- `semio-plugin-instance-commands` (reserved-routes `ReservedCommitMetadata`, `ActionIngressSource.original_press_selected`): fixed concurrently by the in-flight authors; no change from me.
- `semio-os-infinite-world-cursors` `engagement_session_active`: the field had been dropped from `WorldSelectionState` while bridge, authority-core, cursors and tests still use it. Restored it (declaration and initializer) in `♾️infinite/🌍️world/🧩️components/🦀️.rs`.
- `semio-os-infinite-world-interaction`: the relocate refactor (concurrent) was finished by its author except `object_world_instance` in `🧵️interaction/🌪️vortex-relocate-and-terrain/🦀️.rs`, which lacked a lifetime (E0106); added `<'a>` to the signature.
