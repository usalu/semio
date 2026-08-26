# Shared Typed-Command Full-Operation RED Scout

Date: 2026-08-26  
Scope: read-only source/static scout of the shared typed-command route in `framework/os/plugin/component.rs`. No production file was changed and no runtime claim is made.

## Verdict: RED and Central

The Phase-8 ledger has eleven concrete bounded-first-step factories but admits zero command rows because the shared typed-command route is not yet a retained full-operation state machine. This is the central admission dependency: app-local proofs must remain fail-closed until preparation, reducer execution, output validation, ephemeral publication, emit/commit, and exposure all execute through one cancellable persistent job.

## Exact Current Gaps

- `dispatch_typed_command_inner` performs `refresh_cache`, draft/interaction/presence/transient snapshots, child-root capture, document revision/generation reads, envelope access, cancellation-scope construction, and worker-session construction outside a full-operation job.
- The only worker payload is `BoundedFirstStepCommandJob<A>`. Its single `step` synchronously builds views, calls `A::ephemeral`, calls `A::handle`, validates the complete output, writes the completion, and returns terminal completion.
- `bounded_command_output_bytes` iterates every mutation and child emit, invokes binary encoders for every mutation, and whole-serializes effects/events/UI scope. The root gate explicitly rejects this as a monolithic output pass.
- `ActiveToolCommandStage` has only `AwaitWorker` and `CommitReady`; it has no prepare/reducer/output-validation/ephemeral/emit/expose cursors.
- The full-operation type and required stage vocabulary do not exist. The permanent gate requires `TypedCommandFullOperationJob<A>` with stages `typed-command-prepare`, `typed-command-reducer`, `typed-command-output-validation`, `typed-command-ephemeral`, `typed-command-emit`, and `typed-command-expose`.
- Output, presence/transient application, dispatch/emit commit, final freshness validation, preview/checkpoint publication, cancellation, and bounded close must stay under the same retained authority. Moving spellings without cursorizing their real work is not acceptable.

## Required Packet Boundary

The remediation packet should own only the shared plugin component plus its focused fixture/gate/report. It must:

1. Replace the two-stage active shell with the six-stage retained full-operation state machine.
2. Capture immutable event-maintained roots in O(1) and move preparation into the job.
3. Keep the app reducer as one proof-governed semantic unit only for exact bounded-first-step rows; app-owned resumable jobs remain nested retained producers.
4. Replace whole output measurement with typed field/item/byte census and validation cursors, fixed caps, preflight, progress, checkpoint, and exact owner handback.
5. Incrementally apply ephemeral and commit outputs, validating revision and generation immediately before exposure.
6. Prove zero-fuel no-op, cancellation at every transfer, stale generation/base rejection, max/+1, output parity with the test-only external codec oracle, interrupted close, terminal idempotence, and no direct `A::handle` bypass outside the reducer stage.
7. Keep all incomplete paths fail-closed until every stage is genuinely wired.

## Evidence

The machine ledger is `📊️all-tool-job-coverage-live-2026-08-26.json`. Its live result is `productionFactories=11`, `productionRegistrations=1`, `productionDispatches=3`, `boundedRows=0`, and `remainingCommands=884` occurrences (`773` unique command rows). The permanent static gate correctly labels even Demonstrator's already-proved `changeSchema` row as blocked by the shared full-operation route.
