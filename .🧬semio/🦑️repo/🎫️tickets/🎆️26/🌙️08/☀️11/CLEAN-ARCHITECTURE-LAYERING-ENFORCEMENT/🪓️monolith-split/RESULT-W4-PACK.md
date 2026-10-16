# Result W4 PACK: `semio-framework-pack` carved into twelve producer crates plus a thin facade

Executor: Sonnet 5.5, foreground, no sub-agents, 2026-10-15. Brief: `📓️WAVE4-BRIEF.md` including the added god-object rule. Tools and generators live in `pack-carve-tools/` (`run_all.py` regenerates the whole change from the pristine tree, `union.py` is the differential, `apply.py`/`sync.py` write it into the repo). Logs are `🗑️generated/w4pack-*.log`.
`PK` = `🧰️framework/🔨️modules/🎒️pack`.

## Status in one paragraph

The split is done and applied. `semio-framework-pack` (lib name `pack`) is now a facade of 7 files, 17,516 bytes and 250 active lines; its former content lives in twelve new producer crates, every one of them far under the gate in lib, test and wasm closures (largest lib 156,167 bytes, largest test closure 209,829 bytes, largest 2,721 lines). Every public path of the old crate (`pack::format`, `pack::record`, `pack::async_`, `pack::http`, `pack::io`, `pack::intrinsic`, `pack::snapshot`, `pack::codec`, `pack::value`, `pack::ids`, the flat root names, `corruption_testing`) is unchanged, re-exported explicitly (no globs) from the facade, so no consumer needed an edit. The patched full-resolution differential is clean in eight configurations. **Nothing was compiled by cargo:** `bun repo cargo` is refused ("Finite original maintenance authority required") and the Nx route did not give a verdict (see Verification 8). Type-level properties (method and field privacy, trait scope, coherence) are covered by scans, not by rustc; that is the main residual risk.

## What the investigation found

- 33 files, 804,976 bytes, 11,174 active lines in the lib closure; test closure 106 files, 1,449,699 bytes, 17,086 lines. The `format` module was one 3,630-line file whose last 2,366 lines (`RetainedCanonicalSource`) are self-contained retained cursors that read the framing through 10 names; the `value` module was a 3,380-line root with five private families (wire vocabulary, schema hash, retained cursors, controlled encoding, borrowed preflight) coupled to it only through `use super::*`.
- Dependency direction before the carve was already acyclic: format <- record (value) <- intrinsic; async <- http; format <- io. Nothing in `format` uses `record`.
- Peers had already carved `pack-source`, `pack-error`, `pack-json`; I continued in the `pack-source` style (recipe manifest, `📋️project.json` with `scope:framework` + `language:rust`, owner row with `packages: []`, routes in `launch.json`).
- **God-object rule:** no inherent `impl` spans crates and no extension trait was introduced. The largest inherent impls (`BorrowedProjectedPackCursor` 35 KB, `RetainedRecordBodyCursor` 35 KB, `RetainedPackCatalogCursor` 34 KB, `PackFile` 25 KB) each sit whole in one crate far below the limit, so no struct had to be decomposed. `orphan.py` verifies that every impl of every new crate targets a local type or local trait (0 findings).

## Resulting layout

Lib roots live at `<node>/🦀️.rs`, packages at `<node>/📦️packages/🦀️rust`; module files stay where they were and are mounted with `#[path]`, each owned by exactly one crate (`union.py` asserts it; only the shared test helper `⏱️trace/…/📥️requests/🦀️.rs` is mounted by several crates under `cfg(test)`, as `pack-error`/`pack-json` already do).

