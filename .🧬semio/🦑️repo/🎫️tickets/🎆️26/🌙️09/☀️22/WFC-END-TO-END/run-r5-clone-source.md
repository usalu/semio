# Clone Source Retirement Failures

The shared clone source and close paths now forward `OriginalRetirementOutcome`, keeping original compound failure ownership and actual work. The projection close surface follows the same contract. Source laws use a generic error observer that debits each original receipt without flattening a retirement failure into `ValueError`.

A schema-backed fixture and owning test cover a primary failure, independent work refusal and physical receipt refusal in one actual close turn; the test checks all fields, work, receipt and retained owner. The TypeScript suite validates the same portable fixture with Ajv and JSON.

The first focused framework-workspace Cargo run found 17 production and test type errors. After migration, the second found two test-only compilation issues; both were fixed. The third, fourth and fifth runs exited 0 with 15 passing tests each. The fifth includes the widened clone cursor advance contract and the compound-failure pointer-identity assertions. The separate clone-source-testing helper now forwards the typed failure through its original fixture bank. Its first run passed one of two tests; the other included one-time fixture JSON initialization in its allocation observation. After moving initialization before observation without changing physical assertions, the rerun exited 0 with both tests passing.
