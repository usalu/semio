# SDK Source Verification Observation

The full cold caller40304 timed out at30000.10ms after its qualified Native preparation completed in685.385ms. The last real stage was SDK source verification. No verification result was accepted, no deadline was increased, and the managed stack was not stopped.

## Causal Workload Before Optimization

The actual declared pristine snapshot b960382808311edb859d364f38fe226b53b5bf8870f8d2ded7014484fb072fa0 has a sealed2,017,181-byte source index:11,705entries comprising10,192files,1,479directories and34links. File bytes sum196,685,599; the current65,536-byte chunks require12,589successful chunks; maximum path depth12. These are bounded readonly index DATA observations, not Native allocation claims or a new SDK publication.

The existing verifier preserves every sealed file hash, readonly/executable/type/size observation, descriptor identity before/after reading, rooted ancestor identity, bounded continuation and actual context recheck. Three cold clients repeat roughly590MB of file hashing plus all descriptors and root checks. Source inspection alone cannot establish which category dominates; no hash skipping, cache shortcut or timer adjustment is justified by these counts.

## Changed Instrumentation

The current SDK index owner enables the already existing private descriptor metrics only under the existing repository profiler. One complete nx.sdk-source.verify span records real wall time; one fixed aggregate records opened files, consumed bytes/chunks, cumulative open/stat/read/hash-update/yield/close times and fixed category counts/times for actual context checks, rechecks, parent checks, file metadata, publication, index parsing/identity and hashing. Worker times are cumulative and may overlap under the unchanged16-worker policy; they are NOT to be added together as wall time.

All operations still invoke the original functions in the same order. The observing context delegates its check/recheck to the original object with the same control and receiver. Metrics are emitted on completion or failure/cancellation, retaining the actual primary failure. No per-file profile row explosion, arbitrary heartbeat, new authority token or Source acceptance path is added. Profiling off keeps the original verification body.

Neutral observation fixture/schema and actual collector tests first refused the missing implementation and now pass1test/8assertions. They preserve exact returned/thrown object identity and compare the same count/duration trace with independent SQLite aggregation; Ajv validates the closed corpus. The actual SDK index module parses successfully. New paths are below256characters (maximum185).

Production owner: tools/sdk/source/derivative/index/🟦️.ts, SHA256 afafb0878a28fd2193172d112da1ce4efd8e69ffca9015be6b3216abc387b407. Fixed aggregator and its canonical tests/schema/fixture are under that owner/⏱️observation. Evidence: 🗑️generated/sdk-verification-metrics-red.log, sdk-verification-metrics-integrated.log and sdk-verification-instrumentation-parse.log.

## Measurement Coordination

The actual706549d3… SDK authorizer map has1,559sources. Three unchanged Print artifact facade/schema sources are present, so the entire Print tree is NOT excluded. Frontend's exact edited pdf/representation implementation/schema and planned tectonic-template-compilation implementation are absent; the latter already lies outside this closed role, so its new representation import does not add a path from the existing admitted facades. This exact membership permits the one changed-instrumentation full caller run to proceed in parallel with lossless PDF work.

The next measured run must use the actual normal Node Original/context through owned SDK refresh, the full caller file, the same cold cache and30second law, all assertions and NX_DAEMON unset. A synthetic read context or Bun-only benchmark would not establish production context-check cost and is not substituted. No production optimization has yet been applied.

## Original Context Identity Refinement Before Capture

The first opt-in wrapper projected a structural {...context} while invoking the original methods with their original receiver. No authority failure was demonstrated, but the projection was unnecessary. While66194 still waited at door.capacity and had captured no Source, instrumentation was refined to retain the literal original context object throughout verification, tree traversal and hashing. A private AsyncLocalStorage contains only timing DATA; actual context method callsites are measured without replacing their receiver or Original/control.

