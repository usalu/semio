# F6 — 📜️docx (ecma-376): OpText/OpBinary + DiffCodec

## Summary

Both sides — `DocxDiff`'s `protocol::DiffCodec` and `DocxMutation`'s `protocol::OpText`/
`protocol::OpBinary` — are **hand-rolled (3a+3b)**, exactly matching `f6-recon-report.md` §8's row
26 verdict. This was **verified for real**, not assumed: the derive was actually added, compiled,
and the real compiler errors captured, before being reverted and replaced with the hand-rolled
implementation.

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::docx"` → **47/47 passed** (0 failed),
  including the 2 new law tests.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1075/0** (0 failed) — every test in
  the crate, at the moment this session's work landed.

## Step 1 — real classification (derive attempted, reverted)

### Diff side

Temporarily added `#[derive(dsl::DslDiff)]` to `DocxDiff` and `#[derive(dsl::DslRecord)]` to
`DocxDocumentDiff`/`DocxParagraphDiff` (cascading one level to force the deeper errors to surface),
ran `cargo check -p semio-s-plugin-stdio --lib`, captured two independent real failures, then
reverted both derives (git diff after revert shows zero change beyond the intended new code):

```
error[E0277]: the trait bound `DocxOpcDiff: DslField` is not satisfied
   --> …/📜️docx/…/🔺️diff/🦀️component.rs:226:21
    |
226 |     pub opc: Option<DocxOpcDiff>,

error[E0277]: the trait bound `IndexedTripleDiff<DocxBlockDiff, DocxBlock>: DslField` is not satisfied
   --> …/📜️docx/…/🔺️diff/🦀️component.rs:165:22
    |
165 |     pub body: Option<DocxBlocksDiff>,
    (DocxBlockDiff is a genuine data-carrying enum: Paragraph/Table/Replace — 3a)

error[E0277]: the trait bound `std::option::Option<std::string::String>: DslField` is not satisfied
   --> …/📜️docx/…/🔺️diff/🦀️component.rs:114:23
    |
114 |     pub style: Option<Option<String>>,
    (tri-state Option<Option<T>> — 3b, same root cause as GifDiff)
```

Both `DocxParagraphDiff.style` and `DocxStyleDiff.based_on` are `Option<Option<String>>` — 3b
applies independently of the 3a enum blocker.

### Mutation side

Temporarily added `#[derive(dsl::DslOps)]` to `DocxMutation`, same `cargo check`, captured:

```
error[E0277]: the trait bound `DocxSnapshot: DslField` is not satisfied      (SetSnapshot)
error[E0277]: the trait bound `docx::…::DocxBlock: DslField` is not satisfied (InsertBlock/SetBlockContent — 3a, direct enum payload)
error[E0277]: the trait bound `docx::…::DocxBlockPath: DslField` is not satisfied (every path-carrying variant — DocxBlockPath itself isn't DslRecord-derived)
error[E0277]: the trait bound `docx::…::DocxStyle: DslField` is not satisfied (InsertStyle)
```

`SetSnapshot`'s `DocxSnapshot` recursively reaches `DocxBlock` (via `document.body`) and `XmlNode`
(via every `extra_*_properties` raw-retention field) — same 3a root cause `DocxDiff` hits.
`InsertBlock`/`SetBlockContent`'s `block: DocxBlock` carries the enum directly as a variant field,
independent of `SetSnapshot`. Confirms both sides land on HAND-ROLL, and confirms they'd fail for
the same underlying reason even if `DocxBlockPath`/`DocxStyle` were given `#[derive(dsl::DslRecord)]`
(a separate, shallower blocker also present).

## Step 2 — hand-rolled implementation

Followed `f6-recon-report.md` §5's grammar template exactly (hex for strings/bytes, bracket-depth-
aware `split_top_level`, `[0]`/`[1,x]` for `Option<T>` including nested double-option for tri-state,
single-uppercase-letter tag prefix for data-carrying enums). Two design choices beyond a literal
copy of the svg/gif precedent, both staying inside this file's own ownership boundary:

