# Resident Primary Native R1 — Compile RED

## Actual Terminal

The single authorized canonical attempt exited **1 during Rust lib-test compilation**: **65 coded errors,9 warnings,0 tests executed**. No nextest test-run ID or current successful binary inventory was produced. The source declares32 tests (prior25 plus7); this is not32 executed failures.

The compiler's actual terminal is:

```text
error: could not compile `semio-framework-value-resident` (lib test) due to 65 previous errors; 9 warnings emitted
NX Running target test for project @semio-tech/value-resident-rs failed
```

All source/catalog holds and the sole native slot were explicitly released after terminal/postcapture. No retry, fix, Wasm/OS/Store/Plugin/WGPU selection, loader rerun, source publication, quota change or cleanup occurred.

## Complete Evidence

- [Readable complete tool output](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️member-resident-primary-native-r1-2026-08-28.md).
- Exact tool-result strings: [initial chunk01](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-chunk-01-2026-08-28.json), [terminal chunk02](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-chunk-02-2026-08-28.json). The terminal is chunk fd149f,7240 characters, exit1.
- [Full original compiler JSONL](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-compiler-diagnostics-2026-08-28.jsonl):274148bytes, SHA **404da071c9f337be7762f3e0c0b4422dc550335c1e81b08750b7b109dbaee70e**; actual cmp0 against the package fingerprint output. All77 records are preserved:65 coded errors,9 warnings,one abort-summary error record andtwo failure notes.
- [All rendered diagnostics](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/📓️resident-primary-native-r1-full-diagnostics-2026-08-28.md): SHA **c8048d4f5363826beb4917b98961991a79b706237007c8fe9581ed28b3287e42**.
- [Structured diagnostic summary](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-diagnostic-summary-2026-08-28.json) and [terminal result](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-result-2026-08-28.json).
- [Captured source roster](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-source-roster-2026-08-28.md):32 actual source declarations, not reconstructed test-run output.

The shared runner prints compact failure headlines; those are not substituted for the full compiler details. The current fingerprint was read and preserved before any further authorized native work:

```text
/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad/debug/.fingerprint/semio-framework-value-resident-ca9d6776d76d4aa3/output-test-lib-semio_framework_value_resident
```

The first terminal retrieval completed and stored fd149f in orchestrator memory. A subsequent attempt to store its absent session_id raised an orchestration serialization error; a retrieval-only poll then confirmed the session was already closed. The original stored terminal result was saved unchanged to chunk02. This was not a second Cargo/Nx invocation and no raw terminal output is missing.

## Exact Owner, Command and Capture

Owner: /root/retained_child_publication. The existing resident route received **no extra arguments**:

```sh
NX_DAEMON=false NX_CACHE_PROJECT_GRAPH=false NX_ISOLATE_PLUGINS=false CARGO_BUILD_JOBS=2 CARGO_TARGET_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧱️cargo-target-cad' SEMIO_TEST_ARTIFACT_DIR='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️native-artifacts' SEMIO_BUILD_BUDGET_MS=3600000 RUST_BACKTRACE=1 SEMIO_COVERAGE=0 SEMIO_TEST_LEVEL=exhaustive bun x nx run @semio-tech/value-resident-rs:test --skip-nx-cache
```

Its current script calls runCargoTestBudgeted(["semio-framework-value-resident"], repoRoot, ["--lib"]) and rejects arguments. Explicit exhaustive environment remains; no invented no-fail-fast flag was passed through the no-argument route. The compiler failed before any behavior/fail-fast distinction could arise. Jobs2, master retained cargo-target-cad, build ceiling3600000, coverage0 and existing artifact directory stayed unchanged.

[Exact manifest](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-capture-manifest-2026-08-28.json) / [unchanged established nofollow controller with this manifest path](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-capture-command-2026-08-28.json).

Fresh [before](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-before-2026-08-28.json) at2026-08-28T05:10:04.215Z and [after](/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/INTERACTIVE-JOB-RUNTIME-REFACTOR/🧪️resident-primary-native-r1-after-2026-08-28.json) at2026-08-28T05:10:51.262Z: **all80 full rows equal;16 resident-domain members equal; all reads stable;0 critical hash mismatches; no missing input**. Full comparison includes hash,bytes,device,inode,mode,nlink,mtime,ctime and read-stability fields. This wall interval is capture provenance, not a callback/test timing measurement.

The source envelope retains all prior73 declared resident/loader inputs, adds the four primary Rust/JSON/schema/controller files, the newly held normalization source, and the exact release/current-loader receipts. The last two are **provenance, not compiled Rust**. Current catalog f56de40a, D5ef65775, N00a0c985, rootSb505485c, launchseedcd173452/outputa558a308 and resident package metadata match the root-reviewed hold. No WGPU projected file or TS/schema/controller was counted as executed Rust.

