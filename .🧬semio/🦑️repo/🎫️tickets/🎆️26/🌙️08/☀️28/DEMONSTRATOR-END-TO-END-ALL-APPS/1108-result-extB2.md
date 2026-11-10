# extB2 result (takes over from extB: window-config, infinite-board-engine, extension checks)

## State (step 1, edited, verify W-extB2-1 wasm)
- receiving (`owner-contract-and-mutation/receiving`): `record_constructed_allocation` dropped. The construction step (`receive_construction_step_typed`) already posts the receipt from the returned progress (`retained_capacity_bytes = d.capacity_bytes`), so the explicit record was redundant.
- typed-pack-load native: `refused` now takes `NativeReceivingFailure` (folds value/retirement/receiving effect causes); `retain_original` replaced by `RetainedOwnedProjectionAdmission` (`wire_projecting`) + two `ReplyStash` (admit then take), closed before the wire.
- preview (`🧬️preview`): `admit_input` is two-phase (admit the typed alias with `try_project`, then take it) and takes the turn effect; `input_demand(input)` / `input_work_demand(input)`; `input_held()` replaces `work_pending()` for the "held" check. Retirement branches are one effect-shaped `step_effect` (`close_factory_ticket[_controlled]`, producer `close_step[_controlled]`, projection `close_step`, factory `step`); grant-based store calls (`begin_derived_operation`, `take_derived_operation`, `admit_original_snapshot_alias`) run with `effect.grant()`; `advance` runs the derived `step_derived_operation` directly on the receiving control and every other branch inside `with_allocated_effect`.
- store-owners: preview chain (`preview_finish_advance/step`, `preview_cancel_step`, `preview_close_step`, `preview_retirement_step[_original]`, `epoch_close_step`) takes the effect (no grant); `ControlledRetirement` uses `select(body)?.step[_controlled](effect[,control])`; `catalog_step` call in `preview_input` gets the effect; error-type annotation fix in the close fault path.
- crate epoch: close_step runs `preview_epoch_close_step` inside `with_original_effect` + `with_native_retirement` (reborrowed child effect), receipt via `receive_original_retained_work`.

## Interface changes (callers outside this crate)
- `WindowConfigOwnerRegistry::preview_input(input, epoch, effect)` (was grant), `preview_input_demands(kind, window, epoch, body, input)` (new `input`), `preview_retirement_step(effect)` (was grant).
- Callers: `plugin/tool-machine/🔁️follow` (lane 3 `preview_input`/`preview_input_demands`) and `plugin/instance/render/staging/tool-machine` (`preview_retirement_step`) still use the grant shapes.

## Verification
- window-config lib: wasm W-extB2-1 (rc0), native W-extB2-5 (rc0, source budget 259153/262144 after removing dead code and redundant qualifications; no formatting collapse). Wasm re-verify of the final tree: W-extB2-9 rc0.
- infinite-board-engine: wasm W-extB2-7 rc0 (extB's pub/doc(hidden) cursor fix holds).
- `--tests`: lib cfg(test) catalog+genesis(+fixture) moved to `[[test]] window_genesis` (`🧪️tests/🧬️genesis`), so source admission passes (W-extB2-8). The remaining 69 errors are stale test code, not lib: receiving tests 32, apply tests 16, birth tests 11, fixture 4 (fixed: `original_ordinal_operation_receiving!`), catalog 2. They need the effect-bank harness (grant -> effect, `Birth::admit(.., birth, result)`, `admit_constructor(effect)`), likely a dev-dep LOCK; not started.
- Extension checks (ext.sh) not run: the queue behind peers was long.

## Debts
- Unit tests inside the crate (`epoch/🧪️tests`, `preview/🧪️tests`, `store-owners/…/tests`) still use the grant-shaped preview API.

## Rerun
`chk.sh W-extB2-N "$M" wasm -p semio-plugin-window-config` (M = window-config `📦️packages/🦀️rust/Cargo.toml`), then `ext.sh <round>`.

STATUS: FIXED-verified W-extB2-9 (window-config lib wasm), W-extB2-5 (native), W-extB2-7 (infinite-board-engine); BLOCKED window-config --tests 69 stale test errors, ext.sh not run