1. **Generic triple codecs.** `DocxDiff` already introduced its own generic
   `IndexedTripleDiff<D,T>`/`NamedTripleDiff<K,D,T>` collection-triple engine (prior wave's work,
   not mine) instead of bespoke per-collection triple structs like svg/gif use. I wrote ONE
   `enc_indexed_triple`/`dec_indexed_triple` pair and ONE `enc_named_triple`/`dec_named_triple` pair
   (generic over the diff/item type parameters) and reused them across all 7 instantiations this
   artifact needs (`body`, `runs`, table rows, table cells, `styles`, OPC content-type entries, OPC
   parts, OPC relationship lists, OPC relationships-by-owner) — rather than hand-writing 7 near-
   identical bespoke encoders the way svg/gif's own (non-generic) diff shapes required. This is a
   deviation from the letter of the recon's "one bespoke `enc_*`/`dec_*` pair per collection" advice,
   but a direct, justified consequence of `DocxDiff` already being generic where svg/gif's diff types
   are not.
2. **Value encoders shared, diff/mutation-specific encoders split by file.** `DocxDiff`'s file
   exports (`pub(crate)`) every FULL-ITEM codec (`enc_block`/`enc_style`/`enc_opc_part`/`enc_rel`/
   `enc_ct_entry`/`enc_rel_owner_entry`/`enc_xml_node`/primitives) since both the diff's own
   `added`/`Replace` payloads and the mutation's typed payloads need them — same intra-artifact
   reuse precedent `SvgDiff`/`SvgMutation` established. `enc_docx_snapshot`/`enc_opc_package`/
   `enc_docx_document`/`enc_opc_content_types` (whole-`DocxSnapshot`-shaped, only ever needed by
   `SetSnapshot`'s `OpText`) live in the mutations file only, matching where `SvgMutation` puts its
   own `enc_svg_snapshot`.

Grammar highlights actually exercised by the tests below:
- `DocxBlockDiff`: `P[...]` (Paragraph diff) / `T[...]` (Table diff) / `R[<block>]` (wholesale
  replace on a paragraph<->table kind change).
- `DocxBlock` (full item, for `added`/`Replace`/`InsertBlock`/`SetBlockContent`): `P[...]` / `T[...]`
  — recursive through `Table -> rows -> cells -> blocks`.
- Tri-states: `style: Option<Option<String>>` and `based_on: Option<Option<String>>` both use nested
  `encode_option(outer, |inner| encode_option(inner, enc_str))` — `[0]` unchanged, `[1,[0]]` cleared,
  `[1,[1,<hex>]]` set.
- Top-level `DocxDiff` line: `opc=... document=...` (absent token = unchanged, same recipe as
  svg/gif). Top-level `DocxMutation` line: `keyword arg=value ...`.
- `encode_diff`/`encode_op` = the text bytes verbatim (same simplification svg/gif/`WriterDiff` use
  — satisfies every `DiffCodec`/`OpText`/`OpBinary` law without a second wire format).

## Step 3 — tests added

- `diff_codec_text_binary_roundtrip_law` (new `handcrafted_diff_codec_tests` module in
  `🔺️diff/🦀️component.rs`) — exercises `DocxBlockDiff`'s `Paragraph`/`Table` variants incl. a
  nested table-cell block list, both tri-states (`Some(Some(_))` AND `Some(None)`, the latter
  required a fixture fix mid-session — see Deviations), and every OPC-layer removed/modified/added
  flavor (content-types defaults/overrides, parts, relationships-by-owner) via a real `between()`
  result in both directions.
- `op_text_binary_roundtrip_law` (added to the existing `tests` module in
  `🧬️mutations/🦀️component.rs`) — exercises every `DocxMutation` variant incl. `InsertBlock`'s bare
  `DocxBlock` payload (both a plain paragraph and a `Table` carrying nested rows/cells/blocks),
  `SetSnapshot`'s whole `DocxSnapshot` (OPC parts/content-types/relationships-by-owner plus the
  typed document/styles), and `SetStyleBasedOn`'s `Option<String>` transitions.