Native repo-owned closure is five Rust files: canonical authority/test module, original release baseline and seven-phase child, and new primary child. The four embedded JSON inputs are canonical capacity/admission plus release/primary ticket JSON. Cached third-party dependencies/compiler are identified by Cargo.lock/toolchain and the retained target; this is not a claim to byte-snapshot the entire cache. A comm-only process check after terminal found no cargo/rustc/cargo-nextest executable.

## Source Attribution

All65 coded errors have primary spans in the new ticket primary Rust leaf. The nine warnings are separate:five unused-qualification warnings in the existing test module andfour Atomic::fetch_update deprecation warnings in the unchanged authority. No warning was suppressed or repaired.

| Code | Count | Attribution |
| --- | ---: | --- |
| E0432 | 1 | Intended missing import surface:five proposed primary/recovery types at line5. |
| E0599 | 46 | Intended absent calls to six proposed root methods. |
| E0609 | 16 | Intended absent primary/recovery/registration metadata needed by same-crate observations. |
| E0109 | 1 | Independent fixture shadowing error at line690. |
| E0618 | 1 | Same fixture shadowing error at line690, not an additional behavior defect. |

Thus63 diagnostics expose the deliberately unimplemented API/metadata, whiletwo diagnose one actual fixture source defect. Neither group executes any ownership, null-allocation, short-grant, poison, close or overflow assertion.

### Missing API Surface

E0432 names ResidentPrimaryAnchor, ResidentPrimaryBacking, ResidentRecoveryCursor, ResidentRecoveryMode and ResidentRecoveryPin. It is one compiler error for five imports, not five failed tests.

| Missing method | Diagnostics | Current ticket source lines |
| --- | ---: | --- |
| `reserve_primary_consumer` | 11 | 174, 267, 304, 326, 385, 471, 652, 676, 693, 703, 707 |
| `begin_primary_recovery` | 8 | 185, 354, 517, 572, 645, 651, 655, 660 |
| `prepare_primary_consumer` | 9 | 175, 275, 278, 281, 306, 333, 386, 448, 474 |
| `advance_primary_recovery` | 7 | 193, 357, 522, 527, 531, 574, 579 |
| `begin_primary_consumer_close` | 7 | 201, 241, 352, 388, 429, 480, 654 |
| `capture_primary_consumer` | 4 | 189, 535, 544, 549 |

| Missing metadata access | Diagnostics | Current ticket source lines |
| --- | ---: | --- |
| `ResidentAccessGuard<'_, LedgerState>.primary` | 4 | 130, 246, 254, 395 |
| `&ConsumerHeader.registration` | 1 | 103 |
| `&ConsumerHeader.recovery_pins` | 1 | 103 |
| `&LedgerState.primary` | 3 | 106, 114, 115 |
| `&LedgerState.last_consumer_registration` | 1 | 114 |
| `&ConsumerPage.registration` | 1 | 116 |
| `&LedgerState.recovery` | 4 | 117, 118, 119, 119 |
| `ResidentAccessGuard<'_, LedgerState>.last_consumer_registration` | 1 | 691 |

These are observed missing surfaces, not authorization for their implementation or evidence that the eventual layout/ownership semantics are correct.

### Independent Fixture Shadowing

Actual E0618 names the exact source chain:

```text
179 fn ordinary<C: Send + 'static>(...) -> Result<(), ResidentFault>
682 let (before, after, primary, ordinary, pe, oe) = capacity_observation?;
690 ordinary::<u8>(&exhausted, ResidentPartition::Control)?;
```

At690, ordinary is the local Result<ResidentStep,_> bound at682, not helper179. E0109 rejects generic arguments on that local value; E0618 rejects calling it. The compiler includes both the helper and shadowing binding spans. This exists independently of the missing primary APIs and must not be relabelled as intended missing-API RED. I did not rename the binding, alter assertions, mount an implementation or rerun. Root/Dag own the next reviewed correction boundary.

## Complete Coded-Error Index

Record numbers refer to the77-record JSONL/rendered appendix; all listed lines are in the captured new primary ticket leaf. Detailed secondary spans, source snippets, suggestions and explanations remain in the full original JSONL.

