# Directory Capacity Boundary

## First Course Gate FixedVolume Diagnostic

**Actual diagnostic result:** the retry exited 1, expired false, before test collection. Refused meter root `/repo/.cache/b/l/m`, mode `reserve`, filesystem bytes 66,060,288, physical ceiling 67,108,864, used/measured bytes 4,435,968, request bytes 66,060,288, reserved/admitted bytes 0. The measured existing tree fits the physical ceiling; adding a fresh full payload envelope does not. This is measured usage, not a cached estimate. Exact caller/allocation ownership still requires inspection before selecting recovery semantics. Root removed the private diagnostic terminal 0, restoring exact prehash `62b9b4f0324b3458b086521c4b5ee70ebb79e7a0cdb0f05b73279739ab019b79`; build-input marks the meter diagnostic inactive. No test assertions passed and no permanent repair is inferred from the diagnostic.

After genuine full dependency sync succeeded, `run @semio-tech/course:test --configuration=release` exited 1 before test collection through FixedVolume meter admission (fixed.ts94, borrower.ts26). Root's next actual invocation repeats that same target with a private-only diagnostic at the unchanged meter gate. It records root, request mode, filesystem/physical limits, used/request bytes and measured/admitted estimates only on refusal. The exact four-overlay chain was verified first; directory Source prehash `62b9b4f0324b3458b086521c4b5ee70ebb79e7a0cdb0f05b73279739ab019b79`, diagnostic hash `fef88b1ffa1348d57dec4b8f2829235e9d603ada3b1d76e2bd8acb28375a0930`. Bun transpilation accepted it before writing. Receipt `🗑️generated/private-meter-instrument.json` and build-input temporaryDiagnostics mark it active. Root must restore the exact prehash after terminal before applying any permanent repair. Original guard/caps/custody remain unchanged; no cause or test success is inferred before actual metrics. Diagnostic log: `🗑️generated/2026-10-19-private-fixed-capacity-diagnostic.log`.

## Confirmed Runtime Failure

Parent’s actual private Original dependency sync exited 1 before the installer. Parent instrumented only the private pure capacity boundary, reran Original, recorded the five real values, and restored that temporary diagnostic afterwards. Exact observed DATA: filesystemBytes 2,146,435,072; maximumPhysicalBytes 2,147,483,648; reservedBytes 0; usedBytes 4,096; requestBytes 2,146,435,072. Parent’s terminal log is `private-sync-capacity.log`. This worker ran no private Original or private mutation.

The observed request equals the payload filesystem envelope F, which the repository budget derives as physical partition capacity C minus one MiB. Root metadata occupies four KiB. Thus U+R is F+4,096, exceeding F but remaining below C by 1,044,480 bytes. This is not a Docker outer physical-reservation observation and no outer reservation is inferred.

## Source Trace

`process/artifacts/native-build/storage/directory/🟦️.ts` admits with `used+request<=filesystemBytes` and `reserved+request<=maximumPhysicalBytes`. Its meter always supplies reservedBytes zero and measures actual allocated blocks beneath the tree. `storage/fixed/🟦️.ts` passes the immutable actual filesystem/physical partition bounds to that meter. Repository `caching/storage/enforcement/budget/🟦️.ts` derives F=C-1MiB and defaults maximumArtifactBytes to F. The Store reservation owner acquires exactly policy.maximumArtifactBytes before installer writes. The durable Native journal independently totals every reserved/committed/retiring allocation and enforces its unchanged physical capacity C.

The first Store therefore encounters a deterministic mismatch: the full payload entitlement is added to already allocated root metadata and checked against the payload envelope rather than the total physical envelope. The exact failure is reproduced algebraically from actual numbers, not inferred from a high-level installer error.

## Repair Analysis In Progress

