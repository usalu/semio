# Prepared Supervised Headless Laws

These are ticket-only design inputs while native gate24680 freezes the framework headless runtime and tests. No frozen source was edited. The prepared schema, eight cases and independent Ajv/Node ordering oracle are headless-supervised-schema.json, headless-supervised-cases.json and headless-supervised-oracle.ts. All eight independent cases validate.

The planned public API is run_headless_supervised(job, operation, clock, site, supervisor, finish). The supervisor receives the actual mutable job and may return an owned R. It runs initially before business execution, and after acknowledgement of a genuine nonterminal borrowed outcome. Empty preparation callbacks do not count as a yielded outcome. A genuine terminal Complete, Cancelled or Fault reaches finish before any post-step supervisor. HeadlessOutcome carries Complete and Fault bytes by borrowed slice and a payload-free Cancelled variant.

| Case | Preparation Callbacks | Genuine Yields | Business Steps | Supervisor Calls | Finish Calls | Result |
|---|---:|---:|---:|---:|---:|---|
| initial cancellation |0|0|0|1|0|supervisor:cancelled|
| initial budget stop |0|0|0|1|0|supervisor:budget|
| post-yield stop |0|1|1|2|0|supervisor:budget|
| terminal complete wins |0|0|1|1|1|finish:complete|
| terminal cancelled wins |0|0|1|1|1|finish:cancelled|
| terminal fault wins |0|0|1|1|1|finish:fault|
| blocked supervisor result |0|1|1|2|0|supervisor:budget|
| preparation is not supervision |2|1|1|2|0|supervisor:budget|

The terminal cases configure the supervisor to stop once the business step counter reaches1. Calling it after a terminal event would therefore incorrectly replace the terminal result; both the counter and returned result expose that bug. Two paid preparation callbacks returning no borrowed outcome precede the last case's real Yield, so accidental supervision after empty callbacks also fails the case.

The native fixture should extend the existing real InteractiveJob fixture rather than introduce a runtime testkit or a second caller. Each semantic work span increments a business counter and emits an actual paid JobOutcomeBorrow::Yield before the supervisor can observe it. Physical preparation/capture turns are counted separately. The fixture records parent.source_identity(), operation and generation, and checks the same stable paid identity during close; wrapper addresses are not identities. Site stock and lifetime fuel are once-issued finite values. Cancellation and budget flags are public supervisor inputs, separate from a genuine terminal Cancelled outcome.

For blocked-supervisor-result, an external static AtomicBool keeps the actual job's close port blocked. The returned HeadlessFailure must retain the exact supervisor R, refuse early into_result conversion and keep its independent Drop counter at0. Opening that same port must permit close_step to finish from the original source, after which into_result returns the original R and one final Drop occurs. No renewed caller, stock or cancellation alias may be introduced. Every effectful reported close progress must carry positive work and fit the original grant; total retained capacity must equal total release at terminal emptiness.

Complete and Fault fixture payloads are32785 bytes, crossing two full16KiB pages; Cancelled has no payload. A static byte source avoids an uncharged fixture Vec. The fixture uses the public RetainedPayloadBuilder and receives each original initialization, append, seal and close outcome. Finish checks every byte and exact length. The native owner's staged headless capture must account for its own backing/copies before finishing; it must preserve partially captured bytes across turns, then close capture and original pages through the same source. Copy receipts must include both the fixture payload append and headless capture rather than merely reporting the original page copies. Capture turns must not trigger additional supervisor calls. No claim of this new API or its native laws passing is made before implementation and a fresh authoritative gate.

After the frozen run reaches a terminal/consumed-source boundary, the intended canonical fixture location is framework job/headless/🧪️tests/🧫️supervision. The prepared ticket data can be authored there alongside concise native law consumers once the native owner finalizes the public API. The existing headless package has ample source headroom after the cohesive framework move; actual Source Admission remains authoritative.

The frozen snapshot was subsequently cancelled by the authorized execution owner. The design is now activated as nine canonical portable cases and native consumers, plus a same-source capture/ACK-denial currency law. See run-r2-headless-supervision.md for the current implementation and validation state.