| Crate (`semio-framework-…`) | Node under `PK` | Contents | Internal dependencies |
|---|---|---|---|
| `pack-format` | `📐️format` | header, footer, segments, symbols, chunk table, manifest, `PackWriter`, `PackFile`, recovery, `ControlledPackWriter` | pack-error, pack-source |
| `pack-format-retained` | `📐️format/🎮️retained` (new) | retained page/source, varint, segment and anchor cursors | format |
| `pack-format-catalog` | `📐️format/🗂️catalog` (new) | manifest, symbols, chunk-table and catalog cursors, `RetainedPackSymbolTable`, `retained_inline_symbols` | format, format-retained |
| `pack-record-schema` | `🌱️value/🧬️schema` | record wire tags, element tags, shape navigation, `PackSchemaGraph`, `schema_hash`, `controlled_schema` | pack-error, pack-json |
| `pack-record-retained` | `🌱️value/🛬️decode` (new root file) | `RetainedValueCursor`, `RetainedRecordBodyCursor`, source decode, receiving, stack storage | format-retained, format-catalog, record-schema; dev: record |
| `pack-record-encode` | `🌱️value/🛫️encode` | controlled/borrowed encoding, operation pages, owned wire cursors, the six `encode_*_controlled/_into` entry points | format, record-schema |
| `pack-record-preflight` | `🌱️value/🫳️preflight` | `measure_document_borrowed` | format, record-schema |
| `pack-record` | `🌱️value` | owned canonical encode/decode, documents, record bodies, `scalar_witness`, `record::intrinsic` adapters | format, record-schema, record-encode |
| `pack-intrinsic` | `🌱️intrinsic` | `decode`, retained intrinsic body and document clone | format(+retained,+catalog), record, record-retained |
| `pack-async` | `⏳️async` | `AsyncPackSource`, cancellation, read scheduler | pack-error |
| `pack-http` | `🌐️http` | `HttpPackSource`, `RangeTransport` | pack-async |
| `pack-io` | `🔌️io` | `FilePackSource/Sink`, `write_atomic`, `StreamingPackWriter`, `recover_file` | format, pack-source |
| `pack` (facade, lib `pack`) | `📦️packages/🦀️rust` | `pub mod format/record/async_/http/io/intrinsic` with explicit `pub use`, root flat re-exports (`🦀️.rs` component unchanged), `snapshot`, `corruption_testing`, facade tests | all of the above |

DAG (downward only; checked by `union.py` ownership, manifests and `manifests.py` deps):

```
pack-error, pack-source, pack-json (peer crates)
  format -> format-retained -> format-catalog
  record-schema        (leaf)
  record-encode, record-preflight -> format, record-schema
  record -> record-encode, record-schema, format
  record-retained -> format-retained, format-catalog, record-schema
  intrinsic -> record, record-retained, format-*, 
  async -> http ;  format -> io
  pack (facade) -> everything
```

## Census (canonical `readRustSourceClosureV1`, `🗑️generated/census.ts`, host cfg; gate 4000 lines / 32 files / 262,144 bytes)

Before: `semio-framework-pack` 33 / 804,976 / 11,174 (over the byte gate, closure not compilable). Test closure 106 / 1,449,699 / 17,086.

After, lib closure from `w4pack-census-after.log` (files / bytes / active lines), then test and wasm closures from `pack-carve-tools/census_stage.py` (`w4pack-verify-final.log`; `cfg(test)`, default feature `deflate`):

| Crate | lib | test | wasm32 lib |
|---|---|---|---|
| `pack` | 7 / 17,516 / 250 | 20 / 96,622 / 775 | not separately counted |
| `pack-format` | 2 / 95,643 / 1,378 | 5 / 127,109 / 1,937 | 2 / 95,643 / 1,400 |
| `pack-format-retained` | 1 / 53,189 / 987 | 5 / 100,620 / 1,517 | 1 / 53,189 / 1,046 |
| `pack-format-catalog` | 3 / 91,266 / 1,542 | 9 / 157,089 / 2,265 | 3 / 91,266 / 1,542 |
| `pack-record-schema` | 3 / 41,746 / 643 | 3 / 41,746 / 643 | 3 / 41,746 / 643 |
| `pack-record-retained` | 4 / 125,219 / 1,668 | 16 / 198,642 / 2,444 | 4 / 125,219 / 1,668 |
| `pack-record-encode` | 6 / 156,167 / 1,284 | 15 / 209,829 / 1,690 | 6 / 156,167 / 1,284 |
| `pack-record-preflight` | 2 / 38,116 / 207 | 7 / 58,147 / 427 | 2 / 38,116 / 207 |
| `pack-record` | 3 / 107,487 / 1,906 | 7 / 153,762 / 2,721 | 3 / 107,487 / 1,906 |
| `pack-intrinsic` | 3 / 43,964 / 386 | 10 / 81,206 / 810 | 3 / 43,964 / 386 |
| `pack-async` | 1 / 21,927 / 485 | 3 / 34,733 / 782 | 1 / 21,927 / 485 |
| `pack-http` | 1 / 12,301 / 263 | 3 / 20,260 / 424 | 1 / 12,301 / 259 |
| `pack-io` | 1 / 12,134 / 201 | 3 / 26,237 / 504 | 1 / 12,134 / 20 |

