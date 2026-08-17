# F6 — 📕️xlsx (ecma-376) — OpText/OpBinary/DiffCodec Report

## Scope
Implemented `protocol::OpText`/`protocol::OpBinary` for `XlsxMutation` and `protocol::DiffCodec` for
`XlsxDiff`, per `f6-recon-report.md` §9's procedure, followed literally. No other files touched.

## STEP 1 — real classification (both sides confirmed HAND-ROLL)

The recon's own row for xlsx (§8, row 11) was `CHECK-ENUM-ELSEWHERE`, flagged as "likely actually
HAND-ROLL" pending a real compile check. Verified for real:

**Diff side**: added `#[derive(dsl::DslDiff)]` to `XlsxDiff`, ran
`cargo check -p semio-s-plugin-stdio --lib` (`f6-xlsx-diff-check1.txt`/`check2.txt` in this folder).
Root cause, isolated by incrementally adding `#[derive(dsl::DslRecord)]` to nested structs until the
real blocker surfaced:
```
error[E0277]: the trait bound `XlsxCellValue: DslField` is not satisfied
  --> …/🔺️diff/🦀️component.rs:72:23   (pub value: Option<XlsxCellValue>)
help: the trait `DslField` is not implemented for `…snapshot::component::XlsxCellValue`
  --> …/📸️snapshot/🦀️component.rs:26:1   (pub enum XlsxCellValue)
```
`XlsxCellValue` (`Number(f64)`/`SharedString(usize)`/`InlineString(String)`/`Boolean(bool)`/
`Formula{expr,cached}`/`Empty`) is a genuine data-carrying enum reachable from `XlsxCellDiff.value` —
confirms §3a (enum-in-tree). Independently, the top-level `opc: Option<XlsxOpcDiff>`/
`workbook: Option<XlsxWorkbookDiff>` fields also failed until nested structs got `DslRecord`, AND the
shared `NamedTripleDiff<K,D,T>` generic collection-triple type this file's collections use has no
`DslField` impl at all (no blanket impl for arbitrary generic structs) — a second, independent
structural blocker beyond the enum.

**Mutation side**: added `#[derive(dsl::DslOps)]` to `XlsxMutation`, ran `cargo check` again
(`f6-xlsx-mutation-check1.txt`). Confirmed independently:
```
error[E0277]: the trait bound `XlsxCellValue: DslField` is not satisfied
  --> …/🧬️mutations/🦀️component.rs:45:16   (SetCell { .. value: XlsxCellValue })
error[E0277]: the trait bound `XlsxSnapshot: DslField` is not satisfied
  --> …/🧬️mutations/🦀️component.rs:23:19   (SetSnapshot { snapshot: XlsxSnapshot })
error[E0277]: the trait bound `XlsxSheet: DslField` is not satisfied
  --> …/🧬️mutations/🦀️component.rs:27:16   (InsertSheet { sheet: XlsxSheet })
```
`SetCell.value: XlsxCellValue` carries the enum-shaped payload DIRECTLY as a variant field (same root
cause), `SetSnapshot`/`InsertSheet` reach it transitively. Both derive attempts were reverted after
capturing the real errors; the citations are preserved as doc comments on `XlsxDiff` and
`XlsxMutation` themselves (matching the gif/svg precedent's citation style).

Both sides land on **HAND-ROLL** — recon's row was correct once actually verified, and the reason is
exactly the pattern flagged as needing verification (`XlsxCellValue` is indeed a variant-shaped value
enum reachable from the diff, per the plan's completeness spec: Number/SharedString/InlineString/
Boolean/Formula/Empty).

## STEP 2b — hand-roll implementation

Followed §5's grammar template exactly: bracket-depth-aware `split_top_level`, hex encoding for
strings/bytes, `[0]`/`[1,x]` for `Option<T>`, `[removed];[modified];[added]` for collection triples,
single-uppercase-letter tag prefix for the data-carrying enum, space-separated `name=value`/
`keyword arg=value` top-level lines, `encode_diff`/`encode_op` = the text bytes verbatim.

One addition beyond the gif/svg precedent, needed because this artifact reuses ONE generic collection
type (`NamedTripleDiff<K,D,T>`) across SIX distinct instantiations (cells/sheets/shared_strings/
ct-entries/parts/rel-lists, plus relationships nesting a rel-list triple as its own `D`): a generic
`enc_triple`/`dec_triple` pair parameterized by per-field encode/decode closures, rather than six
near-identical bespoke encoders (would violate the "concise code" rule for no benefit). `f64` fields
(`XlsxCellValue::Number`) use `f64::to_string()`/`str::parse::<f64>()` directly — std's shortest-
round-trip float formatting round-trips exactly, no manual bit-pattern encoding needed, and none of
`.`/`-`/`e`/`inf`/`NaN` clash with the grammar's separators.

Per §5's reuse convention (svg's `SvgDiff`↔`SvgMutation` precedent), the diff module's primitives and
full-VALUE (not diff) codecs were marked `pub(crate)` so the mutations module reuses them directly
instead of duplicating: `hex_encode`/`hex_decode`/`enc_str`/`dec_str`/`parse_u32`/`parse_usize`/
`enc_f64`/`dec_f64`/`split_top_level`/`strip_brackets`/`encode_option`/`decode_option`/
`enc_cell_value`/`dec_cell_value`/`enc_cell_key`/`dec_cell_key`/`enc_cell`/`dec_cell`/`enc_sheet`/
`dec_sheet`/`enc_part`/`dec_part`/`enc_ct_entry`/`dec_ct_entry`/`enc_target_mode`/`dec_target_mode`/
`enc_rel`/`dec_rel`/`enc_owner_rels`/`dec_owner_rels`. The *_diff-shaped encoders (which the mutations
side never needs) stay private to the diff module.

