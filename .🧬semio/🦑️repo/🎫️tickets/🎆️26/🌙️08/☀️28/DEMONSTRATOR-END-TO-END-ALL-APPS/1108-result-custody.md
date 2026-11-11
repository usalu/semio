# Lane custody result (runtime ops custody callers + job-receiving tests)

## State (08:20, unverified; first compile W-custody-1 queued: `chk.sh W-custody-1 🌎️hub/Cargo.toml wasm -p semio-plugin-plugin-program`)
- window-view.rs: admit(cx,admission,failure,slot) via `born`; close_old effect-shaped (`begin`/`step_granted`); new-field move via `funds_retirement`+`paid`.
- manifest/🦀️.rs: imports original_steps `{born,retired}` (crate::retired gone); window/refresh admit take `&mut receiving.frame,&mut receiving.frame_failure`; `capture_dispatch_step` uses `born`.
- 🧰️ops/🦀️.rs: removed the stale `built_with`/`built`/`retired` copies.
- typed-operations intent.rs / response.rs: `admit`/`admit_surface` take `(admission,failure)` before `output`; each owner holds `frame`+`frame_failure` for its own children (retired with `next_owner_demands::<OriginalConstructionFailure>` + `frame.close_step`); funded moves via `moved`, shell drops via `forgotten`, effect-shaped closes via `begin`/`step_granted`; response phase 4 uses `SharedUtf8::admit(text,grant,&mut actual)` + `receive_original_retained_work` (charged `receipt(d)`).
- 🪟️surfaces/{native,read}.rs (in plugin-program): same pattern; cursor advance/take/close now reply-shaped (`forward`+`replied` with `advance_into/take/begin_close_into/close_into`); `admission.advance/take_output/return_inputs(effect)`; source handback via `take_authority_into`; stale local `built/built_with/retired` removed. Cargo.toml of plugin-program gained `semio-plugin-job-receiving` (LOCK NEEDED, claims log).
- view-model: new `🔗️shared-source/🦀️.rs` = `impl OriginalSourceProjection<ViewModel> for Arc<ViewModel>` (required by `admission_borrowed(authority)`, orphan rule forces it into semio-framework).
- ui_contract `OriginalUiArguments::admit` is already effect-shaped (`turn`+`with_original_effect`), no port needed.

## Debts / open
- (DONE unverified, coordinator request) turn/receiving: see section below. Former debt text: call `OriginalResponseActionSource::admit` / `OriginalIntentActionSource::admit`: they must own `frame`+`frame_failure` slots; the crate also still uses grant-shaped `retired`. Not ported (not in the brief).
- Tests not yet ported: job-receiving pack/copies/transaction 🧪️tests; runtime-ops metadata-tests.rs, response-tests.rs, 🧪️tests/original-view-read.rs.

## turn/receiving (coordinator request, unverified)
- Source crate `semio-plugin-reactor-receiving-source` now re-exports `original_steps::{begin,forgotten,retired}` (dep added, LOCK NEEDED); its grant-shaped `retired` and the copies in completed/fault are gone.
- `RetainedResponse`/`RetainedIntents` hold `BirthFrame{admission,failure,retirement}`; admit calls pass the slots; frame closes before the source witness; `terminal()` includes it. Metadata/intent-source releases use `forgotten`.
- forms/staged/auxiliary/completed/fault closes are effect-shaped (`ControlledRetirement::step(body,effect)`, `begin`). Their 🧪️tests (staged-tests, completed tests) are not ported.

## Latest runs
- W-custody-1 (wasm plugin-program): red in view-model (Arc<ViewModel> not RetireOwned) -> fixed with ArcLease (see above).
- W-custody-2 (wasm plugin-program + runtime-ops): stopped at upstream `semio-plugin-document-media` (20 errors, media retirement trait drift in 🎞️media/🛫️intrinsic + 💥️fault; pubmedia claimed 07:52). My crates were not reached. Rerun: `chk.sh W-custody-3 🌎️hub/Cargo.toml wasm -p semio-plugin-plugin-program -p semio-plugin-runtime-ops` after pubmedia FIXED; then native, then `tst.sh` on job-receiving.
- Debts: original-view-read.rs, staged-tests/completed tests, job-receiving tests unrun, possible Arc<ViewModel> RetireOwned fallout in NativeReceivingOriginal<_,Arc<ViewModel>> (metadata.rs).

STATUS: BLOCKED semio-plugin-document-media red (pubmedia lane) stops W-custody-2 before plugin-program/runtime-ops; all custody edits unverified