All rows are `overLimit false` and inside the 3500 / 28 / 230000 target. The facade's integration targets are separate units: `pack_record_laws` 21 / 195,853 / 1,198, `pack_record_schema_laws` 8 / 47,009 / 587, `pack_schema_hash` 2 / 24,359 / 308, `original-owned-wire` 2 / 13,648 / 66. `closureCompilable` stays `false` for every row only because their dependency `semio-framework-replication` (peer-owned tier 0, 57 files, 1,039,702 bytes) is oversized; all other first-party dependencies are within limits (`oversizedDeps` 1 = replication).

## Why this partition (and the intermediate steps that were rejected)

- `format` is split at the `RetainedCanonicalSource` region: the retained cursors use the core only through ten names (`MAGIC`, `HEADER_SIZE`, `FOOTER_*`, `FORMAT_VERSION_MAJOR`, `Header`, `Footer`, `Superblock`, `RawManifest`, `REQUIRED_KNOWN_MASK`) and the core uses none of them. A first two-crate cut left the retained half at 244,471 test bytes, so it was split again along the existing component seam (admission cursors vs catalog cursors; the only edges are the pub segment/varint/source event types) and the 717-line laws file was split by component, shared helpers copied into both halves.
- `value` falls apart along its private families because they only share constants and shape helpers: those moved into `record-schema` (also home of the schema hash, which `encode` and `preflight` need), which makes `encode`, `preflight` and `retained` leaves of the codec instead of children of it. `record::intrinsic` stays in `record` because it adapts the owned document functions.

## Changes beyond wiring (all minimal)

1. **Visibility widened to `pub`** (narrowest that works across a crate boundary): `REQUIRED_KNOWN_MASK`; `RawManifest` and its fields (format); `retained_page_close_demands`, `RetainedVarintFault::into_catalog_fault`, `RetainedVarintCursor::preview` (retained); every `TAG_*` and `ELEM_*`, `elem_shape_of`, `table_spec_of`, `block_inner_shape`, `statements_variants`, `map_inner_shape`, `NumKind`, `elem_tag_for_shape`, the `controlled_schema` module with `sort` and `compare_text` (record-schema); `intrinsic_document` (encode); `borrowed_schema_hash` (preflight). `pack-format`'s `retained_page_close_demands` moved to `retained` where both cursor crates need it. `pub(super)` items at module roots became private. None of these names is re-exported by the facade, so the public API of `pack` is unchanged.
2. **Imports instead of `super::*`:** the glob children of the old roots now import exactly what they use; `crate::codec`, `crate::value` and the flat `crate::KIND_*`, `ByteRange`, `PackLimits`, varints, … became `protocol::codec::…`, `protocol::value::…` (rewritten at token level, never inside comments or strings); `crate::format::X` and `crate::record::X` became the owning crate's path. Trait imports needed only for method resolution (`CompressionCodec`, `PackSource`, `PackSink`) are kept when their methods are called.
3. **Tests follow their subject.** Moved with their module: format unit and absolute-varint tests (format); source producer tests and the source/segment half of the retained laws (retained); catalog producer tests, symbol-decode tests and the new catalog half of the laws (catalog); storage, source and receiving tests plus the retained-cursor half of the old record unit file, now `🌱️value/🛬️decode/🧪️tests/🔬️unit` (record-retained); the codec half of the unit file and scalar-witness tests (record); cursor and refusal tests (encode); schema tests (preflight). The one assertion that read the private `check_allocation_layout` from a retained test became a new test in the format unit file. Tests that drive the whole stack through public paths (operation pages, intrinsic media, borrowed preflight, intrinsic boundary, schema storage) became two integration targets of the facade, `pack_record_laws` and `pack_record_schema_laws` (`🧪️tests/🔬️record-laws`, `🧪️tests/🧮️record-schema-laws`), which exercise `pack::record::…` exactly like an external consumer.
4. **`feature "deflate"`** is declared by `pack-format`, `-retained` and `-catalog` and forwarded by the facade; `#![allow(async_fn_in_trait)]` (with the R7 rationale) moved to `pack-async` and `pack-http`; test dev-dependencies were recomputed per crate (`block_on` needs `semio-framework-async` with `entrypoint` as dev-dependency).
5. **Fixed on the way:** the record ownership test and fixture were already failing at HEAD (stale dependency list including `ureq`); they now describe the real record crate and the facade, and pass.

