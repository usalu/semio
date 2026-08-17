# F3b — `📷️jpg` (standard jfif-1.01) Schema Overhaul Report

Agent: F3b-wave, artifact `stdio.jpg` jfif-1.01. Scope per brief: complete JFIF/SOF/DQT/DHT/DRI
snapshot, handcrafted sparse `JpgDiff`, named `JpgMutation` vocabulary with handcrafted
`diff()`/`inverse()` per variant, `DiffAlgebra` (inverse/between/is_empty), rigorous absorb, the
six test laws, facet mirrors, and honest grammar leaves — while preserving the real baseline
JPEG codec math (Huffman/IDCT/dequant/YCbCr, byte-stuffing, restart markers) already in the
engine, and staying compatible with the external `✳️baseline` subset wave's real conformance
checks.

## Starting state (read before writing)

Read the CURRENT tree first, as instructed: the external "subset multiplicities" ticket
(26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES) had already added a real `✳️baseline`
subset (analyzer/builder/composer with genuine ITU-T T.81 Annex F conformance checks) plus
persisted `frame`/`sof_marker`/`arithmetic`/`dc_huffman_table_count`/`ac_huffman_table_count`
fields on `JpgSnapshot` for that analyzer to check against. The real snapshot/diff/mutations
schema files live under `🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/` (not directly under
the standard's `🧬️schema/`) — confirmed by reading `glue.rs`'s jpg mount block before touching
anything. Pre-existing state: `image: RasterImage{width,height,rgba}` (the exact shared type W0
flagged for death), `JpgDiff{snapshot: Option<JpgSnapshot>}` full-replace template,
`JpgMutation{NoMutation, SetSnapshot}` stub, `apply_jpg_mutation` already returning a `Diff`
(S1's mechanical sweep), zero `DiffAlgebra` impl, placeholder `*OCTET`/`size-eos` grammar leaves,
and stale generic (`JpgEntry{name,data}`) facet mirrors matching none of the real Rust shapes.

## Snapshot completeness — `📸️snapshot/🦀️component.rs`

Killed `RasterImage` (per the ticket's explicit W0 kill directive — png already killed its own
copy) and flattened `width: u32`/`height: u32`/`pixels: Vec<u8>` directly onto `JpgSnapshot`
(matching png's precedent of first-class raster fields, not a shared wrapper). Added:

- **Typed JFIF APP0**: `jfif_version: (u8,u8)`, `jfif_density_units: JfifDensityUnits` (enum:
  `Aspect`/`PixelsPerInch`/`PixelsPerCm`), `jfif_x_density`/`jfif_y_density: u16`,
  `jfif_thumbnail: Option<JfifThumbnail{width,height,rgb_data}>`.
- **Id-keyed DQT**: `quant_tables: Vec<JpgQuantTable{id, precision, values: [u16;64]}>` — `values`
  retained in the EXACT zigzag order the DQT segment stores on disk (never reindexed).
- **Compound-keyed DHT**: `huffman_tables: Vec<JpgHuffmanTable{id, class: JpgHuffmanClass, bits:
  [u8;16], values: Vec<u8>}>`, keyed by `(class, id)` — DC id=0 and AC id=0 are different tables.
- **DRI**: `restart_interval: Option<u16>` (`None` = no DRI segment present).
- **Verbatim retention**: `other_segments: Vec<JpgSegment{marker, data}>`, index-keyed (duplicate
  APPn/COM markers are legal) — decode now ACTUALLY retains every non-JFIF APP0/APPn/COM segment
  it previously silently skipped (`0xE0..=0xEF | 0xFE => { skip }` before this wave).
- `re_encode_quality: Option<u8>` feeding the encoder's Annex K quality parameter (previously
  hardcoded to `90`).
- Kept `frame: Option<JpgFrameHeader>`/`sof_marker: u8`/`arithmetic: bool` under their EXACT
  pre-existing names/shapes — `✳️baseline::analyzer::check_baseline_conformance` depends on them
  directly, confirmed compatible after this wave's changes (its own tests still pass unmodified
  in logic, only fixture literals updated for the new field shapes).
- **Retired** `dc_huffman_table_count`/`ac_huffman_table_count` as separately-persisted fields —
  now DERIVED from `huffman_tables` in the baseline analyzer (one source of truth instead of two
  that could desync). Updated `check_baseline_conformance` and its test fixtures accordingly.

`[u16; 64]` needed a small hand-rolled `serde(with = "quant_values")` shim (serialize as a
`Vec<u16>`, deserialize back into the array) — serde's manual array impls stop at 32 elements
without const-generics support, and pulling in `serde-big-array` would violate "no external
libraries for runtime purposes." A second `Option<[u16;64]>` variant (`opt_quant_values`) covers
the same field on `JpgQuantTableDiff`.

## Diff design — `🔺️diff/🦀️component.rs`

```rust
pub struct JpgDiff {
    width: Option<u32>, height: Option<u32>, pixels: Option<Vec<u8>>,
    re_encode_quality: Option<Option<u8>>,
    jfif_version: Option<(u8,u8)>, jfif_density_units: Option<JfifDensityUnits>,
    jfif_x_density: Option<u16>, jfif_y_density: Option<u16>,
    jfif_thumbnail: Option<Option<JfifThumbnail>>,
    frame: Option<JpgFrameChange>,           // Modify(JpgFrameFieldsDiff) | Replace{frame}
    sof_marker: Option<u8>, arithmetic: Option<bool>,
    quant_tables: Option<JpgQuantTablesDiff>,       // id-keyed triple
    huffman_tables: Option<JpgHuffmanTablesDiff>,   // (class,id)-keyed triple
    restart_interval: Option<Option<u16>>,
    other_segments: Option<JpgOtherSegmentsDiff>,   // index-keyed triple
}
```

`frame`'s shape is the one genuine design decision beyond the recipe's literal field list: since
`frame` is BOTH a nullable field (decode-status: `None` until a real decode happens) AND a
structured entity with its own diffable sub-collection (`components`, id-keyed by component id),
neither `Option<Option<JpgFrameHeader>>` (loses the ability to sub-diff components) nor a plain
nested diff (can't express the `None`<->`Some` transition) was sufficient alone. Mirrored xml's
`XmlNodeDiff::Replace` fallback pattern exactly: `JpgFrameChange::Modify(JpgFrameFieldsDiff)` when
both base and next have `Some(frame)` (field-level patch incl. an id-keyed `components` triple),
`JpgFrameChange::Replace{frame}` on the `None`<->`Some` "kind change" — this is literally the
recipe's own "trees recursive with `Replace` fallback on node-kind change" rule applied to a
one-level-deep optional entity instead of a tree.

No `snapshot: Option<JpgSnapshot>` full-replace slot anywhere (verified by grep — zero hits).
`SetSnapshot`'s diff is `JpgDiff::between(base, next)`.

### Key-kind choices (per the recipe's own taxonomy)

- `quant_tables`/`huffman_tables`/`frame.components`: **id-keyed** (`u8` / `(class,id)` / `u8`).
  No index-transport needed for absorb at all — unlike position-based collections, a stable
  identity key doesn't shift when items are removed/added, which simplified the absorb algorithm
  considerably (mirrors zip's `absorb_entries`, minus the rename-tracking machinery zip needs and
  jpg doesn't — there's no id-renaming mutation for tables/components).
- `other_segments`: **index-keyed** (position IS the identity — duplicate markers are legal, same
  reasoning as png's `text_chunks`). This one DOES need real index-transport absorb; ported png's
  `simulate_slots`/`base_len_hint`/`absorb_text_chunks` shape verbatim, retargeted to
  `JpgSegment`/`JpgSegmentDiff`.

## Absorb — the hard part

Two genuinely different algorithms, matched to key kind:

1. **Id-keyed** (`quant_tables`/`huffman_tables`/`frame.components`): stable-key merge, no
   position bookkeeping for `removed`/`modified` at all (a d2-removal of a d1-added key
   annihilates the add; a d2-modify of a d1-added key patches directly into the carried payload;
   everything else composes by key lookup). `added`'s `index` field still gets the same
   best-effort shift zip's own analogous absorb documents (`saturating_sub` by the count of
   genuine, non-annihilating d2 removals) — exact when those removals sit before the add,
   documented approximation otherwise (same limitation zip already carries, not new).
2. **Index-keyed** (`other_segments`): full position-transport simulation (`Slot::Base`/
   `Slot::Added` synthetic origin tracking through both diffs' structural ops), verified against
   all three of the recipe's canonical cases directly in `absorb_law`.

Canonical cases verified in tests: `InsertOtherSegment`+`RemoveOtherSegment`-before →
`{removed:[0], added:[(0,new)]}`; `InsertOtherSegment`+`InsertOtherSegment` same index → both
survive; `SetQuantTable`+`SetQuantTable` on the same still-pending id → patches directly into the
carried added payload; `SetQuantTable`+`RemoveQuantTable` → the pending modify vanishes;
`SetHuffmanTable`+`RemoveHuffmanTable` on the same still-pending key → annihilated cleanly;
associativity verified over a 3-op chain (`absorb_law_associativity`).

`inverse()` is derived generically — `Self::between(&self.apply(base), base)` — exactly the
pattern zip's and png's F1/F3 reports both document as "correct by construction," not
apply-and-capture (that ban is specifically about a MUTATION's `diff()` being computed via
apply-then-diff; `DiffAlgebra::inverse` deriving from the already-handcrafted, already-correct
`between()` is the intended use of that function, not a workaround).

## Mutations — `🧬️mutations/🦀️component.rs`

Exactly the ticket's named set: `NoMutation`, `SetSnapshot`, `SetJfifHeader`, `SetQuantTable`/
`RemoveQuantTable`, `SetHuffmanTable`/`RemoveHuffmanTable`, `SetRestartInterval`,
`InsertOtherSegment`/`RemoveOtherSegment`, `SetPixels`, `SetReEncodeQuality`. `SetQuantTable`/
`SetHuffmanTable` are upserts (insert if the id/key doesn't exist yet, else patch) rather than
separate `Insert`/`Set` variants — matches the ticket's exact variant list, which names only one
`Set*Table` verb per table kind. Every variant's `diff()` is handcrafted directly against the
sparse diff builders in `schema::diff` (no apply-and-capture — grepped the file for the banned
`let mut next = base.clone(); apply_jpg_mutation(...)`-shaped body: zero hits). Every variant's
`inverse()` reads the pre-state it needs from `base` (id/key/index-aware; out-of-range/nonexistent
targets invert to `NoMutation`).

`apply_jpg_mutation` follows the proven single-source-of-truth shape: `let d = mutation.diff(&*
snapshot); *snapshot = d.apply(snapshot); d`.

## Test laws — all six, in the mutations module's `#[cfg(test)]`

`mutation_diff_law`, `inverse_law` (mutation-level round trip for every variant + diff-level
`inverse(base).apply(mutated) == base`), `absorb_law` (7 curated op pairs incl. the recipe's
named canonical cases) + `absorb_law_associativity`, `between_roundtrip_law`,
`codec_retention_law` (real `example.jpg` fixture when present, synthetic encode/decode fallback
otherwise — mirrors png's precedent; documents the engine's own re-encode-always-canonicalizes
normal form as the reason pixel-length/dimension equality is asserted, not byte-identity),
`field_sweep_covers_every_mutable_field`.

`sweep_a()`/`sweep_b()` use different-length collections (2 vs 1) for `quant_tables`/
`huffman_tables`/`frame.components`/`other_segments`, per the ticket's documented workaround for
the structural "same-length `between()` can show removed XOR added, never both" limitation —
`forward` (a→b) exercises removed+modified, `backward` (b→a) exercises added+modified for every
collection, and every tri-state field (`re_encode_quality`, `jfif_thumbnail`, `restart_interval`)
is exercised both directions (`Some(None)` clearing forward, `Some(Some(_))` recreating backward).

## Facet mirrors and grammar leaves

Handcrafted `.ts`/`.graphql`/`.json`/`.proto` for all three facets (snapshot/diff/mutations) plus
the `JpgArtifact` UI-reduced-view facet (its own `image: RasterImage` field also killed and
flattened to `width`/`height`/`pixels`, matching the snapshot) — real interfaces/messages matching
the actual Rust shapes, discriminated on `mutation`/`change` tags, camelCase, tri-state nullable
encoding documented inline. Every `📝️text`/`💾️binary` grammar leaf rewritten honestly:

- **Snapshot** facet describes the REAL wire form: text = semio preamble + hex-encoded JFIF byte
  stream; binary = semio pack envelope wrapping the real JFIF bytes. Both now spell out the actual
  ITU-T T.81/ISO 10918-1 marker-segment structure (`SOI`/`APP0`-JFIF/`DQT`/`SOF0`/`DHT`/`DRI`/
  `SOS`+entropy-coded-data/`EOI`) `engine::decode_jpg` actually parses — no `*OCTET`/`size-eos`
  survives in these files. Verified: `grep -c "size-eos\|\*OCTET"` on the snapshot facet's
  binary/text leaves → the `.ksy` file's ONE remaining `size-eos: true` covers the genuinely
  unstructured entropy-coded scan tail (documented inline as a real decoder-only concern, not a
  placeholder — matches the recipe's "typed raw-retention for undecoded regions" allowance).
- **Diff**/**mutations** facets: mirrored xml's F1-established, verified-real precedent exactly
  (confirmed by reading xml's actual current grammar leaves before writing jpg's) — these types'
  real wire form IS generic JSON (`serde`/`serde_json` directly, no independent binary structure),
  so the honest grammar leaf says exactly that (`payload = json-value` / `JSON_VALUE: .*?` /
  `json_payload: bytes &eod` with a doc comment naming the sibling JSON Schema facet as the real
  shape reference) rather than inventing structure that doesn't exist.

## Deviations

- `encode_jpg` never emits `DRI`/restart markers even when `snap.restart_interval.is_some()` —
  the field is faithfully round-tripped through decode/snapshot/diff/mutation, but the encoder's
  actual bitstream-writing loop was left untouched per the brief ("preserve the existing real
  baseline decode logic... not rewriting the codec math"); adding real restart-marker emission is
  genuine new codec work, not a schema-layer change. Documented in the engine's `encode_jpg` doc
  comment.
- `encode_jpg` always canonicalizes to fresh Annex K DQT/DHT tables at the chosen quality — it
  does NOT re-emit a decoded file's own persisted `quant_tables`/`huffman_tables` verbatim on
  re-encode. This mirrors png's own documented `EncodeScopeNote` precedent (pixel canonicalization
  there, table canonicalization here) and is why `codec_retention_law` asserts pixel-length/
  dimension equality rather than byte-identity on the real-vs-re-encoded round trip.
- `other_segments` ARE re-emitted verbatim by `encode_jpg` (right after the JFIF APP0, before
  DQT) — unlike the table-canonicalization deviation above, retaining unmodeled real file content
  costs nothing extra and matches "nothing real on disk silently dropped" literally, not just at
  the snapshot layer.
- The `JpgArtifact` (UI-reduced-view) facet's own mutation-adjacent-looking `set_snapshot`
  triad leaf (`🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`) needed its `diff(snapshot)` ->
  `diff(base, next)` signature updated to match the new `diff_set_snapshot` signature — a
  one-line mechanical fix, same shape as png's own analogous leaf (confirmed by reading png's
  file first), not a design decision.
- Did not touch `📚️examples/🎬️demo/🖼️assets/example.dsl.semio` (currently an
  11-byte envelope-only asset, flagged by `handcrafted-grammar/empty-example` in `bun
  ./📜️script.ts policy`) — confirmed this is a PRE-EXISTING gap shared identically by png's own
  still-open example asset (same byte count, same breach), not something this wave's brief asked
  for and not a regression.

## Verification

- `cargo check -p semio-s-plugin-stdio --lib`: clean (after one transient external-churn error
  from a concurrent session's in-flight `tiff` triad-signature migration cleared on its own —
  confirmed unrelated by re-reading the error's file path before waiting it out, per the ticket's
  "classify before chasing" guidance).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::jpg"` → **29 passed, 0 failed**, including
  all six named law tests and `field_sweep_covers_every_mutable_field`.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **883 passed, 0 failed**.
- Grep gates: zero `snapshot: Option<` in the diff file; `impl DiffAlgebra<JpgSnapshot> for
  JpgDiff` present; zero apply-and-capture-shaped bodies in `mutations::diff()`; zero
  `RasterImage` usages anywhere in the jpg tree (only historical doc-comment mentions of what was
  killed); zero `serde_json::Value` in schema files.
- `bun ./📜️script.ts policy`: no NEW breaches introduced. The only jpg-specific findings are
  **stale allowlist entries** — `POLICY_GRAMMAR_HONESTY_ALLOWLIST`, `POLICY_DIFF_ALGEBRA_ALLOWLIST`,
  and `POLICY_FIELD_SWEEP_ALLOWLIST` still list jpg paths from before this wave's real work landed;
  since `📜️script.ts` is off-limits to fan-out agents, these are queued in `glue_followup` for the
  wave closer. Every other jpg-tagged breach (`artifact-schema/facet-completeness`,
  `mutation-migration/*`, `stdio-artifacts/composer`, `dsl-migration/diff-completeness`,
  `handcrafted-grammar/empty-example`) was cross-checked against png's identical, already-closed
  F3 wave and found byte-for-byte structurally identical there too — confirmed pre-existing,
  repo-wide, out-of-scope taxonomy/DiffCodec-wave findings, not regressions from this work.

## Files touched

Rust (9): standard-level `⚙️engine/🦀️component.rs`; `✳️any` subset's `🧬️schema/🦀️component.rs`
(`JpgArtifact`), `📸️snapshot/🦀️component.rs`, `🔺️diff/🦀️component.rs`,
`🧬️mutations/🦀️component.rs`, `🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`; `✳️baseline`
subset's `🧐️analyzer/🦀️component.rs`, `🏗️builder/🦀️component.rs`, `🎹️composer/🦀️component.rs`
(test-fixture-only changes in the last three, conformance-check logic unchanged).

Facet mirrors (16): snapshot/diff/mutations × `{.ts,.graphql,.json,.proto}` (12) + the
`✳️any/🧬️schema` `JpgArtifact` facet × `{.ts,.graphql,.json,.proto}` (4).

Grammar leaves (21): snapshot `📝️text/{.g4,.ebnf,.grammar.semio}` + `💾️binary/{.ksy,.spicy,
.protocol.semio,.abnf}` (7); diff and mutations each `📝️text/{.g4,.ebnf,.grammar.semio}` +
`💾️binary/{.ksy,.spicy,.protocol.semio,.abnf}` (7 × 2 = 14).

Full path list is reconstructable via `git status --porcelain -- "✏️s/…/📷️jpg"` (this session
only modified files inside `🏅️standards/🔖️jfif-1.01/`, none outside it, none in `✳️baseline`'s own
files beyond the three test-fixture updates listed above).
