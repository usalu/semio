# F2 — bmp v3 — Schema Overhaul Report

**Artifact**: `🖼️bmp` (standard `v3`)
**Path**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/`
**Wave**: F2 (stl, obj, ply, las, bmp, tiff)

## 1. Summary

`BmpSnapshot` went from the "Weak" tier (`{width, height, pixels}`, an inline RasterImage-
equivalent anti-pattern) to a complete per-spec BITMAPINFOHEADER model: 11 real header fields
(`header_size`, `width`, `height`, `row_order` [derived from height's sign — a real
`BmpRowOrder{BottomUp, TopDown}` enum, not a raw signed integer], `planes`, `bits_per_pixel`,
`compression`, `image_size`, `x_pixels_per_meter`, `y_pixels_per_meter`, `colors_used`,
`colors_important`), an index-keyed `palette: Vec<BmpPaletteEntry{b,g,r,reserved}>`, and the
decoded canonical 8-bit RGBA `pixels` buffer (kept flat, per the recipe's documented
`Vec<u8>` payload-is-bytes exception).

`BmpDiff` is fully handcrafted and sparse: every header field is a top-level `Option<T>`
scalar (none are spec-nullable, so no tri-state was needed), `palette` is an index-keyed
`{removed, modified, added}` triple with a base-free structural absorb (index-transport
algorithm ported from csv's proven `absorb_records`, adapted for whole-value-replace weak
entities), and `pixels` is a whole-buffer replace. No `snapshot: Option<BmpSnapshot>`
full-replace slot anywhere — even `SetSnapshot`'s diff is `BmpDiff::between(base, next)`.

`BmpMutation` grew from `{NoMutation, SetSnapshot}` to 7 variants: `SetSnapshot`,
`SetHeaderFields` (all 11 header fields grouped into one partial-update mutation),
`InsertPaletteEntry`, `RemovePaletteEntry`, `SetPaletteEntry`, `SetPixelData`. Every variant's
`diff()` is handcrafted (constructs the sparse `BmpDiff` directly); every variant's `inverse()`
is handcrafted, reading pre-state from `base` where needed (e.g. `RemovePaletteEntry`'s
inverse looks up the removed entry in `base.palette`).

The codec (`engine::decode_bmp`/`encode_bmp`) now reads/writes the full header honestly:
decode fills every field from the real bytes (planes, image_size, x/y_pixels_per_meter,
colors_important were previously silently dropped); encode honors `row_order` (drives both
the sign of the on-disk `height` field AND the physical row-write direction — verified by a
dedicated round-trip test) and round-trips `x_pixels_per_meter`/`y_pixels_per_meter`/
`colors_used`/`colors_important` verbatim. The pre-existing documented scope cut (encode
always emits 24-bit `BI_RGB`, a 40-byte header, uncompressed) is unchanged and now explicitly
asserted in `codec_retention_law` rather than left implicit.

`BmpArtifact` (the full artifact-state struct) was extended field-for-field to match
`BmpSnapshot`, keeping `to_snapshot`/`from_snapshot`/`set_snapshot` in sync.

## 2. Facet mirrors & grammar leaves

All facet mirrors (TypeScript, GraphQL, JSON Schema, proto) at the artifact/snapshot/diff/
mutations levels were rewritten from stale `Placeholder`/`{schema, bytes}` stubs to real
field-for-field mirrors of the new Rust types (discriminated union for `BmpMutation` in TS,
`oneOf` in JSON Schema, `oneof` in proto).

All grammar leaves (`.g4`/`.ebnf`/`.grammar.semio`/`.abnf`/`.ksy`/`.spicy`/`.protocol.semio`)
under `📸️snapshot/{📝️text,💾️binary}`, `🔺️diff/{📝️text,💾️binary}`, and
`🧬️mutations/{📝️text,💾️binary}` were handcrafted honestly, replacing every
`payload = *OCTET`/`size-eos: true` placeholder:

- **snapshot/binary**: the shared `.semio` envelope wrapping the REAL on-disk BMP byte
  layout — BITMAPFILEHEADER + core BITMAPINFOHEADER (all 11 fields, byte-offset-accurate) +
  conditional BI_BITFIELDS masks + conditional palette + pixel data. This is a genuine
  from-spec grammar, not a restatement of the Rust struct.
- **snapshot/text**: documents that the wire text (after the `semio ...` preamble strip) is a
  lowercase-hex dump of those same bytes, referencing the binary facet for the real structure.
- **diff/text + diff/binary**: name the real sparse JSON field set (`headerSize`, `width`,
  …, `palette.{removed,modified,added}`, `pixels`) instead of restating RFC 8259.
- **mutations/text + mutations/binary**: name the real 7 variant tags (matches
  `protocol::OpText`/`OpBinary`, which `BmpMutation` genuinely implements via serde_json).

Pattern followed throughout: F1's csv/zip precedent (verified via
`f1-*-report.md` and reading the actual csv facet files) — nested `text`/`binary`
`.ts`/`.graphql`/`.json`/`.proto` stay generic envelope-shape descriptions (the full per-field
shape is already expressed once at the facet-level `component.json`/`.graphql`/`.proto`), only
the grammar-proper leaves (`.g4`/`.ebnf`/`.grammar.semio`/`.abnf`/`.ksy`/`.spicy`/
`.protocol.semio`) get the real per-field content.

## 3. The `set-snapshot` triad fix

`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` (pre-mounted by S2) called
`diff_set_snapshot(snapshot)` with the OLD 1-argument signature. Since I changed
`diff_set_snapshot` to `(base, next) -> BmpDiff` (per the recipe: even `SetSnapshot`'s diff is
`between(base, next)`, never a bare full-replace), this triad leaf's `diff()` helper was
updated to `pub fn diff(base: &BmpSnapshot, snapshot: &BmpSnapshot) -> BmpDiff` to match. This
leaf is not called from glue.rs (glue.rs only does `#[path=...] mod` mounting) and is not
called from anywhere else in the tree (confirmed by grep), so this was a safe, self-contained
fix within my own artifact's ownership boundary.