## Wiring (recipe steps)

- Manifests: twelve `Cargo.toml` (role `framework`, id `pack-<x>`, exactly the three `rust-source-boundary` keys at the ceiling, `[lints] workspace`, `[lib] path = "../../🦀️.rs"`, `workspace` resolving to `🧰️framework`); the facade manifest rewritten (new dependency list, features, boundary table, two new `[[test]]` targets). Eleven new `📋️project.json` (`@semio-tech/pack-<x>-rs`); the record crate reuses the existing `@semio-tech/framework-pack-record-rs` project (tags added).
- `🧰️framework/Cargo.toml` members: twelve rows, byte-sorted among the pack rows. `[profile.wasm-prototype.package.…] opt-level = 2` rows for the twelve crates in both `Cargo.toml` files (the facade row alone would have dropped the optimisation of the moved hot codec).
- `Cargo.lock` (framework and root): hand-derived entries for the twelve packages and the new facade dependency list (framework lock with dev-dependencies, root lock normal dependencies only, matching the existing entries). **Not produced by cargo.** `lock.py` holds the derivation.
- `📋️build-owners.json`: eleven rows before `@semio-tech/pack-source-rs` in name order (`packages: []`); the record crate keeps its existing row.
- `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc`: 72 identical routes each (6 per crate: build/check/test × prototype/release, groups `4_build`/`4_gate`/`4_test`), inserted after the last pets route, order keys `900.062xx`, `922.049xx`, `903.589xx` (checked unused in both files).
- Scripts: `PK/📦️packages/🦀️rust/📜️script.ts` runs all thirteen packages in `test` and targets each native selector at its new owner (varint → format; borrowed preflight → preflight lib plus the facade integration target; stack storage and record receiving → record-retained; retained intrinsic and intrinsic schema → intrinsic; schema storage → `pack_record_schema_laws`); `PK/🌱️value/📦️packages/🦀️rust/📜️script.ts` runs the record family plus both facade targets.
- Source-text readers: the interactivity audit in root `📜️script.ts` and its self-test in `🛢️db/🧪️tests/🔬️interactivity-db-io-direct-writer/🟦️.ts` pin the spelling `begin_identity_segment(crate::KIND_CHUNK`; both now say `protocol::codec::KIND_CHUNK` (and the forbidden strings likewise); `🚰️source` fixture consumers list the four crates that now depend on the byte owner instead of the facade; three rows of the error-cause fixture whose predicate lives in the retained source cursor now name the retained crate's file (the pinned `sha256` values were already stale at HEAD and no test checks them); the record ownership test and fixture.
- Not touched on purpose: `🔒️dependencies.json` (generated 2026-10-03 inventory, still lists `ureq` for pack), the boundary corpus (UI only), `nx.json`.

## Verification (every command run, with its actual result)

