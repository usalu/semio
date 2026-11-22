# Native Layout IO Storage Recovery

The previous locked macOS run, session 83968, exited 101 at 17:37 CET. The repository reported zero free space in the mounted 40.00 GiB compiler image after managed cleanup. Rust could not create a file encoder while compiling `semio-os-directory-schema`. No test result was recorded in that invocation.

At 19:06 CET, the fleet reported 61.8 GiB free on the host. The managed Rust partition was measured at 19.99 GiB used, with a 32.80 GiB machine-budget quota and a 40.00 GiB mounted image. The available image space and machine-budget allowance were approximately 20.01 GiB and 12.81 GiB respectively. The cache ledger reported zero eviction due. The compiler lane was free of Layout I/O owners. Cargo PID 26304 was queued for a published XLSX history test.

One Layout I/O retry was launched at 19:07 CET as exec session 82445. It includes the library tests and both declared integration targets, using the shared prototype store and the locked dependency graph. The run began in the normal memory admission queue.

```sh
SEMIO_OFFLOAD=always repo cargo --store=shared --configuration=prototype -- test --manifest-path '✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📦️packages/🦀️rust/Cargo.toml' --locked --lib --tests
```

The managed run completed source admission for 831 compilation units and 6,510 paths and received three local compiler slots with a 3.0 GiB rustc memory budget. It compiled through the framework instance-command dependency that had failed in an earlier source snapshot.

The repository source-closure verifier also passed against the current applied source for macOS and Windows MSVC configurations. Both measured 28 paths and 229,857 bytes for the production library, 32 paths and 236,531 bytes for the test library, 2 paths and 31,933 bytes for Drawing birth, and 6 paths and 70,943 bytes for Interchange. The Windows evidence is a source admission check on a macOS host.

## First Native Result

Session 82445 finished compiling all three test targets in 13 minutes and 15 seconds without a storage error. All five library tests passed. All three Drawing Birth tests failed with the same invariant, `Drawing import birth lost original custody`. Cargo stopped before executing Interchange and exited 101. The compiler image had 29.27 GiB physical space free after the run released its lane. A focused backtrace replay is active as session 98715.

## Drawing Birth Admission Repair

The focused retry reproduced all three failures and exited 101. Its backtraces place every failure inside `admit_into`, including the first underfunded admission. The empty Layout owner used to contain a default `RetainedFieldPreparation`. Its underlying field cursor only reports terminal emptiness once it has started closure. This caused the precondition on Drawing Birth admission to refuse a fresh owner before reaching the original-source capture.

The field now has a vacant place until the source has completed admission. A separate paid action initializes the resident field using its framework Work quote and a copy demand for the field and its place. Cancellation continues to close a field once it exists. The shared admission law also asserts directly that a newly created owner has no outstanding custody. The macOS source check and scoped diff validation passed. Production has 28 paths and 231,270 bytes, the test library has 32 paths and 237,944 bytes, Drawing Birth has 2 paths and 31,977 bytes, and Interchange has 6 paths and 70,943 bytes.

The full locked native retry is running as session 78757 with Cargo's `--no-fail-fast` and a Rust backtrace. This will collect every test target's result even if an earlier target fails. The managed compiler image had 25.37 GiB free before the retry.

The Windows MSVC source configuration reproduced the repaired macOS closure totals in its verifier run. Managed compilation of the full retry passed source admission for 831 units and 6,511 paths and received two local compiler slots under raised memory pressure.

The independent TypeScript strict/Ajv/Lodash oracle was rerun against the shared Drawing Birth fixture. It passed three cases, 833 turns, 1,790 undergrant denials, 18 cancellation cuts, three Source substitutions, and 18 retired original births. It reported zero strict diagnostics.

## Full Native Results After Admission Repair

Session 78757 compiled and linked all targets in 47 minutes and 50 seconds. The five library tests passed. Drawing Birth passed its admission and cancellation laws. Its exact original parent-reference test exhausted the advance turn bound and failed at the final take demand because preparation was still incomplete. Interchange passed 20 of 35 tests. Its failures cover IDML identity, base64, traversal and bounds, JSON snapshot expectations, a printable-scene pack literal, and a STEP coordinate witness. The mounted compiler image remained above 5 GiB physical headroom during target linking despite critical host memory pressure.

The repository retried automatically after its prototype profile constraints document changed during the command. The second invocation was waiting for a new six-GiB compiler-space reservation without producing further test evidence. After recording the completed run, I cancelled that owned replay. The command exited 130 and released its leases. A direct replay of the previously linked Drawing Birth harness independently reproduced the remaining progress-bound failure.

## Exact Funding of the Parent Reference

The live `ArtifactChildCloneCursor` quoted one Work for its first source binding. `RetainedCloneRef::bind` correctly refuses that turn because the original shared storage quotes three Work for the alias. No state changes on refusal, so the same one-unit quote recurs until the test's advance bound. The Layout law now reports the next physical and Work quote on a stall and rejects an exactly funded zero-progress turn that selects the same quote again.

The framework-owner patch delegates the binding quote to the original Source. It also quotes the actual nested field output move, records the parent drain flag and phase under paid Work, and advances closure as its own action. This addresses the next mismatches visible in the same exact-funded path. The output and close preflights use the same demand and caller body as their actions. The patch passed a read-only apply check and the installed Rust grammar parser. Live application and a grouped native rerun are pending release of the current guest compiler sources.

## Live Image Headroom Before the Grouped Retry

Physical Rust-image availability was measured at approximately 1.10 GiB on the mounted 40.00 GiB image. The capacity ledger's 11.01 GiB eviction target is a policy estimate. It does not establish that those bytes can be reclaimed without disturbing live work. The Layout guest kept its exclusive lane custody and a fresh six-GiB admission record. The record's 30-minute forecast had fully elapsed, so the admission ledger predicted zero remaining writes without reserving physical capacity. Another live native owner still forecast approximately 1.93 GiB of writes. The image admission floor was 1.20 GiB.

There were three managed lanes on disk, each held by a live writer. The active Cargo retention process showed fresh unit and collector markers and free-space measurements that moved between about 1.03 and 1.17 GiB. Its planner excludes live lanes and can collect eligible incremental sessions and cold units. That automatic reclamation was insufficient to establish physical headroom for another Layout test run. The audit made no cache changes and started no compiler.

The canonical `repo cache gc --dry-run --area=rust` command exited zero. Its Cargo owner scan found 25 superseded component runs totaling 2.8 GiB. It reported one component run held by a live build and one retained. It planned zero incremental-session removals and zero of 3,573 build-unit removals because all build units were held. The generic loose-cache plan also removed zero. This gives a governed stale-run reclamation option if authorized and separate from the running Cargo collector. The dry run did not change the cache.

The authorized targeted `repo cache gc --area=rust` run exited zero. Cargo removed 25 superseded component runs totaling 2.8 GiB and nine eligible incremental sessions totaling 448 MiB. Eligibility for those sessions changed between the preview and the actual scan. Cargo protected the running component and removed zero build units. Physical image free space was 3.30 GiB after recovery, while the Layout guest's final Rust process remained active. No native compiler was started as part of recovery.
