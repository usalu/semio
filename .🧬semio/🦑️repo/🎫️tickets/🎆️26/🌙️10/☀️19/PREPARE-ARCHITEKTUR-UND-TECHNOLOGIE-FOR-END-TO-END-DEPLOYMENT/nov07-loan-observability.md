# Original Loan Observability

## Finding

No existing bounded non-secret projection identifies the selected resource loan and its limiting deadline or ancestry at issuance, remaining, or settlement. The private ControlScope retains command, requested milliseconds, start, epoch deadline, parentControl, follows and progress, but its WeakMap and traversal are private. remaining() reduces invocation, followed roots, parent rows, epoch, monotonic and progress cutoffs to a scalar and throws the fixed deadline error without retaining or reporting the winning row (routing/📥️invocation/🏃️process/🟦️.ts:196-201, 260-336). A live pre-expiry control.remainingMilliseconds() provides only the aggregate; once expired it throws. Settlement drains and retires the scope without emitting those facts (lines 222-224, 367-371). An aggregate sample can establish near exhaustion, but cannot name the restricting ancestor or distinguish tied cutoffs.

## Existing diagnostics and projections

- SEMIO_PROGRESS_DEADLINE_TRACE=1 enables progressDeadlineTraceV1. controlFor emits traceCreated only for renewable controls, with command label, rounded idle window and maximum in minutes, and units (lines 355-358). The process sampler emits traceWork with child and CPU values when child CPU work is observed (line 239). The budget schema describes those two events (🏃️process/⏱️budget/🧬️schema/🔣️.json:152-170); they do not carry a unique loan identity, issuance time, actual effective limit, parent/follows linkage, settlement, or winner. No trace was captured in the reviewed run. ProgressDeadlineExpiredV1 reports idle versus maximum for a progress deadline (routing lines 290-293, 318-320), a different diagnostic from fixed resource-loan exhaustion.
- readIssuedResourceControlCapabilitiesV1 checks a live control and returns invocation capabilities (lines 338-341), not loan facts. requireIssuedResourceControlDescendantV1 and requireIssuedResourceControlRefinementV1 only assert supplied live relationships (lines 344-345, 470-471); they do not enumerate or identify a limiting ancestor and themselves require live controls.
- scriptResourceProcessEnvironmentV1 computes a bounded child process envelope from a live control, ancestry and carried envelope (lines 411-419). Its deadline projects child authority, subject to maximum-only handling, ceiling and carried limits; it does not attribute the actual minimum. The envelope includes capabilities and must not be logged as a non-secret projection. readIssuedScriptProcessEnvelopeV1 exposes carried data only for a live invocation (lines 463-464).
- readAcceptedPhysicalCleanupV1 returns immutable funding facts only for the exact live accepted cleanup token (lines 437-440, 443-448). It is not a general loan identity or ancestry view and does not apply without actual enrollment. whenAcceptedPhysicalCleanupSettledV1 observes completion only, with no loan facts.

## Scope

Source inspection only; no execution jobs, control substitution, expiry change or verification bypass. This finding is limited to the canonical routing/process owner and its directly used progress-deadline diagnostic APIs.
