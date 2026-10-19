# result-lfix2 — puzzle wasm bridge caller, store-disposer fixtures, `✏️s/🧑‍💻dev/**` fixtures

Executor `lfix2`. No cargo, no git write, no package.json/bun.lock edits. Tools: `lfix2-tools/` (this ticket folder, input scripts only; `devlib.py` builds on `lfix-tools/ports/wrapper.py`). Scratch scanners lived in the session scratchpad.

## 1. Puzzle wasm bridges: a production source for the original caller context

### 1.1 Finding

No other plugin has a production wasm-bindgen bridge that threads a `StepContext`: every other plugin's `🌉️wasm` bridge was deleted (layout, gis, fem, cad, raster, writer, process3d, generation2d/3d, animate, imperative, shooting: stub files citing ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`, "nothing ever built it for wasm32-unknown-unknown"). The puzzle bridges (`Puzzle3dArtifactVcs`, `Puzzle5dArtifactVcs`) are the only ones left, and nothing in TS consumes their envelope-load surface (the 3d story only uses `puzzle3dParseDslJson`).

The nearest production precedent for a host that is its own outermost caller is `os-host` (`🏠️os-host/🦀️.rs`, `AssetDecodeOwner` + `run_asset_decode_turn`): it owns one cancellation root and one `JobPayloadAuthority`, lends a **fresh** `StepContext::with_payload_authority` per host turn (`StepBudget::from_duration(MAINTENANCE_LANE_FUEL, default_now_us, MAINTENANCE_LANE_WALL_US, grant)`, fresh receipt) and retires both in paid turns. The guest-supplied contexts (`OriginalActorGuestEntry`) belong to the wasm component host, not to a wasm-bindgen page. The page's JavaScript is the outermost caller, so the handle owns the roots.

### 1.2 Implementation

