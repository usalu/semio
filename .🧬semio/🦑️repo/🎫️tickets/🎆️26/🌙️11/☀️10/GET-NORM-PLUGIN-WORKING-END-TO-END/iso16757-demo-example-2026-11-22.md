# ISO 16757 demo example test

Root cause: the demo DSL decodes fine and has zero failing checks (the panic message printed an empty list). `report.complies()` is false because check `iso16757.5.6.1.ifcView` is hard-coded `UnverifiedBasis` (Part 5 section 6 rules not assessable), and the engine deliberately counts unverified as non-compliant; the sibling test `🔬️compliance-report` already asserts `!complies()` for this reason. So no document can satisfy `complies()`; the demo test's expectation was wrong, not the document, schema or decoder.

Fix: `🎬️demo/🧪️tests/📚️example/🦀️.rs` now asserts no failing checks and that exactly `iso16757.5.6.1.ifcView` is the only unverified check (stricter than before on unverified, accurate on compliance). Engine and DSL untouched.

Note: `🧪️tests/🧩️example/🦀️.rs` is an unmounted stale duplicate with the old assertion; left alone.
