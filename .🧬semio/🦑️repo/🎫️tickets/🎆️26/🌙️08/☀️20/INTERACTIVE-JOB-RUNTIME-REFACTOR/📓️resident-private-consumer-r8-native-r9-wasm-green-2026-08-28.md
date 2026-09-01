# Resident Private Consumer: R8 Native And R9 Wasm

## Actual Outcome

R8 canonical native target exited0: **17 tests run,17 passed,0 skipped**, one library binary, nextest exhaustive profile,0.038s reported test duration. Actual nextest run ID:8f1757c9-df2e-436b-bd2e-8267c768b123. The metadata identifies aarch64-apple-darwin and the retained master target below.

Only after this complete native PASS, R9 canonical check-wasm exited0. The unchanged router runs wasm32-wasip2 first and wasm32-unknown-unknown second. Actual completion lines report0.32s and0.39s respectively. These are two compile checks, no Wasm test/runtime execution.

The source508b78726ae6747f476fdb7d60938b3d2349ea300ef8fc55d555502a3500c49f and testsebde45c9d5ff7f5276e7a33f464601c23b6018d3e412c67616beaeea488f297e match the original immutable17 release. Discovery5ef65775df39b8a8e435ffb48d6a7b41070364911b7e398de0f22cdc5b138956 matches the root-read taxonomy phase release. All64 selected regular-file tuples were identical before native, after native, and after Wasm. Full captures: [selected inputs](./📓️resident-private-consumer-r8-selected-inputs-2026-08-28.md).

No production, test, feature, limit, pool, source pin, generated output, target path or runner was changed by this packet. Source/compiler holds were released immediately after R9 and post-capture; process inspection reported no cargo/rustc/cargo-nextest process.

## Invocation And Evidence Boundaries

Native: existing @semio-tech/value-resident-rs:test accepts no arguments and calls runCargoTestBudgeted for only semio-framework-value-resident --lib. Explicit exhaustive environment, coverage0, jobs2, build budget3600000ms, existing retained Cargo target/native-artifact root. This router does not append --no-fail-fast; the actual all17 pass/zero skip is the complete observed result, not a claim about a modified fail-fast setting.

The source roster below contains17 actual fn declarations. The native footer independently reports17 run from one binary with0 skipped; no narrowed filter or extra selector was passed. Passing per-test stdout was **not captured by this invocation**. The actual retained artifact directory contains only binaries-metadata.json, not passing test logs. No rerun, reconstructed stdout or inferred DEBUG values.

Native artifact path:
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-Qctg94

Native output ends with Nx's advisory that it detected a flaky task. This advisory is preserved verbatim; there was no native failure, retry or second invocation in this R8 packet. Do not infer a flaky assertion from that text.

Both Wasm checks print four existing AtomicUsize::fetch_update deprecation warnings at source471/480/567/647. They are retained unmodified; no cargo fix or compatibility change was run.

Standalone private-consumer structural laws are the result. No actual RuntimeAppCell/Opening parent funding, registered Store/FIFO field authority, separate free/refund protocol, callback completion, UI/WGPU admission, Plugin lifecycle, OS-kernel six-law execution, production publication or full-repository readiness is inferred. R7's pre-Cargo discovery failure remains preserved as historical failure, not rewritten as success.

## Actual Native Command And Complete Output

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache
```

```text

> nx run @semio-tech/value-resident-rs:test

> bun ./📜️script.ts test

