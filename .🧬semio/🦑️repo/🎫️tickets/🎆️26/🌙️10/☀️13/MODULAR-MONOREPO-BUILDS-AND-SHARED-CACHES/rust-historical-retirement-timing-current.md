# Historical Retirement Timing And Next Scope

## Observed Current Preview

Read-only preview 53749 completed exit 0 under its authored 120000 ms Original. No image was reopened for this analysis, no preview was retried and no canonical unit was retired. The immutable witness, physical 4 GiB admission floor and all original guards are unchanged.

Existing generated artifacts localize the phase without inventing a runtime profiler:

| Existing observation | Elapsed | Qualification |
| --- | ---: | --- |
| Redirected transcript creation to actual process-capture completion | 1502 ms | Includes Bun load, immutable code/charge authentication, image EX, common writer/store maintenance admission and process child; artifact timestamps, not exact internal timers |
| Actual process-capture file creation to final output write | 135 ms | Existing captured process artifact; not exact whole-child CPU time |
| Process-capture completion to plan publication | 116375 ms | Includes actual available-byte statfs, sequential build and incremental scans, pure planner and606-byte plan publication |
| Plan publication to final transcript write | 88 ms | Includes closing/rechecks and final log; artifact timestamps |

The process→plan interval cannot be a Cargo compiler/CPU-core gate: this command starts no Cargo/rustc and requests no CPU reservation. The code acquires cache-prune and both canonical store maintenance leases before process capture. After process completion and before plan output there is no lease-acquisition call. Image EX and common writer were already authenticated before the process step. No rooted lsof or physical profile lock is run by preview, because it returns before deletion admission. The six ambient compiler processes are diagnostic inventory only. Their existence is not evidence of CPU/I/O contention or an attribution of this interval to another SDK operation.

The transcript contains 1995 scan progress publications after the process-complete record and before the plan. The current walker checkpoints every 64 metadata operations. Since each of the two scanner walks has its own remainder, those records imply at least 127680 and fewer than 127808 checkpointed filesystem-operation attempts. They do not distinguish lstat from readdir, completion latency or each scanner's share. Publication is the real issued control's awaited stderr write/continuation; the continuation yields with setImmediate, not a timed sleep or SQLite admission. No per-operation failure or missing row is inferred from those counts.

## Source Path And Repeated Work

Actual `budget/⚙️execution/🟦️.ts` runs `scanCargoBuildUnits(root,...true)` to completion, then `scanCargoIncrementalUnits(root,...true)` to completion, for both initial plan and guarded recensus. `CacheWalk.stat`, `measurement` and `entries` each await one physical operation at a time. Unit directories and incremental sessions are measured sequentially. BigInt measurement preserves the same-scan path/device/inode/size/timestamp provenance.

Each scanner independently calls `findNamedDirs(root,name)`. It searches directory depth ≤ 8 and stops only at a directory matching its requested name. The build search therefore descends through other incremental/output/dependency subtrees; the incremental search independently descends through build/output/dependency subtrees. Subsequent measurements reread eligible unit subtrees. This is a concrete repeated serial metadata path. Its measured share is not established by the retained transcript. A change that simply skips known names or stops both searches at either kind could alter current coverage, so that shortcut is not proposed. Likewise an initial plan and a guarded recensus must remain distinct physical observations; caching the first result into the second would defeat the guard.

The existing read-only historical inventory had a33.963 s total in its earlier defining observation. It scanned in a different order and under a different actual filesystem/cache/workload state. The current artifacts cannot establish why the later scan was slower. No warm/cold-cache or SDK-contention explanation is claimed.

## Narrow Next Scope

The interval is localized to scanning/planning, but build-vs-incremental, discovery-vs-measurement and actual filesystem-vs-progress cost remain unresolved. Next implement only bounded opt-in diagnostic facts, with schema-first language-neutral fixtures and independent SQLite arithmetic checks:

