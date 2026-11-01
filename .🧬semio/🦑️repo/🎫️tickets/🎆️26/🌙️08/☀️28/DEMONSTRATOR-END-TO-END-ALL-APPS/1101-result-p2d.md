STATUS: WASM GREEN for the whole lane (lib, wasm32-wasip2, `p2d-w6a`/`p2d-w6b` rc 0). NATIVE LAWS PARTIAL: green = os-flow-drawing 42, os-vcs 4, 2d-regions 5, tool-run 5, ui-runtime-presentation 24 (+1); RED = see table (test-port debt and four semantic failures owned by the peer's value/2d/pixels migration). Nothing below is claimed beyond its log.

## Wasm (`--lib --target wasm32-wasip2`, `repo cargo --store=shared --configuration=prototype`)
- w6a, root `Cargo.toml`, rc 0 `Finished`: os-flow-drawing, os-kernel-neural-engine (+model, budgeted, registry), os-vcs, 2d-booleans, 2d-regions. Log `🗑️generated/1101/p2d-w6a.txt`.
- w6b, `🧰️framework/Cargo.toml`, rc 0 `Finished`: pixels, 2d-path-stroke, 2d-trace, 2d-retire, ui-runtime-presentation, mesh-engine, tool-run. Log `p2d-w6b.txt`.
- Workspace split (package specs fail otherwise): root resolves flow-drawing, neural-engine*, os-vcs, 2d-booleans/regions; framework workspace resolves pixels(+`-tests`), 2d-*, ui-runtime-presentation, mesh-engine, tool-run. Drivers: `1101-tools/p2d/{w,t,each,rerun-fw}.sh` (`MAN=` picks the manifest). Test-only `cargo test` on 2d-booleans/regions needs the framework manifest.
- Open design item `admit_native_resource_failure`: already redesigned in the tree (`UiNativeFailurePlace` wraps `OriginalDomainReservation<UiNativeResourceTurnFailure>`), compiles on wasm. Its native law file `semio-framework-ui-runtime-presentation --test native_resource_custody` has 4 of 5 FAILED (below), so the failure-place behaviour is NOT proven.

## Fixes I made (all compile-verified by w6/e-runs unless noted)
- `📡️replication/🎮️mutation/♻️retirement/🦀️.rs`: three `return RetainedWorkOutcome{..}` inside a `Result` closure -> `return Err(ValueError::invariant(..))` (was blocking replication-mutation-messages and so the whole framework closure).
- `🏗️mesh-engine/🌲️canonical/📥️decode/🦀️.rs`, `🧠️neural/⚙️engine/🌱️cache/🦀️.rs:37`: `?` inside `close_original_retained_step` closures (now return `RetainedWorkOutcome`) -> early `return RetainedWorkOutcome{result:Err(..),work:0}`.
- Tests ported to the final API: pixels companion `♻️retirement/🧪️tests/🦀️.rs` (BooleanJob/BitmapTraceJob/PathBooleanJob infallible `new`, `advance(budget,ledger)` via `funded_turn`; the three physical boolean/trace tests PASS in `p2d-e1b-pixels-tests.txt`), `🌿️vcs/🗃️history-ledger/♻️retirement/🦀️.rs` (`retire_owned` returns `RetainedWorkOutcome`, read birth now advances to `Complete`; os-vcs 4/4 green `p2d-e3a-os-vcs.txt`), `🧠️neural/⚙️engine/🧵️retirement/🦀️.rs` (+ new `ValueRetirement::next_reserve_depth_demand`, test grant depth taken from it; model 16/18 green `p2d-e4a-...model.txt`) and its test `RetirementStep::Complete(0)/Progress(..,1)`, `◻️2d/🔀️booleans/🛤️paths/🧪️tests` (`drain` u64), `◻️2d/🔍️trace/{🧪️tests,🎟️work/🧪️tests}` (trait path `retirement::RetireOwned`, ledger import), registry test stall law now `progress != default || Complete` (UNVERIFIED effect: still stalls), mesh canonical test `canonical()` on `admission_owned` (UNVERIFIED, crate test target still blocked by the decode test).
- `[DEBUG]` text I added to the stroke test is removed again.

## Native per-crate results (logs `🗑️generated/1101/p2d-e{1,2,3,4,5}*`)
| crate | result | cause / owner |
| os-flow-drawing | GREEN 42/42 | `p2d-e1a-os-flow-drawing.txt` |
| os-vcs | GREEN 4/4 (after my port) | `p2d-e3a-os-vcs.txt` |
| 2d-regions | GREEN 5/5 | |
| 2d-retire | GREEN (0 tests) | |
| tool-run | GREEN 5/5 | |
| ui-runtime-presentation | 24 ok, `native_resource_custody` 1 ok / 4 FAILED | `describe_mesh_on_original` returns None (`📦️resources/🧪️tests/🦀️.rs:78`), receiver `Busy` (:129,:134), "original mesh packet failed its exact receiving frontier" (:38). Behavioural, ui-scene mesh receiver/failure place; not fixed |
| os-kernel-neural-engine | 55/56, 1 FAILED `dictionary_writer_source_preserves_original_text_and_independent_json_bytes` | `NativeEncodeControl::charge` OwnershipLimit (`🌱️value/🛫️encode/🦀️.rs:133`, 4 MiB cumulative over thousands of 0/1-item turns). Owner: value-encode/pack-json |
| neural-engine-model | 16/18; 2 FAILED copied-bytes law (`🧵️retirement/🧪️tests/🧵️retirement/🦀️.rs:37,56`: copied 0 vs 11, 48 vs 59) | copied-byte accounting of the retirement port changed; decide intended law with value-retirement owner |
| neural-engine-registry | 7/9, 2 FAILED `registry_*` "original registry grant stalled" (turn 206/450) | `RegistryRetirement::close_step` -> `ControlledRetirement::step` returns an empty `Progress` while not terminal; likely depth/reserve demand under-report like the model's queue reservation (`📋️queue/🦀️.rs:29`). Owner: value-retirement |
| neural-engine-budgeted | test build RED | `⏱️evaluation/🧪️tests/🦀️.rs` uses removed `semio_framework_job::StepContextOwner` (orphaned since HEAD; replacement `OriginalJobFixtureCaller`, which has no per-turn fuel/deadline) |
| mesh-engine | test build RED | `🌲️canonical/📥️decode/🧪️tests/🦀️.rs` same `StepContextOwner`; its fuel/deadline-denial laws need a faithful redesign, not a rename |
| 2d-booleans | 24/27, 3 FAILED | `paths::tests::cancellation_covers_every_path_stage...` (phase `Admitting` vs expected), `path_boolean_retirement_composes_child_owners...` (`🛤️paths/🧪️tests/🦀️.rs:23`), `tests::translated_coordinates_preserve_topology` ("Boolean epsilon is below coordinate precision"). Peer's ◻️2d port; not fixed |
| 2d-path-stroke | 5/6, 1 FAILED `large_crossing_strokes_fit_coverage_and_dash_expansion_is_bounded` | with `dash=[1e-6,1e-6]` no "budget" error arrives within 1000 turns of 4096 (error was `None`): the ledger-priced turns no longer reach `MAX_POINTS` in that window; recalibrate the turn bound or the dash |
| 2d-trace | test build RED | tests pass borrowed `&[u8]` masks but `BitmapTraceJob<M>` retirement needs `M: RetireOwned` (`🧪️tests/🦀️.rs:27,36,37`); convert to owned `Vec<u8>` + `.clone()` |
| pixels (lib tests) | test build RED | `📷️png/📥️decode/♻️close/🧪️tests/🦀️.rs` is mounted in the lib (`📷️png/📥️decode/🦀️.rs:204`) but needs `crate::retirement::tests::observed` + serde_json, which only the companion has; belongs in the companion (testsplit) |
| pixels-tests (companion) | 67 passed, 15 FAILED, 4 HANG (killed by me, pid recorded) | failed: compositing x6 (`compositor_*`), affine sampling x1, png_decoding x3, image_decoding x3, `retained_physical_mask_leases...`. Hang (>2000 s, 88 CPU-min): `blocking_decode_matches_the_independent_png_oracle...`, `blocking_decode_reads_jpeg...`, `encoded_orientation_reads_jpeg...`, `png_decode_truncated_fixture_prefixes_never_publish` — suspect an unbounded loop in the ported decode path. Log `p2d-e5-pixels-tests.txt` (those four skipped) |

## Not done / not verified
- wasm32-wasip2 test targets (only lib check done). Native law compile of mesh-engine and neural-engine-budgeted. The failures in the table above are not fixed.
- Raster workspace and `semio-framework-pixels` plugin callers (surface-paint, raster-io, editor) were not rechecked in this run.
