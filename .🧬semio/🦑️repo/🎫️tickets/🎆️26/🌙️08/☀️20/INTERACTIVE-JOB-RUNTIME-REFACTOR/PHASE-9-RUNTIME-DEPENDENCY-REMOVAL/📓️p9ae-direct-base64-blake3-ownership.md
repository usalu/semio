# Phase 9AE Direct Base64 and BLAKE3 Ownership

## Scope

This file-disjoint packet audited every direct Rust `base64` and `blake3` manifest row outside
`compose/`, Puzzle, and renderer Rust. It used the ownership decisions already proved in:

- `📓️p9w-stdio-blake3-boundary.md`: BLAKE3 consumers use the neutral
  `semio-framework-hash` facade.
- `📓️p9x-stdio-base64-boundary.md`: stdio owns its narrowly scoped encode-only codec.
- `📓️p9y-replication-owned-base64.md`: replication owns a strict portable codec, but unrelated
  plugins must not acquire a replication dependency merely to encode media.

No Puzzle or renderer Rust source was touched.

## Result

Eight direct third-party rows were deleted:

- Seven `blake3` rows:
  - `semio-s-plugin-animate`: three one-shot hashes now call
    `framework_hash::hash_bytes`; the facade dependency already existed.
  - `semio-s-plugin-process`: one one-shot hash now calls the facade; an internal
    `semio-framework-hash` path dependency replaced the external row.
  - `semio-s-plugin-raster`: one one-shot hash now calls the facade; an internal
    `semio-framework-hash` path dependency replaced the external row.
  - `semio-framework-os-mcp`: eight one-shot hashes now call the already-declared facade.
  - `semio-s-plugin-architect`, `semio-s-plugin-forms`, and `semio-s-plugin-note`:
    the rows were stale; their complete Rust source trees contained no `blake3` use.
- One `base64` row:
  - `semio-s-plugin-flow-extension-brep`: the row was stale; the extension source contains no
    direct Base64 call and delegates its wire work to the owning flow boundary.

The migrated calls preserve output identity because `framework_hash::hash_bytes` currently calls
the same `blake3::hash(bytes).to_hex().to_string()` primitive. The affected final source census is:

- direct `blake3::` / `use blake3` references: **0**;
- BREP-extension `base64::` / `use base64` references: **0**.

No Cargo lockfile update was required: neither third-party identity disappeared from the workspace,
and the internal hash package was already present in the lock graph.

## Rejected False Positive

`semio-framework-2d` initially appeared source-empty, but its crate path-includes the shared OS
engine. The first native compile proved a real use at
`🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs:114`:
`EngineKey(*blake3::hash(&data).as_bytes())`. The current facade exposes lowercase hex, not raw
`[u8; 32]`, so the row was restored. This avoids changing the engine-key contract or widening the
packet into the OS kernel boundary.

## Exact Residual Census

| Census | Before | After | Deleted |
| --- | ---: | ---: | ---: |
| repo-wide direct `blake3` rows | 17 | 10 | 7 |
| in-scope direct `blake3` rows | 14 | 7 | 7 |
| repo-wide direct `base64` rows | 18 | 17 | 1 |
| in-scope direct `base64` rows | 15 | 14 | 1 |

The seven remaining in-scope BLAKE3 rows all have real production source use:
`semio-framework-hash`, pack, replication, OS kernel, 2D's path-included OS engine, plugin host,
and database. The fourteen remaining in-scope Base64 rows likewise have real calls: compiler, OS
kernel, OS host, database, Infinite, flow, Process, Lowpoly, Layout, CAD, Remodel, Draw, Raster, and
Space. Moving those unrelated media/storage codecs into replication would violate the P9Y boundary;
a neutral owned codec must be placed before those rows can safely disappear.

Because both names remain real workspace dependencies, this packet deliberately claims direct-row
deletion, not ecosystem-identity deletion.

## Gates

- `cargo metadata --no-deps --format-version 1`: **PASS**.
- `cargo fmt --all -- --check`: **PASS**.
- `bun ./📜️script.ts verify dependencies`: **PASS**; baseline **238**, current **179**, removed
  **59**, additions **0**. This packet removed eight direct Rust rows without deleting an ecosystem
  identity; six concurrent JavaScript identity removals landed between the initial and latest census.
- `bun ./📜️script.ts verify dependencies list rust | jq 'length'`: **63**.
- `bun ./📜️script.ts verify dependencies list js | jq 'length'`: **116**.
- The dependency list still contains exactly one `rust:base64` identity and one
  `rust:blake3` identity, as required by the retained real consumers.
- `cargo check -p semio-s-plugin-note --lib --message-format=short`: reached the package and
  **BLOCKED** at exit 101 by **764** existing interactivity async-refactor errors (for example
  synchronous trait signatures now returning futures). Its output contained no unresolved
  `base64` or `blake3` finding.
- An Nx native test sweep was stopped because it exercised unrelated test surfaces and held the
  shared Cargo lock. Before cancellation it independently exposed the same async-refactor class in
  Architect (**3,829** test compile errors) and proved the 2D raw-hash use above.
- Remaining isolated native/release/WASM compiler attempts are intentionally paused at the
  coordinator's request while P4/P3 own the shared Cargo target. No separate cache or target
  directory was created. This is a shared-tree scheduling block, not a dependency-resolution
  failure; the final compiler matrix will be appended in the serialized lock window.

## Conclusion

This packet removes every high-confidence stale row in its file-disjoint cohort and routes every
compatible one-shot BLAKE3 caller through the owned facade. The residual rows are evidence-backed,
not allowlisted. The next safe Base64 packet is placement of a neutral owned codec boundary; the
next safe raw-hash packet is a typed `[u8; 32]` facade API with parity coverage before migrating
OS engine, pack, replication, host, and database contracts.