The aggregate/SQL corpus still passes, and a concurrent-scope law now confirms timing isolation and original method receiver identity:2tests/13assertions,0fail. The index module parses successfully. Current index SHA2567163e1d4b9f9ea0cfac2b361e2f9797d44f0b1c2ff7ba27ce0c0de39e75ee701 supersedes the initial afafb087… instrumentation hash. The existing tree owner also delegates its same original context checks through the observer. No check, recheck, descriptor call, hash, rooted identity observation or cancellation point was removed.

The SAME normal measured run66194 remains owned; it was not canceled, restarted or requeued for the review. Its eventual SDK closure must be checked against the original-context hash above. No measured result is claimed while it remains unadmitted.

## Typed Synchronous And Asynchronous Observation

The parent Linux semantic gate found TS2352 in the initial generic Promise conversion. This was a typing defect, not a measured verifier failure. The observer now exposes a synchronous measure method returning T directly and a separately typed asynchronous measureAsync method returning Promise<T> via await/finally. No generic Promise cast remains. Actual sync context/index/identity calls stay synchronous; metadata, parent lookup, file metadata, hashing, publication and context recheck use the async path.

Focused tests now pass3/3,18assertions, including exact asynchronous resolved object and rejection identity against the same neutral corpus. The relevant strict TypeScript semantic check covers173files and reports0diagnostics; sdk-observation-semantic.cjs is retained as input and 🗑️generated/sdk-observation-semantic.log as evidence. Index module parsing also passed.

Current frozen index SHA256864907abf9a9297b603634c93b69e6436132798771228888ec52b70b1c016dc6 and observer b29c6c5d2800c27bd98b5cf7e58f46ae513ab3847f247704d34a46cc99c885b6 supersede7163e1d4… and the earlier collector. Run66194 remained genuinely queued and uncaptured during this correction and retains the same handle; its actual admitted SDK hashes will be checked before attributing measurements. No unchanged retry or production optimization has occurred.

## Authoritative Measurement — No Reader Optimization Justified

Run66194 authoritatively exited1. Actual SDKf0173ccf1e6b5e903e331513281a81fd2401b8e3d7c8e14cf6f649b404c7ebd0 captured the exact corrected index864907ab… and observerb29c6c5d…; its1,560-source map excludes frontend's exact edited Print representation/compiler/test/project paths, while retaining the three unchanged artifact facade/schema rows. Evidence is saved in sdk-verification-captured-membership.json.

The complete caller file executed11tests:4passed,7failed,22expect calls,31.61seconds. First law timed out at30000.10ms. Actual failed Node child1414789 entered nx.sdk-source.verify at28836.341ms and ran1145.210ms before cancellation. It consumed3,823files/89,941,493bytes/5,023chunks, with113,209context checks totaling175.781ms and36rechecks totaling1.189ms; hash-update CPU totaled60ms. Open/stat/read/close and parent totals are cumulative concurrent worker measurements, not elapsed wall time.

A separate completed verification sample1414581 checked all10,192files/196,685,599bytes/12,589chunks in1387.259ms wall. It performed327,100context checks totaling282.082ms,91rechecks totaling2.787ms and97ms hash-update CPU. Its cumulative hashing interval sum13210.180ms is NOT13seconds elapsed; those intervals overlap under16workers. Another completed sample wall1320.344ms corroborates the scale. The ordinary parent sample wall2767.231ms is recorded separately and not substituted for the failed Node child. The runtime of those completed process samples is not established solely by their profile or Node-style warning prefix; only the failed child is proven registered Node by its actual owned command.

Therefore the evidence disproves the working hypothesis that Source verification itself consumes the30second law. The failed child spent roughly28seconds BEFORE entering verification. No schema/hash/descriptor/recheck optimization is justified from this result. The existing verifier remains behaviorally unchanged and every original check remains.

The next parent-authorized task is to measure the missing pre-verification stages: admitted runtime-entry Source capture/binding, tooling-store/inventory opening and birth/lease work. Only bounded real wall spans will be added; the same cold cache, Original/control,30second law and all assertions remain. No unchanged retry or prewarm will substitute for that causal measurement.

