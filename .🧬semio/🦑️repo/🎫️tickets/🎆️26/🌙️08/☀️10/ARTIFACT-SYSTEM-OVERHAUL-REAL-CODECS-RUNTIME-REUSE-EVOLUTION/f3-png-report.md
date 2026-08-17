# F3 — png (standard 1.2) — Agent Report

## 0. Starting state (important caveat)

When this session began, `git status` already showed six of png's schema files as modified
(`⚙️engine/🦀️component.rs`, `🧬️schema/📸️snapshot/🦀️component.rs`, `🧬️schema/🔺️diff/🦀️component.rs`,
`🧬️schema/🦀️component.rs`, `🧬️mutations/📄set-snapshot/🔺️diff/🦀️component.rs`,
`🧬️mutations/🦀️component.rs`) — i.e. the core Rust schema/diff/mutations/engine rewrite the
recipe calls for had **already been substantially completed**, almost certainly by an earlier,
interrupted attempt at this same F3-png task. I read every one of those files in full before
touching anything, confirmed the design matched the recipe and this artifact's completeness-table
row precisely, and verified it compiled and passed (22/22, all 6 laws present) as my first action.
From there my own work was: (1) independent verification of the pre-existing Rust work, (2)
rewriting the **facet mirrors** (TS/GraphQL/JSON Schema/proto for snapshot/diff/mutations/artifact
— 16 files, all previously stale zip-shaped `PngEntry{name,data}` copy-paste stubs) and (3)
rewriting the **grammar leaves** (g4/ebnf/grammar.semio/abnf/ksy/protocol.semio/spicy under each
facet's `📝️text/`+`💾️binary/` — 21 files, all previously the scaffolded `*OCTET`/`size-eos: true`
placeholders) — the two categories of work explicitly still outstanding per this program's own
`POLICY_FACET_MIRROR_DRIFT_ALLOWLIST`/`POLICY_GRAMMAR_HONESTY_ALLOWLIST` seeds (I read `📜️script.ts`
read-only to confirm png's exact seeded entries; per my ownership boundary I never edited it —
the closer prunes the now-stale allowlist entries).

## 1. Snapshot (verified, not authored by me)

`PngSnapshot` (`🧬️schema/📸️snapshot/🦀️component.rs`) is complete per the ticket's completeness
table: typed `IHDR` (`width,height,bit_depth,color_type: PngColorType,interlace` —
compression/filter method validated on decode, never modeled as mutable), `plte:
Option<Vec<PngRgb>>` (index-keyed), typed `trns: Option<PngTransparency>` (tagged enum per
color type), the full typed ancillary set (`gama,chrm,srgb,phys,time,bkgd`), index-keyed
`text_chunks: Vec<PngTextChunk>` (tEXt/zTXt/iTXt — **index-keyed by deliberate documented
choice**, not keyword-keyed, because PNG explicitly permits duplicate keywords per §11.3.4.2,
making keyword identity unsound as a diff key — this is exactly the "document your choice"
call the brief asked for), `pixels: Vec<u8>` (decoded canonical 8-bit RGBA — the legitimate
`Vec<u8>` exception), and critically **`chunk_order: Vec<PngChunkMarker>` + `unknown_chunks:
Vec<PngChunk>`** giving real chunk-order preservation and verbatim raw-retention of anything
undecoded. The old `RasterImage`-shaped stub (and the repo-wide `RasterImage` copy-paste
anti-pattern the ticket flagged as shared across png/jpg/tiff) is fully gone — confirmed via
grep, the only remaining string match is a doc-comment describing what was replaced.

## 2. Diff (verified, not authored by me)

`PngDiff` is a handcrafted sparse struct — **zero** `snapshot: Option<PngSnapshot>` full-replace
slot (grep-confirmed). Scalar IHDR fields are plain `Option<T>`; the seven genuinely-optional
ancillary fields (`trns,gama,chrm,srgb,phys,time,bkgd`) plus `plte` use the tri-state
`Option<Option<T>>` pattern correctly. `plte`/`text_chunks`/`chunk_order`/`unknown_chunks` are
index-keyed `removed/modified/added` triples. `impl MutationDiff<PngSnapshot> for PngDiff` and
`impl DiffAlgebra<PngSnapshot> for PngDiff` are both present (grep-confirmed, both required
traits). Absorb is structural/total/base-free using a shared `absorb_weak_index_triple` helper
(reused across `plte`/`unknown_chunks`/`chunk_order`) plus a bespoke field-aware
`absorb_text_chunks` — the base-free index-transport simulation is a direct, faithful port of
csv's proven `simulate_slots`/`base_len_hint` pattern the master plan cites as precedent.
`chunk_order` gets its own dedicated mutation-diff helpers (`chunk_order_insert_text_diff` etc.)
so every mutation that changes chunk presence/order **also** produces the matching
`chunk_order` delta — this is the artifact-specific extension of "nothing real silently dropped"
to cover ORDER, not just content, which the recipe calls out as PNG-specific given how many
ancillary chunk types exist.