1. At the real initial/recensus scan calls, time build and incremental scanners separately with performance.now. Emit one closed summary per scanner, only when the explicit diagnostic switch is enabled; default behavior and aggregate formula remain unchanged.
2. In the actual same CacheWalk, count ordinary stat, BigInt measurement, readdir and awaited continuations, and sum their measured elapsed time. Separate discovery vs unit/session measurement spans without another walk or holding file descriptors. Emit no per-file rows and no file contents. Keep finite counters/number validation and a fixed output bound (at most four scanner summaries and a small fixed number of phase summaries per retirement operation).
3. Exercise diagnostic extraction/formatting using actual temporary metadata fixtures under the existing ticket output directory and one genuine finite Original. Use an independent Python os.lstat / SQLite aggregation oracle. This does not require Rust image access. Confirm the existing units, same-scan maximum provenance, cancellation, original thrown values and failure drainage are unchanged.

No scanner concurrency or coverage change is included in that diagnostic checkpoint. No additional runtime image operation is authorized here.

## Candidate Optimization After Measurement And Review

If metadata serialization is confirmed, use a small closed concurrency policy for independently owned unit/session measurements, preserving original input order at result collection and provenance-budget retention. Every accepted filesystem request must settle before success or failure returns; stop further admissions after the first failure/cancellation, await all accepted readers and preserve the primary error. Do not use fail-fast Promise.all that releases image/writer custody while other requests remain live. A shared walker must retain finite in-flight/queue metadata, the original continuation checks and complete same-scan maximum reduction.

A combined discovery walk is a separate coverage decision requiring a portable hostile tree with build/incremental names nested inside the opposite kind and independent Python enumeration. It must preserve current discoverability without widening depth or hiding data; no inference from normal Cargo folder names is enough. The guarded recensus remains fresh, after original physical profile and full-rooted descriptor admission.

A future maintenance request can have its own justified finite whole-operation budget declared before birth, sufficient for both complete scans, bounded system probes and settlement. That does not extend this preview's carried Original, change compiler 240000 / Actor 600000 limits, relax any image floor, or authorize a retry/retirement. Canonical preview/retire and Actor remain held until separately coordinated.


## Analysis Handoff

No production Source, image/provider/base lease, witness, guard, scanner ordering or admission policy changed in this timing analysis. No new runtime operation was launched. The current evidence already rules out a long pre-scan queue/auth/process phase in this specific preview, while it does not establish the internal filesystem/progress/discovery/measurement shares. The proposed diagnostic checkpoint remains explicit; a measured optimization and any new independently preauthored maintenance budget are subsequent coordinated work, not an extension of Original 53749.


## Qualified Opt-In Diagnostic Checkpoint

Root approved only diagnostic Source and a real metadata fixture without opening the Rust image. The new schema/data owner is `library/⚡️caching/🧹️pruning/⏱️measurement`. The actual existing `CacheWalk` now optionally counts and times its own ordinary stat, BigInt measurement, readdir and awaited continuation operations, plus discovery/measurement/enumeration spans. It emits one summary in its own finally boundary, marking complete/failed without serializing a thrown value. Optional diagnostic delivery cannot replace the scanner's original result or thrown value. Every filesystem request remains individually awaited; no pool, new coverage rule or concurrency was introduced.

`budget/⚙️execution` snapshots the exact `SEMIO_CARGO_RETIREMENT_TIMING=1` data switch and passes initial/recensus labels to the two existing sequential scan calls. The default emits no timing summary and retains the original scanner order, aggregate formula, provenance and fresh guarded recensus. A full retirement has at most four summaries. Each full line, including its fixed DEBUG prefix and newline, is bounded to 2048 UTF-8 bytes (8192 bytes total). Each record contains only fixed scalar fields, counts, elapsed milliseconds and the root; no per-file list or content is emitted. Missing/invalid switch values do not enable the diagnostic. These data fields issue no authority or budget.

The schema-first formatter test genuinely failed missing export (rust-scan-timing-red1.log). After that owner existed, the real temporary filesystem test genuinely failed with 0 summaries instead of 2 (rust-scan-timing-red2.log), before scanner integration. Current portable/Ajv/SQLite and actual carried-Original tests are BOTH 2 tests / 66 assertions, and expanded ES2022/noUnchecked strict is collected exit0. The existing unchanged recency/provenance regression is BOTH 1 test / 75 assertions, including same-scan maximum Source and backwards file recency. No Rust image, compiler, SDK publication or canonical preview/retire was run for this checkpoint.