────────────
[32;1m Nextest run[0m ID [1m8f1757c9-df2e-436b-bd2e-8267c768b123[0m with nextest profile: [1mexhaustive[0m
[32;1m    Starting[0m [1m17[0m tests across [1m1[0m binary
────────────
[32;1m     Summary[0m [   0.038s] [1m17[0m tests run: [1m17[0m [32;1mpassed[0m, [1m0[0m [33;1mskipped[0m
[0m[31m[DEBUG] Nextest artifacts retained at /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts/semio-nextest-Qctg94[0m



 NX   Successfully ran target test for project @semio-tech/value-resident-rs



 NX   Nx detected a flaky task

  @semio-tech/value-resident-rs:test

Flaky tasks can disrupt your CI pipeline. Automatically retry them with Nx Cloud. Learn more at https://nx.dev/ci/features/flaky-tasks


```

## Actual Wasm Command And Complete Output

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive bun x nx run @semio-tech/value-resident-rs:check-wasm --skip-nx-cache
```

```text

> nx run @semio-tech/value-resident-rs:check-wasm

> bun ./📜️script.ts check-wasm

    Checking semio-framework-value-resident v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust)
warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:647:43
    |
647 | ...   unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_|...
    |                                         ^^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default
help: replace the use of the deprecated method
    |
647 -         unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
647 +         unsafe { pointer.as_ref().aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
    |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:471:24
    |
471 | ...   header.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Cou...
    |                      ^^^^^^^^^^^^
    |
help: replace the use of the deprecated method
    |
471 -         header.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
471 +         header.aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:480:60
    |
480 | ...   unsafe { source.pointer.as_ref().header.admissions.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_ad...
    |                                                          ^^^^^^^^^^^^
    |
help: replace the use of the deprecated method
    |
480 -         unsafe { source.pointer.as_ref().header.admissions.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
480 +         unsafe { source.pointer.as_ref().header.admissions.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
    |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:567:22
    |
567 | ...   node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |                    ^^^^^^^^^^^^
    |
help: replace the use of the deprecated method
    |
567 -         node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
567 +         node.aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |

warning: `semio-framework-value-resident` (lib) generated 4 warnings (run `cargo fix --lib -p semio-framework-value-resident` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.32s
    Checking semio-framework-value-resident v0.1.0 (/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust)
warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:471:24
    |
471 | ...   header.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Cou...
    |                      ^^^^^^^^^^^^
    |
    = note: `#[warn(deprecated)]` on by default
help: replace the use of the deprecated method
    |
471 -         header.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
471 +         header.aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:480:60
    |
480 | ...   unsafe { source.pointer.as_ref().header.admissions.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_ad...
    |                                                          ^^^^^^^^^^^^
    |
help: replace the use of the deprecated method
    |
480 -         unsafe { source.pointer.as_ref().header.admissions.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
480 +         unsafe { source.pointer.as_ref().header.admissions.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
    |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:567:22
    |
567 | ...   node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |                    ^^^^^^^^^^^^
    |
help: replace the use of the deprecated method
    |
567 -         node.aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
567 +         node.aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?;
    |

warning: use of deprecated method `std::sync::atomic::Atomic::<usize>::fetch_update`: renamed to `try_update` for consistency
   --> 🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust/../../🦀️.rs:647:43
    |
647 | ...   unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_|...
    |                                         ^^^^^^^^^^^^
    |
help: replace the use of the deprecated method
    |
647 -         unsafe { pointer.as_ref().aliases.fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
647 +         unsafe { pointer.as_ref().aliases.try_update(Ordering::AcqRel, Ordering::Acquire, |count| count.checked_add(1)).map_err(|_| ResidentFault::Count)?; }
    |

warning: `semio-framework-value-resident` (lib) generated 4 warnings (run `cargo fix --lib -p semio-framework-value-resident` to apply 4 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 0.39s



 NX   Successfully ran target check-wasm for project @semio-tech/value-resident-rs



```

## Actual Binary Metadata And Source Roster

Metadata was read after the run. The following source roster is a source inspection, not independently printed per-test success output.

```text
{"rust-build-meta":{"target-directory":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad","build-directory":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad","base-output-directories":["debug"],"non-test-binaries":{},"build-script-out-dirs":{},"build-script-info":{},"linked-paths":[],"platforms":{"host":{"platform":{"triple":"aarch64-apple-darwin","target-features":"unknown"},"libdir":{"status":"available","path":"/Users/ueli/.rustup/toolchains/nightly-2026-07-07-aarch64-apple-darwin/lib/rustlib/aarch64-apple-darwin/lib"}},"targets":[]},"target-platforms":[{"triple":"aarch64-apple-darwin","target-features":"unknown"}],"target-platform":null},"rust-binaries":{"semio-framework-value-resident":{"binary-id":"semio-framework-value-resident","binary-name":"semio_framework_value_resident","package-id":"path+file:///Users/ueli/Documents/semio/%F0%9F%A7%B0%EF%B8%8Fframework/%F0%9F%94%A8%EF%B8%8Fmodules/%F0%9F%8C%B1%EF%B8%8Fvalue/%F0%9F%92%BE%EF%B8%8Fresident/%F0%9F%93%A6%EF%B8%8Fpackages/%F0%9F%A6%80%EF%B8%8Frust#semio-framework-value-resident@0.1.0","kind":"lib","binary-path":"/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/deps/semio_framework_value_resident-ca9d6776d76d4aa3","build-platform":"target"}}}53:fn resident_capacity_consumes_the_shared_capacity_and_invalid_vectors() {
67:fn resident_capacity_all_axes_refuse_before_safe_integer_overflow() {
90:fn resident_capacity_data_and_control_are_disjoint_and_never_defaulted() {
110:fn resident_capacity_constructor_owns_no_heap_backing() {
183:fn resident_admission_native_layout_has_one_fixed_root_and_separate_move_costs() {
204:fn resident_admission_short_and_foreign_refusals_preserve_live_consumer() {
248:fn resident_admission_caller_loss_and_parent_move_preserve_original_page_and_consumer() {
281:fn resident_admission_record_mutation_unwind_and_exact_parent_handoffs_never_cold_drop() {
327:fn resident_admission_final_original_root_release_requires_its_own_grant() {
340:fn resident_admission_first_access_refusal_allocation_boundary() {
363:fn resident_admission_busy_access_preserves_original_root_without_allocation() {
377:fn resident_admission_inline_gate_keeps_poison_sticky_after_callback_unwind() {
397:fn resident_admission_foreign_repopulation_cannot_trigger_last_consumer_drop() {
451:fn resident_admission_exact_layout_and_partial_consumer_cancel_release_once() {
481:fn resident_admission_injected_allocation_failure_preserves_original_reservations() {
518:fn resident_admission_private_consumer_phase_qualifies_access_and_record_aliases() {
544:fn resident_admission_all_three_page_backings_use_exact_layout_and_short_grants() {

```

## Preserved Raw Artifacts

- [R8 native raw](./🧪️member-resident-private-consumer-r8-2026-08-28.md)
- [R9 Wasm raw](./🧪️member-resident-private-consumer-wasm-r9-2026-08-28.md)
- [64-record pre/post source capture](./📓️resident-private-consumer-r8-selected-inputs-2026-08-28.md)

No cleanup, deletion, relocation, publication or source repinning was performed.
