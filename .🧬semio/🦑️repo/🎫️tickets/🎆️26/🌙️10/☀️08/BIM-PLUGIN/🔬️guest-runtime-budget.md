# Guest Runtime Source Budget

The `semio-plugin-host-guest-runtime` unit aggregates many modules and source files outside its own folder. Splitting a file within that unit does not lower the aggregate budget. The 20.7 KB escrow module is a coherent candidate for a separate package, but it depends on crate-private guest runtime types and needs an explicit interface and dependency split. Snapshot calls and component codec are similarly coupled to runtime internals. No safe standalone test-only extraction was identified from the initial inspection.

The bootstrap selector and its fixture-driven Rust test were relocated to the interpreter runtime, which owns `OwnedSemioExport`; the host now calls the exported selector. The fixture and SQLite-backed TypeScript oracle remain at their original paths. `repo test-file` passes the oracle's eight cases. Native compilation and the resulting source-budget admission remain to be verified.