The actual fixture creates 69 small files under a unique owned ticket-generated directory, fixes initial file timestamps, then compares production unit byte/recency results to independent Python os.lstat plus SQLite aggregation. A default scan is silent and matches the initial diagnostic scan. The fixture changes one own file and observes the new 88-byte build row on recensus before deliberately throwing undefined; that exact value remains the rejection. Separate real Error identity and actual issued-control cancellation also remain intact. All accepted serial requests finish before rejection, the four summary statuses are complete/complete/failed/failed, and actual continuation checkpoints are exercised. The fixture tree is removed in finally; only bounded generated evidence remains. Maximum actual fixture materialized path was 173 UTF-16 units.

Temporary fixture diagnostic facts (illustrative, not a canonical cache-performance measurement):

| Pass | Stat calls | BigInt measurement calls | Readdir calls | Continuations |
| --- | ---: | ---: | ---: | ---: |
| build initial complete | 14 | 70 | 10 | 1 |
| incremental initial complete | 83 | 3 | 10 | 1 |
| build recensus failed | 14 | 70 | 10 | 1 |
| incremental recensus failed | 83 | 3 | 10 | 1 |

The small fixture demonstrates that the incremental discovery pass visits the opposite build subtree (83 ordinary stat calls versus 3 actual measurement calls). It does not establish the canonical 1131-unit distribution. The new summaries can localize that distribution in a separately authorized future read-only preview. No canonical measurement, budget change, retirement or Actor retry is authorized by these test results.

All own handles are collected terminal and no image or tools reader was opened. Production diagnostic Source is frozen for review/coordinated capture. Both actual editor catalogs register `rust-headroom/📜️script.ts scan-timing-test prototype|release`; expanded inventory-types includes the new fixture.

| Retained generated evidence | SHA256 |
| --- | --- |
| rust-scan-timing-red1.log | `abd2979e5620aa233dd7d118e3fd0e04b2c2a39b886d83415bbdef5f75efbf84` |
| rust-scan-timing-red2.log | `1b26aea6a797aa8c2bd2e68ed7b0202dabe42af7b7f1a460e689956f5f478f21` |
| rust-scan-timing-prototype4.log | `f4d877b85a3ec1e195258a2e9b77172f813b49ea1dcfef2f6c0b44d03934f24b` |
| rust-scan-timing-release2.log | `79308c310e41e244ba2169eaca380d19fa935e7732a610f3b5a6839cf33342d7` |
| rust-scan-timing-types3.log | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| rust-scan-recency-regression-prototype.log | `c7a4866ab641c3912e2615b8dd0fd6e0c024a3936cd56529a5ec1f12e497290a` |
| rust-scan-recency-regression-release.log | `c4741d84d72ed5b9847b7a1873414d19d45d92557ef6cb4d35f97aa4fb8a687e` |
| rust-scan-timing-fixture-prototype.json | `eb35c7ebf3d73e93408a89a4e8994221f2a7beb51c81a17b0a581033fda57759` |
| rust-scan-timing-fixture-release.json | `d98dd430860053dd0e5e673da4eb6a3c96b8b5e0cb92b558063bd2e1f6b05faa` |


## Single Authorized Canonical Timed Preview 18377

Root authorized exactly one nondeleting prototype preview with the diagnostic switch enabled and the unchanged 120000 ms Original. It is collected terminal exit 0; no retry, retirement, compiler, Actor or new-budget operation was launched. Both real initial scanner summaries completed; there is no failed/partial scan in this run. All own image/tools/code/writer/store scopes are now closed.

| Actual pass | Total ms | Discovery ms | Measurement ms | Enumeration ms | Stat calls | BigInt measurement calls | Readdir calls | Continuations |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| build | 15379.043 | 3203.306 | 11807.406 | 368.331 | 11595 | 47049 | 4061 | 979 |
| incremental | 18144.148 | 16385.595 | 1660.803 | 97.750 | 52354 | 8672 | 4061 | 1016 |

Build produced 1001 units. Incremental returned 159 rows including stale finalized-session observations; the retirement planner consumes its 130 crate units and intentionally excludes those 29 stale-session rows, so the actual plan census is 1131 units, not 1160. Complete logical total remains 3598655060 bytes. The actual new availability is 4268638208 bytes (deficit 26329088 against 4 GiB), versus 4293222400 in preview 53749. The observation does not identify the cause of that physical free-space change; matching unit logical totals do not prove every other image artifact is unchanged. Current preview selected 6 units / 30586885 logical bytes and zero unmet bytes:

