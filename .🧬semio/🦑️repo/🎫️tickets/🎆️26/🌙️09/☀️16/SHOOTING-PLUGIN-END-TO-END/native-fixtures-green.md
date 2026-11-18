# Native fixtures: shooting lib tests (partial, not green)

Command (all runs `SEMIO_UNIT_CACHE_REMOTE=off`, see "Build infrastructure"):
`repo cargo --store=shared --configuration=prototype -- test --no-fail-fast --manifest-path <shooting rust workspace> -p semio-s-artifact-shooting-shooting -p semio-s-artifact-shooting-shooting-tests -p semio-s-artifact-shooting-shooting-shooting --lib -- --skip two_instances_converge_disjoint_edits_via_backbone`

## Final counts (run 22, last full run)

| package | before | after |
| --- | --- | --- |
| semio-s-artifact-shooting-shooting (sqlite snapshot) | 12 pass / 5 fail | 14 pass / 3 fail |
| semio-s-artifact-shooting-shooting-tests | 27 pass / 0 fail | 27 pass / 0 fail |
| semio-s-artifact-shooting-shooting-shooting (editor) | 18 pass / 49 fail (+1 overflow) | 22 pass / 45 fail (+1 overflow, skipped) |

The three crates are NOT green. The remaining failures all trace to framework fixtures that were written for a per-turn host and are driven here through one long-lived lent context; that needs a fixture redesign, not a local patch (see "Open root causes"). The sibling comparison is at the end.

## Root causes found and fixed (with evidence)

1. Editor, 29 x `OriginalEffectAuthority<ConstructionProducer>` dropped un-retired in `ensure_fixture_cancellation_issuer`.
   `InstanceBirth` owns an `OriginalConstructionAdmission` whose authority must be retired through `close_step` (the runtime does it in the opening, `birth_construction`). The fixture dropped it, and a `?` error path dropped it too, which masked the real error. The fixture now retires the admission on every exit path.
2. The same tests then stalled forever in birth ("exceeded its authored turn bound", measured with temporary stall probes): `StepContext::original_receiving_outcome` returned `Ok(false)` at `admit_original_return_table`, which only raises `birth_requested` on the Source budget. The production host serves that request before each turn (`OriginalJobReturnBirth::service`); `OriginalFixtureScope` lends ONE long-lived context from `lend_context()`, which has no host to serve it, so the Source return table never existed. `OriginalFixtureCaller::admit` now births the return table and publisher up front (`OriginalJobFixtureCaller::birth_returns`), as `OriginalJobFixtureCaller` users already do.
3. Editor, 9 x "UI arena has no accepting lifetime registration": the process-wide UI value arena is registered only by the real guest entry. Fixtures need the framework's own `UiArenaFixture`. Added `FixtureUiArena` (thread-reentrant wrapper over `UiArenaFixture`, caller crate + re-exports) and used it in the shooting `ShootingApp` fixture and `window_body`. All 9 vanished from the failure list.
4. Sqlite, 3 x "original cursor birth differs from its defining layout": `DecodeReport` (`pack/value/document-codec`) declared `sequence_birth_bytes(..)` but its cursor is the inline fields type (56 bytes). It now declares `size_of::<Self::RetirementCursor>()`. Found by a temporary type-name probe in `retirement_cursor_birth_demand`.
   Same defect, same fix, in `CodecFrame<P>` (store codec native-retirement) and `TypedSnapshotExportFrame` / `TypedSnapshotImportFrame<P>` (io-mechanism): they used `Box<dyn RetirementCursor>` + `sequence(vec![..])` with a sequence-sized birth declaration; converted to `retirement_fields_type!` / `retirement_fields_seed!` with the exact cursor size.