Deduplicated profiles and all cumulative/wall fields: 🗑️generated/sdk-source-verification-measurements.json. The complete raw run remains caller-environment-sdk-verification-measured.log.

## Missing Pre-Verification Wall Spans

After the measured reader result ruled out a reader optimization, the parent assigned the earlier gap. The next checkpoint adds only bounded existing-profile spans around actual runtime-entry Source capture, publication and command construction; and nested startup store, birth and lease/body operations. No operation is moved, validation skipped, timer changed or Original/control replaced. The existing profiler records real start/end times and nested self time, not a heartbeat.

A neutral six-span contract and independent TypeScript AST test first failed because the production spans were absent. The corrected complete startup suite passes22/22,284assertions, retaining every original lease-order/Python/orchestration/value/error law. Evidence: preverification-spans-red.log and preverification-spans-final.log. A temporary unmatched profiling parenthesis in the first implementation was corrected before the final pass; it supplies no runtime measurement.

Current frozen startup SHA25683f9b147179f19e18966483f808bb69d0003d2bde141c4a42ef3a15957e76a5f; runtime-entry producer66c99d18032d5b1ed48cce3c802a6021ea61b46945864aa81f5f161eea5cc4de. The SDK verifier and typed observer remain unchanged at864907ab…/b29c6c5d…. The next single full cold caller run will refresh the SDK normally and locate the real pre-verification stage before any optimization. The unchanged30second law and full failure/fallback assertions remain the acceptance gate.

## Actual pre-verification wall result: 1293

The same complete unfiltered `repo:test-caller-environment` run settled with authoritative exit 1. All eleven laws were selected: 4 passed and 7 failed; the first parallel-client law timed out after 30,002.76 ms. Its owned registered Node child 1456926 was canceled by the test, rather than refusing an SDK hash. Later Original-admission failures name that first failed child and do not establish seven independent causes.

The actual published authorizer namespace is `d531630be98307cb9cda0ee5673a8096e25a92acbbe552f685b97a6477ef1639` (1,560 Source rows). Captured and current startup/runtime-entry/verifier/observer hashes match; `🗑️generated/preverification-captured-membership.json` records exact paths and hashes. The three unchanged Print facades are admitted; edited Print normalizer/compiler/test/project files remain excluded.

Authenticated target Node PID 1456926 reports these monotonic wall spans:

| Phase | Start ms | Duration ms |
| --- | ---: | ---: |
| Native preparation | 664.166 | 612.284 |
| Admitted entry capture | 1277.676 | 101.968 |
| Entry publication | 1379.682 | 13.043 |
| Entry command | 1392.802 | 18.924 |
| Tooling store scope | 1412.226 | 28565.445 |
| Service birth, nested in store | 22337.798 | 7639.825 |

The pre-birth store interval is 20,925.572 ms. The store inclusive duration overlaps the nested birth; these durations must not be summed. No SDK verification or startup-lease span starts in this Node child. This disproves runtime-entry capture/publication and native compilation as the dominant cost in this run, and localizes the first large gap to tooling-store admission before its body. It does not identify which individual image/reservation/recheck operation took that time. Source boundaries are `withRepositoryToolingStoreV1` → `withRepositoryCacheStoreV1`; the cold opening path performs `openRepositoryCacheStoreImageV1`, `repositoryCacheStoreReservationV1`, and `repositoryCacheStoreBodyV1` storage recheck before invoking the birth callback. No concrete optimization has been made.

Inputs and output: `🗑️generated/caller-environment-preverification-spans.log` retains the complete test and original child stderr; `🗑️generated/preverification-profiles.json` deduplicates actual complete profile objects by PID/total. The first child was terminated at the unchanged 30-second law boundary. All hashes, descriptor checks, controls, deadlines, fixtures and verifier behavior remain unchanged.

