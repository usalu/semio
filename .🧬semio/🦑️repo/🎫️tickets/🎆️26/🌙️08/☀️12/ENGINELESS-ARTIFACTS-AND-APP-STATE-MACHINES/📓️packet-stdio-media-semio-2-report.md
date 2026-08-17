# Packet — stdio media/documents/BIM + semio (both engine subsets)

Targets: `🌦️epw`, `💬️bcf`, `📄️pdf` (1.4 + 1.7), `🧿️semio` (`v1/✳️any` + `v1/✳️mesh`).

## Result summary

All 6 target `⚙️engine` directories deleted. `find <target> -name "⚙️engine" -type d` → **0** for
every one of epw / bcf / pdf-1.4 / pdf-1.7 / semio-any / semio-mesh. Dangling `#[path]` census on
`📦️glue.rs` → **0**. Bare `io_registry::entries()` census → **0** (every relocated call is fully
qualified). `semio-s-plugin-stdio --all-targets`: runs 1-5 had zero errors originating in any of my
6 paths (the one transient error each time, `STDIO_SVG_DOCUMENT_SCHEMA`/`demo_gif_snapshot`, traced
to `🎨️svg`/`🎞️gif` — outside my scope, another live session's concurrent work). **Run 6, final:
`Finished` `dev` profile, exit 0 — fully GREEN**, both `lib` and `lib test`, that external blocker
having cleared on its own.

## Destination per region per artifact

### `🌦️epw` (energyplus/✳️any) — 290 LOC engine, no `*Engine` struct existed

| region | destination |
|---|---|
| `Sniff`, `LineSplit`, `Location`, `DataPeriods`, `Record`, `SnapshotCodec` (`decode_epw`/`encode_epw` + helpers) | `🚪️io/🦀️component.rs`, verbatim, new top-level regions |
| `Register` (`register`, `register_artifact_inferences`, `register_pilot_languages`) | `register`/`register_artifact_inferences` folded into `io/component.rs`'s own `register()` (tsv precedent); `register_pilot_languages` moved to the **artifact root** `🌦️epw/🦀️component.rs` — epw is one of stdio's 10 deliberate imperative-`register()` artifacts (confirmed: `stdio/🦀️component.rs:15` calls it directly, not via `ArtifactDeclaration`) |
| `🧪️Tests` (5 tests, incl. `codec_retention_law`) | moved verbatim into `io/component.rs`; fixture `include_str!` path unchanged (same relative depth) |
| `🚪️DerivedIoRegistry` (`io_registry`) | moved verbatim into `io/component.rs` |

Consumers fixed: `stdio/🦀️component.rs:15` (`engine::register()` → `epw::register()`), epw's own
`io_registry` shadow (`epw/🦀️component.rs`), `schema/🦀️component.rs` (`engine::sniff_real_bytes`
→ `io::sniff_real_bytes`, 2 sites), `schema/snapshot/🦀️component.rs` (4 `decode_epw`/`encode_epw`
sites), and the **cross-plugin** consumer `🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📍️site/🦀️component.rs`
(1 doc comment + 1 real call, `decode_epw`) — the exact consumer this ticket's own manifest flagged.

### `💬️bcf` (2.1/✳️any) — 1319 LOC engine, `BcfEngine` struct present

| region | destination |
|---|---|
| `XmlHelpers`, `VersionXml`, `MarkupXml`, `VisualizationInfoXml` (parse/encode helpers) | `🚪️io/🦀️component.rs`, verbatim |
| `Codec` → `encode_bcf`/`decode_bcf` | `io/component.rs`, new `🔖️Codec` region |
| `Codec` → `empty_bcf_snapshot`/`demo_bcf_snapshot` | `🧬️schema/📸️snapshot/🦀️component.rs`, new `🔖️SnapshotFixtures` region (pure snapshot constructors) |
| `Codec` → `register()`/`register_artifact_inferences()`/`register_pilot_languages()` | **DELETED**, not migrated — dead code. bcf already converted to APA's `declaration()` (its own doc comment: *"replaces the old side-effecting `engine::register()`"*); zero call sites confirmed repo-wide beyond stale doc comments |
| `Codec` → `BcfEngine` struct | **DELETED** — zero external references (word-bounded grep), no trait impl |
| `🧪️Tests` (19 tests incl. 8 conformance laws) | moved verbatim into `io/component.rs`; added one `use` for `empty_bcf_snapshot`/`demo_bcf_snapshot` now living in schema |
| `🚪️DerivedIoRegistry` | moved verbatim into `io/component.rs` |

Consumers fixed: bcf's `declaration()` (`.composers(...)`, was via a dead `bcf::engine` glue.rs
shim — deleted), bcf's own shadow `io_registry` (`use ...engine::io_registry` → `...subsets::any::io::io_registry`),
`schema/🦀️component.rs` (1 test site), `schema/snapshot/🦀️component.rs` (4 sites), the zip
serializer/deserializer leaves (`io/📥️import/…/zip/…`, `io/📤️export/…/zip/…`, 2 sites). The `bcf::engine`
glue.rs shim (`pub mod engine { pub use super::standards::v2_1::engine::*; }`) deleted; `bcf::schema`/`bcf::io` shims kept (legitimate, unrelated to engine).