* `✏️s/🔌️plugins/🧩️puzzle/🎮️commands/📤️jobs/🦀️.rs`: new `BrowserCaller` (region `🌐️BrowserCaller`) built on `OriginalCallerSourceParent`. `admit(turn_grant)` admits cancellation root, payload authority and source parent from one grant; `turn(|cx| ..)` lends a fresh maintenance-lane context per exported call (clock `default_now_us`, installed by `semio_framework_async`'s `wasm_bindgen(start)` in the browser); `retire_step()` pays the parent's terminal retirement in turns quoted by `retirement_demands()`; `Drop` asserts paid retirement like the framework's own roots. Native unit test `a_browser_caller_lends_fresh_contexts_over_one_root_and_retires_in_quoted_turns` (same file).
* 3d bridge `…/🧊️3d/…/✏️editor/🌉️wasm/🦀️.rs` and 5d bridge `…/🖐️5d/…/🌉️wasm/🌉️bridge/🦀️.rs`: the handle holds `caller: RefCell<BrowserCaller>` and `closed: Cell<bool>`, created in `create()` from `mounted_policy.maintenance` (the plugin runtime's own per-turn grant). `sealEnvelopeLoad`, `pollEnvelopeLoad` (maintenance + advance in one turn) and `closeStep` run inside `caller.turn(..)`; `closeStep` closes the app first, then retires the caller, and answers true only when both are done. Lifecycle failures project to JS through `lifecycle_to_js` (original `Fault` when the cause is one), value refusals through `value_to_js`.
* Pre-existing breaks in the 3d bridge fixed on the way (it could not have compiled for wasm32): `fault_bridge` was never in scope (now `#[path]`-includes the shared `puzzle/🌉️wasm/⚠️diagnostic` module like the 5d side does), `#[wasm_bindgen]` structs had `pub` non-`Copy` fields (`pub app`, `pub operation` duplicating its getter) → private.
* `…/🧊️3d/…/✏️editor/📦️packages/🦀️rust/Cargo.toml`: wasm32 target deps `semio-framework-diagnostic`, `serde_json` (needed by `fault_bridge`). **`🧊️3d/Cargo.lock` needs a refresh.**

## 2. Store-disposer fixtures re-authored against `ArtifactOwnedDisposer::close_step(owner, RetainedCloneGrant) -> PluginLifecycleStep`

The old `(items, bytes)` and `PluginCloseStep::{Pending,AwaitingInput,Blocked,Complete}` shapes map to: quote the next turn with `retirement_demands(&owner, body)`, fund exactly that quote with `maximum_items = items`, and read `PluginLifecycleStep::{Progress(progress),AwaitingInput,Blocked,Complete(_)}`. `released_items`/`released_bytes` assertions became `progress.copied_items`/`progress.released_bytes` against the grant.

| Fixture | What it asserts (preserved) |
| --- | --- |
| gis `gisterrain/…/🚪️io/💾️binary/🧬️mutations/🧪️tests/🔬️unit` | terrain document store closes terminal-empty; one item and a page of release per step; a fresh store never blocks. Also: owners via `funded_bounded_artifact_store_owners` + exact install (the `bounded_document_store_owners()` no-arg form is gone). |
| gis `gismap/…/✏️editor/🧪️tests/🔬️unit` (preparation law) | the Store used by the one-item preparation law closes terminal-empty. |
| gis `gismap/…/🧬️schema/🧬️mutations/🧪️tests/🔬️unit` | GIS document store closes terminal-empty after apply/replay. |
| process3d `…/🧰️owned/🧪️tests/🔬️retained-laws` | `close_store` and the displaced-store phase 2 close (with `process3d_demand_grant`); additionally the two initializer drivers (`owned_store_measured`, neutral timeline replay) moved from `StepOutcome` to `Option<JobOutcomeBorrow>` with a 4096 fuel budget (fuel 1 cannot pay a turn any more). |
| draw `…/🧪️tests/🔬️gesture-operation-owner` | instance owner closes every slot parity under one page per step (`close_step(cx, grant)`), cancelled owner retires within `RETAINED_BYTES/page + 2` maintenance steps (`maintenance_step(cx, grant)`); both run under an `OriginalFixtureScope`. |
| cad `…/🧰️command-fixtures-and-retained-lanes` | empty draft lane closes (owners funded through `fund_document_store_owners` over the app's `build_draft_store_owners(grant)` and `draft_store_owners_source_demands()`), transient lane: a zero-item grant yields an empty `Progress`, then completes. |
| cad `…/🧬️geometry-media-and-owners` | cancelled geometry child import: `prepare_child_one(grant)` / `close_child_one(grant)` (new `Result<..>` + progress-carrying `ChildEmitPreparationStep`), zero-item grant is a no-op `Pending`/`Progress(default)`, retirement reaches the exact empty witness; media-pricing close loop on `close_step(grant)` + `Pending{progress}`/`Complete{progress}`. |
| lowpoly `…/🧲️transform/🧪️tests/🔬️unit` (found by grep, not in the lfix list) | same disposer ladder as shooting/wfc. |

## 3. `✏️s/🧑‍💻dev/**`

Scanner clean afterwards (`stale.py`, `stale2.py`, `stale3.py` over `✏️s/🧑‍💻dev`: no rows, was 159 sites in 36 files). 48 `.rs` files changed.

* generation3d editor (`🧩️composition/🧪️tests/🧊️generation3d/…/✏️editor`): `context::Generation3dAppFixture(pub app, pub OriginalFixtureScope)`, Drop = paid `close_registered_fixture_app`, helpers (`dispatch`, `act_with_view`, `settle`, `render*`, `select_*`, `drain_*`, `drive_preview_run*`, `dispatch_effect_command`, `owed_run_actions`) now take the fixture; `close_ladder_witness` removed (the framework close names the stalled authority itself); `close_instance_operation_owner` / `retire_instance_operation_owner` own a local scope. 27 user files ported by `dev_gen3d_users.py` (`close_registered_fixture_app`, `settle_history_verb`, `settle_registered_typed_operation`, `load_document`, `assert_undo_redo_round_trip`, 3-argument `handle_action` → `dispatch_fixture_action`, `dispatch_typed`, UFCS archive calls); probes keep the raw `&Generation3dApp` closure parameter.
* `🧪️tests/🔬️brep-extension`: `settle`, `dispatch_effect_command` (→ `dispatch_fixture_command`), `drive_preview_run` take a trailing `cx`; `retire_execution_owners` pays under its own scope.
* generation3d viewer: `Generation3dViewerFixture(.., scope)`, same helper set; `eval-chain`, `status-contract`.
* `🎬️sequence-runtime` (context + `🪟️windows`), `🧩️generation3d-example-geometry/🚚️mesh-delivery` (`retire_eval_session` on the quoted `FlowEvalSession::close_step(grant)`), `🌊️flow/🌿️catalogue` (`RegistryRetirement::close_step(grant)`), `📐️cad/📐️step-session` (`GeometryPortRetirement::retire_cold`, zero-grant `Progress(default)` + not-terminal for the old `Blocked` law), trinity graph editor tests (13 `render` → `render_registered_fixture_app`, 2 `settle_history_verb` gained `cx`, `jack_query` gained `cx`).
* `retire_flow_eval_session` (generation3d `flow-operators`) uses the session's own `next_close_*` quotes.

## 4. bim

`✏️s/🔌️plugins/🏙️bim` still carries the port (editor fixture, viewer unit test and the gesture owner test use `OriginalFixtureScope`/`close_step(owner, grant)`); `stale.py --list`, `stale2.py`, `stale3.py` over it print nothing. No re-apply needed.

## 5. Verification (static)

* Scanners: `lfix-tools/scan/stale.py|stale2.py|stale3.py` over `✏️s/🔌️plugins` (only the known false positives of result-lfix remain: own `close_step`/`render`/`maintenance_step` methods, the `layout/🎞️intrinsic` orphan) and over `✏️s/🧑‍💻dev` (empty).
* Lexical balance check of all 119 changed `.rs` files in the touched plugin and dev trees (strings, raw strings, chars, comments aware): 0 unbalanced.
* Signatures read from source for every framework call introduced (`dispatch_fixture_action/command`, `settle_*`, `render_registered_fixture_app`, `OriginalCallerSourceParent`, `JobPayloadAuthority::admit`, `CancelToken::admit_root`, `FlowEvalSession::close_step`, `GeometryPortRetirement`, `fund_document_store_owners`).
* Not run: cargo, the oracle (blind to signature drift, see result-lfix 2.1).

## 6. Crates to compile (`bun repo cargo --store=shared … -- test --no-run -p <crate>`)

Plugins: `semio-s-plugin-puzzle-jobs`, `semio-s-artifact-gis-gisterrain-operations`, `semio-s-artifact-gis-gismap-gis2d-unit-tests`, `semio-s-artifact-gis-gismap-inferences`, `semio-s-artifact-process-process3d-host`, `semio-s-artifact-draw-drawing-editor-tests-2`, `semio-s-artifact-cad-cad`, `semio-s-artifact-lowpoly-lowpoly-editor`.
wasm32 (cfg `wasm32` non-p2, invisible to native cargo): `cargo check --target wasm32-unknown-unknown -p semio-s-artifact-puzzle-3d-editor -p semio-s-artifact-puzzle-5d-host-admission-tests` after the `🧊️3d/Cargo.lock` refresh.
Dev: `semio-s-composition-laws` (targets `generation3d-app-laws-*`, `sequence-runtime`, `generation3d-example-geometry`, `generation3d-incremental-eval`), `semio-s-dev-cad` (`step_session`), `semio-s-dev-trinity-jack`, `semio-s-flow-composition` (`flow_catalogue`).
Previous list (result-lfix 4) plus: `semio-s-artifact-gis-gisterrain-operations` is new (it holds the terrain mutation test).

## 7. Open items and risks

1. **`InteractiveJob` trait generation gap (pre-existing, blocks compile of the crates above).** At HEAD `semio_framework_job::InteractiveJob::close_step` is `(&mut self, cx) -> Result<InteractiveJobCloseStep, OriginalTurnFailure>` and `InteractiveJobCloseStep` has only `Pending{progress}`, `Blocked`, `Complete{progress}`. 213 plugin files still implement `close_step(&mut self, grant)`, and 75 files (e.g. `semio-s-plugin-puzzle-jobs` `job_close_step`, the cad media job, draw) construct `InteractiveJobCloseStep::Refused { kind, progress }`, which does not exist. `semio-s-plugin-puzzle-jobs`, and through it both puzzle bridges, will not compile until that job migration lands. My cad pricing and flow-session edits follow the on-disk impls (`CadMediaWork::close_step(grant)`) and will need the same follow-up port.
2. Trinity dev graph tests were already half-ported by a peer: helpers with a raw `&mut VcsArtifactApp` parameter write `&mut app.app` (e.g. `select_ast`). I did not touch those lines; if `VcsArtifactApp` has no public `app` field they fail to compile.
3. Cad pricing test and process3d initializer drivers use `RetainedCloneGrant`/budget values chosen from the quoted demands and the fixture policy (fuel 4096, retirement loan 32 KiB copy / 256 KiB capacity / 1 MiB release for the flow registry retirement), not from a run.
4. `OriginalFixtureScope` budget (32768 fuel for the whole test body) applies to the newly scoped dev tests as well (long loops such as `retire_instance_operation_owner` open their own scope for exactly that reason).
5. `BrowserCaller::turn` starts every call with a fresh receipt; the roots are born from `mounted_policy.maintenance` (32 items, 16 MiB capacity, depth 4096) which is assumed to fund `SealedShared::admit` and the payload ledger birth (`shared_retirement_allocation_bytes::<JobPayloadOperationLedger>()`); the native unit test asserts it but could not be run.