No implementation edit or cap change has been made at this checkpoint. A proposed narrow boundary correction separately validates request<=F, durable reserved+request<=C and measured used+request<=C. This retains the journal and actual physical ceilings, accepts four KiB metadata plus the full F entitlement within C, and rejects physical C+1, already-over-C commits and requests above F. Existing meter fixtures currently use F as their physical cutoff and need deliberate semantic review against the intended F payload/C total distinction before changing those expected cases. A broader change could carry the actual durable reserved total into measurement, but that requires updating the request contract/callers and must not be introduced merely to bypass the concrete failure.

Root owns all managed/private runs and the temporary runner. No private cache bytes were modified by this worker, no current-source compiler claim is made, and no guessed physical capacity was admitted.

## Implemented Repair

Parent confirmed the narrow physical-C correction and retained the durable journal ceiling. The pure boundary now independently requires R<=F, U+R<=C and reserved+R<=C. Remaining bytes report the smaller measured/journal physical headroom after the request. The meter’s pre-refusal remeasurement uses C as its physical threshold. No numeric cache policy, filesystem envelope, global cap, journal cap, Original authority, compiler ingress or Docker backing check was changed. The schema documents the F payload/C whole-tree distinction without changing its closed field names. The authored one-MiB allowance now accommodates real directory/journal metadata rather than being unreachable behind an F whole-tree check.

Language-neutral Admission fixtures cover the actual 2GiB Store numbers, empty/full payload requests, metadata headroom, exact C, C+1, requests above F, already-over-C commit and durable reserved boundary/overrun. The SQLite oracle independently checks all three inequalities; Ajv independently validates actual schema admission. Existing many-small-write expectations now reach the already declared C physical ceiling, while each individual request remains bounded by F. Real-tree tests still refuse accumulated small files and a large write beyond C; no larger ceiling is introduced.

## Actual Focused Verification

The schema/fixture/lifecycle regressions were run first against the old implementation: two focused tests failed, including the four-KiB-plus-full-F entitlement case. Evidence: `🗑️generated/2026-10-19-directory-capacity-red-focused.log`. A prior incorrectly unprefixed Bun file selector began workspace discovery; only that worker-owned process was terminated, terminal143, and its log is retained without a passing claim. The corrected `./` selector executes only the intended file.

After the repair, the focused actual directory suite passed five tests /112 assertions, covering Admission+Ajv+SQLite, the existing running meter corpus, actual neutral host trees, commit/reset/reopen and the actual unchanged Native journal reducer. Evidence: `🗑️generated/2026-10-19-directory-capacity-final.log`. The journal test admits exactly C in aggregate and rejects C+1; it invokes the real reducer, whose implementation was not edited. A final test-only type-narrowing edit was followed by a repeated lifecycle check, pass one test/18 assertions, in `2026-10-19-directory-capacity-lifecycle-final.log`.

These direct focused source tests used host Bun1.3.14 under parent’s pure-test authorization. They ran no Original/dependency installer/compiler or private-builder mutation. Pinned private Bun1.4.2 and Linux behavior remain for parent’s actual managed retry; the direct test result is not relabeled as that integration gate.

## Warm Store And Persisted Payload

The current actual Store owner first reads the bounded charge receipt. With a receipt, it recovers that exact durable allocation ID. Without the receipt, it observes the real journal and recovers the unique matching Store owner; only absence of both paths requests a new full entitlement. `recoverNativeStorageReservationV1` retains the existing allocation under its journal transaction and does not call reserve. The subsequent Store recheck measures the current tree with request zero. A committed allocation is not committed or charged again.

The added Store test extracts the actual current reservation function through TypeScript AST, preserves its implementation body, uses neutral actual receipt file I/O and the real directory meter, and supplies explicit test ports for journal/lease/control dependencies. It does not issue or claim Native authority. The independent SQLite oracle shows a warm U=F+4096 tree fits C when recovered with request zero, while a genuinely new F request at that same U must refuse U+F>C. The actual source branch test passed one test/13 assertions: fresh acquisition once, receipt recovery once and journal-fallback recovery once. Its first test harness attempts exposed and corrected namespace-export/transpiler wrapper mistakes; only final `🗑️generated/2026-10-19-store-recovery-final.log` is the passing evidence.

