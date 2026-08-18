# Wave 1 complete — and the db-ownership blocker for Wave 2

## Wave 1 result (all green)

| gate | result |
|---|---|
| `cargo test -p semio-framework-replication --features deflate` | **182 passed** |
| `cargo check … --target wasm32-unknown-unknown` (replication) | green |
| `cargo test -p semio-framework-os-kernel --lib` | **820 passed** |
| `cargo check -p semio-framework-os-kernel --lib --target wasm32-unknown-unknown` | green |
| `cargo test -p semio-framework-os-kernel-db --lib` | **424 passed** |
| `cargo check -p semio-hub` | green |
| `cargo test -p semio-framework-server` | **5 passed** (contract) |
| replication TS vitest (20-fixture byte parity) | **1 passed** |
| os TS vitest | **184/185** (1 pre-existing: missing wasm `pkg/` artifact, host crate untouched) |

Shipped: `🧰️framework/🔨️modules/📡️replication` (crate `semio-framework-replication`, `[lib] name = "protocol"`)
+ `🧰️framework/🔨️modules/⚠️diagnostic`; TS twin `@semio-tech/framework-replication` with the 20 wire
fixtures as the single canonical copy; `🧰️framework/🛍️products/🖥️server` registered as the fourth
product with a real `🔨️modules/🧬️contract` crate; kernel/db/hub rewired; launch.json + workspaces updated.

The kernel no longer owns the wire contract — it re-exports it as a facade, so the ~41 crates that
alias `semio_framework_os_kernel as protocol` needed **no change at all** (the plan budgeted a
45-crate sweep; the facade made it unnecessary). db and hub now depend on the replication crate directly.

## The blocker: db cannot move under `server` without inverting the dependency

The user's Wave-2 decision ("db is server technology and should move there") rests on a premise the
code contradicts. **db depends on the os document model**, so `server → db → os-kernel` would make
the server product depend on os — the exact invariant this whole product exists to establish.

Measured residual coupling of db on the os kernel, *after* the protocol extraction removed
frames/envelopes/ids/codec/diagnostic from the picture:

| os-owned surface db uses | occurrences | what it is |
|---|---|---|
| `pack::PackFile`, `write_atomic`, `read_footer_only`, `FOOTER_SIZE`, `os_pack` | ~40 | the `.spk` pack **container** format + io (stayed in os) |
| `store::pack_rt` | 8 | wire-value encoding over the os document model |
| `store::ArtifactStore` / `ArtifactPack` / `ArtifactCommand` / `ArtifactDsl` / `merge_base` / `create_document_envelope` | ~13 | the os document model itself |
| `dsl::to_dsl_value` / `from_dsl_value` / `DslValue` | 11 | schema-erased value type |
| `vcs::ArtifactVcs` / `Author` / `Checkpoint` / `Alternative` / `VcsError` | 6 | version graph |
| `pack_testkit::*` | 4 | test-only corruption fuzzing |

Relocating db to a neutral framework module does **not** help: db would still depend on the os
kernel from there. The dependency is on the os document model, not on the directory it sits in.

## Options

**A — Server defines the port, the instance supplies the engine (recommended).**
db stays where it is. The server product's `🎭️authority` defines a `DocumentAuthority` trait
(submit batch → receipt/outcome, welcome/bootstrap, frontier) in terms of `protocol` types only.
Hub — which already depends on both server and db — implements that trait over `db::ArtifactHandle`
and registers it. Server never names db, so `server → os` never exists. Zentrale can register a
different engine or none. Costs one trait boundary; buys the invariant immediately.

**B — Extract db's remaining os dependencies first, then move db.**
Pack container + `DslValue` + the `ArtifactStore`/`pack_rt`/vcs slice move into framework modules,
making db os-free; db then moves under server as originally decided. This is the fullest expression
of the target architecture but is a second extraction of comparable size to Wave 1, touching the os
document model that ~35 plugin crates build on.

**C — Move db under server and accept `server → os`.**
Matches the literal instruction, violates the invariant. Not recommended.
