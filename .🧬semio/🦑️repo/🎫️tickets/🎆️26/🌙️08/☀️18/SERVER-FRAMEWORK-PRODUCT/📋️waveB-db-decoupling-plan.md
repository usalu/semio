# Wave B — make db os-free, then move it under 🖥️server

User decision (2026-08-18): extract db's os dependencies first, then move db (option B).

## What db actually still needs from os — measured, classified

| need | where used in db | status |
|---|---|---|
| `.spk` pack **container**: `PackFile`, `PackWriter`, `Footer`, `Manifest`, `FOOTER_SIZE`, `write_atomic`, `read_footer_only`, `REQUIRED_CHUNKED`, `REQUIRED_FOOTER_CHAIN`, `Inference*`/`infer_field` | `🗄️storage`, `📸️snapshot`, `⚙️engine`, `⌨️cli` | **unconditional production** |
| `dsl::DslValue`, `to_dsl_value`, `from_dsl_value` | `📄️artifact` (11) | **unconditional production** |
| `store::pack_rt::encode_wire_value` / pathmap encoding | `📄️artifact` (8) | **unconditional production** |
| `store::ArtifactStore` / `ArtifactPack` / `ArtifactDsl` / `ArtifactCommand`, `vcs::ArtifactVcs` / `Author` / `Checkpoint` / `Alternative` / `VcsError` | `⚙️engine` **inside `#[cfg(feature = "vcs")] mod vcs_integration`** (from l.265) | **feature-gated only** |
| `vcs::` in `🕸️version-graph`, `🗜️compact`, `🔢️index` | — | **doc comments only** |
| `pack_testkit::*` | `🧪️testkit` | **test-only** |

The blocker is therefore far narrower than the raw grep counts suggested: three real things (pack
container, `DslValue`, `pack_rt` wire-value encoding), plus one feature-gated integration that
already has a clean seam — `🕸️version-graph` documents itself as a "`vcs`-type-free trait" whose real
implementation `db_engine` supplies.

## Steps

**B1 — pack container → `🧰️framework/🔨️modules/🎒️pack`** (crate `semio-framework-pack`, `[lib] name = "pack"`).
Moves `🎒️pack/{📐️format, 🔌️io, ⏳️async, 🌐️http, 🧪️testkit, 🔢️value}` + the `🦀️component.rs` facade
(~5,500 lines). The `.spk` container is a file format, exactly as product-neutral as the `.spr`
record format already extracted in Wave 1. It depends on `protocol::codec` (already extracted), so
it sits above replication and below both products. Kernel keeps `os_pack` as a facade re-export,
same pattern that made Wave 1's 41-crate sweep unnecessary.

**B2 — `DslValue` + `to_dsl_value`/`from_dsl_value` → framework.**
Defined at `🗣️dsl/🧬️schema/🦀️component.rs:400` / `:2325` / `:2330`. Requires splitting the schema
file. OPEN: check whether the existing `🧰️framework/🔨️modules/🧬️schema` module is the right home
before creating a new one — the schema-erased value type plausibly belongs there.

**B3 — `store::pack_rt` (`🏪️store/🦀️component.rs:1275`) → moves with the pack value codec.**
It is wire-value encoding over `DslValue`, so it follows B1+B2, not the document store.

**B4 — `db_engine::vcs_integration` out of db.**
Feature-gated today. Use the existing `🕸️version-graph` `VersionGraph` trait seam: db keeps the
trait, the `vcs`-backed implementation moves to the instance (hub) or the os side. db then has no
`vcs`/`store` dependency at all, conditional or not.

**B5 — move db → `🖥️server/🔨️modules/🛢️db`,** crate `semio-framework-server-db`, `[lib] name = "db"`
unchanged (so hub source is untouched). Delete the three dead root aliases
`semio-framework-os-kernel-db-{state,storage,wal}` (referenced by no crate).

Gate after each step: `cargo test -p` for replication / pack / kernel / db / server, `cargo check -p semio-hub`,
plus the wasm `--lib` gate. Never judge `cargo check --workspace` — a concurrent session is
mid-restructure of plugin-host and renderer-wgpu (see `s0-baseline.txt`).

## Then

Wave 2 (server core: `🎭️authority`, `🗄️storage`, `🛡️policy`, `📡️gateway` + testkit instance) and
Wave 3 (hub onto `Server::builder`) proceed as in the approved plan, with db reachable from the
server product without any os edge.

---

## B1 — DONE (pack container promoted)

New framework module `🧰️framework/🔨️modules/🎒️pack`, crate `semio-framework-pack`, `[lib] name = "pack"`.
Moved: `📐️format`, `🔌️io`, `⏳️async`, `🌐️http`, the container half of `🧪️testkit` (the DSL-free
corruption fuzzers), and the container half of the facade.

Deliberately left in `💻️os/🔨️modules/🎒️pack` because they speak `os_dsl::schema`:
`🔢️value` (record value codec), the arbitrary/law half of `🧪️testkit`, `⌨️cli`, and the
`encode_document`/`decode_document`/`encode_record_body` facade region. `🔢️index` is mounted by
nothing — dead, left untouched.