Both tests assert the three LAWS the trait contracts require: `!printed.contains('\n')`,
`parse(print(x)) == x`, `decode(encode(x)) == x`.

## Verification (real, this session)

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` with `dsl::DslDiff` added to `DocxDiff` | 2 independent real errors captured (§Step 1), reverted |
| `cargo check -p semio-s-plugin-stdio --lib` with `dsl::DslOps` added to `DocxMutation` | 4 real errors captured (§Step 1), reverted |
| `cargo check -p semio-s-plugin-stdio --lib` (final, my code in place) | clean, 0 errors |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::docx"` | **47/47 passed** |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1075/0** |
| Literal-text `dsl-migration/diff-completeness` policy check (`📜️script.ts:3185`) | `DocxDiff`'s file now contains `DiffCodec for` — self-satisfies the check without editing `📜️script.ts` or the allowlist |

Saved evidence in this ticket folder: `f6-docx-test-scoped.txt` (docx-scoped run),
`f6-docx-test-full-crate.txt` (whole-crate run, 1075/0).

## Deviations from the recon's §5 template

1. **Generic triple codecs instead of one bespoke pair per collection** — see Step 2, point 1 above.
   Justified by `DocxDiff`'s own prior-wave generic collection-triple types; not a deviation in
   spirit (still bracket-depth-aware, still the same `[removed];[modified];[added]` shape), only in
   how many times the shape gets typed out.
2. My first fixture draft for the new `diff_codec_text_binary_roundtrip_law` test did not actually
   exercise the `based_on: Some(None)` tri-state transition (both snapshots' shared "keep" style
   started and ended at `based_on: None`) — the test's own trailing assertion caught this (real
   `cargo test` failure, not a passed-but-untested gap), fixed by giving `snapshot_a`'s "keep" style
   a non-`None` `based_on` so the `a -> b` transition genuinely clears it.
3. Per the ticket brief, did **not** touch the separate OPC-diff-type-duplication cleanup F5 flagged
   (`DocxOpcDiff` etc. staying docx-local rather than hoisted to `zip::opc`) — implemented
   `DiffCodec`/`OpText`/`OpBinary` for exactly what already exists, duplication included, per
   explicit instruction not to conflate the two.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — F6 doc comment on `DocxDiff` citing the real compile errors; new `//#region 🔖️HandcraftedDiffCodec`
  (primitives, XML/value/diff-value codecs, generic triple codecs, top-level `print_diff`/`parse_diff`,
  `impl protocol::DiffCodec for DocxDiff`); new `handcrafted_diff_codec_tests` module with
  `diff_codec_text_binary_roundtrip_law`. (+651/-0 lines net new region, plus the doc comment and one
  import line.)
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — F6 doc comment on `DocxMutation` citing the real compile errors; replaced the `serde_json`-backed
  `OpText`/`OpBinary` stubs with the hand-rolled grammar (path/segment codecs, whole-`DocxSnapshot`/
  `OpcPackage`/`DocxDocument`/`OpcContentTypes` codecs, `print_docx_mutation`/`parse_docx_mutation`);
  added `op_text_binary_roundtrip_law` to the existing `tests` module; added `OpText`/`OpBinary`/
  `DocxPathSegment`/`HashMap`/OPC-type imports.
- Ticket-folder scratch (kept per repo rules): `f6-docx-test-scoped.txt`,
  `f6-docx-test-full-crate.txt`.

**Ownership respected**: no edits to `📦️glue.rs`, `📜️script.ts`, `🏪️store`, the `dsl`/`protocol`/
`schema` framework crates, or any other artifact's files. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` not
touched (not needed — the real `DiffCodec` impl makes the literal-text check pass on its own).