| Selected unit | Bytes | Protected |
| --- | ---: | --- |
| `prototype/incremental/original_caller_capture-1fwu90kd4ldz7` | 4556726 | False |
| `prototype/incremental/original_caller_capture-3q0bha5atz042` | 4267141 | False |
| `prototype/incremental/original_caller_source-2sgkmpnminpi6` | 4904987 | False |
| `prototype/incremental/original_caller_source-3lzhk4ux1gh2c` | 5468504 | False |
| `prototype/incremental/original_child_source-242i28qh7i198` | 5153601 | False |
| `prototype/incremental/original_child_source-2eh1uxf34j7ga` | 6235926 | False |

The same actual root guard remains device 16777260 / inode 185 / 8192 bytes / nlink 1 before and after. Independent lstat facts are observation, while the existing private actual EX proof authenticates custody. Preview performs no physical profile lock/FD/recensus/deletion admission, so those later checks are not claimed by this result.

This run's two scanner totals sum 33523.190 ms. It cannot explain or erase preview 53749's 116375 ms process→plan interval, because the filesystem/cache/workload state differs and was not independently sampled. It does identify present serial cost: incremental discovery 16.386 s and build measurement 11.807 s together account for 28.193 s of scanner time. Awaited continuation costs are 164.196 ms and 142.478 ms, so this run does not support blaming progress scheduling for the dominant elapsed cost. Operation milliseconds are actual awaited wall times, not CPU utilization or physical disk throughput.

### Narrow Measured Parallel Scope For Review

Target a closed maximum of eight simultaneous metadata requests, only in two existing loops: child stat batches inside `findNamedDirs`, and pending-path measurement batches inside `measureUnit`. Keep each discovery traversal and every directory name/depth rule unchanged; do not skip opposite subtrees or cache an earlier discovery/census. Reduce completed facts in the original batch input order. Bytes and deterministic maximum Source reduction retain their exact formula, and provenance retention remains at the existing ordered points. No compiler, lease, physical FD or deletion command is parallelized.

Every accepted request must settle before the batch resolves/rejects. Stop admitting the next batch after any accepted failure/cancellation; preserve a rejected undefined as an error and deterministic primary failure ordering. Add an awaited continuation barrier so the existing 64-operation publication/continuation cannot overlap or be overtaken by later admissions. The same carried Original and enclosing image/writer remain retained through this drain. Initial and guarded recensus are independent new executions with the unchanged physical profile locks, full rooted FD checks, exact recensus and protected-working policy between them.

Before wiring, schema-first neutral schedules should prove bound 8, no extra admission after failure, late accepted sibling drainage, ordering and original undefined/Error identity. Independent SQLite event-order checks and actual Python metadata enumeration must match complete unit bytes/recency/max provenance. A real temporary tree with nested build/incremental names, missing entries and symlinks must retain coverage and no-follow behavior. This is a proposal, not an implementation or authorization for another image run. The real canonical speedup and two-scan fit remain unqualified until a separately authorized measured run.

| Generated evidence | SHA256 |
| --- | --- |
| 🗑️generated/rust-historical-timed-preview1.log | `00fcc1cad254e62e18c2c6a4815931d0658ffd87c75d1ab35cafcedc067a30c5` |
| 🗑️generated/root-rust-headroom/turn-Dts2pb/plan.json | `771642c45ddb27c47eb788816c2da64970103d7c716e78789d3fd33bc49d22c9` |
| 🗑️generated/rust-historical-timed-preview-summaries1.json | `943b2f582ffa6323560ca7833e8393d792439d51ca8e6eba80b99bb1532d0c38` |
| 🗑️generated/rust-historical-timed-preview-inode1.json | `53b40e0f5b09f07356be14c57dfd9de130aa13cae045686c114e4a5a8df6d57e` |
| 🗑️generated/rust-historical-timed-preview-inode2.json | `53b40e0f5b09f07356be14c57dfd9de130aa13cae045686c114e4a5a8df6d57e` |


## Qualified Bounded Metadata Batches

Root approved only the two measured metadata loops after reviewing timed preview 18377. The actual `findNamedDirs` child-stat loop and `measureUnit` pending-path measurement loop now use the closed schema policy of eight accepted metadata requests per batch. A batch awaits Promise.allSettled before reducing any result or returning an error. Rejected results are inspected in input order, preserving an undefined rejection and the first input-ordered Error regardless of completion order. Any read or reduction failure prevents the next batch. Directory enumeration remains singly awaited during ordered reduction; no compiler, lease, physical profile lock, descriptor process or deletion is parallelized.

