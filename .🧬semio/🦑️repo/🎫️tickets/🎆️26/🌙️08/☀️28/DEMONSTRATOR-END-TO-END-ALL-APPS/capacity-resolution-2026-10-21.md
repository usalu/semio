# Capacity Resolution 2026-10-21

High owner source audit. Parent owns integration and native diagnostic ingress; runtime_execution owns the fresh unit command and its newly exposed owner-discovery failure. No accounting or safety-policy edits were made. No additional heavy command was queued, and no foreign process was stopped.

## Fresh Evidence

The previous low-memory refusal is no longer the current host state. Read-only observation of `/Users/ueli/Library/Caches/semio/host-capacity/sample.json` at 00:54:05 UTC recorded a 32,768 MB host with 15,824 MB available, normal pressure, 3,811 MB swap used, zero swap-out rate, and load 13.66. The exact source sample timestamp was 1792544045428.

The ledger then held one fresh run grant, PID 79940, four jobs, 9,728 MB reserved, 732 MB observed, admitted at 1792544029547. Its one pending cargo request, PID 80070, asked for four preferred jobs and a 3,584 MB minimum. runtime_execution independently confirmed that its fresh unit invocation was admitted with four of eight jobs, had not launched cargo, and later failed before Vitest at `Canonical generated import path required` in owner discovery. Therefore the cargo waiter is independent; this evidence does not support a missing inherited parent grant.

The existing read-only command `repo capacity status --json` immediately returned `unknown command "capacity" for "repo"` with exit 1. It did not enter a capacity queue. Current TypeScript source declares that command, and the parent is checking native frontend routing/publication with the supported global tool flag ingress. This refusal is diagnostic ingress evidence, not permission to call raw system probes or bypass capacity.

## Accounting Finding

No confirmed memory double-counting, stale-grant defect, or unsafe pressure classification was established by this audit.

The process owner decision function uses the actual host free-memory sample and subtracts only the part of a ramping grant's reservation that is not already represented by its observed process-tree residency. At the fresh snapshot, the grant's unrealized portion was 9,728−732=8,996 MB; available headroom was 15,824−6,553−8,996=275 MB. This explains why an independent 3,584 MB cargo request waits even though the run has just been admitted. Counting the observed residency a second time would be wrong; the reviewed algorithm avoids that subtraction. The separate budget calculation is a binding upper bound rather than an additional subtraction from physical headroom.

The five-minute ramp is authored policy, after which actual observed residency controls committed accounting. It must not be removed merely to admit an unrelated waiter sooner. The source preserves the 6,553 MB floor for this 32 GB host and the existing swap-out, critical-pressure, file-table, attachment and load controls.

## Supported Smaller Reservation

For a retried single-task unit owner, the supported outer scheduling control is `--parallel=1` before any argument separator, or the equivalent `SEMIO_BUILD_PARALLEL=1`. The build owner reads this preference, sizes a real one-job request, and rewrites the granted outer scheduling argument so the tool cannot exceed that grant.

The authored run fixed block is 1,536 MB. Task request sizing conservatively includes the build kind, whose per-job estimate is at least 2,048 MB, even for a test target. A one-job run therefore still requires 3,584 MB above the unchanged floor. Four jobs reserve 9,728 MB. The smaller preference reduces the simultaneous reservation and intended outer concurrency; it is not an override of pressure, a lower safety floor, a promise that internal test workers are bounded by that flag, or an arbitrary smaller compiler estimate.

No second unit request was queued to exercise this preference: runtime_execution already owns the admitted unit's next confirmed failure. The next action is to fix that concrete owner-discovery rejection and retry through its existing owner with the scheduling preference chosen explicitly by the parent. Native capacity admission itself has now succeeded; it is no longer accurate to report all execution as still memory-blocked.

## Source Inventory And Limits

