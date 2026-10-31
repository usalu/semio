STATUS: BLOCKED (foreign, frozen area). actions-b errors from runtime-p31 (`last_turn_work` on `ControlledRetirement`) are already gone: the method is back at `🌱️value/♻️retirement/🎮️controlled/🦀️.rs:164`, nothing to fix in flow. chk7 now stops in `🌱️value/♻️retirement` mid-migration (`RetirementStep::Failure/Progress` gained a `u64` arg; call sites in `📋️original-vector` l.54 and `📦️box` l.37 not yet updated). Not editable by me (addendum 00:30). Rerun `1030-tools/perf/flow.sh N` when the retirement lane finishes.

Audit: `1030-audit-perf.md` (15 findings with file:line, frequency, cost, owner).

## Fixes
1. flow Catalogue pane (audit 1): `✏️editor/🖥️host/🦀️.rs` new `catalogue_sections(extra_json) -> Arc<Value>` (cached per `flow_extension_registry_generation` + extras text; same data as the old host path: registry sections + extras + static sections); `📌️panels/🛍️catalogue/🦀️.rs::render(config, labels, windows)` no longer rebuilds a `FlowHost` from the snapshot; both call sites in `🎛️flow-play-app/🦀️.rs` updated. Test: `catalogue_sections_are_built_once_and_equal_the_host_projection` (Arc::ptr_eq witness plus equality against the real host `catalogue_json`, the third-party-style oracle).
2. puzzle5d editor projection (audit 2): `🧬️schema/🎞️play-snapshot/🦀️.rs` gains a per-root `projection: OnceLock<Arc<DslValue>>` (`projection_with`, cloned with the root, reset on `apply`, retired as a third deferred cursor item); `✏️editor/🧩️entities/🦀️.rs::puzzle5d_editor_projection` now reads that cached document view (1 pass instead of ~5). Test `the_editor_projection_is_cached_per_root_and_equals_the_uncached_derivation` (closure `unreachable!` witness proves no re-derivation for repeat reads and clones; equality with the uncached derivation).

## Checks
- `🗑️generated/1030/perf-chk1-puzzle.txt` rc 0. `perf-chk1-flow.txt` rc 1: E0277/E0053/E0061/E0308/E0433/E0507/E0624/E0027 in files listed above. Rerun `1030-tools/perf/chk.sh N` once flow editor compiles, then `cargo test` the two new laws.
- Open items (fem/fem2d preview + cold diff/apply, energy lookups/bridge, flow main window double projection, puzzle Arc<Value> remainder): see audit table.

## Flow editor compile fixes (coordinator request; resid result does not claim them)
- `✏️editor/🦀️.rs`: `extern crate semio_framework_os_kernel as dsl;` (E0433, `app_commands!` needs `::dsl`).
- `direct-store-command-work`, `direct-store-factory-and-child-group-route`, `graph-operation-route`: `ArtifactCommandInputs` destructure gains `snapshot_read: _` (E0027).
- `graph-operation-route`, `contributions-route-and-instance-owner`: each macro-generated private `bounded_first_step_tool_proofs` gets a `pub(super) fn proofs()`; `flow-play-app` calls `::proofs()` (E0624).
- `flow-play-app`: `pending_effects` takes `ArtifactPendingView` (E0053); the retained-work closure is typed `-> Box<dyn ArtifactCommandWork<EditorApp<FlowPlayApp>>>` (E0308).
- `contributions-route-and-instance-owner`: `owner.step(cx, grant)` and `OriginalTurnFailure::from(fault.debit_original(cx))` (E0061/E0277).
- `host-only-retained-route`: `FlowCommand::clone(&taken.command)` (the command is now immutable custody; small copy) (E0507).
- `retained/effect`: `#[value(send_only)]` on `FlowHostEffectSources` (E0277 Sync).
## Main window (audit 8)
`main/🦀️.rs::render`: the pack JSON of the live host snapshot is now serialised only when the extras do not already carry `host_snapshot_json` (it was always serialised and discarded). The remaining host rebuild + second `to_host_snapshot` cannot be fused without a framework API yielding the DAG snapshot from a live snapshot: FOR framework/flow host owner.
## FOR fem / fem2d / energy
Audit rows 3-7: fem/fem2d gumball `preview` per render per window, `to_reference` in cold `diff`/`inverse`/`apply`; energy `Rows` re-projection and cold bridge. Not touched.

## Late fixes
- crate-root `extern crate semio_framework_os_kernel as dsl;` (in `📦️packages/🦀️rust/🦀️.rs`, not the editor module); `request.instance_operation_owner.clone()` x3; `FlowHostEffectPayload`/`FlowHostEffectJob` made `pub` (E0446). chk4 failed only on a transient Cargo.lock/2d-path-stroke move; chk5 on the value/sort gate above.