Discovery retains its original name/depth/symlink decisions and directory-stack order. Measurement reduces each batch in pending-pop order; filesystem completions cannot reorder its aggregate. Traversal read timing is concurrent, so this does not claim an atomic whole-tree snapshot. Coverage, byte sum, maximum timestamp formula and deterministic path/field tie rule remain unchanged. Unit/session provenance is still retained at the prior ordered points. Initial and guarded recensus always start fresh filesystem walks; no earlier facts are cached into a later admission. All charge, private proof, image, common writer, profile lock, full rooted FD, exact recensus and protected-working guards are unchanged.

A shared private continuation barrier retains the original 64-operation interval. After the interval begins, later metadata admissions await its actual original publication/continuation. Only one such continuation runs at a time. Failure permanently poisons that walk's barrier with the same original reason, including undefined, so late calls cannot resume it as a new epoch. Existing accepted metadata requests are drained before the scanner rejects.

The existing fixed-size diagnostic adds only `maximumInFlight` (integer 0..8), counting pending actual metadata calls; its at-most-four lines of at-most 2048 bytes each remain unchanged. Operation milliseconds are cumulative awaited request time and can now overlap; wall stage/total times, not the sum of request times, decide actual elapsed improvement. No canonical speedup or two-census fit has yet been measured.

### TDD And Independent Oracles

The missing batch/continuation owners genuinely failed first (rust-metadata-batch-red1.log). Neutral schedules then qualified empty work,19-input three-batch ordering, both undefined-vs-Error precedence cases, cancellation and second-batch failure. Reverse completion schedules prove no reduction before accepted siblings settle, maximum 8, ordered output and no later batch after failure. SQLite independently checks event counts/order and that reduction occurs with no active accepted reader. A held continuation at 64 blocks later admissions; releasing it permits the next continuation at 128, with maximum one active and late failure refusal. These schedule drivers are explicitly data/algorithm tests and issue no authority.

After adding the diagnostic peak counter but before wiring parallel metadata, the actual temporary-tree test genuinely failed because its real serial peak was 1. Current BOTH final batch gates are 3 tests / 126 assertions. A single genuine carried 120000 ms Original owns all temporary metadata and real Python child settlement. The fixture includes nested opposite-kind build/incremental directories, stale and working sessions, equal timestamp maxima, a directory symlink to an own separate sibling tree, a dangling symlink and absent roots. Independent Python os.walk/os.lstat reproduces the complete eligible unit census, full bytes/recency and every max-source field; bidirectional SQLite EXCEPT comparisons confirm no missing or altered row/field. Actual peak is 8 in both scanner passes. The actual resulting coverage is 2 build units, 2 incremental crate units and 1 stale finalized session. Symlink targets remain excluded and the working crate stays protected.

A separate real issued child control is canceled while eight accepted batch readers are held. Seven settle while the final sibling remains blocked; the operation remains pending, the genuine parent control stays live, and no ninth read is admitted. Releasing the final sibling propagates the exact cancellation marker only after every accepted callback settles. Both own fixture directories are removed in finally; bounded generated evidence remains. This does not claim automatic authority enrollment for arbitrary detached callers: actual scanner/custody callers continue to await the returned operation.

| Current qualification | Result |
| --- | --- |
| Metadata batch schedules + actual Python/SQLite tree | BOTH 3 / 126, GREEN |
| Fixed summary / actual fresh metadata / original errors | BOTH 2 / 66, GREEN |
| Existing backwards recency / provenance regression | BOTH 1 / 75, GREEN |
| Expanded history/Native/diagnostic ES2022 + noUnchecked strict | exit 0 |

Both editor catalogs register the actual scan-batch-test prototype/release prerequisite. The permanent script remains the existing rust-headroom/📜️script.ts. No runtime external dependency, structural authority bridge, namespace enrollment, Source witness refresh, image operation, SDK publication, canonical preview/retire or Actor run was introduced. All own handles are collected terminal. This production metadata epoch is frozen for independent review and any separately authorized canonical measurement.

