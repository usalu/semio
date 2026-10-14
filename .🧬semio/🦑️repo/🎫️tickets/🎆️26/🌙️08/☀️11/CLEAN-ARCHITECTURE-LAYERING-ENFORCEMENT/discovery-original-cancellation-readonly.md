# Original Discovery Cancellation Read-Only Audit

Observation on 2026-10-14 at HEAD `7c8af563c74abe9a1c8b0f751224c7201311dfa3`. No source edits, tests, frontend, initializer, native operation, process signal, or resource action were performed. This report is the only created artifact. Existing Native10 result establishes actual CANCELLED status1 after1195.025 seconds, with Original Script process interrupted primary. The observed120.5 second pending cancellation and high CPU establish missing latency acceptance; an unsymbolicated sample does not establish a root cause.

## Proven Control Semantics

General Process `🧭️routing/📥️invocation/🏃️process/🟦️.ts:31–48` creates the actual controller, registers SIGINT/SIGTERM, retains the original epoch deadline and monotonic initial allowance, and provides `yieldContinuation` through `setImmediate`. Its progress publication writes stderr synchronously. General Script `🧭️routing/📥️invocation/🟦️.ts:42–48` checks original signal/deadline before publication, after publication, and after awaiting that same actual continuation. The primitive is already a real event-loop continuation. Replacing it or granting another controller/deadline is unsupported.

## Concrete Owning Source Gaps

Repo library `🏗️build/🕸️graph/📥️inference/🟨️.mjs` owns the selected discovery flow. `yieldInferenceV1:1574` advances only every16 outer units. The other15 calls resolve an async no-op, so their awaits permit microtasks but do not request the caller event-loop continuation or perform original checks.

`relativeScriptInputs:499–532` recursively resolves and parses the entire command/module closure synchronously. It has no invocation/control argument, original check, continuation, admitted file count, or aggregate source-byte limit. Selected owner admission calls it after matching source inputs at1627; earlier targetScriptSources can also reach this closure. Cycle sets prevent repeated traversal but do not bound a large acyclic closure.

The directory walk at1623 advances once per16 directories; a single directory's synchronous readdir and all entries have no cooperative control. At1624 all candidates of a pattern are matched synchronously, including all exclusions per candidate, with no original check or continuation. At1630–1635 the selected compile-import loop yields once per16 modules; each module's complete synchronous analysis/import resolution is unbounded by that outer cadence. Command recipe token enumeration at1611 likewise has no original control checkpoint.

The parser owner is `⚡️caching/📥️inference/🟨️.mjs:16–40`: TypeScript createSourceFile plus recursive forEachChild is synchronous; its16MiB cache-retention limit bounds retained facts, not input source bytes, parse time, closure size, or CPU work. Observation `👁️observation/🟨️.mjs:158–173` offers asynchronous file reads but deliberately retains synchronous parser semantics. Switching only to observedSourceAnalysisAsyncV1 therefore does not make parsing cooperatively cancellable.

Inference readPhysicalSource:43–50 uses observed synchronous lstat/readFile primitives and validates physical ancestors/type. No original-signal/deadline parameter or input-byte ceiling is present. These are filesystem primitives, not operation-owning cancellation controls. No claim is made that a Root System implementation can preempt a synchronous parser or that these reads caused the recorded delay.

## Repair And Portable Test Mount

The owning repair seam is the selected Source admission/closure computation, forwarding `context.invocation` unchanged into bounded async closure work. Check original signal/deadline for each admitted unit; request the same caller continuation between bounded work batches and before/after expensive semantic phases. Bound module count, aggregate input bytes, candidate/entry counts and parser input bytes using authored Source quotas. Preserve observations, resource interpretation, cache revision authority and original publication semantics. A single synchronous TypeScript parse still needs admitted finite input or an independently owned cancellable parser implementation; an outer await alone cannot preempt it. No new authority, renewed deadline, fallback Source or initializer belongs in this repair.

Portable test mount: schema-authored small synthetic acyclic module closure plus one directory with many candidate entries, all under the ticket generated output authority. Use a single original caller invocation with real setImmediate continuation and schedule abort via a host callback after observed progress. Assert identical original signal identity, original decreasing deadline, no post-abort source read/publication, and bounded admitted units between callback and refusal. Separate an expired original-deadline case and each authored quota refusal; do not assert wall-clock latency from synchronous fake callbacks. Validate corpus with existing TypeScript import oracle and schema validator; run through existing bun/nx command ownership. This is a proposed test, not an executed or passing test.

## Exact Read Source SHA256

Paths below are relative to the framework modules or Repo library indicated above; hashes identify the actual bytes read, not immutable runtime Source receipts. Concurrent later edits may change them.

- General Process invocation: `66d001007c289158aeb927fa9ded6c782e33db14f461b10301b53967684f958a`.
- General Script invocation: `3c161d5872c8fca2390dc5723a1a437d2b7098ef0f718a4c83fc37caef78dd23`.
- Repo build inference: `bdcafc1b5b802ebca5dbba30cf45e398228badf8a9609ca5af1f5088d030f75a`.
- Repo caching inference parser: `df19c14caea4c957cf545ce12bb3f47bebda29dabfcfd1fc8090f07824c89513`.
- Repo build observation: `34a821b3e97eda46bff9b19a9b6c69fe525b0b020916141a333c959cb6750172`.

HighScope owns resource-owner versus compile-dependency, normalized native object inputs and registry-index reuse repair. This audit does not duplicate those edits or qualify any later native run.