Reviewed source:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/⌨️command/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json` hostCapacity and workers declarations
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📏️decision/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🎫️gate/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/📡️sampling/🟦️.ts` process-tree observation and injected probe interfaces
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧪️tests/🟦️.ts` neutral fixtures, schema agreement, SQLite decision oracle, fairness and sizing tests

The existing source already test-drives the decision with language-neutral cases and an independent SQLite oracle and validates schema agreement with Ajv. Those tests were read, not executed here. No policy estimate was changed without measured resource evidence. A later source-read tool call was delayed, so this report does not claim a completed internal Vitest-worker propagation audit or a confirmed native publication freshness cause. Parent and runtime owners retain those integration responsibilities.

## Confirmed Graphless Diagnostic Ingress Fix

Two concrete source defects were confirmed after the initial accounting audit. First, caller child bootstrap derived a fictitious Nx launch for capacity/leases/remote-execution because a hand-maintained exclusion list omitted those schema-declared neutral controls. The shared buildOperationHasNeutralBootstrapV1 now derives dependency-free, neutral-in-both-tools bootstrap ownership from the existing operation catalogue. Child capability expansion uses it while retaining special tools/reset launches and watch descendant behavior. No capability grant, Original custody, host floor or resource admission rule was weakened.

Second, runBuildCliV1 tested args[prefix] for early capacity/leases dispatch before stripping global --tool options. Under the supported stale-native fallback, [repo,--tool=nx,capacity,status,--json] therefore missed early diagnostic dispatch and entered general build execution. The route now uses the existing pure parseBuildInvocationV1 only to normalize the diagnostic/help selection. All other requests retain their original arguments for existing admission.

Parent verified the corrected public route `repo --tool=nx capacity status --json`: exit0 in about0.1seconds, with no graph/store/bootstrap wait. Its official sample reported available16687MB, normal pressure, zero swap-out, floor6553MB, unrealized8265MB and headroom1869MB. This is actual native diagnostic execution evidence. A bare stale executable still refusing capacity is a separate publication freshness issue; no native source edit was needed.

Schema-first regressions extend the existing caller-birth fixture/schema:9new neutral launch cases (21total),14classification cases checked against an independent SQLite query over the authored operation catalogue, and8global-option dispatch cases. An actual runBuildCliV1 TypeScript-AST port test verifies capacity/leases/help routing without calling graph execution; a separate failing Nx-bootstrap spy guards neutral control capability expansion.

The declared focused owner was launched once: ticket artifact environment plus `repo run @semio-tech/repo-lib:test-build-caller-birth --parallel=1 --workers=off --output=stream`, session7167. Initial state was a legitimate capacity wait (need3584MB;1817MB spare;2running,1ahead). Actual birth test result remains pending; no unit pass is claimed.

Changed implementation/test files, relative to repo-library/🏗️build:

- `📜️script.ts`
- `⚙️operations/🟦️.ts`
- `📥️caller/📤️child/🟦️.ts`
- `📥️caller/🧪️tests/📥️birth/🟦️.ts`
- `📥️caller/🧪️tests/📥️birth/🔣️.json`
- `📥️caller/🧪️tests/📥️birth/📋️schema.json`

Own stalled source-reader kernel was reset after confirming parent source readers were unaffected; reads resumed immediately. No foreign execution or lease holder was stopped.

### Complete Global Option Boundary

Independent review identified that `--lang` before a control command still hid the command from early dispatch. All declared public globals were inspected: `--tool`, `--workers`, `--help`/`-h`, and `--lang`; graph policy already has its own parser. The operation argument projection now consumes value-bearing globals from the operations catalogue after selection validation, preserves help and the opaque `--` tail, and carries the resolved language into early handlers/help. Ordinary admitted commands retain the original normalized argv. Five neutral diagnostic cases and one localized bootstrap case were added, plus an independent SQLite catalogue/argument projection oracle. This also changes invocation and option owners; no capacity floor, reservation, ramp or foreign process was changed. Native caller-birth execution remains in source/graph preparation at this checkpoint.

Live verification of `repo --tool=nx --lang=en capacity status --json` passed exit 0 immediately after the global-option fix. Official sample: available 13,896 MB, normal pressure, zero swap-out, floor 6,553 MB, unrealized reservations 1,025 MB, headroom 6,318 MB, three one-job grants and no waiters. The caller-birth native target is still preparing graphs/source imports; no Bun result exists yet.

### Actual Caller-Birth Result and Graph Retry Investigation

Native caller-birth session 7167 terminated exit 1 before executing Bun tests with `Build source authority keeps changing during inference` at graph/🟦️.ts. The source loop admits nine attempts (0 through 8); `build:refresh` follows an invalidated candidate, so the repeated 45-owner passes are retries, not independent dependency scopes. The retained output is `🗑️generated/capacity/caller-birth.log`. The previous trace-authority branches did not emit the collected stale-fact evidence.

The graph owner now emits an opt-in `[DEBUG] graph-retry` document through the existing `SEMIO_BUILD_TRACE_AUTHORITY=true` option. A closed neutral diagnostic schema declares membership, admission, observed-facts, producer-error, owner-registry and publication-facts reasons. Documents contain only content/membership hashes and at most eight bounded workspace-relative source paths; outside paths are redacted and raw observed facts/environment values are omitted. Current file digests use the existing bounded physical-source descriptor reader. Diagnostic projection failure falls back to a reason-only document and does not change graph admission, retries or safety. Helper/schema are included in defining graph authority. Neutral scenarios plus the SQLite reference/Ajv assertion live in caller-birth; their native execution remains pending. Parent owns the new focused source-demand route for the next actual run.

The resumed source inventory is recorded and deduplicated in `files-2026-10-21.md` (12 exact source/schema/fixture/test paths, including the invocation path already present). Final diagnostic fixtures contain 14 routing cases, including scoped option normalization, and 22 bootstrap cases. Fresh trace-enabled source-demand session 27260 passed memory admission with one job/3,584 MB and is currently preparing source closures; no test assertion result is claimed.

Fresh source-demand session 27260 completed all 1,039 project inferences and 4,107 input closure expansions and successfully published its graph. It emitted no retry document. Timings from the actual owner: initial admission 48,339 ms; inference 161,795 ms; final admission 167,738 ms; expansion 38,248 ms; graph publication 47,779 ms. It then entered Nx storage planning; Bun assertions have not yet executed. The earlier invalidation loop has not reproduced in this fresh stable-source run. Independent Low snapshots found all 26 defining baseline sources and an expanded 40-file set unchanged during the checked interval.

Parent confirmed actual retry diagnostic execution in its mode session 94452: reason `observed-facts`, unchanged membership, and a stale owner-discovery file whose expected digest began `e739` while the current digest began `0e0`, together with the announced fixture/test repairs made after that invocation began. This validates that the diagnostic identifies a real source change before test execution. Parent terminated only its own obsolete mode invocation and retained the trace; fresh mode verification is parent-owned. No guard was weakened. The source-demand session remains live in build-tool verification.

### Fresh Source-Demand Terminal Result

Session 27260 terminated exit 1 before Bun assertions. Graph publication succeeded, then the immutable authorizer probe importing the derived Nx namespace refused `Compiler-captured physical Source byte identity changed` (Node 24.15.0, Bun 1.3.14). Low independently detected outside-fleet changes to the revision schema at 01:35:50 UTC and revision implementation at 01:37:16 UTC during this invocation. The trace remains under `🗑️generated/capacity/source-demand.log`; no cache or receipt was edited. This is a source-identity refusal, not a test assertion result. A fresh owning retry is appropriate only after this terminal proof and the current source save boundary.

Current 26-file fingerprints confirm only the two announced revision files differ from the fresh-run baseline. After terminal proof, source-demand session 89418 was launched once with the same supported single-job/workers-off configuration and tracing. It initially waits legitimately: 3,584 MB requested, 616 MB spare above the unchanged 6,553 MB floor, four grants running. No parallel duplicate or capacity bypass was introduced.

Read-only determinism review of `closeBuildOwnerAdmissionV1` found that returned owners, files, target/input entries, providers and generated producer dependencies are explicitly sorted; no ordering defect was established. Fresh session 89418 has reached all 1,039 project inferences and final selected closure checks with no retry document so far. Assertions remain unexecuted at this checkpoint.

Fresh session 89418 emitted an actual `observed-facts` retry with unchanged membership (`cfd0cc78…`): project-assembly/🟨️.mjs expected `5eb20e1d…`, current `06376a34…`. Its current filesystem mtime is 2026-10-21T01:44:41.472Z, compared with the prelaunch source snapshot 2026-10-21T01:38:34.743Z. The run remains live and has entered another candidate attempt; no assertion result is claimed.

Parent authorized SIGINT only for this obsolete own pretest session 89418 after the exact project-assembly change was proven. It settled terminal exit 1 with `Original Script process interrupted`; the cancellation and prior stale document are retained in `🗑️generated/capacity/source-demand-fresh.log`. The source path is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs`, mtime 2026-10-21T01:44:41.472Z, current hash `06376a34d7002dce7ada9d3f34a88c4a33f21a2fbb45daeac9c0f8d7f47c77e5`, expected hash `5eb20e1d0252a2d1756dd8df67ab11dec7d41303aca42cd6018039e290417ddc`. Low was asked to add this source to its future baseline. No foreign job or agent was stopped. No new infrastructure invocation will start until the main unit reaches actual Vitest; parent will batch source-demand, mode, birth and publication owning targets through one fresh admission. All assertions in these pending regressions remain unclaimed.