| Retained generated evidence | SHA256 |
| --- | --- |
| rust-metadata-batch-red1.log | `6fb771d0607754294f933dfe8e145015e86d7130f8f83c58d8f6a6317a3d0a5c` |
| rust-metadata-batch-red2.log | `bea52652437ad1dc8a7457a224e601b4cba8404152970e4bf6d2ae5a21a3b6ce` |
| rust-metadata-batch-prototype3.log | `bdac0d11d838544acaabe6094a9366b252ffaf0bdf300c3422400c53f3315f8b` |
| rust-metadata-batch-release2.log | `48454727325700dd425096048be760da23be8cca2ecbcde1623fdef2fa469165` |
| rust-metadata-batch-types3.log | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| rust-metadata-timing-prototype1.log | `f2234d66b32073b3e60f9ea69dbca9c60494006c3446e3a823ae191543137dde` |
| rust-metadata-timing-release1.log | `3638a4ed1a865495dbb724aa7dd0ec22d4360edd860f3eacc0e5bfba706a5dd8` |
| rust-metadata-recency-prototype1.log | `325a17ce3b89beb88a844165eb1521078d28c06187afc6a34ca09b278739aa58` |
| rust-metadata-recency-release1.log | `7f3aa774bc330ccf3534809129433e05dd020700258855d70639968945a82eb9` |
| rust-metadata-batch-fixture-prototype.json | `88c49f3c95e5408d3ce2dde999efb762a38d1bba0c3cdf4ad28e577c4dd5269e` |
| rust-metadata-batch-fixture-release.json | `5f5cc6e7003b0b524b41549652950a1ed851f1d110df11af39e43ac434129326` |


## Single Authorized Batched Canonical Preview 99070

Root authorized exactly one timed nondeleting prototype preview on the frozen eight-request implementation, with the unchanged 120000 ms Original. It is collected terminal exit 0. Both initial scanner summaries completed; no partial/failed summary, timeout, retry, compiler, Actor or new-budget run occurred. All own image/tools/code/writer/store scopes are closed.

| Actual pass | Wall ms | Discovery ms | Measurement ms | Enumeration ms | Peak pending metadata |
| --- | ---: | ---: | ---: | ---: | ---: |
| build | 22011.731 | 8174.102 | 11973.795 | 1863.834 | 8 |
| incremental | 27299.844 | 20942.379 | 5247.360 | 1110.105 | 8 |

The two scanner wall times sum 49311.575 ms versus 33523.190 ms in the prior serial timed observation 18377: this run does not demonstrate a speedup. It is not a controlled identical-workload comparison, so a causal performance regression or ambient contention source is not established either. Cumulative request milliseconds now overlap and must not be compared as if they were serial wall time. No additional retry or performance attribution follows from this result.

Complete plan census remains 1131 units / 3598655060 logical bytes; build 1001 and incremental 159 summaries include the same 29 stale-session rows excluded from planner input. The metadata call counts exactly match the prior timed observation: build 11595 stat/47049 BigInt measurement/4061 readdir and incremental 52354/8672/4061. The complete resulting plan JSON equals the prior timed plan, including all six selected rows and their same-scan max-source facts. Actual available bytes remain 4268638208; selected bytes 30586885, retained 3568068175 and unmet 0. Every selected row still has lockHeld:false; no protected working row is selected. This comparison does not assert that unrelated unscanned paths are immutable.

Exact root guard observations remain device 16777260 / inode 185 / 8192 bytes / nlink 1. Actual opaque proof authenticates the retained EX, while lstat facts corroborate unchanged path/inode. Preview did not execute physical profile lock/FD/recensus/deletion admission; those actual guarded retirement checks remain unqualified in this run. No retirement is queued.

| Generated evidence | SHA256 |
| --- | --- |
| 🗑️generated/rust-historical-batched-preview1.log | `fa423db79f153708516edadd889071110308e2a7ee98aace5295c51042e87dc8` |
| 🗑️generated/root-rust-headroom/turn-xVjZpk/plan.json | `771642c45ddb27c47eb788816c2da64970103d7c716e78789d3fd33bc49d22c9` |
| 🗑️generated/rust-historical-batched-preview-summaries1.json | `ceccbceecf8c5682f0ac899327793fef8ae813fd7af0903e9daf9b2c83a71506` |
| 🗑️generated/rust-historical-batched-preview-inode1.json | `53b40e0f5b09f07356be14c57dfd9de130aa13cae045686c114e4a5a8df6d57e` |
| 🗑️generated/rust-historical-batched-preview-inode2.json | `53b40e0f5b09f07356be14c57dfd9de130aa13cae045686c114e4a5a8df6d57e` |