| Record | Code | Primary line(s) | Actual message |
| ---: | --- | --- | --- |
| 1 | E0432 | 5 | unresolved imports `crate::ResidentPrimaryAnchor`, `crate::ResidentPrimaryBacking`, `crate::ResidentRecoveryCursor`, `crate::ResidentRecoveryMode`, `crate::ResidentRecoveryPin` |
| 11 | E0599 | 385 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 12 | E0599 | 645 | no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 13 | E0599 | 572 | no method named `begin_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope |
| 14 | E0599 | 326 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 15 | E0599 | 471 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 16 | E0599 | 267 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 17 | E0599 | 517 | no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 18 | E0599 | 386 | no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 19 | E0599 | 333 | no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 20 | E0599 | 522 | no method named `advance_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 21 | E0599 | 574 | no method named `advance_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope |
| 22 | E0599 | 275 | no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 23 | E0599 | 474 | no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 24 | E0599 | 651 | no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 25 | E0599 | 388 | no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope |
| 26 | E0609 | 395 | no field `primary` on type `ResidentAccessGuard<'_, LedgerState>` |
| 27 | E0599 | 527 | no method named `advance_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 28 | E0599 | 352 | no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope |
| 29 | E0599 | 579 | no method named `advance_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope |
| 30 | E0599 | 278 | no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 31 | E0599 | 652 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 32 | E0599 | 480 | no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope |
| 33 | E0599 | 531 | no method named `advance_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 34 | E0599 | 354 | no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 35 | E0599 | 281 | no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 36 | E0599 | 654 | no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope |
| 37 | E0599 | 535 | no method named `capture_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 38 | E0599 | 429 | no method named `begin_primary_consumer_close` found for struct `ResidentLedgerRoot` in the current scope |
| 39 | E0599 | 241 | no method named `begin_primary_consumer_close` found for reference `&ResidentLedgerRoot` in the current scope |
| 40 | E0609 | 246 | no field `primary` on type `ResidentAccessGuard<'_, LedgerState>` |
| 41 | E0609 | 254 | no field `primary` on type `ResidentAccessGuard<'_, LedgerState>` |
| 42 | E0599 | 544 | no method named `capture_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 43 | E0599 | 357 | no method named `advance_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 44 | E0599 | 655 | no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 45 | E0599 | 185 | no method named `begin_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope |
| 46 | E0599 | 549 | no method named `capture_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 47 | E0599 | 304 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 48 | E0609 | 130 | no field `primary` on type `ResidentAccessGuard<'_, LedgerState>` |
| 49 | E0599 | 448 | no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 50 | E0599 | 174 | no method named `reserve_primary_consumer` found for reference `&ResidentLedgerRoot` in the current scope |
| 51 | E0609 | 103 | no field `registration` on type `&ConsumerHeader` |
| 52 | E0609 | 103 | no field `recovery_pins` on type `&ConsumerHeader` |
| 53 | E0599 | 660 | no method named `begin_primary_recovery` found for struct `ResidentLedgerRoot` in the current scope |
| 54 | E0609 | 106 | no field `primary` on type `&LedgerState` |
| 55 | E0599 | 189 | no method named `capture_primary_consumer` found for reference `&ResidentLedgerRoot` in the current scope |
| 56 | E0609 | 114 | no field `last_consumer_registration` on type `&LedgerState` |
| 57 | E0609 | 114 | no field `primary` on type `&LedgerState` |
| 58 | E0609 | 115 | no field `primary` on type `&LedgerState` |
| 59 | E0609 | 116 | no field `registration` on type `&ConsumerPage` |
| 60 | E0609 | 117 | no field `recovery` on type `&LedgerState` |
| 61 | E0609 | 118 | no field `recovery` on type `&LedgerState` |
| 62 | E0609 | 119 | no field `recovery` on type `&LedgerState` |
| 63 | E0609 | 119 | no field `recovery` on type `&LedgerState` |
| 64 | E0599 | 175 | no method named `prepare_primary_consumer` found for reference `&ResidentLedgerRoot` in the current scope |
| 65 | E0599 | 306 | no method named `prepare_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 66 | E0599 | 676 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 67 | E0599 | 193 | no method named `advance_primary_recovery` found for reference `&ResidentLedgerRoot` in the current scope |
| 68 | E0109 | 690 | type arguments are not allowed on local variable |
| 69 | E0618 | 690 | expected function, found `Result<ResidentStep, _>` |
| 70 | E0609 | 691 | no field `last_consumer_registration` on type `ResidentAccessGuard<'_, LedgerState>` |
| 71 | E0599 | 201 | no method named `begin_primary_consumer_close` found for reference `&ResidentLedgerRoot` in the current scope |
| 72 | E0599 | 693 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 73 | E0599 | 703 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |
| 74 | E0599 | 707 | no method named `reserve_primary_consumer` found for struct `ResidentLedgerRoot` in the current scope |

## Nonclaims and Next Boundary

The original25 tests were part of the unfiltered source selection but **did not execute** in this run. Historical resident R11 remains its own25/25 result. R1 proves only this current whole lib-test compile refusal and exact unchanged capture, not primary recovery semantics, live Opening funding, Store FIFO authority or Runtime/Plugin/WGPU behavior.

All artifacts remain in the master ticket. Native slot is idle and holds are released. No next command is scheduled or authorized by this report.