Concrete fresh-batch argv, exact four root tasks, missing cross-product target semantics, registered suite/oracle inventory and all owned terminal proofs are prepared in `infra-batch-2026-10-21.md`. It has not been executed or dry-run. Parent will notify when actual unit Vitest execution permits its one invocation.

## Live Resumed Sample At 2026-10-21T02:14:25Z

Supported `repo --tool=nx --lang=en capacity status --json` again exited0 immediately. Host available10063MB, normalpressure, swap-out2.12MB/s, floor6553MB, headroom3510MB, no waiters. Five existing run/Cargo grants remain owned and active; no process or agent was stopped. This confirms usable diagnostics and active admission with the application pair preparing; it does not assert a test pass.

## Supported Queue Inspection

Current early CLI normalization supports `repo --tool=nx --lang=en leases status --json` without graph preparation. The leases owner command explicitly declares status/JSON and defaults to `.cache/b/l/m/data`; optional --all, --violations=<n> and --lease-directory=<path> are authored there. Parent requested this route for one bounded observation of existing queue edges; no cancellation or other queue mutation was suggested. The lease CLI may also include a host-admission section, omitted if its sampler is unavailable.

## Reduced Actual Unit Workload

High audited actual normal/long unitcallbacks as pure source/schema/render/fs work; none invokes nativeproducer, server, Vite build or childrepo. Authored default `{projectRoot}/**/*` with no observationrole promoted unrelated appsite/runtime/E2E TypeScript into native compile discovery. After closed16-entry neutralfixture+Ajv/TypeScript AST regression, parent declared only test/canonical-architecture sourceexecution+freshness with every actual configured and guarded executableentry retained. Default broadfreshness hashing, forwarded levels, Originaltest wrapper, generatedproducerdependencies and build/E2Enativeauthority remain. This reduces requested work semantically; memory savings and actual tests still unmeasured pending imageaccess. Normal Vitest maxWorkers1 and outeronejob/workersoff remain in effect.

