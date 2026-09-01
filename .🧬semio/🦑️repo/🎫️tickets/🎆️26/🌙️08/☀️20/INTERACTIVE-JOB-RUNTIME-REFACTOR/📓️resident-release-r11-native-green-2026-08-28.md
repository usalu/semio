# Resident Release R11 Native Result

## Actual Terminal

The one authorized canonical attempt exited **0**. Actual nextest run `ff0bb224-29e7-4505-addb-a5a8e2c8ad43`, exhaustive profile: **25 tests run,25 passed,0 skipped,one binary,0.149s**. Nx reported the target successful. This is the complete current standalone resident library selection, not Opening/Store/Plugin/WGPU verification.

[Complete raw tool output](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-resident-release-r11-2026-08-28.md) and [exact escaped tool-result strings](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release-r11-tool-output-2026-08-28.json) are preserved. The latter retains CRLF/ANSI bytes as JSON escapes; Markdown rendering is not the byte-exact stdout authority. No retry, repair, extra selector, Wasm command or quota change ran.

Source/catalog/compiler holds were explicitly released immediately after the terminal and post-capture. Subsequent peer edits are outside this snapshot.

## Exact Command And Runner Scope

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache
```

The existing target rejects all extra arguments and calls `runCargoTestBudgeted(["semio-framework-value-resident"], repoRoot, ["--lib"])`. Explicit exhaustive environment selects the unchanged existing profile. It does **not** add a no-fail-fast switch; none was invented or passed through this no-argument route. All25 actually completed successfully, so no fail-fast cancellation occurred.

The original target directory and jobs2/build budget3600000/coverage0 settings remain. No new output directory was selected. Actual binary metadata identifies `aarch64-apple-darwin` and the existing `nightly-2026-07-07-aarch64-apple-darwin` libdir, binary `debug/deps/semio_framework_value_resident-ca9d6776d76d4aa3`.

The build/list stage completed successfully, enabling the test run. Its compiler diagnostic file contains **eleven warnings plus one “11 warnings emitted” summary record**, zero error records. These were read after the run from the exact package fingerprint and preserved, not inferred absent because the runner's successful captured-build output was terse:

- [Full compiler diagnostic JSONL](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release-r11-compiler-diagnostics-2026-08-28.jsonl).
- [Rendered compiler warnings](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-release-r11-compiler-warnings-2026-08-28.md).

No warning suppression or deprecated API repair is included.

The preserved diagnostic JSONL was byte-compared with the original fingerprint:27925 bytes, SHA-256 `644e7bb4bda5de3a16cdb6d0058e136b62893bb46553c366bd692e14a0f4495a`, exact equality.

## Captured Source And Loader Boundary

[Before](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release-r11-before-2026-08-28.json) / [after](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release-r11-after-2026-08-28.json): **all73 declared full tuples equal**, including hash,size,device,inode,mtime; all read-stability flags true. Resident domain re-enumeration has16 members before/after, no additions/removals. [Manifest and exact capture command](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release-r11-capture-manifest-2026-08-28.json) are preserved.

The native repo-owned Rust closure comprises the canonical authority, canonical test/allocator module, included baseline Rust and included future-seven Rust. Their include_str inputs comprise the resident capacity fixture, admission fixture and ticket release JSON. Existing Cargo manifest/lock/toolchain/config, native test profile, package/domain routers, shared budget/loader/discovery/catalog and declared loader provenance are captured. This is the declared repo-source/loader envelope; third-party cached compiler/library binaries are identified by existing Cargo.lock/toolchain/binary metadata, not claimed independently byte-snapshotted.

Exact approved source:

| Input | SHA-256 |
| --- | --- |
| Resident authority | `e23ec4068c261ef56020e4aaafd97e3bd304a6503a58e9dc1b7a3c6de576dbd3` |
| Canonical tests and allocator | `e81bcca1121f724891f75f007322c61e61d46ea9dc71a601c413aeb6afbba175` |
| Included unchanged seven-law Rust | `8949cb8507c798758108a5b77d01221d8c87ff9d5feb2cd4f8522cca67d55019` |
| Included unchanged baseline Rust | `2e73f918e3100c7a232edf22032edfe87ab225ba2d40fa232c3814d5c4420c6f` |
| Ticket release JSON | `2c82d7ad51115a6c5d2dc85bec5d0b2c31818275dcd4f68d7995d6556dcf828c` |
| Taxonomy catalog | `7800b09ba8644260ba818e0aff7c51bbe9e6271a0bb374b3595790baa3b577d7` |
| Discovery | `5ef65775df39b8a8e435ffb48d6a7b41070364911b7e398de0f22cdc5b138956` |

The capture retains WGPU package/projection files as **loader provenance**, not compiled WGPU source or a WGPU test selection. Ticket controller/schema and source-review reports are also not Rust execution. Opening7, OS six-law/rejected-page and Plugin tests were not selected.

## Actual Output Availability And Source Roster

The canonical runner uses fail-only current/final test status. It printed the aggregate25/25 footer, not individual passing test names or passing stdout. Direct inspection of `semio-nextest-OXUFKu` found only `binaries-metadata.json`; **passing stdout was not captured by this invocation**. There is no claimed missing-output recovery and no rerun to fabricate it. [Artifact metadata/source roster record](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release-r11-artifacts-2026-08-28.json) preserves the actual existing metadata and source mapping.

The following25 names are independently read from the captured Rust source (17 original+baseline1+seven phases), not reconstructed individual nextest result lines. The aggregate actual run count matches this complete unfiltered source roster with zero skips:

| Source-declared test | Captured declaration |
| --- | --- |
| `tests::resident_capacity_consumes_the_shared_capacity_and_invalid_vectors` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:54) |
| `tests::resident_capacity_all_axes_refuse_before_safe_integer_overflow` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:68) |
| `tests::resident_capacity_data_and_control_are_disjoint_and_never_defaulted` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:91) |
| `tests::resident_capacity_constructor_owns_no_heap_backing` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:111) |
| `tests::resident_admission_native_layout_has_one_fixed_root_and_separate_move_costs` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:184) |
| `tests::resident_admission_short_and_foreign_refusals_preserve_live_consumer` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:205) |
| `tests::resident_admission_caller_loss_and_parent_move_preserve_original_page_and_consumer` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:249) |
| `tests::resident_admission_record_mutation_unwind_and_exact_parent_handoffs_never_cold_drop` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:282) |
| `tests::resident_admission_final_original_root_release_requires_its_own_grant` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:328) |
| `tests::resident_admission_first_access_refusal_allocation_boundary` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:341) |
| `tests::resident_admission_busy_access_preserves_original_root_without_allocation` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:364) |
| `tests::resident_admission_inline_gate_keeps_poison_sticky_after_callback_unwind` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:378) |
| `tests::resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:398) |
| `tests::resident_admission_exact_layout_and_partial_consumer_cancel_release_once` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:452) |
| `tests::resident_admission_injected_allocation_failure_preserves_original_reservations` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:482) |
| `tests::resident_admission_private_consumer_phase_qualifies_access_and_record_aliases` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:519) |
| `tests::resident_admission_all_three_page_backings_use_exact_layout_and_short_grants` | [source](/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/🦀️.rs:545) |
| `tests::release_baseline::resident_current_api_charge_remains_after_allocator_return` | [source](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🧪️baseline/🦀️.rs:43) |
| `tests::release_phases::resident_release_record_keeps_charge_after_actual_free` | [source](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🦀️.rs:149) |
| `tests::release_phases::resident_release_cancellation_covers_allocated_and_reserved_frontiers` | [source](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🦀️.rs:194) |
| `tests::release_phases::resident_release_short_grants_preserve_every_original_phase` | [source](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🦀️.rs:215) |
| `tests::release_phases::resident_release_aliases_block_destruction_and_live_payload_drop` | [source](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🦀️.rs:237) |
| `tests::release_phases::resident_release_concurrent_close_frees_and_refunds_once` | [source](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🦀️.rs:275) |
| `tests::release_phases::resident_release_poison_after_free_keeps_pointerless_charge` | [source](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🦀️.rs:303) |
| `tests::release_phases::resident_release_metadata_is_inline_and_measured_before_detach` | [source](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-release/🦀️.rs:338) |

The baseline's source iterates both Data and Control and performs exact post-free resource assertions; the current unfiltered run passed. Historical R10 remains Data semantic RED with Control unrun there. R11 does not invent per-partition numeric debug output or new physical size measurements absent from its retained stdout.

## Scope And Nonclaims

The accepted result covers the actual standalone tests for separate original-root Destroy/Free/Refund/Clear, aliases, cancellation/short grants/concurrent close, pointerless poison residue, metadata and original17 regression. The tests and capacities were unchanged; dynamic Layout expressions observed the candidate's actual compiled types. The report does not substitute model outcomes for this native result.

It does not prove original RuntimeAppCell/Opening funding, a private funded Store FIFO receiver, Store detach/SyncSession forwarding, whole-callback timing, arbitrary original-root loss, unknown/live poison disposal, or WGPU publication. The implicit close latch remains the reviewed candidate detail, not a new grant/limit. No live consumer adoption occurred in this executor packet.

All earlier REDs and raw evidence remain preserved. Native lane is idle after this single terminal.
