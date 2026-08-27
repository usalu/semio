# TXT Registered Metadata Review

The five direct TXT leaves have canonical Rust/descriptor primaries and complete derived descriptor/provenance tests. Their bounded metadata integration is supported by the 34-test actual-source harness, including one independent five-token SHA-256 oracle, and the actual registered STDIO test binary. This is not acceptance of the full TXT mutation root or its remaining surfaces.

The first registered invocation built successfully but selected zero tests because its filter omitted `standards::v_utf_8::subsets::any`. That exit 0 is not a runtime pass.

The corrected registered invocation selected exactly 50 TXT schema tests. It finished with 48 passed, 2 failed, and 5961 skipped, exit 1. Nextest run: `154b9eb1-6faa-469b-aa5d-fff00103cd43`. Transcript: `🧪️txt-metadata-registered-corrected.log`.

An independent retained-binary invocation first asserted the exact 50-test selection and all five metadata-test names. Its run `5ca09276-d37e-4e23-b052-f1d0496ca46b` also finished with 48 passed and the same two failures. Transcript: `🧪️txt-registered-selection/🧪️root.log`; selection: `🧪️txt-registered-selection/🧫️run-tkN9uq/🔣️selected-tests.json`. Its binary SHA-256 before execution was `9eb6232b467ad08a3c422761565aa9e00f63e8f8d73485ca7a9bf212ae4c6b47`. The test runner exits the process on failure, bypassing the harness `finally`; no post-run executable fingerprint or results file was recorded, and no unchanged-executable claim is made for this failed invocation.

Both failures are in the schema-owner tests `committed_grammar_and_protocol_files_parse` and `protocol_walk_law`. Both reject the committed mutation binary protocol at line 10: `chain tag u8`, with `unknown protocol directive u8`. The 48 successful tests include all five canonical metadata/provenance tests. The protocol defect has a bounded repair lane, TXT-PROTOCOL-22, which must retain wire behavior and prove all five real encoded operation frames. Neither test is removed, ignored, or weakened.

Current full TXT acceptance remains red. The generic protocol repair, canonical aggregate/mount and remaining required-language cutover, mandatory metadata propagation, and complete owner-level gates remain required.

The retained-binary harness was corrected to use the repository's budgeted child-process handle without exiting its evidence-owning parent. A fresh replay again ran exactly 50 tests, with the same 48 passes and two failures, child exit 100 and no signal. It now retained the complete results and confirmed the executable SHA-256 was unchanged: `🧪️txt-registered-selection/🧫️run-Pd4fdZ/🔣️results.json`, transcript `🧪️captured-failure-root.log`, Nextest `40e2239b-7baa-4aa6-8cd6-e863c30afccd`. This repairs evidence retention, not the protocol defect.
