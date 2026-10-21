# Optional Graph Quota Pressure Regression — 2026-10-21

The admitted unit 74822 terminated with exit 1 before Vitest: `Closed optional graph cache quota required` in the optional graph storage constructor. The constructor rejected a closed integer snapshot solely because observed physical available bytes fell below the required floor. Writer maintenance can legitimately defer while existing readers retain custody. This is an observation of pressure; reservations must still decline.

## Test-First Save Boundary

Before the runtime edit, the following files were saved while the old `state.minimumAvailableBytes>state.availableBytes` rejection remained present:

- `🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json`: sibling closed `OptionalQuotaCase` contract. Existing OptionalQuota, mandatory Headroom, policies and schema ownership hashes unchanged.
- `🏗️build/📥️inputs/💾️storage/🧪️tests/🔣️.json`: twelve language-neutral cases covering pressure, exact floor, successful commit, release, logical cap and six malformed-state classes.
- `🏗️build/📥️inputs/💾️storage/🧪️tests/🟦️.ts`: the actual constructor and reserve implementation extracted from source with TypeScript; Ajv independently validates state and cases, SQLite independently checks closed integer state and arithmetic admission, then exact used/pending state is asserted after each action.

The low-headroom case expects successful construction and denial of both zero-byte and positive-byte reservations without mutation. At the exact floor, zero peak succeeds and positive peak fails. A one-byte funded commit consumes remaining headroom; release returns pending bytes. Logical used bytes above maximum, negative/fractional/unsafe/string counts, extra and missing fields remain rejected.

The parent owns the runtime change: remove only the physical-floor predicate from construction; retain closed integer validation and used<=maximum. Reservation capacity/floor checks, fresh statfs publication checks, ledger enforcement, exclusive reader custody and mandatory publication headroom remain intact.

## Owning Native Route

`repo --tool=nx run repo-graph-maintenance:test --parallel=1 --workers=off --output=stream`, with this ticket/artifact environment. Existing project `📥️inputs/💾️storage/📋️project.json` invokes its authored script test route, whose explicit Bun entries include this test file, ledger/phase/admission and graph headroom suites. No new command or dry run was launched for this boundary. Assertion execution and red/green results remain pending; source inspection alone is not a passing test.

## Review and Execution Handoff

Independent source review caught the exported constructor modifier in the new AST extraction. The test now strips only its leading export modifier before TypeScript transpilation, so new Function receives a standalone function. Parent confirmed the combined review complete, saved the narrow runtime predicate deletion, and launched its exact maintenance target as handle 8489 to generated/quota-proof.log. Results remain pending.

After that review boundary, exact repo-lib:test-build-source-demand launched once as own handle 46731, with prototype/trace, one job, workers off and direct complete stdout/stderr retention in generated/sourcedemand-quota.log. Its first observation is capacity waiting, not a test start. Parent owns quota execution and runtime_execution retains E2E 98132. No foreign process or agent was stopped.

Latest meaningful retained Source-demand observation: shared image queue advanced past the former PID37466 request, and the own 46731 command now requests exclusive `.cache/b/g` access at position 5/6 behind PID40609’s shared reader. Runtime_execution identifies PID40609 as its retained E2E process; its unit is queued at 6/6. No Bun assertion, terminal error or browser start marker has appeared. This is image custody progress rather than a new memory-admission failure, and no foreign process/reader was stopped.

## Concurrent Diagnostic Dependency

Parent independently observed an external concurrent storage.mjs edit at 02:56:39 adding optionalGraphQuotaObservationV1 and appending its bounded JSON projection to the constructor refusal prefix. The narrow physical-floor repair remains retained. The existing quota test harness previously extracted only the constructor; its invalid cases would therefore reference an undefined new helper. The harness now extracts the actual saved helper and constructor together, strips export modifiers, asserts exactly those two function declarations, and retains the unchanged Ajv/SQLite fixtures and refusal-prefix checks. No runtime source was reverted or changed by this agent.

Parent confirmed maintenance handle 8489 terminated on its original deadline before test assertions. This is not a failed quota assertion or a passing proof. Source-demand 46731 remains retained; no duplicate native command or restart was launched for the harness update.

## Source-demand 46731 Terminal Custody Failure

The native handle returned terminal exit 1. Direct stdout/stderr in generated/sourcedemand-quota.log is untruncated. It advanced through graph headroom authentication, then failed before any Bun test header/assertion.

The enforced SEMIO_LEASE_ORDER violation is hold-and-wait: PID58126 held repository-graph-facts:/Users/ueli/Documents/semio:shard:13 exclusively (graph rank3), while requesting native-fixed-image:/Users/ueli/Documents/semio/.cache/b/g shared (image rank6). The blocker PID57763 had held the conflicting resource for15s; waitedMilliseconds=15466, enforced=true, atMs=1792551714811. The guard directs the owner to release graph-facts custody before waiting or acquire image custody first.

Stack: process/leases/order react line224, contention252, queued lease awaitQueuedTurn485. This is an actual resource custody failure, not a quota assertion failure, stale source digest, memory admission refusal or a passing Source-demand regression. Parent, runtime_execution and independent reviewer were notified immediately. The guard was preserved, no process or agent was stopped, and no fresh command/restart followed this terminal result.