If payload remains but both its receipt and durable journal funding are absent, this fix intentionally does not manufacture a recoverable charge: a full-F reservation over more than the existing metadata allowance still refuses. Ambiguous owners, retiring funding and actual measured U>C also remain refusals. Such a condition needs genuine existing custody plus authored charged maintenance/GC or a demonstrated caller-accounting defect; no source inspection here authorizes deleting an orphan tree or evicting another invocation. The parent-owned next Original run is required to determine the actual private Store outcome. This worker performed no full warm Store/Native integration run and makes no installer/release success claim.

## Changed Sources And Handoff Digests

- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts` — SHA-256 `62b9b4f0324b3458b086521c4b5ee70ebb79e7a0cdb0f05b73279739ab019b79`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧬️schema/🔣️.json` — SHA-256 `105360b8055268c6b3f9fad729b087b59f82af0ec400f8f93e832c0a78a88bcf`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧪️tests/🟦️.ts` — SHA-256 `ae893b16732ce8bbb0e7f9ab868c3fd7b6473956c9ebe824544d76784e8759bf`.
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🧪️tests/🧫️fixtures/🔣️.json` — SHA-256 `49860ac5462c971b329430b97bc9d567b6aede1e7fcba1cd00b3816a52523387`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🟦️.ts` — SHA-256 `45f9f86acb10ec7bb86700b3370660899bd7ca544e52aa229da5592ded90bdcd`.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🧪️tests/🔣️.json` — SHA-256 `7222e352d87771ba7b0f0fbf546c8d92a0fa2b126319233be892b58fb78c161a`.

Only the first two runtime/schema files are needed for parent’s minimal private fix transfer. The other four are permanent regressions and language-neutral fixture DATA. No runtime Store or journal implementation changed. All worker-owned processes and test sessions are terminal; no private builder command was launched by this worker.

## Actual Integration Outcome And Recovery Decision Tree

Parent's genuine repaired Original sync passed the capacity boundary and reached the real dependency installer. It then refused duplicate generated workspace identity before bindings changed; see `2026-10-19-workspace-payload.md` for that independent repair and exact provenance. This establishes the original four-KiB metadata refusal was removed without manufacturing installer/release success.

The following is current Source recovery analysis, not an instruction to remove private files:

1. A reservation failure before the journal append has not issued the requested Store allocation or installer bindings transition. The existing partition/image receipt still requires ordinary admission; the failure grants no deletion authority.
2. The Store opening owner (`⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts`) recovers the exact receipt allocation, or the unique matching durable journal owner if its receipt is absent. Neither path charges a new full payload. Missing both forms of funding requires genuine new reservation; ambiguous/retiring/mismatched funding refuses.
3. A settled partition is opened under its actual receipt/envelope/mapping guards. A `.publication` intent blocks ordinary settled admission; only the authored publication operation, holding its actual exclusion/custody, can validate bounded intent/candidate metadata, prove payload before/after and complete receipt publication. Do not delete pending intent or adopt unverified candidates manually.
4. Dependency storage (`⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`) holds queued whole-Store and Source-binding writers. It retains ready generation prefixes and current Source targets during obsolete-generation selection. An unready generation resumes only when immutable input capture matches; constrained discard of its own unready generation requires its authored no-receipt/path/symlink guards. Ready generations need their canonical receipt and installed payload verification.
5. An installer failure receives bounded/redacted diagnostics. Only the specifically classified cache-link refusal has the authored bounded Bun cache-purge retry. Other failures do not authorize ad hoc cache deletion. Successful installer output must pass installed-path/execution/barrier/ready-receipt checks before Source bindings publish.
6. A Source transition reports actual completed bindings, active directory/intent and retained capsules on incomplete failure. Preserve those receipts/capsules until genuine recovery settles them; an incomplete report explicitly does not prove whole allocation retirement.

No private builder/cache mutation or managed recovery command was executed by this worker during this inspection.

## Managed Tool Preparation Audit