### A real bug found and fixed during STEP 3 (worth flagging)
The first `diff_codec_text_binary_roundtrip_law` run failed: `NamedTripleDiff.removed` containing
`["xl/workbook.xml", ""]` (the OPC package-ROOT relationship owner, a real, common case — `""` is a
legitimate `HashMap<String, Vec<OpcRelationship>>` key per `zip::opc::OpcPackage`'s own doc comment)
round-tripped to `["xl/workbook.xml"]`, silently dropping the empty-string entry. Root cause: every
`dec_*` list-splitting call chained `.filter(|s| !s.is_empty())` after `split_top_level` — copied
defensively from the gif/svg precedent's idiom — to treat "no items" as "no items", but this ALSO
drops a legitimately-**empty-string-encoded** item (here: `""`'s hex encoding IS `""`) when it
appears alongside other non-empty items in the same list. `split_top_level` already returns
`Vec::new()` for a genuinely empty input string on its own (`if s.is_empty() { return Vec::new(); }`)
— the filter was both redundant for the "0 items" case and actively harmful for the "1+ items,
one of them empty" case. Fixed by removing all 12 occurrences of `.filter(|s| !s.is_empty())` across
both files (diff module: 5, mutation module: 7) — `split_top_level`'s own empty-input short-circuit
is sufficient and correct. Re-ran the roundtrip law: passes. (A theoretical residual ambiguity
remains — a list containing EXACTLY ONE item whose own encoding is `""` is indistinguishable from a
truly-empty list, since both stringify identically to `[]`; this is an inherent limitation of
comma-joined lists without a length prefix, shared with the gif/svg precedent's own grammar, not
introduced by this fix, and not hit by any real xlsx fixture — flagged for awareness, not fixed here
since it would require deviating from the established repo-wide grammar convention.)

## STEP 3 — tests

- `diff_codec_text_binary_roundtrip_law` (new, `handcrafted_diff_codec_tests` module in the diff
  file): exercises every `XlsxCellValue` variant (incl. `Formula.cached` and a value containing raw
  `,`/`:`/`[`/`]` bytes-through-hex), the OPC content-types/parts/relationships triples (incl.
  `OpcTargetMode::External`), and both `opc`/`workbook` top-level tokens present together, alone, and
  absent (`XlsxDiff::default()`, `between(&a, &empty)`, `between(&empty, &a)`).
- `op_text_binary_roundtrip_law` (new, in the mutations file's existing `tests` module, reusing its
  `sweep_b()` fixture): exercises every `XlsxMutation` variant, incl. `SetSnapshot`'s full nested
  `XlsxSnapshot` (opc parts/content-types/relationships incl. `OpcTargetMode::External`, workbook
  sheets/cells/shared strings) and `SetCell`'s direct `XlsxCellValue` payload across every variant
  (`Number`, `SharedString`, `Boolean`, `InlineString` with odd characters, `Formula` with and without
  `cached`).

Both assert `!printed.contains('\n')`, `parse(print(x)) == x`, `decode(encode(x)) == x`, per the
trait's LAWS.

## STEP 4 — real verification

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` | clean, 0 errors (xlsx-scoped) |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::xlsx"` | **43 passed, 0 failed** (incl. both new law tests) |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1033 passed, 0 failed** |
| `bun ./📜️script.ts policy` — `dsl-migration/diff-completeness`, xlsx-scoped | **0 breaches** (was flagged before this session; `grep` confirms xlsx absent from the rule's output; other un-migrated stdio artifacts like las/zip/gif87a/pptx/ifc still correctly appear) |

Full command outputs saved in this ticket folder: `f6-xlsx-diff-check1.txt`/`check2.txt`/`check3.txt`,
`f6-xlsx-mutation-check1.txt`/`check2.txt`/`check3.txt`, `f6-xlsx-test1.txt` (blocked mid-run by an
unrelated concurrent session's in-progress edit to `☁️ply`'s snapshot module — confirmed via `git
status` showing that file actively modified by another session, not touched by this one; polled until
it cleared, per the "concurrent cargo workspace churn" pattern), `f6-xlsx-test2.txt` (first real run,
1 failure — the `""`-owner bug above), `f6-xlsx-test3.txt` (43/43 after the fix),
`f6-xlsx-full-crate-test.txt` (1033/0), `f6-xlsx-policy-run.txt`.

Note: `f6-xlsx-test1.txt`'s failure was NOT an xlsx bug — it was `ply`'s
`STDIO_PLY_DOCUMENT_SCHEMA` import being mid-edit by a concurrent F6 session on another artifact.
Confirmed via `git status --short` showing that file modified, unrelated to this session's changes.
Polled `cargo check --tests` every ~20s until it cleared (3 attempts), then proceeded — did not touch
`ply` at all.

## Files touched (real, live, not reverted)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — doc comment citing the real `DslDiff` derive failure on `XlsxDiff`; hand-rolled
  `impl protocol::DiffCodec for XlsxDiff` (full grammar + generic `enc_triple`/`dec_triple` +
  per-value/per-diff codecs for every nested type, `pub(crate)`-exposed primitives for mutation-side
  reuse); new `handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — doc comment citing the real `DslOps` derive failure on `XlsxMutation`; hand-rolled
  `OpText`/`OpBinary` for `XlsxMutation` (reusing the diff module's `pub(crate)` primitives) replacing
  the previous `serde_json`-based stub codecs; new `tests::op_text_binary_roundtrip_law` test.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-xlsx-diff-check1.txt` through `check3.txt`,
  `f6-xlsx-mutation-check1.txt` through `check3.txt`, `f6-xlsx-test1.txt` through `test3.txt`,
  `f6-xlsx-full-crate-test.txt`, `f6-xlsx-policy-run.txt`.

No shared files touched: `glue.rs`, `📜️script.ts` (incl. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` — 0
stdio entries, unchanged, correctly not used as a shortcut), the `dsl`/`protocol`/`schema` framework
crates, and every other artifact's files were all read-only for this session.

## Deviations from §5's template

1. Added a GENERIC `enc_triple`/`dec_triple` pair (parameterized by per-field closures) instead of
   six bespoke per-type triple encoders, since `NamedTripleDiff<K,D,T>` is reused six times in this
   file (gif/svg had no reusable generic collection type to abstract over). Documented in a doc
   comment on the region.
2. Removed the `.filter(|s| !s.is_empty())` guard the gif/svg precedent uses on every list-splitting
   call site (12 occurrences across both files) — see the "real bug found" section above. This is a
   correctness fix, not a style deviation: the guard was actively wrong for this artifact's real data
   (an empty-string OPC relationship owner key), and `split_top_level`'s own empty-input handling
   makes the guard unnecessary in every case.
3. `f64` encoding (`Number(f64)`) has no precedent in the gif/svg/binary pilots (none of them have a
   float field) — used `f64::to_string()`/`str::parse::<f64>()` directly, noted as std's own
   shortest-round-trip guarantee, no manual bit-pattern encoding.
