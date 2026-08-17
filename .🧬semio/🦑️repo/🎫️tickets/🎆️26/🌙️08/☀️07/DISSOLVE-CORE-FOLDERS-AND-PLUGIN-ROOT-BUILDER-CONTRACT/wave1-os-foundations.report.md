# Wave 1 OS foundations report

## Summary

Dissolved four `🫀️core` grab-bags under OS kernel modules **pack**, **db**, **spr**, and **dsl** into named concept sibling folders. Updated in-module glue (`📦️packages/🦀️rust/📦️glue.rs` for kernel + db), facades, and all in-tree `#[path]` consumers. Retired `🫀️core/🦀️component.rs` with `compile_error!` guards. Removed `dsl_core` / `db_core` crate-alias wiring from kernel glue; db glue now exposes `db_ids`, `db_durability`, `db_policy`, `db_version_graph` with root re-exports.

**Note:** `cargo check --workspace` was not run to completion — workspace member `semio-s-plugin-flow-extension-core` path is missing (unrelated Wave 1 flow work). Compile validation is deferred to Wave 2 integration.

## Pack (`🎒️pack`)

| Sibling | Role |
|--------|------|
| `🆔ids` | ContentHash, ChunkId, segment kinds |
| `🧾️codec` | PackError, limits, varint/bytes/CRC, CompressionCodec |
| `🚰️source` | PackSource / PackSink traits |

- Kernel `os_pack`: `ids`, `codec`, `source` + glob re-exports.
- In-tree: `crate::os_pack::core::` → `crate::os_pack::`.

## Db (`🛢️db`)

| Sibling | Role |
|--------|------|
| `🆔ids` | DocumentId, ActorId, DbError, DbLimits, check_len |
| `💾️durability` | DurabilityClass, Frontier, EpochFence, resume tokens |
| `🎚️policy` | Priority, DbCapabilities, DbConfig, profiles |
| `🕸️version-graph` | VersionGraph, Emit |

- Db glue: `db_ids` … `db_version_graph`; removed `db_core` module.
- Facade: `pub mod core` replaced by `ids`, `durability`, `policy`, `version_graph` submodules.
- In-tree: `db_core::` → crate-root types; `pack_core::` → `pack::`.

## Spr (`📡️spr`)

| Sibling | Rust mod | Role |
|--------|----------|------|
| `🆔ids` | `ids` | Identifier newtypes, HLC |
| `🔢️scalar` | `scalar` | `scalar` submodule codecs |
| `📖️dictionary` | `dictionary` | DictBuilder / DictReader |
| `🔐️crypto` | `crypto` | Crypto trait seam |
| `🧾️wire` | `wire_codec` | Protocol errors, REC_*, policies, WireCodec primitives |

- `wire_codec` avoids clashing with existing `📡️wire` channel module (`pub mod wire`).
- In-tree: protocol types at `crate::os_spr::…`; identifiers at `crate::os_spr::ids::…`; WireCodec helpers at `crate::os_spr::write_*` / `read_*` (re-exported from `wire_codec`).

## Dsl (`🗣️dsl`)

| Sibling | Role |
|--------|------|
| `📍️span` | TextSpan |
| `⚠️diagnostic` | TextError, Diagnostic, Fault, Limits |
| `🔤️token` | Symbol, TokenKind, escape/number/unit |
| `🔍️lexer` | `lex`, `is_bare_ident`, tests |
| `🎖️trust` | Sanitized / SchemaValid |

- Kernel `os_dsl`: concept modules + glob re-exports; removed `dsl_core` alias and `pub mod core`.
- Facade `pub use` updated; in-tree `os_dsl::core::` / `dsl_core::` → `crate::os_dsl::`.

## Glue files touched

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🎒️pack/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️component.rs`

## Created (concept folders)

- `🎒️pack/🆔ids`, `🧾️codec`, `🚰️source`
- `🛢️db/🆔ids`, `💾️durability`, `🎚️policy`, `🕸️version-graph`
- `📡️spr/🆔ids`, `🔢️scalar`, `📖️dictionary`, `🔐️crypto`, `🧾️wire`
- `🗣️dsl/📍️span`, `⚠️diagnostic`, `🔤️token`, `🔍️lexer`, `🎖️trust`

## Retired

- `🎒️pack/🫀️core/🦀️component.rs`
- `🛢️db/🫀️core/🦀️component.rs`
- `📡️spr/🫀️core/🦀️component.rs`
- `🗣️dsl/🫀️core/🦀️component.rs`

## Deferred (Wave 2)

See manifests in this ticket folder:

- `deferred-pack.json`
- `deferred-db.json`
- `deferred-spr.json`
- `deferred-dsl.json`