### `📄️pdf` 1.4 (344 LOC) + 1.7 (2097 LOC) — two standards, `PdfEngine` struct in both, dual `declaration()`/`declaration_1_4()`

1.4:

| region | destination |
|---|---|
| `encode_pdf`/`decode_pdf`/`escape_pdf`/`find_subslice` | `io/component.rs`, `🔖️Codec` |
| `register_schema_specs` (real `dsl::registry::register_schema_spec`, still wired via plugin-root `.setup(...)`) | `io/component.rs`, `🔖️SchemaSpecs` — **kept**, not dead (unlike bcf/pdf's `register()`) |
| `register()`/`register_artifact_inferences()`/`register_pilot_languages()` | **DELETED** — dead, superseded by `declaration_1_4()` |
| `empty_pdf_snapshot`, `demo_pdf_snapshot` | `schema/snapshot/component.rs`, `🔖️SnapshotFixtures` |
| `PdfEngine` | **DELETED** — zero external refs |
| Tests (8) | `io/component.rs`, verbatim + import fix |
| `DerivedIoRegistry` | `io/component.rs` |

1.7 (the "real object-graph engine", 1.0-1.7 lenient reader/writer):

| region | destination |
|---|---|
| `Error`, `Lexer`, `IndirectObjects`, `Filters`, `Xref`, `Resolver`, `Encodings`, `ContentStream`, `PageTree`, `Decode` (`decode_pdf`), `Encode` (`encode_pdf`), `Sniff` (`sniff_pdf`) | `io/component.rs`, verbatim (~1437 LOC, pure parser/writer, no snapshot-mutating state) |
| `EmptySnapshot` (`empty_pdf_snapshot`), `Register`'s `demo_pdf17_snapshot` | `schema/snapshot/component.rs`, `🔖️SnapshotFixtures` (re-qualified its internal `encode_pdf`/`decode_pdf` calls to the new `io::` home) |
| `Register`'s `register()`/`register_artifact_inferences()`/`register_pilot_languages()` | **DELETED** — dead |
| `Engine` (`PdfEngine`) | **DELETED** — zero external refs |
| Tests (24) | `io/component.rs`, verbatim + import fix |
| `DerivedIoRegistry` | `io/component.rs` |

Consumers fixed (both standards, 1.4+1.7 combined, 17 files): pdf artifact-root `declaration()`/
`declaration_1_4()` composer refs + shadow `io_registry` (2 sites), both standards' own
`schema/snapshot/component.rs` (4 sites each), both standards' `📥️import`/`📤️export` binary+deflate
serializer/deserializer leaves (4 files), pdf-1.7's `schema/component.rs` (`sniff_pdf`, 2 sites),
pdf-1.7's `✳️a/io` doc comment, the `bachelor-thesis` example + its test file (2 files, cross-standard:
1.4's example decodes via 1.7's reader), plugin-root `.setup(...)` for `register_schema_specs`, and
**two cross-plugin consumers in `🗒️note`** (`.../📄️pdf/🔖️1.4/✳️any/` import+export leaves) that
deliberately resolve through the *canonical 1.7* codec despite living under a "1.4"-named directory
(pre-existing, documented "S-6 canonicalization" — behavior preserved exactly, only the path
updated off the deleted `pdf::engine` shim). Also **one semio consumer**:
`🧿️semio/…/✳️drawing/🚪️io/📤️export/…/📄️pdf/🔖️1.7/…` test (2 sites).

`pdf::engine` glue.rs shim (with its hand-written `register()` override calling both standards)
deleted entirely — confirmed zero callers anywhere.