## Source-demand 46731 Terminal Custody Failure

The native handle returned terminal exit 1. Direct stdout/stderr in generated/sourcedemand-quota.log is untruncated. It advanced through graph headroom authentication, then failed before any Bun test header/assertion.

The enforced SEMIO_LEASE_ORDER violation is hold-and-wait: PID58126 held repository-graph-facts:/Users/ueli/Documents/semio:shard:13 exclusively (graph rank3), while requesting native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g shared (image rank6). The blocker PID57763 had held the conflicting resource for15s; waitedMilliseconds=15466, enforced=true, atMs=1792551714811. The guard directs the owner to release graph-facts custody before waiting or acquire image custody first.

Stack: process/leases/order react line224, contention252, queued lease awaitQueuedTurn485. This is an actual resource custody failure, not a quota assertion failure, stale source digest, memory admission refusal or a passing Source-demand regression. Parent, runtime_execution and independent reviewer were notified immediately. The guard was preserved, no process or agent was stopped, and no fresh command/restart followed this terminal result.

Graph/image custody owner repair and ten-case schema-first regression are now saved and independently source-reviewed; see graph-image-custody-2026-10-21.md. No native verification was launched for that boundary.

## Fresh Three-Owner Admission

At 2026-10-21T03:44:32.957Z supported graphless capacity and lease diagnostics both exited0. All three current goal-owned checks are admitted with one job and3584MB reservation each: quota PID19334, unit PID19644, Source-demand PID19742. Available10274MB, pressure normal, swap-out0, floor6553MB unchanged. Source-demand now waits on exclusive graph-image custody behind four actual shared readers (including the current quota/unit readers); waitFor graph reports zero cycles. Foreign tasks and every agent remain running. Quota observed RSS3890MB and unit1771MB are live usage observations, not predicted peaks or proof of eventual success. Actual tests have still not begun. Raw diagnostics are generated/capacity-current.json and generated/leases-current.json.

