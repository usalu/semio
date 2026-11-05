# Birth Construction Audit

Read-only audit of current process routing owner, 2026-11-05. No source edits or heavy/native run.

## Constructors

TypeScript AST inspection of 🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts found exactly two explicitly Birth-typed object constructions, matching both invocations.set calls. Birth is private to this file; no other alias registration path appears.

1. withScriptProcessEnvelope, line243: scope(), exact invocation, admitted envelope, projectedWires:new Map(). It seeds rememberProjectedEnvelopeV1(birth,admitted) before registering invocation. Independent process birth correctly owns a fresh numeric clock map.
2. withInvocationSignalV1, line306: scope(birth), alias invocation, same birth.envelope, projectedWires:birth.projectedWires, optional ownedCancellation. Root correction is present. Signal alias correctly shares exact original projected wire clock records rather than resetting them with a fresh map.

Both withScriptInvocationSignalV1 and withHandledScriptInvocationSignalV1 funnel through the second constructor, so handled and unhandled signal aliases receive the field. Further nested aliases recursively share it. Scope spread supplies live/accepting/pending/failures/retirements and parent; it does not supply Birth-specific fields. ControlScope retains birth as a reference via controlFor, and physical cleanup enrollment retains existing control/birth; neither constructs another Birth. Receiver enters withScriptProcessEnvelope, so actual child process gets its own legitimate process-local map.

## Validation

Used existing installed TypeScript parser through repo tools exec --light bun. AST reports both constructors with projectedWires and zero parse diagnostics. Source search also found exactly two invocation registrations. This is targeted constructor/syntax review, not a complete dependency graph typecheck. No remaining missing Birth field or obvious local field-type inconsistency found: map element shape matches envelope plus ProjectedDeadlineV1|null; alias assigns the same map type.