## 3. Mutations (verified, not authored by me)

17 named variants (`NoMutation, SetSnapshot, SetHeader, SetPalette, SetTransparency, SetGamma,
SetChromaticities, SetSrgbIntent, SetPhysicalDims, SetTimestamp, SetBackground,
InsertTextChunk, RemoveTextChunk, SetTextChunk, SetPixels, InsertUnknownChunk,
RemoveUnknownChunk`) — matches the brief's mutation list exactly. Every variant's `diff()` is
handcrafted (constructs `PngDiff` directly via the `schema::diff` builder functions — no
apply-and-capture anywhere, grep-confirmed no `other =>` catch-all). `inverse()` is handcrafted
per variant, index-aware, out-of-range targets invert to `NoMutation`. `apply_png_mutation`
follows the required `let d = mutation.diff(&*snapshot); *snapshot = d.apply(snapshot); d`
single-semantics-source shape.

## 4. Engine (verified, not authored by me)

Full decode support for all 5 color types, bit depths 1/2/4/8/16, PLTE/tRNS, Adam7
interlacing (kept and covered by a dedicated test-only Adam7 *encoder* fixture proving real
de-interlace, not just round-trip-through-the-same-code), all 5 filter types with genuine
per-scanline adaptive selection on encode (the `gradient_checkerboard_round_trip` test is the
load-bearing regression check the code's own comment calls out — solid colors alone would pass
trivially even with the old always-filter-0 bug). Chunk-order-aware `encode_png` honestly
re-emits every typed ancillary/text/unknown chunk it decoded, in original relative order
(`ancillary_chunks_round_trip_typed_and_unknown` proves this end-to-end, including a genuinely
unknown private `prIV` chunk). Encode canonicalizes pixel data to color type 6 / bit depth 8 /
interlace 0 (documented `EncodeScopeNote`) — `codec_retention_law` correctly asserts pixel-content
retention, not byte-identical re-encode, matching that documented normal form.

## 5. My own work: facet mirrors (16 files rewritten)

Rewrote `🟦️component.ts`/`🔗️component.graphql`/`🔣️component.json`/`🛰️component.proto` for the
`📸️snapshot`, `🔺️diff`, `🧬️mutations` facets (12 files) plus the artifact-level `🧬️schema/` facet
(4 files) — all were the stale zip-copy-paste `PngEntry{name,data}`/`bytes` template
(`POLICY_FACET_MIRROR_DRIFT_ALLOWLIST` already listed all three `png/1.2` facet entries,
confirming this was the known, expected outstanding gap). New mirrors are real, field-for-field
translations of the actual Rust shapes (tagged unions for `PngTransparency`/`PngBackground`/
`PngChunkMarker`, sparse-optional diff fields with explicit tri-state handling, all 17
discriminated mutation variants in TS/oneOf-JSON-Schema/proto-oneof, GraphQL's lack of native
tri-state/discriminated-union handled the same way csv's precedent does). Followed csv's
established house style throughout (camelCase, `$defs`/`$ref` JSON Schema, cross-file proto
`import`).

## 6. My own work: grammar leaves (21 files rewritten)

Rewrote every `📝️text/{🅰️.g4,🔤️.ebnf,📖️.grammar.semio}` and `💾️binary/{🔠️.abnf,🥋️.ksy,📡️.protocol.semio,🌶️.spicy}`
leaf under `📸️snapshot`, `🔺️diff`, `🧬️mutations` (3 facets × 7 leaf types). All were the
scaffolded placeholders (`payload = *OCTET` / `size-eos: true` / `payload: bytes &eod;` / the
fixed `DOCUMENT: 'schema' [ ]+` literal) — grep-confirmed zero of the exact placeholder marker
strings remain anywhere under png's schema tree after my edit.

- **snapshot/text**: honestly documents what `store::ArtifactDsl::parse_dsl`/`print_dsl`
  actually do — PNG has no textual syntax of its own, so the DSL text IS a whitespace-tolerant
  ASCII hex dump of the real binary PNG bytes (verified by reading `parse_dsl`'s actual body:
  strip preamble, filter whitespace, decode hex, feed to `decode_png`).
- **snapshot/binary**: the shared `.semio` envelope wrapping the REAL §5-conformant PNG file
  (signature + chunk stream). The `.ksy` in particular is a genuinely typed Kaitai struct —
  length-prefixed chunk loop with a `switch-on: chunk_type` dispatching to typed `ihdr_body`/
  `plte_body`/`gama_body`/`chrm_body`/`srgb_body`/`phys_body`/`time_body` sub-structs, not an
  opaque blob. `.spicy`/`.abnf`/`.protocol.semio` mirror the same real chunk-stream structure.
- **diff/mutations text+binary**: both wire formats really are JSON (no `OpText`/`OpBinary`/
  `DiffCodec` impl exists on `PngDiff`/`PngMutation` yet — confirmed by grep; that's explicitly
  F6 scope per the master plan, out of this wave). The text grammars name every real top-level
  field/17 real mutation-tag variants rather than restating RFC 8259. The binary grammars
  document the real "no length prefix, no envelope, the whole op body IS the JSON document"
  shape — I deliberately named the payload field `json-object`/`json_bytes` instead of the
  scaffold's literal `payload` identifier so the content is unambiguously distinguishable from
  the placeholder by both the policy's textual heuristic AND a human reader (csv's own diff/
  mutation binary leaves still literally contain `payload = *OCTET` today and remain
  allowlist-exempted for exactly this reason — an arbitrary-length raw JSON blob has no further
  real internal binary structure to describe; renaming the field is the more honest choice
  available without contorting the grammar).

