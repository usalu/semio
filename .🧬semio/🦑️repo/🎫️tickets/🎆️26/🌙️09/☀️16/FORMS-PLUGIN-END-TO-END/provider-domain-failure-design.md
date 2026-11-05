# Geometry Failure Transport Design

Status: a concrete provider compile-path mismatch is proven; shared production changes are held while root native93510 runs.

The provider has two double-question-mark sites in 🧵️embedded-evaluation/🦀️.rs: normal tessellation and cancellation-time tessellation closure. Both now receive GeometryWorkFailure. It is an owned OriginalDomainFailure retaining primary ValueError, complete GeometryWorkProgress (retained currencies and consumed CPU), and receipt_refusal. There is no From conversion into OriginalTurnFailure. String conversion or extracting primary alone would lose evidence.

## Existing API Boundary

The current canonical OriginalDomainReservation supports prepare, with_body and admit_original_domain_failure. Each refuses a cancelled context. The actual StepContext::with_original_retirement does not mask is_cancelled, and this is correct: geometry cancellation must observe the real cancellation state. The failure context has no retirement-admission capability. Related Flow pipeline callers demonstrate the existing reservation and close-ladder pattern but do not supply a cancellation-admission API. Thus the existing API cannot transport a cancellation primitive's failure on the genuine cancelled caller.

## Proposed Funded Retirement Route

Add explicit retirement-only counterparts to the existing normal reservation API, sharing private implementation rather than changing normal semantics:
- OriginalDomainReservation::prepare_retirement
- OriginalDomainReservation::with_retirement_body
- admit_original_domain_retirement_failure

Extend OriginalFailureContext with two actual context observations: authenticated original-retirement permission and retirement_should_yield. The StepContext implementation validates its original source and reports its existing original_retirement scope flag. These APIs require that genuine scope, preserve the original source identity, wallet, clock binding and finite grants, use the existing same-caller loans, and retain the actual cancellation observation for the callback. They do not derive another context, clear cancellation, renew deadlines, manufacture fuel or raise a cap. Normal APIs continue refusing cancellation/deadline expiry.

## Provider Integration

Retain one OriginalDomainReservation<GeometryWorkFailure> per operation. Quote its real frame birth before a primitive may fail. Run the normal geometry call inside with_body and move any failure into the paid slot. Keep success/cache outputs in the operation's existing owned cache, so no successful mesh can be dropped merely because an enclosing callback later refuses. Admit the original failure on the next funded frontier without reposting the child's already-debited work.

During close, enter the existing genuine original-retirement scope, use the explicit retirement route for cancel_tessellation_step, and preserve the same operation/generation identity. Move a cancellation failure intact into the paid frame and transport it through the retirement-only admission route. Retire unused reservation frames through close_original_retained_step so actual work and all retained currencies are debited once, including error/cancellation paths. Include the reservation in demand selection and unconditional terminal emptiness.

## Exact Planned Files

Shared production (held until root's terminal boundary):
- 🧰️framework/🔨️modules/🧵️job/⚠️failure/🎟️context/🦀️.rs
- 🧰️framework/🔨️modules/🧵️job/⏱️context/⚠️failure/🦀️.rs
- 🧰️framework/🔨️modules/🧵️job/⚠️failure/📦️domain/🦀️.rs
- 🧰️framework/🔨️modules/🧵️job/⚠️failure/🦀️.rs (explicit new transport reexport)

Shared laws and schema:
- 🧰️framework/🔨️modules/🧵️job/⚠️failure/📦️domain/🎟️reservation/🧬️schema/🔣️.json
- 🧰️framework/🔨️modules/🧵️job/⚠️failure/📦️domain/🎟️reservation/🧪️tests/🧫️fixtures/🔣️.json
- 🧰️framework/🔨️modules/🧵️job/⚠️failure/📦️domain/🎟️reservation/🧪️tests/🦀️.rs

Provider:
- ✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧵️embedded-evaluation/🦀️.rs
- existing provider native/unit and language-neutral oracle sources/fixtures, without another Rust file

Draft neutral input and schema are provider-domain-failure-law.json and provider-domain-failure-law.schema.json in this ticket. They specify normal versus funded-retirement admission and a geometry failure with separate nonzero body/receipt/work fields. They are design inputs, not a production implementation or native pass.

## Required Proof

Use actual OriginalJobFixtureCaller contexts, actual cancellation and clock expiry, same/foreign source identities, zero/insufficient funds and denied retained axes. A pending source must remain resident with the same allocation/body pointer until exact paid transfer or complete controlled close. Verify all primary/work/receipt fields through the owned projection, no duplicate body/CPU debit, and empty terminal state after both success and cancellation. Independent Ajv plus JSONPatch move/identity oracle validates the neutral ownership expectation; actual Rust laws must exercise the reservation and provider routes.
