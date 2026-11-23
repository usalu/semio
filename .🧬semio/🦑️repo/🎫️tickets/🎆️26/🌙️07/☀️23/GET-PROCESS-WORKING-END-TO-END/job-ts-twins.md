# Job TS twins: 31 failures to 0

Result: `bun test` over all `🧵️job/**/🧪️tests` TS twins is 153 pass, 0 fail. No Rust product code changed, so the Rust job tests are unaffected (only JSON schemas and TS tests were edited; no shared fixture the Rust tests read was modified).

Every failure was drift in the TS twin, not a Rust regression. Source-text laws now read the family (`readSourceFamilySource`) or the owning crate file and assert the current invariants.

## Schema or fixture drift (Rust fixture is truth; schema and twin updated)
- `♻️retirement/🎟️context/🧬️schema`: added `issuedArchiveClose` (4 integers plus `pending`/`refused` consts). Fixed 5 tests.
- `📬️outcome/📦️builder/🧬️schema`: `initialization` now equals the dense one-turn header schema (`🏗️initialization.json`). The old per-binding turn loop test became "initializes its scalar header and empty live prefix in one paid turn".
- `⏱️context/🎟️source/🏠️native/🔣️.json`: added `individualClosure`, `nativeExecution`, `guard`, `escrowBirth`. Twin now also checks individual closure with an independent SQLite relation.
- `⏱️context/🧪️testing/🎟️caller/🧬️schema`: `workLoans` const set to the fixture (remaining 13,12,10,10,9). Twin starts loans at `fixture.fuel`.
- `🛬️receiving/🎟️native-turn/🧪️tests/birth-chain.*`: schema and twin rewritten for `fresh`/`refresh` stage vectors and fuel cuts (born iff fuel >= fresh sum; cuts include needed-1 and needed).

## Source-text laws re-pointed to the current owner
- Facade tree no longer reaches carved crates: grant, close-demand, descriptor, erasure, owner twins use `readSourceFamilySource(job)`. Grant regex `work_grant` now uses a word boundary (crate name `job_work_grant` is legitimate).
- Payload owner moved to `📄️payload/🧱️payload-owner`; descriptor ownership to `📦️builder/🎟️descriptor` (`OriginalOutcomeDescriptor`/`OriginalOutcomeView`); `fn consume_retained` is a trait method; builder no longer posts receipts itself (returns `RetainedWorkOutcome`).
- Changed signatures: `InteractiveJob::step(cx, return_claim) -> InteractiveJobStep`, `close_step(effect)`, `publish_last(bytes,output,cx)`, `admit_loan(&mut owners.source,cx)`, `source.close_step(parent,cx)`, decode `receive_observed_work`, `close_producer_step<T,E>`, `cx.original_clock_binding()`, park `validate_borrowed_context`.
- Loan twin: job stepping lives in the driver (`▶️drive`), loan source asserts it does not call `job.step(`.
- Capture twin slices `advance_typed_operation_output_inner` (public fn is now a thin wrapper).