## Single Authorized Actual Canonical Retirement 83670

Root separately reviewed the nondeleting result and independent metadata-batch audit, then authorized exactly one canonical prototype retirement through the same qualified API with timing enabled and the unchanged 120000 ms Original/policy. Original 83670 is collected terminal exit 0. This is an actual completed retirement, not a preview result or inference from missing paths. The operation independently rescanned/replanned; the earlier six-row preview supplied no deletion authority. No automatic retry, compiler, Actor or new-budget run was launched.

Actual retained custody authenticated immutable RC6013 code/receipt, original prototype charge, exact Native image EX proof and common compiler writer. The complete process inventory observed one ambient compiler only as diagnostics. Before deletion the real physical profile lock helper retained one exact profile lock and complete rooted descriptor custody (guard helper PID 9489; full inventory 897899 bytes). A new build and incremental recensus completed under that physical custody; every selected row passed the original unchanged recensus predicate. No protected working unit was selected or reclassified.

| Actual scanner pass | Wall ms | Peak pending metadata | Status |
| --- | ---: | ---: | --- |
| initial build | 15283.142 | 8 | complete |
| initial incremental | 17127.887 | 8 | complete |
| recensus build | 1606.114 | 8 | complete |
| recensus incremental | 11163.029 | 8 | complete |

All four fixed summaries completed within the one original request. The observed initial and recensus timings differ substantially; no cache/throughput cause or general performance speedup is inferred. The actual complete two-census/probe/deletion operation did fit this particular unchanged 120 s Original; that is not a future timing guarantee.

Exact completed deletion:

| Unit | Logical bytes |
| --- | ---: |
| `prototype/incremental/original_caller_capture-1fwu90kd4ldz7` | 4556726 |
| `prototype/incremental/original_caller_capture-3q0bha5atz042` | 4267141 |
| `prototype/incremental/original_caller_source-2sgkmpnminpi6` | 4904987 |
| `prototype/incremental/original_caller_source-3lzhk4ux1gh2c` | 5468504 |
| `prototype/incremental/original_child_source-242i28qh7i198` | 5153601 |
| `prototype/incremental/original_child_source-2eh1uxf34j7ga` | 6235926 |

The actual completion count is 6 / 30586885 logical bytes; retained historical logical bytes 3568068175 and unmet 0. Physical availability measured inside the retained operation changed 4268638208→4302499840 bytes, an increase 33861632 bytes. Final physical surplus above the unchanged 4294967296-byte floor is 7532544 bytes. Logical byte count and physical freed bytes are distinct. No partial result or refusal occurred in this operation.

After terminal settlement, independent lstat still observes the same root guard device 16777260 / inode 185 / 8192 bytes / nlink 1. The actual private proof and physical guards supply custody; these observations do not grant new authority. All own child, image, code, tools, writer, store and physical-lock scopes are closed. No Actor/compiler operation follows automatically. Unknown old data and working artifacts remain untouched.

| Retained generated evidence | SHA256 |
| --- | --- |
| 🗑️generated/rust-historical-batched-retire1.log | `d9eca627128b0abcc24b9c7aa10ed0fcd5288041473cc942183ee14d217aee21` |
| 🗑️generated/root-rust-headroom/turn-oTKCC7/plan.json | `771642c45ddb27c47eb788816c2da64970103d7c716e78789d3fd33bc49d22c9` |
| 🗑️generated/root-rust-headroom/turn-oTKCC7/descriptors.stdout | `eb64921886f263fcb0d31963777cb20b7ac74af0621f182a316e844f19b6d761` |
| 🗑️generated/rust-historical-batched-retire-summaries1.json | `6a8e9f4502684e00e9816ff23dbba150676256417300cf077fa8b6dbd819ae4d` |
| 🗑️generated/rust-historical-batched-retire-inode1.json | `53b40e0f5b09f07356be14c57dfd9de130aa13cae045686c114e4a5a8df6d57e` |
| 🗑️generated/rust-historical-batched-retire-inode2.json | `53b40e0f5b09f07356be14c57dfd9de130aa13cae045686c114e4a5a8df6d57e` |