## Read-Only Peer Status Observation

App listing/read-only inspection of `Finish architecture course build` (01a153c4-4ebb-7942-a6fd-ecdfa4235111) reports active, with its latest messages also retaining native invocations in repository startup queues. No message was sent and no process/thread was changed. The peer messages do not establish an exact process8301-to-own-handle mapping, so no orphaned/paused-process or cancellation inference is justified. Registry holder and queue facts remain sourced from the supported lease diagnostic. Only compact relevant status was retained here; the peer work is outside this goal.

## Supported Analysis Fleet Reclamation

2026-10-21T11:36:56.543Z: Four supported analysis-only REPL resets reduced aggregate used heap by about1.45 GiB. Low had no control/child; High Native handles live in the separate executor. Root verified0pending MCP requests, closed only its own idle repo-mcp stdin and observed exit0/no signal before reset. Minimal fs/path/log bindings were restored; all reports and inputs remain on disk. Root48592, Runtime5118 and Acceptance82799 remain live. No agent, build or foreign process was stopped; no forced GC, admission floor or custody bypass.

After the reset wave, supported capacity showed10144 MiB available, normal pressure, swap-out0 and older Cargo2654 admitted normally. Other host work changes concurrently, so this observation is not attributed solely to the heap cleanup and does not establish a test pass. The three goal checks are at normal graph-image exclusive recovery admission.


### Current Unchanged-Policy Host Sample

2026-10-21T13:22:43.264Z: Supported repo capacity status reported15,154MiB available,6,553MiB floor, normalpressure, zero swap-out and observed aggregate load47.17/10cores. OwnSource13310 was previously admitted at15,997MiB. Supported leases diagnostic exited0 and found foreign graphKernelEX45042, owninitialSH5/5 and no cycles. This confirms ordinary memory admission remains possible, while graph custody is currently queued. No foreign job/agent action or policy override. Full output captured under generated for the ticket; exact registry commands are in command-provenance-audit-2026-10-21.md.


### Fresh Frozen-Source Admission Wait

2026-10-21T13:35:55.046Z: Fresh soleSource4598 is waiting normally behind one earlier request. Supported capacity-linear-work.json sample at1792589663687 records14,190MiB available, normalpressure, zero swap-out,6,553MiB floor and6,864MiB unrealized running reservations, leaving773MiB admissionheadroom; requested job is3,584MiB. Six other running grants are observed (including existing5632MiB reservations), aggregate load52.75/10cores. This is new ordinary concurrent reservation pressure, not evidence of renewed fleet analysis leakage or a reason to bypass accounting. No policy/source/governor change or foreign signal. All actual checks remain pending.
