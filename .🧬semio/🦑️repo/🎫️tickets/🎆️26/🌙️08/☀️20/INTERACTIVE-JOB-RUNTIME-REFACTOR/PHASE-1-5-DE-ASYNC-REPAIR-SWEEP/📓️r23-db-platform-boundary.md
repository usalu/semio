# R23 Database Async and Platform Boundary

## Outcome

The database crate now compiles and executes its native authority paths without decorative async bridges, while its portable artifact and wire surface compiles on both repository WASM targets. Native storage engines, the database facade/CLI, crash harness, and authority runner are explicitly absent on wasm32; portable hashes, artifact records, query vocabulary, and asynchronous artifact operations remain available.

## Repairs

- Pure query helpers and their call sites are synchronous.
- Production artifact submission, snapshot, query, and live-refresh paths await their real futures directly instead of entering db_actor::block_on.
- The CLI injects the process-wide headless worker pool into every opened database.
- Test authorities use a test-only injected pool.
- ContentHash is re-exported from the portable glue root.
- Native-only facade, CLI, engine, authority, crash-harness, and filesystem-backed laws use cfg(not(target_arch = "wasm32")); the artifact schema and portable APIs are not hidden by that boundary.

## Verification

- Native debug all targets: cargo check --all-targets — exit 0.
- Native tests: 424 passed, 0 failed.
- Native release all targets: cargo check --release --all-targets — exit 0.
- Browser WASM library: cargo check --target wasm32-unknown-unknown --lib — exit 0.
- WASI library: cargo check --target wasm32-wasip2 --lib — exit 0.
- Formatting: repaired glue, facade, artifact, testkit, and CLI sources passed rustfmt.

Warnings remain in pre-existing qualification, public-async-trait, and future-incompatibility categories; none is a compile error and none changes the platform boundary above.

## Files

- 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/📦️glue.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️component.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️component.rs
- 🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⌨️cli/🦀️component.rs