This is a current-source command map for parent scheduling, not a claim that these prerequisites ran. `bun repo tools setup nx` provisions the selected scheduler and its pinned authorizers; `all` adds Bazel. The command does not install all Rust/WASM/browser tools. Scheduler setup should follow successful managed JavaScript lock/sync. Root `workspace:setup` additionally covers unrelated ecosystems, so it is not a focused course prerequisite.

The native dependency entry (`⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts`) always enters `withRepositoryCargoPreparationV1` with actual Original/control and sealed Store custody. Root `workspace:deps-wasm` prepares pinned wasm-pack 0.15.0, the wasm-bindgen-cli version from Cargo.lock and wasm32-wasip2; its graph includes Binaryen and `workspace:deps-trunk`, which prepares trunk 0.21.14 and wasm32-unknown-unknown. `workspace:deps-cargo` fetches/prepares all declared Cargo workspaces, so it is broader than one selected course target. `workspace:deps-tools` installs cargo-nextest 0.9.140 and cargo-llvm-cov 0.8.7 and is needed only when selected managed checks require them.

The compiler owner (`⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts`) provisions the repository-pinned toolchain through its authenticated Rustup owner; no raw compiler installation is needed. Current `rust-toolchain.toml` selects minimal nightly-2026-07-20 with rust-src, llvm-tools-preview and the two WASM targets. Linux glibc uses the configured toolchain self-contained rust-lld. Actual cc/make/g++/pkg-config from the prepared private OS remain legitimate prerequisites.

Course Vite explicitly uses canonicalWasmBuildVitePlugin; publication cannot assume JavaScript-only execution. Let the managed target graph select its precise compiler/unit dependencies. `@teaching/architecture-course:ci-browser-prepare` performs managed Playwright Chromium OS dependency preparation. The course E2E owner also checks actual Playwright Chromium/headless-shell installation markers and provisions missing browser bytes into the leased Tools path; a separate all-workspace browser target may duplicate that work. Run the release configuration through a fresh actual Original; none of this audit substitutes for compiler, image, stack, browser or course gate terminal evidence.


## Next Genuine Warm-Reader Failure (Diagnosis Pending)

The first real managed `run @semio-tech/course:test --configuration=release` after normalized sync exited1, expiredfalse, before Vitest collection/assertions. Runtime records UTC20:52:01.862→20:56:07.711 (245.849s), immutable command receipt `🗑️generated/private-build-20261019T205201-ace24a38.json`, log `2026-10-19-runtime-course-ts-release1.log`. Last visible activity is `build:parser-source:readText:601856`; the stack is actual directory Admission91→meter109→fixedport94→borrower26.

Read-only source confirms fixedport94 checks exact backing identity, rechecks physical mapping, calls `meter.admit(request)` then rechecks physical mapping again; it does not itself create/repeat an allocation. The generic acquisition always measures reserve with R=allocation.bytes before journal append. Existing Store warm recovery uses exact receipt allocation or unique matching journal owner and rechecks at R0; fresh acquisition alone charges a new full entitlement. Meter commit refreshes real U and resets its estimate, reserve refreshes before an estimated physical refusal, while recheck currently uses cached U+admitted until the timed refresh.

The stack/log lacks actual partition root, request mode, F/C/U/R and cache-estimate components. It cannot distinguish a legitimate actual U>C refusal, an unfunded fresh request over existing payload, or a stale conservative recheck estimate. No new implementation or guard change has been made at this checkpoint. Root was asked to collect bounded actual values; runtime separately owns caller/partition tracing. No private invocation or mutation by this worker.


## Actual Lease Infrastructure Cause And Repair

Parent’s bounded genuine diagnostic retry exited1, expiredfalse. The refused measured request was `/repo/.cache/b/l/m`, mode reserve, F66,060,288 (63MiB), C67,108,864 (64MiB), reserved0, U/measured4,435,968, request66,060,288, admitted0. U+R=70,496,256 exceeds C by3,387,392 bytes; U alone fits C. Thus the unchanged capacity predicate correctly refused a genuinely new full payload charge. This was not the possible cached recheck-estimate asymmetry. Parent restored the exact instrumented directory owner prehash afterward; this repair makes no directory/fixed/runtime funding edits.