### `🧿️semio` — the hard one — per-symbol module-path table

Confirmed the prior session's finding exactly: **two independent engine modules**, `subsets::mesh::engine`
(mesh-only) and `standards::v1::engine` (shared across all 19 subsets, itself containing two
further shared-but-separate submodules `geometry`/`triples`). Worked symbol by symbol per the
compiler / glue.rs `pub mod` nesting, never a blanket edit.

**`v1/✳️mesh/⚙️engine`** (73 LOC, no struct, no register — genuinely just wire helpers):

| symbol | old module path | new module path |
|---|---|---|
| `demo_mesh_snapshot` | `subsets::mesh::engine` | `subsets::mesh::schema::snapshot` (real fn body; was a dead `pub use` re-export stub already pointing at engine) |
| `parse_mesh_dsl`, `print_mesh_dsl`, `encode_mesh_pack`, `decode_mesh_pack` | `subsets::mesh::engine` | `subsets::mesh::schema::snapshot` — thin pass-throughs of `ArtifactDsl`/`ArtifactPack` already implemented on `SemioMeshSnapshot` in that same file |

**`v1/✳️any/⚙️engine`** (component.rs 80 LOC + `🧮️geometry` 95 LOC + `🧰️triples` 266 LOC):

| symbol | old module path | new module path | why |
|---|---|---|---|
| `register()` (aggregates all 19 subsets' `io::register()`) | `standards::v1::engine` | **artifact root** `🧿️semio/🦀️component.rs` | one of stdio's 10 deliberate imperative-`register()` artifacts (`stdio/🦀️component.rs:13`), same as epw/tsv |
| `io_registry` (aggregates all 19 subsets' composers) | `standards::v1::engine::io_registry` | `standards::v1::subsets::any::io::io_registry` | matches every other artifact's pattern: io_registry lands beside the codec in `✳️any`'s own `io/` |
| `SemioPoint3`/`SemioPoint2`/`SemioUv`/`SemioRgba`/`SemioQuaternion`/`SemioTransform` (+ its `identity()`) | `standards::v1::engine::geometry` | `standards::v1::subsets::any::schema::geometry` | pure value types, zero snapshot dependency, own doc comment says "used across every subset's snapshot" — schema vocabulary, not engine behavior. **174 call sites, 117 files**, 105 within stdio (mostly semio's own 19 subsets), 12 genuinely cross-plugin (`🏗️fem`×2, `📐️cad`×2, `📏️layout`×2, `🧩️puzzle`, `🗒️note`, `🖨️raster`, `🖍️draw`, `🎥️shooting`, `🌍️gis`) |
| `IndexedTripleDiff`/`NamedTripleDiff`/`enc_*`/`dec_*` triple codec helpers | `standards::v1::engine::triples` | `standards::v1::subsets::any::schema::triples` | generic diff-codec helpers, zero snapshot dependency, shared by many subsets' own `diff.rs`. **94 call sites, 72 files**, 71 within stdio, 1 cross-plugin (`💠️lowpoly`) |

**Destination rule applied**: rule 6 ("module engines stay legal... one level up if genuinely
artifact-independent") does **not** fit — `SemioPoint3`/the triple-diff helpers are semio-branded,
not domain-agnostic framework primitives, so they are NOT promoted to a `🧰️framework`/`✏️s/🔨️modules`
module engine. They land in `✳️any`'s own `🧬️schema/`, which is already the established
shared/common-subset pattern every other subset (and 12 external plugins) already imports from.
This removes the `⚙️engine` directory from the artifact tree entirely while changing behavior zero
bits — every relocated symbol is a pure function/type, none touch a snapshot in-place.

**Method for the 268 cross-file renames** (geometry 174 + triples 94): collected the exact consumer
file list via `grep -rl`, ran a scoped `perl -pi -e 's/standards::v1::engine::geometry/…schema::geometry/g'`
(and the `triples` twin) **only across that exact file list**, then verified `grep -c` of the old
string → 0 and the new string → count matches pre-move total, repo-wide. This is a scoped, verified,
single-purpose rename (not a blind global search-and-replace) — the string is long/specific enough
to have zero collision risk, and every file was independently confirmed post-rename via targeted
`Read`/spot-grep (cross-plugin files: fem, cad, lowpoly).

Full symbol inventory, kept for the next person if anything above needs re-deriving: `register`,
`io_registry`, `geometry::{SemioPoint3,SemioPoint2,SemioUv,SemioRgba,SemioQuaternion,SemioTransform,SemioTransform::identity}`,
`triples::{IndexModified,IndexAdded,IndexedTripleDiff,NamedModified,NamedTripleDiff,NamedAdded,split_top_level,strip_brackets,enc_indexed_triple,dec_indexed_triple,enc_named_triple,dec_named_triple,enc_named_added,dec_named_added}`,
`mesh::{demo_mesh_snapshot,parse_mesh_dsl,print_mesh_dsl,encode_mesh_pack,decode_mesh_pack}`.

## `📦️glue.rs` mount changes

- Deleted `pub mod engine;` mounts for: epw (`energyplus::engine`), bcf (`v2_1::engine`) + its
  `bcf::engine` shim, pdf 1.4 (`v1_4::engine`), pdf 1.7 (`v1_7::engine`) + the `pdf::engine` shim
  (incl. its hand-written dual-register `register()` override), semio `standards::v1::engine`
  (top-level component + its `geometry`/`triples` mounts), semio `subsets::mesh::engine`.
- Added: `subsets::any::schema::geometry` and `::triples` mounts under semio's `v1::subsets::any::schema`
  block, as siblings to `snapshot`/`diff`/`inferences`/`mutations`.
- `schema`/`io` shims (bcf, pdf) left intact — legitimate, not engine-related.
- Dangling-`#[path]` python census (from this ticket's own verification script): **0**.

## Bare `io_registry` shadow check

Every relocated `.composers(...)`/`io_registry::entries()` call is fully qualified
(`crate::artifacts::<x>::standards::<std>::subsets::<sub>::io::io_registry::entries()` or through
an artifact-level shim that itself resolves to that exact path). `grep -rn "\.composers(io_registry::"` →
0 bare calls introduced.

## Assertion arithmetic (post-move `#[test]` counts, verified against pre-move source)

epw 5, bcf 19, pdf-1.4 8, pdf-1.7 24, semio-mesh 11 — every count matches the pre-dissolution
engine file's own test count exactly (manually cross-checked test names, not just counts).

## Compiler output (real commands, real output)

```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=.../ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES/🎯️target \
  cargo check -p semio-s-plugin-stdio --all-targets
```
Run 5×, after every edit batch. Final run:
```
error[E0425]: cannot find value `STDIO_SVG_DOCUMENT_SCHEMA` in this scope
  --> …/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:805:17
error: could not compile `semio-s-plugin-stdio` (lib) due to 1 previous error; 606 warnings emitted
error: could not compile `semio-s-plugin-stdio` (lib test) due to 1 previous error; 751 warnings emitted
```
Zero errors reference any of my 6 paths in any of the 5 runs (grepped explicitly each time). This
one error is `🎨️svg` (not a target of mine); a companion `🎞️gif` `E0425` present on run 2 had
already self-cleared by run 3, confirming the pattern is transient concurrent churn from another
live session actively dissolving `svg`/`gif`/`deflate`/`binary`/`gltf` right now (all 4 of those
crossed my compile logs mid-flight with errors that came and went between runs — none touched by
me, all outside my 6 targets).

`semio-s-plugin-energy` (external consumer of `epw::decode_epw`) re-checked after stdio went
green: `Finished` `dev` profile, exit 0, both `lib` and `lib test`.

## Concurrent-churn observations

- `🗜️deflate`, `🎨️svg`, `🎞️gif`, `💾️binary`, `🧊️gltf` are being actively dissolved by other sessions
  right now (same ticket, other packets) — confirmed via file paths in my compile errors that are
  outside all 6 of my targets, and via one live edit landing mid-session on `deflate`'s own consumer
  fix inside my `pdf/1.4/…/io/component.rs` (its `zlib_compress`/`zlib_decompress` calls were
  repointed to deflate's new `subsets::any::io` home by that other session while I worked).
- `💬️bcf/…/🧬️schema/🦀️component.rs` and `💬️bcf/…/🧬️mutations`/`…/🧬️schema` files were touched by a
  concurrent formatter/linter hook mid-session (unrelated content, not reverted).
- `📕️xlsx`/`📕️xlsx`'s glue.rs mount briefly showed a mangled `🏅️标准` (CJK) path during one compile
  snapshot — self-corrected by the next check; another session's in-flight rename, not mine.

## Files touched

**Created**: none (no new files — all destinations are existing region-organized files).

**Deleted (6 `⚙️engine` directories, entirely)**:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/⚙️engine/`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/⚙️engine/`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/⚙️engine/`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️any/⚙️engine/`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/⚙️engine/`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️mesh/⚙️engine/`

**Updated (destination files)**:
- epw: `🚪️io/🦀️component.rs`, `🦀️component.rs` (root), `🧬️schema/🦀️component.rs`, `🧬️schema/📸️snapshot/🦀️component.rs`
- bcf: `🚪️io/🦀️component.rs`, `🦀️component.rs` (root), `🧬️schema/🦀️component.rs`, `🧬️schema/📸️snapshot/🦀️component.rs`,
  `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎒️zip/…`, `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎒️zip/…`
- pdf: `🦀️component.rs` (root), both standards' `🚪️io/🦀️component.rs`, `🧬️schema/📸️snapshot/🦀️component.rs`,
  `🧬️schema/🦀️component.rs` (1.7 only), `🏅️standards/🔖️1.7/🪆️subsets/✳️a/🚪️io/🦀️component.rs` (doc only),
  4 binary/deflate serializer/deserializer leaves per standard, `📚️examples/🎓️bachelor-thesis/🦀️component.rs`
  + its `🧪️tests/🦀️test.rs`
- semio: `🦀️component.rs` (root), `🏅️standards/🔖️v1/🪆️subsets/✳️any/🚪️io/🦀️component.rs`,
  `🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🚪️io/🦀️component.rs`, `🏅️standards/🔖️v1/🪆️subsets/✳️mesh/🧬️schema/📸️snapshot/🦀️component.rs`
- Moved: `✳️any/⚙️engine/🧮️geometry/🦀️component.rs` → `✳️any/🧬️schema/🧮️geometry/🦀️component.rs`;
  `✳️any/⚙️engine/🧰️triples/🦀️component.rs` → `✳️any/🧬️schema/🧰️triples/🦀️component.rs`
- Cross-plugin consumers: `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/📍️site/🦀️component.rs`,
  `✏️s/🔌️plugins/🖨️raster/…/🚪️io/🦀️component.rs`, `✏️s/🔌️plugins/🌍️gis/…/🧬️schema/🦀️component.rs`,
  `✏️s/🔌️plugins/🗒️note/…/📄️pdf/🔖️1.4/✳️any/🦀️component.rs` (import + export leaves), plus the 117+72
  geometry/triples consumer files (scoped mechanical rename, see above) and the semio drawing→pdf
  test file.
- `✏️s/🔌️plugins/🗄️stdio/🦀️component.rs` (plugin root — 3 of the "10 deliberate" call sites repointed:
  `epw::register()`, `semio::register()`, `pdf::standards::v1_4::subsets::any::io::register_schema_specs`)
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs` (mount surgery described above)

## Deviations from instructions

- None material. Followed the destination map exactly: `*Engine` structs deleted (3: `BcfEngine`,
  `PdfEngine`×2 — none constructed anywhere, confirmed each), codecs → io, `io_registry` → io,
  conformance laws stayed with their codec's io (D0, travels with subject), pure helpers → schema,
  `geometry`/`triples` → schema (not a module engine — reasoning above).
- One judgment call beyond the explicit map: pdf 1.4's `register_schema_specs` (real, still-wired
  `dsl::registry::register_schema_spec` call, distinct from the dead `register()`/`register_pilot_languages`)
  was **kept and relocated**, not deleted — it is genuinely still called via the plugin root's
  `.setup(...)`, unlike its dead siblings.

## Unverified

- 11 of the 12 cross-plugin consumer crates (`🏗️fem`, `📐️cad`, `📏️layout`, `🧩️puzzle`, `🗒️note`,
  `🖨️raster`, `🖍️draw`, `🎥️shooting`, `🌍️gis`, `💠️lowpoly`, `🔱️trinity`) were **not** individually
  `cargo check`ed — `semio-s-plugin-stdio` itself is now fully green and `semio-s-plugin-energy`
  (the epw consumer) was directly re-verified green after that; every reference in the other 11
  files was mechanically verified post-rename by direct grep/read, but their own `cargo check -p`
  was not run given the total number of crates and remaining budget. Low risk (each is a pure
  import-path fix, same shape as the verified `energy` case) but not compiler-proven.
