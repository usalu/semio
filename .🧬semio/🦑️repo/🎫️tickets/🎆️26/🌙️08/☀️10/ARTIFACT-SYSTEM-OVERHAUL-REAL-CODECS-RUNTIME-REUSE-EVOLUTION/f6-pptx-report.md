# F6 — 🎞️pptx (ecma-376) — OpText/OpBinary + DiffCodec

**Artifact**: `🎞️pptx`, standard `ecma-376`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/`.
**Path taken**: **HAND-ROLL** on both sides (Diff and Mutation), matching the recon report's row 24
guess, confirmed for real (not trusted blindly) via `cargo check`.

## Step 1 — classification (real compiler errors, both sides)

Followed `f6-recon-report.md` §9's procedure literally: added `#[derive(dsl::DslDiff)]` to
`PptxDiff` and, separately, `#[derive(dsl::DslOps)]` to `PptxMutation`, ran
`cargo check -p semio-s-plugin-stdio --lib` for real, captured the actual errors, then reverted the
derive attempts. Full captured output: `f6-pptx-derive-rejection-check.txt` in this folder (both
derives applied simultaneously in that run, 16 real `error[E0277]`/`E0107` diagnostics for pptx's
own files).

**Diff side** (`PptxDiff`) — confirmed HAND-ROLL for **three independent reasons**:

1. **3a (enum-in-tree)**: `PptxShapeDiff` (declared in the diff file itself, tag `TextBox`/
   `Picture`/`Placeholder`/`Replace`) is a genuine data-carrying enum, reached through
   `PptxSlideDiff.shapes: Option<PptxShapesDiff>`. Real error:
   `the trait bound PptxOpcDiff: DslField is not satisfied` / `PptxPresentationDiff: DslField is
   not satisfied` cascading down to the enum leaf (matches `SvgNodeDiff`'s blocker exactly).
2. **3b (tri-state)**: `PptxRunDiff.font_size: Option<Option<u32>>` — same structural blocker as
   `GifFrameDiff`'s `lct`/`transparent_index`.
3. **A THIRD, previously-undocumented blocker beyond `f6-recon-report.md` §3**: this artifact's
   (and `📜️docx`'s, per the recon sweep) generic `IndexedTripleDiff<D, T>` / `NamedTripleDiff<K, D,
   T>` collection-diff engine **cannot be `#[derive(dsl::DslRecord)]`'d at all** — the derive macro
   has zero generics support. Confirmed by actually attempting it directly on
   `IndexedTripleDiff<D, T>`: the derive emits literally malformed codegen —
   `struct IndexedTripleDiff<D, T><D, T>` — producing `error[E0107]: missing generics for struct`.
   This is a **structural blocker independent of 3a/3b**: even a hypothetical pptx-shaped artifact
   with zero enums and zero tri-state fields would still be blocked, since every collection field in
   this recipe's shape (`slides`, `shapes`, `paragraphs`, `runs`, OPC `parts`/`relationships`)
   routes through one of these two generic engines. Flagging this for whichever future agent
   revisits `f6-recon-report.md`'s §3 decision rule — it undercounts the real hand-roll population
   for every artifact using this generic collection-diff pattern (docx confirmed sharing it; likely
   xlsx too per the recon table's OPC-family grouping).

**Mutation side** (`PptxMutation`) — confirmed HAND-ROLL, same root cause as `SvgMutation`:
`SetSnapshot{snapshot: PptxSnapshot}` fails (`PptxSnapshot: DslField is not satisfied` —
`PptxSnapshot` embeds `PptxPresentation`/`OpcPackage`, both routing through the same generic
triple-diff engine, and `PptxShape` is reachable inside), **plus** a second independent hit at
`InsertShape{shape: PptxShape}` / `InsertSlide{slide: PptxSlide}` (both carry the enum-shaped
`PptxShape` DIRECTLY as a variant field, mirroring `SvgMutation::InsertElement`'s `node: XmlNode`
blocker — confirmed real compiler citations quoted verbatim in the mutations file's own doc
comment).

Both derive attempts were reverted after capturing evidence — `PptxDiff`/`PptxMutation` carry no
`dsl::DslDiff`/`dsl::DslOps` derive in the final state, only the hand-rolled trait impls below.

## Step 2b — hand-rolled grammar (svg's template, adapted)

Copied svg's primitive set (`hex_encode`/`hex_decode`/`split_top_level`/`strip_brackets`/
`encode_option`/`decode_option`) into `🔺️diff/🦀️component.rs`, plus two NEW generic helpers this
artifact's collection shape needed that svg didn't (svg's own children/attrs triples are hand-coded
per-type, not generic): `enc_indexed`/`dec_indexed` and `enc_named`/`dec_named`, parameterized over
`IndexedTripleDiff<D, T>` / `NamedTripleDiff<String, D, T>` so every one of pptx's SIX collection
triples (slides, shapes, paragraphs, runs, OPC content-types×2, OPC parts, OPC relationships×2
doubly-nested) reuses the SAME two functions instead of six hand-duplicated ones — a genuine
simplification the generic engine enables that svg's bespoke per-type triples didn't need.

Conventions (all matching svg/gif precedent):
- Hex for strings/bytes (`enc_str`/`dec_str`, `hex_encode`/`hex_decode` for raw `Vec<u8>`).
- `[f1,f2,...]` positional tuples for structs; single-field wrapper structs (`PptxParagraph`,
  `PptxSlideDiff`, `PptxParagraphDiff`, `PptxPresentationDiff`) collapse to that one field's own
  encoding (documented per-function, no redundant extra bracket).
- `PptxShape`/`PptxShapeDiff` (both data-carrying enums): single-uppercase tag prefix — `B`=TextBox,
  `P`=Picture, `H`=Placeholder, `O`=Other (item-only), `R`=Replace (diff-only). The two enums reuse
  the same 4 letters for their shared variants since they're never parsed in the same context.
- Collection triples: `[removed];[modified];[added]`, indexed triples' `modified`/`added` entries
  `idx:diff`/`idx:item`; named triples' `removed`/`modified.key` are hex-encoded string keys,
  `added` is the whole item (already carrying its own key, per `NamedTripleDiff`'s own shape) — no
  `idx:`/`key:` prefix needed there.
- `font_size: Option<Option<u32>>` (the one genuine tri-state field): double-nested
  `encode_option(&d.font_size, |inner| encode_option(inner, |v| v.to_string()))` — outer layer is
  "was this field touched" (always needed inside a fixed positional tuple, unlike svg's top-level
  Diff fields which get that dimension for free from sparse token-join), inner layer is the actual
  tri-state "cleared vs set".
- `OpcTargetMode` (plain unit enum, Internal/External): single-digit tag ("0"/"1"), no bracket
  needed (not data-carrying).
- `encode_diff`/`encode_op` = the text bytes verbatim, same simplification `GifDiff`/`SvgDiff`/
  `WriterDiff` use.
- `relationships: HashMap<String, Vec<OpcRelationship>>` (only reachable via `SetSnapshot`'s full
  `PptxSnapshot`, never via the sparse `PptxOpcDiff`): `enc_opc_package` sorts owner keys before
  encoding, so `OpBinary`'s determinism LAW ("byte-identical output for equal operations") holds
  despite `HashMap`'s unspecified iteration order.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — added `HandcraftedDiffCodec` region: primitives, generic `IndexedTripleDiff`/`NamedTripleDiff`
  codecs, per-value/per-diff-type `enc_*`/`dec_*` functions (all `pub(crate)` where the mutations
  file needs to reuse them), top-level `print_pptx_diff`/`parse_pptx_diff`, `impl
  protocol::DiffCodec for PptxDiff`, and a new `handcrafted_diff_codec_tests` module with
  `diff_codec_text_binary_roundtrip_law`. No existing region (diff/apply/absorb/`SetSnapshot`
  logic from S1-F6b) touched.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — replaced the `serde_json`-stub `impl protocol::OpText`/`impl protocol::OpBinary for
  PptxMutation` with a real hand-rolled grammar (`enc_opc_package`/`enc_snapshot`/
  `print_pptx_mutation`/`parse_pptx_mutation`), reusing the diff file's `pub(crate)` primitives;
  added `op_text_binary_roundtrip_law` to the existing `tests` module. `Mutation`/`diff`/`inverse`
  logic (S1-F6b) untouched.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-pptx-derive-rejection-check.txt` (real
  `cargo check` output with both derive attempts applied), `f6-pptx-test1.txt` (scoped
  `artifacts::pptx` test run, 50/50), `f6-pptx-full-crate-test-final.txt` (whole-crate run,
  1060/0).

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework
crates, `POLICY_DIFF_COMPLETENESS_ALLOWLIST` were all read-only or untouched this session. No other
artifact's files touched (gltf's mid-edit compile breakage encountered twice during this session's
full-crate runs was a different, concurrently-running F6 sub-agent's in-progress work — confirmed
via `git status` showing `🧊️gltf` files uncommitted/modified at the time; resolved itself once that
sibling session's edit landed, not caused by or fixed by this session).

## Verification (all real, this session)

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` (both derive attempts applied, evidence capture) | 16 real `error[E0277]`/`E0107` diagnostics for pptx (see `f6-pptx-derive-rejection-check.txt`) |
| `cargo check -p semio-s-plugin-stdio --lib` (hand-rolled impl, derives reverted) | 0 errors, 0 pptx-related warnings |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::pptx"` | **50/50 passed, 0 failed** (incl. new `diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`) |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1060/0** (transiently interrupted twice by an unrelated concurrent session's in-progress edits to `🧊️gltf`'s test fixtures — confirmed via `git status`, not this session's files; final clean run 1060 passed / 0 failed) |

## Deviations from the recon report's guessed classification

- Row 24 of §8's table guessed "1 enum, 1 tri-state" from a single-file grep heuristic on the diff
  file only. Confirmed accurate for what it checked, but the sweep's methodology (grep the diff file
  for `pub enum`/`Option<Option<`) cannot see the THIRD blocker (generic collection-diff engine,
  §3's decision rule doesn't mention generics at all) — worth flagging for whichever closer-level
  pass eventually revisits §3's rule, since it affects every artifact sharing pptx/docx's
  `IndexedTripleDiff`/`NamedTripleDiff` pattern, independent of enum/tri-state content.
- No deviation from §5's grammar conventions themselves — the two new generic triple-codec
  functions (`enc_indexed`/`enc_named`) are a direct, mechanical generalization of svg's own
  per-type `enc_children_diff`/`enc_attrs_diff`, not a new convention.