Runtime’s finite frozen-Source trace found the actual caller in build input storage `🟨️.mjs`, `withGraphFactWriterV1`: its outer `withRepositoryCacheStoreV1(workspace,"leases",...)` existed only to retain the infrastructure lifetime while acquiring a queued graph-writer lease. It never used that Store allocation or wrote artifact payload. Generic Store acquisition therefore attempted the first `repository:leases-store` full-F artifact entitlement after infrastructure SQLite lease files already existed. Actual l/m had only data/, with no leases.json or storage-events journal; no recoverable prior artifact funding was invented. Exact caller/metadata evidence is generated `2026-10-19-runtime-leases-caller.json` and `2026-10-19-runtime-leases-metadata.json`; actual metrics are in `2026-10-19-private-fixed-capacity-diagnostic.log`. Initial scans restricted to TypeScript omitted this .mjs owner; runtime supplied and verified the concrete frozen caller before implementation.

The schema-first caller repair declares Leases as an infrastructure-reader role with no artifact reservation, and uses the existing actual `openRepositoryCachePartitionV1` owner. It holds that admitted image across the same queued exclusive graph-writer lease, unchanged optional maintenance and actual Graph Store callback. Physical/custody rechecks occur before queuing, after acquisition and after successful Graph Store work. The graph lock releases before awaited partition close on success/failure/cancellation. Explicit failure state preserves even an undefined primary throw while still settling cleanup. No existing allocation is adopted or retired, no missing receipt synthesized, no global policy/ceiling widened, and Graph artifact funding remains unchanged.

Test-first red reached the old actual function and observed one unwanted artifact reservation for the warm4.23MiB infrastructure fixture (0pass/1fail). The new test executes the actual current function body extracted by TypeScript AST with explicit test-only image/control/Graph Store ports; it does not mint Native/Original authority. Twelve closed language-neutral cases cover successful retained lifetime, open/recheck/queue/Graph/body errors, queue/body cancellation, distinct release/close failures, undefined primary cleanup and an actual U>C refusal. Queue admission yields asynchronously while the image remains open. The real unchanged directory capacity function still rejects the full-F overrun and physically over-C reader case. Ajv validates all cases; SQLite independently computes reader-versus-new-allocation admission and observed release-before-close ordering/counts.

Focused final direct run passed1test/193assertions, `2026-10-19-graph-lease-final.log`. Related closed-maintenance plus new regression passed2tests/201assertions, `2026-10-19-graph-lease-related.log`; execution counts overlap, not independent totals. The initial green10-case run is retained separately. Host Bun1.3.14 pure-source exception was used; no private Original, installer/compiler/build or private mutation by this worker. The real next managed course test remains parent-owned and has not yet been claimed successful. All own sessions are terminal.

### Exact Existing-Path Handoff

Only runtime and schema are functional; both existing regression files should accompany them. No new Source path/capture extension is needed.

| Absolute Source path | SHA-256 |
| --- | --- |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🟨️.mjs` | `01be703626a508c0d301e5e1bc0d7bebd33bca1be33384a17e8a4a90c6eda5bf` |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json` | `271c6b72c32e5718690737cb878ebcf504cb2805c689d7998b42a2d93f59ccf0` |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🟦️.ts` | `56d34e213643e6031f27eadca8d658e9a7a132baadd8d9b7e66377deb6f28b50` |
| `/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧪️tests/🔣️.json` | `ccce2e12cf347ab2288a015b35b47a65128f1ae3d755c15f038d17475a773203` |

Independent final source review matched all four handoff hashes and found no concrete issue in actual reader lifetime/rechecks, unchanged queued lock/control, release-before-close ordering or undefined-primary preservation. Reviewer ran no tests/private Original. Parent was notified; no further Source edits are pending.
