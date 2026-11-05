# Cancellation Audit — 2026-11-04

Read-only source audit in the existing AI-MCP-END-TO-END ticket. Only this report was written. No source or Rust edits, builds, Git mutations, or native process interference.

## Finding: P2 — Owned Stop Before Callback Admission Escapes Containment

File: 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts:305,312.

withInvocationSignalV1 checks the narrowed alias before invoking the callback that owns the exact-reason catch. An already-aborted additional signal, or one aborted synchronously after the helper returns its promise but before the scheduled callback runs, throws outside that catch. The enclosing tracked work belongs to the parent birth, so it records the cancellation as a fatal parent failure. Catching the returned rejection does not restore parent usability or successful root settlement.

Reproduced using repo tools exec --light bun with a genuine createScriptProcessEnvelope/withScriptProcessEnvelope parent, an already-aborted controller, and withHandledScriptInvocationSignalV1. Output: boundaryRejected true; rootRejected true; callbackCalled false. Both rejection identities equal the exact owned reason. Expected: cancelled result after drain, callback omitted, parent remains usable. The new fixture covers abort inside the callback only, so it misses this scheduling boundary.

Suggested regression: neutral cases for pre-aborted owned stop and immediate synchronous stop after helper creation; assert exact cancelled result, no callback effect, and subsequent parent control admission. Place containment around initial alias admission as well as callback settlement while keeping parent abort fatal.

## Reviewed Safety Properties

- track at lines211–219 captures its ancestor list before execution and removes every accepted pending registration on fulfillment or rejection. With no ownedCancellation marker the failure behavior remains first-failure fatal, preserving default tracking.
- Exact reason containment checks both signal identity and non-aborted parent signal. Other exceptions and AggregateError values are not matched. A nested descendant records its own cancellation until the owned boundary; suppression starts at that boundary and extends upward.
- drain at222 and settle at223 close admission, repeatedly join accepted work and synchronously seal the empty owner. No accepted-child drain regression found by inspection. Alias authority is retired only after settlement.
- Alias construction at305 retains the exact policy, capabilities and envelope, combines cancellation with AbortSignal.any, and registers private lineage. No restriction or authority broadening found.
- Compiler image settlement at 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts:57 now preserves identical primary/close failure identity; genuinely different failures remain a two-element AggregateError.
- servePublishedMcpStdioV1 at MCP 🟦️.ts:230–244 propagates the gateway stop reason, retains preparation/publication promises, releases publication and awaits cleanup. Identity filtering is narrow. The Dev.runMcpOs boundary supplies the narrowed original and derives build/stage controls from it. Source inspection supports the EOF cancellation result; it does not establish native readiness.

## Target Metadata Follow-Up Still Outstanding

File: 🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🔀️dispatch/🦀️.rs:233–251,256–266.

target_instance stores a clone of caller selectors supplemented with catalog plugin/app ownership. select_target returns only success and no canonical resolved artifact metadata. artifact_instance requires target.artifact_ref to equal the requested artifact ID. Consequently instance-only selection leaves artifact_ref absent and cannot be recovered by the later typed artifact lookup. The target-context report correctly records this gap. Snapshot acceptance must include resolved census metadata retained by the owner and a regression that selects only instanceId before snapshotting the matching artifact. No Rust changes were made while the native gate runs.

## Limits

Reviewed the live source and existing neutral tests; independently ran the minimal cancellation reproduction. Coordinator evidence records passing cancellation/publication laws and actual bounded launcher EOF behavior, with discovery deliberately expired and no native readiness claim. No costly or broad test run was started by this audit.

## Detached Child Cleanup Probe

An additional genuine-envelope Bun probe admitted an operation without awaiting it in the callback, aborted with the owned reason, and then made the admitted child throw a distinct cleanup error after a short asynchronous delay. Observed: boundaryCleanupFailure true; childDrained true; rootCleanupFailure true. Both rejection identities matched the genuine cleanup error, confirming that this detached accepted-child case drains and remains fatal. The root coordinator is adding neutral pre-aborted regressions and fixing the initial-admission finding; this audit report intentionally records the reviewed pre-fix state until that verification is appended by the coordinator.
