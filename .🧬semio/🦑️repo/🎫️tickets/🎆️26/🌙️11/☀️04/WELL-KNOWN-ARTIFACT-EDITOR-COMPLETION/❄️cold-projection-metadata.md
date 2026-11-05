# Cold Projection Metadata Retirement

The final post-handoff cloned flag previously demanded the entire projection frame and omitted its actual write from the receipt. It now demands size_of::<bool>(), reports one copied item/byte and one work unit, and preserves the flag on denied items/copy/depth. Terminal probes remain zero-work. Concurrent mandatory work ports were preserved.

Strict AJV schema validation and independent SQLite transitions passed five cases. The actual Rust integration reproduced RED (0 passed, 1 failed, 0 filtered), then passed GREEN (1 passed, 0 failed, 0 filtered, rc0). Cases cover all three denials, exact funding, and a terminal zero-grant probe. The private test-support helper constructs the actual post-handoff state without payload owners and starts with stale work to verify reset.

Changed ownership: store/paged-one-item/cold-apply Rust demand/receipt and test-support probe; paged-one-item explicit probe reexport; one-item-owners Cargo separately admitted integration target; cold-apply/tests/metadata neutral JSON/schema and Rust test. Ticket cold-metadata-oracle.mjs provides the independent oracle. Existing test-support/test dependencies are reused.

Actual root-workspace command: test -p semio-framework-os-store-one-item-batch --features test-support --test cold-projection-metadata. Generated canonical RED/GREEN logs use these same labels. No bounds, grants, or dependencies were increased. Authoritative source closures before small final edits were library9files/238655B/3049lines and separate test2files/2239B/12lines, fitting unchanged limits; the preexisting oversized broad lib-test closure is not selected. Rust AST passed.