`os_pack` now re-exports the crate rather than mounting its files. The kernel's flat codec/ids/source
re-export list had to go: with `component`'s `pub use pack::*` in the same module it made every
primitive an ambiguous glob (E0659).

Gates after B1: replication 182 · **pack 42** · kernel **778** (820 − the 42 that moved) · db 424 ·
server 5 · hub compiles · kernel + pack wasm `--lib` clean.

Also landed: `every_path_mount_in_this_glue_resolves_to_an_existing_file` in the kernel glue — walks
every `#[path]` literal and asserts the target exists, so a moved-but-unregistered file becomes one
named failing test instead of "os-kernel is red for every session". Resolve against
`env!("CARGO_MANIFEST_DIR")`, not `file!()`'s parent (cargo runs tests from the package root).
Suggested by the peer session on 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME after this ticket
produced the sixth instance of that failure shape.

## Remaining in Wave B

B2 `DslValue` + `to_dsl_value`/`from_dsl_value` out of `🗣️dsl/🧬️schema` (l.400/2325/2330) —
check `🧰️framework/🔨️modules/🧬️schema` as the home first.
B3 `store::pack_rt` (`🏪️store/🦀️component.rs:1275`) follows B2.
B4 `db_engine::vcs_integration` out of db via the existing `🕸️version-graph` trait seam.
B5 db → `🖥️server/🔨️modules/🛢️db` as `semio-framework-server-db`, `[lib] name = "db"` unchanged;
delete the three dead `…-db-{state,storage,wal}` root aliases.

## B2 — DONE (DslValue promoted)

New framework module `🧰️framework/🔨️modules/🌱️value` (crate-less; mounted once by the replication
crate, same pattern as `⚠️diagnostic`). Holds the `DslValue` enum + accessors + `serde_json`
conversions (was `🗣️dsl/🧬️schema` l.398-524), `to_dsl_value`/`from_dsl_value`, and the 716-line
`🔀️serde` bridge (was `🗣️dsl/🔀️dsl-value-serde`).

`WireNode` was inside the moved range but is DSL wire-literal syntax, not value contract — returned
to `🧬️schema`. `FieldValue`/`RecordValue`/`WireEdgeLabel`/`Shape` and every DslValue↔record
conversion stay os-side. `os_dsl::schema` re-exports the three promoted names, so `dsl::from_dsl_value`
and `os_dsl::schema::DslValue` keep resolving everywhere.

Verified the two consumers a peer session flagged as sitting on this path:
`cargo check -p semio-framework-plugin-describe` and `-p semio-framework-os-run` both clean.

Gates after B2: replication **185** · pack 42 · kernel **776** · db 424 · server 5 · hub compiles ·
kernel wasm `--lib` clean.

## B3 — BLOCKED on a real design fork (analysis complete, nothing changed)

db's remaining unconditional os dependency is exactly four call sites in `📄️artifact`
(l.134, 139, 376, 956): `store::pack_rt::encode_wire_value` / `decode_wire_value`.

Their implementation (`🏪️store/🦀️component.rs:1332-1346`) is:

```rust
pub fn encode_wire_value(value: &DslValue) -> Vec<u8> {
    let mut fields = HashMap::new();
    fields.insert(VALUE_BRIDGE_FIELD_ID, FieldValue::Value(value.clone()));
    let record = RecordValue { fields };
    crate::os_pack::encode_record_body(&value_bridge_spec(), &record, &default)
}
```

So the chain is `DslValue` (now neutral ✅) → `RecordValue`/`FieldValue`/`RecordSpec` (os DSL schema)
→ `os_pack::encode_record_body` (the schema-driven value codec, deliberately left os-side in B1
because it speaks `os_dsl::schema`). db's pathmap payload bytes are produced by the **os schema
codec**, and that is the last edge.

Two ways forward, and they are not equivalent:

**B3-a — promote the DSL schema core.** Move `RecordSpec`/`RecordValue`/`FieldValue`/`FieldSpec`/
`Shape` and the pack value codec (`🎒️pack/🔢️value`) into framework modules. Byte format unchanged,
no data migration, db becomes os-free honestly. But this promotes the heart of the os DSL: the
parser, printer, grammar, LSP and ~35 plugin crates all build on those types. It is the largest
single move in this whole programme — bigger than Wave 1.

**B3-b — give `🌱️value` its own neutral encoding.** Add `encode_value`/`decode_value` to the value
module built directly on the replication codec primitives, and point db's four call sites at it.
Small and surgical, but it **changes the on-disk/on-wire byte format of db's pathmap payloads**, so
the TypeScript twin (`decodePackValue`) and any existing `.spk`/WAL content must change with it.
Greenfield rules permit that (no users, no back-compat), but it is a format decision with
cross-language reach, not a refactor.

Recommendation: **B3-b**, scoped as its own packet with the TS twin updated in the same change and
the 20-fixture parity gate extended to cover the new value encoding. B3-a is the "purest" answer but
its blast radius is disproportionate to the one edge it removes.

Nothing has been changed for B3. B4 (`vcs_integration` out of db, via the existing `🕸️version-graph`
trait seam) and B5 (relocate db) are unblocked only after B3 resolves.
