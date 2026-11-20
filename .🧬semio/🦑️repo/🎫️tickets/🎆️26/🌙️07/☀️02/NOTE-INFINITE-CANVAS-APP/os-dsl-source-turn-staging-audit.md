# OS DSL source turn staging audit

Attempt 21 reported an unresolved direct reference to the value control contract when compiling `semio-os-dsl`. Two sites in [the original receiving driver](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🫴️receiving/🦀️.rs:195) referred to that crate without a matching manifest dependency.

The original receiving driver now resolves the source-turn issuer through `semio_framework_value_native_owned::OriginalSourceTurnIssuer` at line 196 and the work calculation through `semio_framework_value::invocation_extent` at line 205. Both crate entry points reexport the original control contract symbols. The operation still uses the same source witness, retained grant, and fuel cap.

A source check resolved both dependency paths from the OS DSL manifest, confirmed the expected reexports and call sites, and found no remaining direct control-contract references in the OS DSL module. The receiving driver was the sole source edit. The existing dependency graph and lock files needed no changes. The fresh guest build will verify the Rust linkage.
