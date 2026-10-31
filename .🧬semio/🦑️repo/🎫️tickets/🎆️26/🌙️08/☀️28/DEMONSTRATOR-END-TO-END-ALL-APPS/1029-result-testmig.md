# 1029 result testmig
Status: IN PROGRESS (11:45 10/29, resumed after usage reset; batch b2 restarted: sourcing, gen2d, playbook, flow, fem2d, puzzle 2d/3d) - migration DONE, native test compile BLOCKED by non-test library drift (see Blockers). Mechanical rewrite of all `.dispatch_typed(` call sites under `✏️s/**` + `🌎️hub/**` DONE by script (233 sites, 118 files); 8 multi-line/odd sites finished by hand/script v2; compile + run per plugin pending (queue; first attempt hit ENOSPC on the rust cache image, 3.3 GiB free).

## Method
- `C/1029-tools/testmig/migrate.py`: `RECV.dispatch_typed(CMD, META, IDENTITY, CX)` -> `semio_framework_plugin::artifact_app_laws::dispatch_original_fixture_command(&mut RECV, &CMD, META, semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT, IDENTITY, CX)` (exact generation3d reference shape; plain-identifier receivers that are `&mut` params become `&mut *recv`).
- NOTE for main: `artifact_app_laws` ALSO gained a `DispatchTypedFixture` trait (`app.dispatch_typed(cmd,&meta,&mut identity,cx)` over `drive_typed`, added by `resume` ~07:11). It would have made the old call shape compile unchanged; per brief this migration uses the canonical action-ingress helper instead (the real UI path), so the trait is unused by ✏️s/🌎️hub.

## Done
- ALL `.dispatch_typed(` call sites under `✏️s/**` and `🌎️hub/**` rewritten to `artifact_app_laws::dispatch_original_fixture_command` (script `C/1029-tools/testmig/migrate.py`; 238 sites in 121 files) EXCEPT the hub space-engine fixture helper (`🌎️hub/🧩️compositions/🪐️space/⚙️engine/🪐️space/🧪️tests/🔬️unit/🦀️.rs:27`, 2-arg pre-cx shape: that whole fixture tree predates the cx/identity/policy API and needs a fixture-scope port, not a call rewrite).
- Hand-ported (stale beyond the call shape): fem2d reconnect test (OriginalFixtureScope + cx on settle/tick/close; also repaired the mangled `Fem2dCommand::AddNode(Fem2dCommand::AddNode{..})` literals), puzzle2d presence test setup line.
- puzzle3d (`semio-s-artifact-puzzle-3d-editor`): `Puzzle3dCommand` has no `ToValue`, so its fixtures go through `dispatch_fixture_action` (action id + args, window id folded into args as the shell sends it) instead of `dispatch_original_fixture_command`; shared `🧰️context` fixture repaired (prelude re-imports for the crate split taken from rustc suggestions, `settle_framework_reserved_admission`/`take_typed_operation_result_page`/`pending_effects` cx+grant arities, `pack::snapshot` -> `dsl::snapshot`).

## Blockers found by the first native `check --tests --workspace` sweep (logs `C/🗑️generated/1029/testmig-<ws>.txt`)
- `semio-framework-plugin-testing-laws` (artifact-app-laws, the library EVERY plugin test depends on) did not compile natively at 10:10: `bounded-viewer-fixture-and-viewer-laws/🦀️.rs:133-134` (`semio_framework_plugin_job::Fault` private, `reference_birth_fault` not in that crate; file edited by a peer at 10:07) -> sourcing/process/... test crates cannot even start. Not touched (changed <10 min ago).
- Native lib drift (wasm lanes green, native red) found per workspace: generation3d (`geometry-inputs` super/Sync/iterator, `io-export`/`io-import` `Generation3dSnapshot: ArtifactDsl`, `preview-eval` 38 errors `FlowEvalSession::preview_mesh_pack`, `sqlite-native` `RetireOwned`), cad (`geometry-import` missing `semio_mesh_snapshot_from_mesh_data`/`mesh_extent`, `cad-transient` `transient_root!` needs `pack`+`semio_framework_os_kernel`, `cad-picking` private `GeometryOrder`, `cad-presence` duplicate `RetireOwned`, `cad-host` `CadStore::new` type annotation, `cad-inferences` stale `crate::CadWorkingScene`, `cad-config` tests `diff`/`inverse` need the trait import).

## Also fixed 11:00-11:40 (stale law-call arities found by `C/1029-tools/testmig/arity.py`)
- 35 hub `🔬️surface` tests + hub norm app-surface: `assert_viewer_never_mutates::<V>()` -> `(INTERACTIVE_COMPONENT, &mut fixture_identity())`; hub energy/mathematical viewer-app tests: `with_registry(.., policy, identity)` + scope + `close_registered_fixture_app(.., policy, cx)`.
- hub wfc/procedural close-ladder + idle-turns: scope + identity/cx on `plugin_load_document(_text)`; hub stdio shipped-fleet: `dispatch_fixture_action`, cx on settle/close; forms + generation3d converge laws: policy+identity; fem2d remove-selection `load_document` cx.
- STILL STALE (old pre-cx shape, need fixture port; listed by arity.py): hub space-engine tree (13 sites), hub stdio editor-catalog (render/handle_action_invocation API), layout intrinsic (5), energy model editor unit (9, dense one-liner file), puzzle2d presence law (helper `assert_presence_round_trip` was removed from the framework; fem2d/fem3d presence tests use the also-removed `assert_registered_presence_round_trip`).

## Per plugin
(native compile results pending: batch b2 queued; previous batch b1 results: gen3d/cad/sourcing/process3d/gismap RED on library or framework drift, see Blockers)

## 17:35 update
- `DispatchTypedFixture` / `dispatch_typed_fixture`: already gone from `artifact_app_laws` (rg: 0 hits in the repo) - nothing to delete.
- b2 results so far: sourcing RED on a library, not a test: `semio-s-artifact-stdio-semio-table-model` (`unresolved import ...value_model::io::sqlite::snapshot::semantic::value_binary`, 1 error) - stdio-semio owner. generation2d check still queued on compiler-image headroom (>2000 s); playbook, flow, fem2d, puzzle2d, puzzle3d follow in `testmig-batch-b2.log`.
- Presence tests (fem2d/fem3d/puzzle2d): `assert_presence_round_trip` / `assert_registered_presence_round_trip` exist nowhere in committed history or the tree, i.e. those tests were already dead before this wave; they need a new presence law in `artifact-app-laws` (not a call-site port) - NOT done.
- NOT done: hub space-engine fixture port, hub stdio editor-catalog, layout intrinsic, energy model editor unit (all need compile feedback on a moving framework API; queue is the bottleneck). Remaining sequence: wait b2 -> fix errors in test crates -> port the above.

## 19:00 update (b2 partial)
- gen2d RED upstream: `semio-framework-value-retirement` native (`dyn RetirementCursor` not Sync, 3 errors) - framework value owner; blocks every dependent.
- playbook: lib `playbook-inferences` RED (store-preparation trait drift: `begin_copy_byte_demand`, `advance` arity, `PlaybookConfigMutation: OpBinaryReceiving` - same bucket as process3d, hostinit). Test-side fixes made: `ColdMutation as _` imports in playbook config/mutations unit tests (`diff`/`inverse` need the trait in scope), `WindowKit` import in the steps window unit test (`TreeWindowKit::window_kind/KIND_ID`). Unverified until the lib compiles.
- flow, fem2d, puzzle2d, puzzle3d still queued (flow >50 min waiting for compiler-image headroom).