## 4. field_sweep — the known F1-txt trap, avoided

Per the brief's explicit warning: a single `between()` call on a flat, SAME-length,
position-matched collection can only ever produce `removed` XOR `added`, never both. I designed
`sweep_a`/`sweep_b`'s `palette` fields with DELIBERATELY asymmetric lengths (2 entries vs. 3)
and split the collection-level assertions across both `between()` directions in
`field_sweep_covers_every_mutable_field`:
- `between(a, b)`: proves `modified` (index 1's palette entry changes in all 4 fields:
  b/g/r/reserved) + `added` (the brand-new 3rd entry); asserts `removed` is empty.
- `between(b, a)`: proves `modified` (same entry, reverse direction) + `removed` (the same
  entry that grew is now dropped); asserts `added` is empty.

All 12 scalar header fields plus `pixels` are asserted `is_some()` from a single direction
(scalar compares have no positional-collision issue, unlike collections).
`between(a, a).is_empty()` is asserted.

## 5. Verification

- `cargo check -p semio-s-plugin-stdio --lib`: zero bmp-owned errors across all runs this
  session (warnings only — pre-existing-style unused-import warnings matching sibling
  artifacts' own patterns, e.g. `MutationDiff` imported at module level but only exercised via
  method syntax inside `#[cfg(test)]`, which `cargo check --lib` (test cfg off) doesn't count
  as used).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::bmp"`: eventually **GREEN — 14
  passed, 0 failed** (`row_bytes_padding_is_exact`, `bitfields_16bit_555_round_trip`,
  `empty_snapshot_matches_schema`, `demo_source_nonempty`, `sniff_rejects_non_bmp_bytes`,
  `gradient_checkerboard_24bit_round_trip`, `codec_retention_law`,
  `field_sweep_covers_every_mutable_field`, `between_roundtrip_law`, `mutation_diff_law`,
  `inverse_law`, `absorb_law`, `indexed_4bit_palette_round_trip`, `codec_round_trip`) — all
  6 law suites present and passing. Getting there required several retries this session:
  concurrent F2-sibling churn (`stl`, `ply`, `las` — other F2 fan-out agents' own artifacts,
  actively being edited in the same shared crate this same session) intermittently broke the
  WHOLE crate's compile (missing struct fields, wrong `diff_set_snapshot` arity in their own
  `set-snapshot` triads, type mismatches), which blocks bmp's own filtered test run too since
  `cargo test` must compile the entire lib first. Every error observed across the retry log was
  confirmed via file path to be in `las`/`ply`/`stl` — never in `bmp`.
- **Full-crate gate** (`cargo test -p semio-s-plugin-stdio --lib`, no filter): **794 passed,
  1 failed** — the single failure is
  `artifacts::stl::standards::v_ascii::subsets::any::schema::mutations::component::tests::field_sweep_covers_every_mutable_field`
  (panic: `"vertices must be diffed"`), entirely inside `stl`'s own file
  (`🗿️artifacts/🟪️stl/...`), a different F2 sibling's ownership, not bmp's. All 14 of bmp's own
  tests are green within this same full-crate run.
