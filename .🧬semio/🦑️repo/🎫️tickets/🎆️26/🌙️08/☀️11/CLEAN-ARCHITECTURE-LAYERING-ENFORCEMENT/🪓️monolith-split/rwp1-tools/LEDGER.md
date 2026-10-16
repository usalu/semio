# RENDERER-WP1 working ledger (tool notes; the deliverable is `../RESULT-RENDERER-WP1.md`)

Layout: module files stay where they are; each crate root `T/<folder>/🦀️.rs` mounts them with `#[path]` and re-exports (`pub use m::*;` or `pub mod m;`). `pub(crate)` -> `pub`. Hub line -> `extern crate <crate> as <module>;` or `pub use <crate>::{modules};`.

Carved (carved.json): mailbox-core, canvas2d-paint, hub-contracts, hub-workspace, media-slots, host-base, contracts, kernel-protocol, kernel-retained, kernel-pool, kernel-runtime, program-bridge, doors, agent-elements, dock, native-platform (accessibility half).

Open: asset-authority and asset-decode (need `AdmittedSurfaceToken` in contracts, WP2 lift), native_temporal (WP2 lift), contracts lifts from WP2/WP4, WP4 host-io cut, WP3 unmounts, media-slots roster law (WP3), Cargo.lock, taxonomy `📥️lifted`.

Gotchas learned: `#[path]` inside an inline `mod x { }` goes through the non-existent directory `x/` when it uses `..` (use a file module instead); a `#[macro_export]` macro is a union-root item in the union oracle; `extern crate` aliases of the old root (`dsl`, `protocol`, `store`, `store_sync`) are extern-prelude names that every module of a crate relies on (`aliases.py`); python 3.9 has no `tomllib`.