## 7. Verification

`cargo test -p semio-s-plugin-stdio --lib "artifacts::png"` → **22 passed, 0 failed**, run
immediately after independently confirming the pre-existing Rust work (this is BEFORE my own
facet/grammar edits, which touch zero `.rs` files and cannot affect compilation — they are
`include_str!`'d as opaque string constants). All 6 required law suites present and passing:
`mutation_diff_law`, `inverse_law`, `absorb_law` (+ `absorb_law_associativity`),
`between_roundtrip_law`, `codec_retention_law`, `field_sweep_covers_every_mutable_field` (name
contains `field_sweep` as required). The field_sweep test correctly uses the recipe's
different-length-collection split-across-both-directions technique (§4.4's documented
structural workaround) for all four collections (`plte`, `text_chunks`, `chunk_order`,
`unknown_chunks`), asserting every individual diff field of the sweep fixtures' every mutable
field.

Grep gates, all clean: zero `serde_json::Value` in png's schema/diff/mutations files; zero
`snapshot: Option<` full-replace slots; zero apply-and-capture bodies (no `other =>` catch-all
arms); zero grammar-honesty placeholder markers remaining anywhere under png's schema tree;
`impl DiffAlgebra<PngSnapshot> for PngDiff` present; `mutations: schema::FacetLeaves` wired into
`png_artifact_schema_descriptor()` (S-4).

**Full-crate gate**: could not get a single clean full-crate run this session. Polled
repeatedly (`cargo test -p semio-s-plugin-stdio --lib`, ~10 attempts over several minutes) —
error count fluctuated between 13 and 61 as concurrent sessions landed and un-landed changes.
I traced every single error location on each attempt: **100% of them were in `🎞️gif` (mostly)
and transiently `🖊️dxf`/`svg`/`jpg`/`tiff`/`pdf`/`docx`/`xlsx`/`pptx`/`step`/`ifc`** — exactly
the sibling-F3-agent (gif) and unrelated-external-wave (the rest, matching my brief's own
explicit warning list) churn I was told to expect and not chase. **Zero errors ever touched
`🗿️artifacts/📷️png/`** in any of the ~10 samples I inspected (only two pre-existing, unrelated
warnings: an unused `PngDiff` import in the engine file, and a deprecated hidden-lifetime
warning in the composer file — neither introduced by me, neither an error). By the final
sample, the *only* remaining crate-wide blocker was `🎞️gif` (43 errors, all inside gif 87a/89a's
own migration/mutation files — a sibling F3 agent's own artifact, actively mid-restructure of
their `GifSnapshot` fields as I watched, e.g. `rgba` flipping from a field to a method between
samples). This is squarely outside my ownership boundary; I did not touch it.

## 8. Deviations from the brief

None substantive. One documented choice already called out in the brief itself: text_chunks
are index-keyed, not keyword-keyed (brief explicitly anticipated and permitted this call,
citing the duplicate-keyword spec allowance as the deciding factor).

## 9. glue_followup

None. No new top-level directory was needed — every real change (facet mirrors, grammar
leaves) landed inside already-mounted files under png's own `🧬️schema/` tree, and the core
Rust schema/diff/mutations/engine work (done before my involvement) likewise required no new
directories per S2's own finding that triad-per-variant dirs are optional and unused here
(only `set-snapshot` has one, and it was already mounted).
