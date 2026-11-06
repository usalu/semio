# Actor Reserved Return Observer

`StepContext::receive_original_reserved_return` adds the actual redemption to both aggregate retained/work observers and separate reserved-return observers. `OriginalActorContextSlot::dispatch_step` can read the exact reserved observers immediately after the producer and before `StepContext` drops; it currently returns only aggregate fields. The wire must retain those aggregate fields, and guest fresh-grant validation must checked-subtract only the genuine reserved part.

The actor also has a repeated-borrow path within one pending epoch. `OriginalActorContext::borrow_context`, `remaining_work`, and the next-epoch `validate_currency` currently treat aggregate work or effects as fresh. A correct change needs actual paid per-epoch reserved observers shared by every such borrow and a funded reset/baseline at the real epoch transition. It must retain both aggregate and reserved values on subtraction or receipt failure. Capturing only an inner producer callback or inferring a redemption from grant sizes does not establish this.

`actor-observer-cases.json` and its schema provide five language-neutral examples, including a valid aggregate larger than the current fresh grant and invalid over-redemption. The independent SQLite oracle passes all five; the actor Source implementation and native laws are not yet updated.

The nested kernel receipt is fresh-conserved: its spent and remaining fields use actual fresh values after checked subtraction of the Source-observed reserved delta. The independently authenticated fixed header retains the full aggregate for SRE reconciliation. Four schema/Ajv/SQLite nested projection cases pass; this is a model and does not establish an implemented actor observer or native proof.