5. Sqlite + editor, "retirement cursor lacks a work quote": `HistoryPageRetirement` and `GroupVisibilityRetirement` (vcs retirement) lacked `next_computational_work`. Added (one unit per paid step, delegating to the active child).
6. Sqlite, "native construction requires its original recipient": the io testing harness (`with_snapshot_file_decoding` / `_encoding`) only lent a Work loan. It now lends the Work and a construction authority for the body and for every close turn (`lent!`), and the decode close selects a failure birth first (`requires_original_birth` / `close_original_birth_step`), as `original_owners::close_decode` does.
7. Renewal under a live claim: `testkit::renew_turn` refuses with "original return source with live claims cannot renew its turn" once the Source budget is bound. A snapshot of the pinned budget shows exactly one live claim (`claims: 1`, `held_items: 7`, `held_copy_bytes: 3600`), the cancellation issuer's lifetime hold. `OriginalReturnBudget::renewable()` and `testkit::return_source_pinned` were added and `fixture_renew_turn` leaves a pinned turn as it is instead of failing.

## Open root causes (not fixed)

A. "original receiving scope still owes its enclosing semantic receipt" (24 dispatch failures plus 12 paid-app-close failures downstream). `OriginalReceivingScope::open` requires the previous scope's spend to have been settled by a semantic receipt. The host settles it when a turn ends and the context is dropped; the fixture context never ends a turn. Fixing it means ending a turn per fixture call (a fresh borrowed context per `dispatch_*`/`settle_*`/`render_*`, like `OriginalJobFixtureCaller::turn_original`) or giving `testkit::renew_turn` a defined settlement for the receiving scope. Every fixture caller in `artifact_app_laws` takes `cx` by parameter, so it is a framework-wide change. Fixing A will also change which of the 45 editor tests fail next.
B. Stack overflow of `two_instances_converge_disjoint_edits_via_backbone`: not recursion. The call depth at the first panic point is 12 frames; the test needs about 70 MiB of stack (bisected with `RUST_MIN_STACK`, passes the overflow point from about 70 MiB). Cause: `VcsArtifactApp<EditorApp<ShootingPlayApp>>` is a 1,000,496 byte inline value (`tools` 257,920, `pending` 232,512, `time_travel` 169,328, `tool_machines` 89,520, `media` 87,616, the rest below 30,000 each), and the `paired_registered_apps_with_members` future is 3,165,152 bytes, larger than the 2 MiB test stack by itself; the nested async laws (`assert_two_registered_instances_converge` -> `_with_members` -> `paired_*` -> `new_registered_app_with_members`) build and move these by value in a debug build. The structural fix is to build and keep the apps on the heap (the production construction already initializes `VcsArtifactApp` in place field by field through `initialize_field_at`, so boxing the large fields would break that machinery; the fixture constructors should hand out `Box<VcsArtifactApp>` or an in-place frame instead of returning the 1 MB value, and the convergence laws should hold boxes). Not done; do not raise the stack.
C. Sqlite, 3 remaining:
   - `actual_artifact_declaration_mount_routes_queryable_files`: "forwarded native allocation requires an actual quoted effect" (past the construction/Work loans now). `original_owners::io_run` builds one `IoRunControl` from a decode and an encode control that need loans, but `IoRunControl<'a,'b>` requires one shared `'b` while each `with_work_recipient` / `with_construction_recipient` closure hands out its own shorter lifetime, so both cannot be lent around one `IoRunControl`. Needs either an `IoRunControl` constructor that accepts independent control lifetimes or a resident-loan install API on the controls.
   - `every_geometry_word_survives_both_erased_native_directions`: "retirement cursor lacks original controlled closure" (another retired owner without `close_step_controlled`/`close_original_step`; the type name probe was removed with the other probes, rerun it in `RetirementCursor::close_step_controlled` default).
   - `independent_complete_geometry_edit_public_io`: "snapshot retirement requires a larger original grant" (`retire_inputs` in `io/control/snapshot`; a retired input made no progress with zero work).
D. Editor `a_gumball_drag_edited_in_history_replays_its_downstream`: a standalone `ArtifactStore::dispatch(Apply)` with `fixture_identity()` has no lent Work authority ("original work requires its original caller loan"); the same shape exists in other plugins' standalone store tests (for example raster `raster_document_text_round_trips_store_with_applied_operation`).
E. Editor: "original retirement lacks a work quote" for the document store close (`shooting document store closes through its exact bounded owners`), another cursor without `next_computational_work`; find it with a type-name probe in the default `next_computational_work`.