1. Baseline before the first edit: canonical census row above; pristine copy of the tree at commit `f856dff0813` in the scratchpad.
2. **Patched full-resolution differential, union sandbox** (`pack-carve-tools/union.py`, `oracle.py`; `rustc +nightly-2026-07-20 --edition 2021 --emit=metadata -o /dev/null`, extern crates removed, external globs neutralised identically in baseline and edited tree). The twelve crates are simulated as modules of the facade crate (`crate::` of each crate rewritten to its module, `pub(crate)` rewritten to `pub(in <crate module>)` so crate-level privacy is enforced, cross-crate paths mapped to modules, extra global allocators dropped, integration wrappers mounted under `cfg(test)`). Result in `w4pack-verify-final.log`, internal (non-external-crate) errors compared as multisets:

   | config | baseline errors (internal) | split errors (internal) | only-split | only-baseline |
   |---|---|---|---|---|
   | native | 273 (15) | 403 (2) | 0 | 14 |
   | native, no deflate | 273 (15) | 385 (2) | 0 | 14 |
   | `--test` | 950 (29) | 1202 (16) | 0 | 14 |
   | wasm32-unknown-unknown | 266 (14) | 396 (2) | 0 | 13 |
   | wasm32-unknown-unknown `--test` | 931 (28) | 1181 (16) | 0 | 13 |
   | wasm32-wasip2 | 266 (14) | 396 (2) | 0 | 13 |
   | `corruption-testing` | 273 (15) | 403 (2) | 0 | 14 |
   | `corruption-testing --test` | 950 (29) | 1202 (16) | 0 | 14 |

   ("only-split 1" in the raw log is rustc's own `aborting due to N errors` line.) The 13 or 14 baseline-only errors are the unresolved flat names (`crate::ByteRange`, `crate::KIND_*`, `crate::ByteSpan`, …) that now come from `protocol::codec` imports instead, which are external and unresolvable in this oracle. The number of external-crate errors rises (about 130 more) because imports are now repeated per crate; they are not compared.
3. **Oracle sensitivity:** in a scratch copy `REQUIRED_KNOWN_MASK` and `TAG_ABSENT` were made private again; the oracle reported `E0603 constant … is private` (5 sites) and `E0408` in native and test (`pack-carve-tools/seed.py`, last lines of `w4pack-verify-final.log`). During the work it also caught every real mistake: a missing `pub` made `pubfn`, unmapped `use crate::format::{…}` trees, missing test imports, a missing `crate::record` alias in the integration wrapper.
4. **Limits of that oracle, and the scans that compensate.** rustc skips the privacy pass once any error exists, and the external crates are missing, so field/method privacy (`E0616`, `E0624`, `E0451`), trait-in-scope and coherence are not reported. Compensating scans (`privscan.py`, `privscan2.py`, `orphan.py`, `unused.py`): (a) every cross-crate private method/field candidate was triaged; this found and fixed `RetainedVarintFault::into_catalog_fault` and `RetainedVarintCursor::preview`; the remaining hits are same-named members of the consumer's own types (checked at the call sites); (b) 0 inherent or trait impls on foreign types; (c) with the facade's crate-wide `allow(unused_imports)` removed, 0 unused imports in native and 0 in test for the carved crates (2 in the integration wrappers are an artefact of the union, where `crate::record` resolves without the alias); (d) imports needed only for trait method resolution are kept; (e) the audit strings of the interactivity gate are all present in `pack-format` and the forbidden ones absent.
5. Losslessness (`lossless.py`, trimmed non-blank line multiset over the 45 touched `.rs` files): 390 lines only in the old text, 581 only in the new; the differences are the intended edits (path rewrites, `pub`, imports, mount lines, duplicated shared law helpers, new docstrings and the allocation test); nothing else was read as changed.
6. rustfmt parse (`+nightly-2026-07-20`, `skip_children`): 45 staged files, 0 failures. `#[path]` and `include!/include_str!` targets of the twelve crate roots, the facade glue and the three integration roots: 0 missing. Longest new path 124 bytes (an existing file).
7. Static registry check on the live tree (`verify_live.ts`): all twelve manifests parse (Bun TOML), `workspace` resolves to `🧰️framework`, `[lib] path` and every path dependency exist, boundary keys exact, members present and byte-sorted, owner row and project name unique per crate, all 72 launch commands present exactly once in both files and their order keys unique, both lock files contain all twelve packages. Output: `ALL STATIC CHECKS PASSED`. `launch.json` and the seed parse (strict JSON and Bun JSONC); `📋️build-owners.json` parses.
8. Cargo / Nx: `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-pack-format --message-format=short` failed twice (start and end) with `error: Finite original maintenance authority required`, before any compile. `bun repo run @semio-tech/pack-source-rs:check …` at the start failed with `Repository target execution contract refused`; `@semio-tech/pack-format-rs:check` first failed with `Active graph readers retain required Nx pool headroom`, then reached `SDK source index verified` and was cut by my alarm; a last attempt (`w4pack-nx-check-format.log`) ran the graph and SDK index, then ended with `NX Original daemon child exited before acquisition` (exit 1), again without any compile or gate verdict. **So no real `cargo check` or `cargo test` has been run for any of these crates.**
9. TypeScript tests of the pack tree (35 `🧪️tests/**/🟦️.ts` files, excluding `🔤️json`, run with `bun test` before and after; logs `w4pack-ts-before.log` and `w4pack-ts-after.log`): identical outcomes except two improvements: `🌱️value/🧪️tests/🧱️ownership` 1 pass / 1 fail before, 2 pass / 0 fail after; `🚰️source/🧪️tests` unchanged 2 pass (45 instead of 42 expectations because four consumers are checked instead of one). Failures that exist at HEAD and are unchanged: `⚠️error/…/🧭️producer-authority` (3), `⚠️error/…/🧱️ownership` (4), `🌱️value/🛬️decode/🧾️source` (1), `🧪️tests/🧭️fixture-placement` (1).
10. The Rust unit and law tests were **type-checked by neither cargo nor rustc**; they are moved byte-for-byte except for the explicit import lines, and the union differential proves every name in them resolves.

## Residual risks (first real compile)

1. **Type-level checks never ran** (see 4): a private method or field I could not see by name, a missing trait import for a method call that does not name the trait, or an inference change caused by an import now spelled differently would only show up at the first real `cargo check`. The scans make this unlikely; the first compile must be run for all twelve crates, `--tests`, and for `wasm32-unknown-unknown` and `wasm32-wasip2`.
2. **`Cargo.lock` is hand-derived** in both locks. If the next `bun repo cargo … -- update --workspace --offline` changes it, accept cargo's version.
3. **Shared test helper under edit by a peer.** While I worked, `⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs` was edited in the working tree from `pub(crate)` to `pub(in crate::m_geometry)` (a different carve). Every crate that mounts it under `cfg(test)` (nine other crates already, plus my record-retained, record-encode, record-preflight and intrinsic crates and the two facade integration wrappers) breaks in its test build if that edit lands as is. Not mine to fix; flagged for the owner of that edit.
4. Each facade integration wrapper and the existing `pack_schema_hash` target compile the shared schema-hash law file; that duplication is old.
5. The interactivity audit itself could not be executed (launcher); its strings were checked statically (Verification 4e).
6. `closureCompilable` stays `false` for all thirteen rows until `semio-framework-replication` is carved (peer).
7. Profiles: the wasm-prototype `opt-level = 2` rows were copied for the new crates to preserve behaviour; a perf owner may want to narrow them to the hot ones.

## Files

New: `PK/📐️format/🎮️retained/🦀️.rs`, `PK/📐️format/🗂️catalog/🦀️.rs`, `PK/🌱️value/🧬️schema/🦀️.rs`, `PK/🌱️value/🛬️decode/🦀️.rs`, `PK/🌱️value/🛬️decode/🧪️tests/🔬️unit/🦀️.rs`, `PK/📐️format/🧪️tests/🔬️retained-pack-catalog-laws/🦀️.rs`, `PK/🧪️tests/🔬️record-laws/🦀️.rs`, `PK/🧪️tests/🧮️record-schema-laws/🦀️.rs`, 12 `Cargo.toml` and 11 `📋️project.json` under the new `📦️packages/🦀️rust` directories (31 files).
Edited (38): the 37 `.rs` files whose `crate::` paths, imports or mounts changed (`PK/📐️format/🦀️.rs` and children, `PK/🌱️value/🦀️.rs` and children, `PK/🌱️intrinsic/…`, `⏳️async`, `🌐️http`, `🔌️io`, the facade `📦️packages/🦀️rust/🦀️.rs`) plus the facade `Cargo.toml`.
Shared edits (15): `🧰️framework/Cargo.toml`, `Cargo.toml`, both `Cargo.lock`, `📋️build-owners.json`, `.vscode/launch.json`, `.vscode/🧩️launch.seed.jsonc`, `PK/📦️packages/🦀️rust/📜️script.ts`, `PK/🌱️value/📦️packages/🦀️rust/📜️script.ts`, `PK/🌱️value/📦️packages/🦀️rust/📋️project.json`, root `📜️script.ts`, the db interactivity test, the error-cause fixture, the record ownership test and fixture, the pack-source fixture.
Consumers re-pointed: none outside the pack tree (the facade keeps every path); the scripts and gates listed under Wiring.

## Open items

- Run the real check and test for the twelve crates once the launcher works, then refresh both locks and delete `🗑️generated/w4pack-*` evidence if it is no longer wanted.
- Once `semio-framework-replication` is carved, the thirteen rows turn `closureCompilable true`.
- Consider moving `semio-framework-pack`'s remaining producer-authority tests (they only use `pack-error`) into `pack-error`; I left that peer crate untouched.
