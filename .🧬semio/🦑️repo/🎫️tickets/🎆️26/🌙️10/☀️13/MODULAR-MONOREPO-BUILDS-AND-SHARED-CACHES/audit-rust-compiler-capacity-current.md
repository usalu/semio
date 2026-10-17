# Rust Compiler Capacity Independent Source Audit

## Actionable Findings

No blocking implementation defect was identified in the inspected resource ownership and compiler-to-consumer slice. One nonblocking regression-coverage weakness remains:

- **P3 — Census law does not enforce the frozen exact census.** `🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🧪️tests/🪆️scope/🟦️.ts:58` counts recognized literal calls but only requires `calls > 80`. It also skips nonliteral option arguments. A future removal of as many as 15 current literal calls, or conversion to unexamined nonliteral options, can leave this test green. Assert the intended 96 calls and 53 files, or independently enumerate actual repository calls and compare the declared roster. This does not invalidate the current logged 53-file/96-call result; it limits the permanent regression law.

## Identity and Evidence

The requested report SHA-256 `09cb81567ee51ac4664212f24611e1530572bdee1835f566005b70dc2102217a` matched. All 78 declared Source/census rows and all 12 terminal-capture rows matched their declared byte sizes and SHA-256 before and after inspection: **90/90, no observed drift**. Whole-file identity includes shared coedited hunks and is not exclusive authorship attribution. The separate before and after inventories preserve every declared and observed value:

- `audit-rust-compiler-capacity-before.md`
- `audit-rust-compiler-capacity-after.md`

Read-only parsing of both final focused logs independently sums to 24 passing tests, 0 failing tests, and 664 assertions, across six serialized component runs. Both contain `census files=53 calls=96`. The declared census JSON contains 53 files. The strict roster contains 82 distinct existing paths. The three empty strict logs have the empty-file hash and zero bytes as declared; **empty diagnostic output does not independently prove a collected exit0**. Handle terminal outcomes remain statements in the frozen report, not independently retrieved process-state evidence in this audit. No tests or compilers were run.

## Source Assessment

`🏃️process/🏗️build/🎟️capacity/🟦️.ts:57-81` snapshots and freezes the closed schema-first exact/available/whole-host request before admission. Exact requests reject above host capacity; available intersects its bound with the host. The actual fair shared/exclusive gate precedes the serialized core picker. Available admission retains only acquired real core leases and requires at least one. Exact retries retire partial ownership. Cancellation checks, progress, continuation and polling remain in the acquisition loop. Retirement attempts all acquired cores and the gate, preserving the primary error when cleanup also fails. Whole-host uses exclusive gate ownership; environment markers cannot lend ownership.

`🏃️process/🏗️build/🎟️capacity/🪆️scope/🟦️.ts:19-37` keeps private AsyncLocalStorage ownership, actual retained namespace and control refinement. Nested available requests use `parallelism-used`, wait when zero, and increment/decrement only the actual grant. Whole-host nested work requires a full-host frame. Accepted children are retained and drained before parent retirement; explicit failed booleans preserve rejection with undefined. Failure closes admission and rejects queued descendants; cancellation removes the corresponding waiter. Resource-control scopes retain Original cancellation/deadline authority and pending descendants; this change introduces no new Original issuer, clock, registrar, adopter or budget renewal.

`🏃️process/🏗️build/🎟️capacity/🪆️scope/🤲️consumption/🟦️.ts:4` awaits the settled CPU scope before calling its returned continuation under the original retained nonCPU control. The actual direct Rust compiler consumes this owner rather than creating a separate post-CPU bypass.

`📚️library/⚡️caching/🦀️cargo/📦️units/🏗️compiler/🦀️rustc/🟦️.ts:26-42` preserves semantic/profile argument admission and the exact hashed `-Zthreads=1` check. Its `compileCpu` owns both cfg and link/dep-info child calls, builds the genuine first-party jobserver from admitted facts, acquires the implicit slot, supplies the real endpoint environment, awaits each owned child and retires slot/server in finally before returning from CPU. Cleanup aggregates preserve primary failure. The returned continuation creates a requested child from original `control`, rechecks reservation, reads bounded consumed.d, validates exact consumed byte evidence and rechecks reservation. It remains lexically within dependency readers, original compile workspace/reservation custody and prototype state operation. It does not inherit jobserver environment or retired CPU control.

The closed Rust resource schema and projection preserve profile DATA. Explicit multi-CGU, default backend, no-LTO profiles request available slots up to CGUs; one CGU, LTO, unknown CGUs and alternate backends request whole-host. The resource class is scheduling intent, not a guessed thread count. Existing semantic admission still refuses unsupported profiles. Requests and returned DATA are frozen; frames and issued jobservers remain privately owned. No fake positive compiler/storage provider was introduced by the observed neutral Source tests.

## Qualification Limits

The focused cohort supports Source and ordinary genuine process/SQLite ownership behavior only. The direct Rust compiler Source test is lexical/AST evidence, not a physical rustc invocation. Endpoint compatibility, actual backend worker behavior, physical input/provider/store/state behavior and publication remain unexecuted. No OS affinity or measured thread claim follows from slot grants.

The preserved broad types2 capture is explicitly failed and unqualified for physical fixtures and VSCode publication arguments. It is not turned green by the focused strict roster, and this audit does not require invented storage/provider owners. The focused strict roster and broad census are separate qualification surfaces. Existing failed logs remain present and hashed. No Source, AGENTS, Git state, ticket lifecycle or goal lifecycle was modified; only these audit Markdown files were written. No compiler, test, Cargo, Native/cache/SDK, network, browser or OS-sampling execution occurred.