## Files changed (this pass)

Framework:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` (`ensure_fixture_cancellation_issuer` retires the birth admission; `fixture_renew_turn` leaves a pinned turn)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🦀️.rs` (re-export `FixtureUiArena`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🦀️.rs` (re-export `FixtureUiArena`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/🦀️.rs` (`FixtureUiArena`, return birth up front)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🎟️caller/📦️packages/🦀️rust/Cargo.toml` (`semio-ui-contract-values` with `testkit`)
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/🦀️.rs` (`OriginalReturnBudget::renewable`)
- `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs` (`testkit::return_source_pinned`)
- `🧰️framework/🔨️modules/🎒️pack/🌱️value/🚪️document-codec/🦀️.rs` (`DecodeReport` birth bytes)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/♻️retirement/🦀️.rs` (`next_computational_work` for `HistoryPageRetirement`, `GroupVisibilityRetirement`)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-retirement/🦀️.rs` (`CodecFrame` fields cursor)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧭️io-mechanism/🦀️.rs` (`TypedSnapshotExportFrame`, `TypedSnapshotImportFrame` fields cursors)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧰️testing/🦀️.rs` (lent Work + construction authorities, failure-birth decode close)

Shooting:
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (`ShootingApp` holds a `FixtureUiArena`)
- `✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` (`window_body` holds a `FixtureUiArena`)

All temporary `[DEBUG]` probes were removed (`grep -rn "\[DEBUG\]"` over `🧰️framework` and `✏️s` is empty). The `io_run` lending attempt in `original-snapshot-owners` was reverted (it does not type-check, see C).

## Build infrastructure (cost me most of the wall time)

- The unit cache restored `wasmtime` and `wasmtime-environ` from the fleet tier (and later from the local store) built against a different `wasmtime_environ` SVH, so `wiggle` / `wasmtime-wasi-io` failed with `E0463 can't find crate for wasmtime`. Reproduced with a bare rustc: "can't find crate for `wasmtime_environ` which `wasmtime` depends on". Workaround used: delete the `wasm*`/`cranelift*`/`wiggle*`/`pulley*`/`winch*`/`wast*` unit entries from `.cache/b/r/m/u/e` and from the lane, and run with `SEMIO_UNIT_CACHE_REMOTE=off`. The cache key does not capture the extern SVH, so a hit can be inconsistent with locally built dependencies; that is a cache defect for the owner of `unit-cache` (see `shared-store-vanishing-artifacts.md`).
- Disk filled twice during linking (`No space left on device`, 500 MB test binaries); reruns passed.
- The repo mcp server (`semio`) failed to connect (CONNECT_TIMEOUT) in this session, so the ticket was not closed with `ticket_close`.

## Sibling comparison

No sibling plugin has passing native lib tests in the current tree, so there was no passing fixture to diff against and no sibling regression to confirm; both candidates fail before any test runs:

- raster (`repo cargo ... -p semio-s-artifact-raster-raster-editor --lib`): `semio-s-artifact-raster-raster-schema` does not compile against the current framework (`NativeReceivingFailure` import, `RetirementCursor::close_step`, `RetireOwned::retirement`, `RetainedCloneCursor::advance`/`begin_close` no longer exist; `🖼️image-payload`, `🗺️owned-map`, `🎯️selection/📦️coverage`).
- procedural generation3d (`-p semio-s-artifact-procedural-3d-editor --lib`): `semio-s-artifact-procedural-3d-testkit` (`🧪️tests/🧰️support/…/🔬️retirement-driver/🦀️.rs:39,53`) calls the retirement driver API with its old two-argument/`.result` shape and does not compile.

The fixtures that raster tests use (`OriginalFixtureScope`, `dispatch_original_fixture_command`, `render_registered_fixture_app`, the io testing harness) are the same code the shooting tests use, so open cause A applies to them unchanged once they compile. The framework changes above touch only fixture/test-support code, `DecodeReport`, the two vcs history cursors, `CodecFrame` and the two typed snapshot frames; none of them is on a raster or procedural source path, and no stale-API sibling crate was edited.