## Lower store-stage instrumentation checkpoint

Only the existing store owner now imports the existing profile interface. The cold opening path wraps the unchanged calls with `nx.store.image-open` and `nx.store.reservation`; the existing initial `storage.recheck()` in the body is wrapped with `nx.store.body-recheck`. Actual invocation/control arguments, await order, storage/image identity, locks, callbacks and error propagation are unchanged. No algorithm, quota, cancellation or deadline change was made.

The neutral span corpus and its closed schema were extended first. The independent TypeScript AST oracle failed on the missing production image-open span (22 pass / 1 fail), then passed all three actual lower-stage operation bindings after implementation. An additional async law checks exact result/rejection identity and disabled-profile output against the same neutral cases. Final focused owner check: **23 tests, 296 assertions, 0 failures**. Logs: `🗑️generated/store-admission-spans-red.log` and `🗑️generated/store-admission-spans-green.log`; changed Source hashes and independent TypeScript parse results: `🗑️generated/store-admission-spans-checkpoint.json`.

## Lower-stage full run 54413: outer daemon barrier

The changed-span full normal caller run settled authoritative exit 1 before selecting/executing the eleven caller laws. SDK `76acc988863c5ab023b945f8712b9cc50cff02ffa62387e0d4fee7d94210608e` captured the exact store hash `327002579583bf3aa255a6ef2f0ff640ea53563fe13bce43f69ef53c48837269`; evidence is `🗑️generated/store-spans-captured-membership.json`. This is not a new timed-law measurement or a passed caller gate.

The normal outer daemon client failed to connect after 6,000 attempts. Its actual diagnostic file `.cache/b/o/m/nx/graphs/76acc988863c5ab023b9/91fdd46f0bae76ac62eb/daemon.log` is 17,567 bytes; a bounded unchanged copy is retained at `🗑️generated/caller-outer-daemon-tail.log`. It records background start at 19:53:10.357 UTC, polling at 19:53:53.377, unavailable at 19:55:10.344. There is no detached-daemon refusal, server-listen, watcher-subscribed, or explicit child exit status in that file. The current client waits for authenticated acquired IPC before polling; on unsuccessful polling it invokes its existing cancellation and awaits child close. The log does not expose the resulting close status, so an autonomous early daemon exit is not proven.

Pinned Nx server startup first awaits `watchWorkspace`, then emits the watcher subscription marker, initializes context, and binds its socket. No subscription marker appears. The logged PathNotFound entries come from directory watch registration under ephemeral Playwright profiles/reports and lease queue directories. Upstream pinned Nx23.2.0 `packages/nx/src/native/watch/watcher.rs` register_watches (54–75) explicitly warns and skips ordinary missing paths; only MaxFilesWatch returns an error. These warnings alone are therefore not a causal failure and should not be repaired by another catch or broader error suppression. Primary source: https://raw.githubusercontent.com/nrwl/nx/23.2.0/packages/nx/src/native/watch/watcher.rs .

A separate concrete scope mismatch needs ownership coordination: patched watchWorkspace passes the repository closed output-exclusion globs and `use_ignores=false`; upstream initial enumerate_watch_paths only receives use_ignores, while additional globs are passed to the event filter. The repository policy excludes `.🧬semio` and declared generated names, yet actual initialization attempts to watch ephemeral directories beneath that excluded tree. This demonstrates initial enumeration is broader than the event policy. It does not measure total enumeration time or prove that it alone exhausted startup availability. No production watcher repair, Native source change, reset, deadline change, or repeat run has been made.

Outer PID1479979 spans (not the timed target Node law): store starts4905.575ms; image-open255.839ms; reservation8952.089ms; initial recheck2917.339ms; birth starts17033.502ms; SDK verification3190.907ms; startup lease begins28871.666ms. The complete outer profiles are in `🗑️generated/store-stage-outer-profiles.json`; they cannot be substituted for the missing target-law measurement.
