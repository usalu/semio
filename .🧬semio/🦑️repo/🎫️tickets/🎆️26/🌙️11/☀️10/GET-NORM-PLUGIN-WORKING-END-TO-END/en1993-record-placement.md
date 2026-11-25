# EN 1993 Record Placement

The snapshot schema now declares `x-semio-rust.file` for its 29 annotated row definitions. The twin planner reads this schema-relative location, groups projections by file and preserves the seven carved row crates. Invalid, missing and foreign-owned paths are refused. Existing schemas without this annotation continue to use their original record home.

The planner caches each source file once per run. The focused `record-homes` tests verified eight Rust plans, seven row homes and no semantic drift for the EN 1993 snapshot; a small fixture also verifies sibling preservation and rejection of an escaping location. Both tests passed (seven assertions). No Cargo, full repository scan, browser or full test suite was used.