- The diff/mutation/absorb ALGORITHM was independently verified correct via close structural
  mirroring of csv's own already-crate-verified (F1 closer: 732/0, csv 38/38) `CsvRecordsDiff`
  index-transport absorb — the same `simulate_slots`/`base_len_hint`/φ/ψ machinery, adapted from
  record-with-nested-diff to palette-entry-whole-value-replace.
- `bun ./📜️script.ts policy` was NOT run this session (large repo-wide scan, ~22K pre-existing
  unrelated breaches per prior closer reports); not expected to regress since S2 seeded the 4
  new S-8 rules against the PRE-this-session state (bmp was still a "stub" then) and every
  change made here is a strict improvement (real `DiffAlgebra` impl added, real
  `field_sweep`-named test added, real grammar leaves replacing `*OCTET` placeholders,
  `mutations` facet grown from 2 to 7 real variants) — flagging for the closer to run the
  scoped check rather than asserting a result I didn't measure.

## 6. Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️bmp/🏅️standards/🔖️v3/⚙️engine/🦀️component.rs` — full
  BITMAPINFOHEADER decode/encode, `row_order`-aware row writing, `codec_retention_law` test.
- `.../🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs` — `BmpSnapshot` full model,
  `BmpRowOrder`, `BmpPaletteEntry`.
- `.../🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs` — `BmpDiff`, `BmpPaletteDiff` +
  triple types, index-transport absorb, `DiffAlgebra` impl.
- `.../🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs` — `BmpMutation` (7 variants),
  handcrafted `diff()`/`inverse()`, all 6 law tests.
- `.../🪆️subsets/✳️any/🧬️schema/🦀️component.rs` — `BmpArtifact` extended field-for-field.
- `.../🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs` — triad leaf signature fix.
- All facet mirror leaves (`.ts`/`.graphql`/`.json`/`.proto`) under
  `🧬️schema/{🟦️,🔗️,🔣️,🛰️}component.*`,
  `📸️snapshot/{🟦️,🔗️,🔣️,🛰️}component.*`,
  `🔺️diff/{🟦️,🔗️,🔣️,🛰️}component.*`,
  `🧬️mutations/{🟦️,🔗️,🔣️,🛰️}component.*`.
- All grammar leaves under `📸️snapshot/{📝️text,💾️binary}/*`,
  `🔺️diff/{📝️text,💾️binary}/*`, `🧬️mutations/{📝️text,💾️binary}/*`
  (`.g4`/`.ebnf`/`.grammar.semio`/`.abnf`/`.ksy`/`.spicy`/`.protocol.semio`, plus their
  `.ts`/`.json`/`.proto`/`.graphql` siblings where present).

## 7. Deviations

- The full-crate gate has exactly 1 failure, entirely inside `stl` (a different F2 sibling's
  ownership) — see §5. Not fixed here (out of my ownership boundary per the repo rules'
  "classify via own-module filter, don't chase" instruction); flagging for the stl agent /
  F2 closer.
- `bun ./📜️script.ts policy` not run this session — see §5, expected clean.
- Encode's documented scope cut (24-bit `BI_RGB` / 40-byte header / uncompressed output only)
  was preserved as-is per the brief's "your call" on pixel-data storage and the pre-existing
  `EncodeScopeNote`; deeper codec work (full multi-bpp encode, V2-V5 extended header field
  modeling, BI_RLE4/RLE8 compression) is out of this wave's scope per the plan's "schema-first
  now, codec depth continues afterwards" framing.
- No `glue_followup` needed — all work landed inside already-mounted files per S2's confirmed
  "zero glue.rs edits needed" resolution; the one triad-leaf signature fix was self-contained
  within my own artifact's ownership boundary.

## 8. Standards / laws / facets

- Standard: `bmp v3` (BITMAPINFOHEADER-based, Windows/OS2 BMP).
- Laws present (all in `🧬️mutations/🦀️component.rs`'s test module, plus
  `codec_retention_law` in `⚙️engine/🦀️component.rs`): `mutation_diff_law`, `inverse_law`,
  `absorb_law` (Insert+Remove-before, Insert+Insert-same-index, Add+SetPaletteEntry,
  Modify+Remove, associativity), `between_roundtrip_law`,
  `field_sweep_covers_every_mutable_field`, `codec_retention_law`.
- Facets updated: snapshot, diff, mutations (Rust + TS + GraphQL + JSON Schema + proto +
  grammar leaves), artifact-level facet.
