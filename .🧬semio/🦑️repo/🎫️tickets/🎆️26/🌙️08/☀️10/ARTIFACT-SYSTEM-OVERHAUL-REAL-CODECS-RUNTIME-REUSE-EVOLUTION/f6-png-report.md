# F6 — 📷️png 1.2 — OpText/OpBinary/DiffCodec Report

**Artifact**: `📷️png`, standard `1.2`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/`.

## Classification — verified for real, not trusted from the recon table

Followed `f6-recon-report.md` §9's procedure literally: added the derive to each side, ran a real
`cargo check -p semio-s-plugin-stdio --lib`, captured the actual compiler errors, then reverted the
experimental derive attempt before writing the hand-rolled implementation.

### Diff side — HAND-ROLL (confirmed, both 3a and 3b)

- **3b (tri-state)**: `PngDiff` has **8** top-level tri-state `Option<Option<T>>` fields — `plte`,
  `trns`, `gama`, `chrm`, `srgb`, `phys`, `time`, `bkgd`. (The recon table's row for png guessed
  **12**; the real, grep-and-compiler-confirmed count is **8** — a real deviation from the recon
  guess, noted per its own "verify, don't trust" instruction. The guess likely over-counted by
  including non-tri-state `Option<T>` ancillary fields or double-counting occurrences outside the
  struct body.) Real captured error:
  ```
  error[E0277]: the trait bound `std::option::Option<PngChromaticities>: DslField` is not satisfied
     --> …/🔺️diff/🦀️component.rs:833:22  (pub chrm: Option<Option<PngChromaticities>>)
  ```
- **3a (data-carrying enum)**: `PngTransparency` (`Indexed{alpha}`/`Grayscale{gray}`/`Rgb{r,g,b}`)
  and `PngBackground` (`Grayscale{gray}`/`Rgb{r,g,b}`/`Indexed{index}`) are both genuine
  data-carrying enums reachable through `trns`/`bkgd`. Real captured error:
  ```
  error[E0277]: the trait bound `Option<PngTransparency>: DslField` is not satisfied
     --> …/🔺️diff/🦀️component.rs:827:22  (pub trns: Option<Option<PngTransparency>>)
  ```
  (Both failures fire simultaneously — this is the same "commonly hits both" case the recon report
  flags in §3.) `PngChunkMarker` (used inside `chunk_order`'s triple entries) is also a
  data-carrying enum (`Text{index}`/`Unknown{index}` alongside 11 unit variants), reachable via
  `PngChunkOrderModified`/`PngChunkOrderAdded`.

### Mutation side — HAND-ROLL (confirmed, 3a)

Real `cargo check` with `#[derive(dsl::DslOps)]` added to `PngMutation`: **42** `DslField`-not-satisfied
errors. Root cause: `SetTransparency{trns: Option<PngTransparency>}` and
`SetBackground{bkgd: Option<PngBackground>}` carry the data-carrying enums directly as variant
fields; `SetSnapshot{snapshot: PngSnapshot}` carries them transitively through
`PngSnapshot.trns`/`.bkgd`. Same root cause as the diff side's 3a blocker — confirms the recon
report's rule that `SetSnapshot` always inherits whatever blocks the Snapshot tree.

**Both sides land on HAND-ROLL** — this matches (and sharpens) the recon table's row 20 verdict.
Real compiler output for both experiments is kept in the ticket folder:
`f6-png-diff-derive-check.txt` (diff-side attempt), `f6-png-mutation-derive-check.txt`
(mutation-side attempt), plus `f6-png-baseline-check.txt` (pre-change clean baseline) and
`f6-png-check2.txt`/`f6-png-full-crate-test.txt` (post-change verification runs).

## What was implemented

### `impl protocol::DiffCodec for PngDiff`
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
— hand-rolled, following the gif89a/svg template from `f6-recon-report.md` §5 exactly:

- Grammar primitives (`hex_encode`/`hex_decode`/`enc_str`/`dec_str`/`split_top_level`/
  `strip_brackets`/`encode_option`/`decode_option`/`enc_list`/`dec_list`) — all `pub(crate)` so
  `PngMutation`'s hand-rolled `OpText`/`OpBinary` reuses them instead of duplicating (same
  intra-artifact reuse svg's diff/mutations pair uses).
- Value codecs for every type reachable from `PngDiff`: `PngColorType`/`PngSrgbIntent` (unit enums,
  reuse their own pre-existing `to_u8`/`from_u8`), `PngRgb`/`PngChromaticities`/`PngPhysicalDims`/
  `PngTimestamp`/`PngTextChunk`/`PngChunk` (positional `[f1,f2,...]` tuples), `PngTransparency`/
  `PngBackground` (single-uppercase-letter tag + bracketed payload — `I`/`G`/`R`), `PngChunkMarker`
  (literal chunk-name tag — `IHDR`/`PLTE`/.../`TEXT[idx]`/`UNKN[idx]`, chosen self-documenting since
  it appears inside `chunk_order` triples).
- Collection-triple codecs for `PngPlteDiff`/`PngTextChunksDiff`/`PngChunkOrderDiff`/
  `PngUnknownChunksDiff`, reusing a shared `enc_triple_body`/`dec_triple_body` pair.
- Top-level grammar: space-separated `name=value` tokens for scalar/tri-state fields;
  `name{[removed];[modified];[added]}` (no `=`) for the three non-tri-state collection triples;
  `plte`'s OUTER tri-state wraps the same triple-body shape bare inside `encode_option`'s
  `[0]`/`[1,<T>]` tag — verified the nested-bracket disambiguation works (both via the roundtrip
  test and by construction: `split_top_level` only tracks `[`/`]` depth).
- `encode_diff`/`decode_diff` = the text bytes verbatim (same simplification `GifDiff`/`SvgDiff`/
  `WriterDiff` use).

### `impl protocol::OpText`/`OpBinary for PngMutation`
`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
— replaced the old `serde_json`-based stub with a real hand-rolled grammar: `keyword arg=value ...`
(space-separated, matches the derive's own handcrafted-wrapper convention), one match arm per
variant, reusing every value codec from the diff file. `SetSnapshot`'s whole-`PngSnapshot` payload
is a single positional 18-field tuple (`enc_png_snapshot`/`dec_png_snapshot`) built from the same
per-field codecs. `encode_op`/`decode_op` = text bytes verbatim.

## Tests added (mandatory, both required)

- `diff_codec_text_binary_roundtrip_law` (new `handcrafted_diff_codec_tests` module in
  `🔺️diff/🦀️component.rs`): exercises `PngDiff::default()`, `between(a,b)`, `between(b,a)`,
  `between(a,empty)`, `between(empty,a)` using two fixture snapshots (`snap_a`/`snap_b`) that differ
  in every mutable field — every scalar, every one of the 8 tri-states in both
  `Some(None)`/`Some(Some(_))` directions (incl. `plte`'s tri-state-wrapping-a-triple shape), and
  every collection triple's removed/modified/added arms. Asserts `print_diff`/`parse_diff` and
  `encode_diff`/`decode_diff` both round-trip and that `print_diff` never contains `\n`.
- `op_text_binary_roundtrip_law` (added to the existing `tests` module in
  `🧬️mutations/🦀️component.rs`): reuses the file's own `all_variants(&base)` helper (already
  covering every `PngMutation` variant incl. every ancillary Setter's `Some(_)` payload) plus two
  extra `SetSnapshot { snapshot: sweep_a() }` / `sweep_b()` cases so the whole-snapshot codec's
  `Some`/`None` branches for every one of its 8 optional fields, and its 3 list fields, are also
  covered. Same round-trip + no-newline assertions.

## Verification (real, this session)

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` with `dsl::DslDiff` added to `PngDiff` (experiment) | Real `DslField` compile errors (3a+3b), captured, reverted |
| `cargo check -p semio-s-plugin-stdio --lib` with `dsl::DslOps` added to `PngMutation` (experiment) | 42 real `DslField` compile errors, captured, reverted |
| `cargo check -p semio-s-plugin-stdio --lib` (baseline, before any png change) | Clean, 219 warnings, 0 errors |
| `cargo check -p semio-s-plugin-stdio --lib` (after the hand-rolled implementation) | Clean, 0 errors — one benign pre-existing-pattern "unnecessary qualification" warning on `impl protocol::OpText for PngMutation` (same pattern already present in csv/dxf/bmp's mutation files, not fixed for consistency) |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::png"` | **24/24 passed, 0 failed** (22 pre-existing + 2 new law tests) |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1060/0 → 1061/0 passed, 0 failed** across two final confirmation runs (count went up between runs from other sessions' concurrent landings, never down — see `f6-png-full-crate-test-final.txt`) |

**Whole-crate run note**: the workspace has multiple concurrent live sessions (per repo rules).
During this session's earlier whole-crate verification attempts, sibling in-progress F6 sub-waves on
`pptx`, `pdf`, and `gltf` (confirmed via `git status` showing those files as
currently-modified-but-uncommitted by other sessions) intermittently broke the FULL crate `--lib`
test build with errors entirely inside those other artifacts' files (`PptxMutation::print_op`/
`parse_op` missing, `PdfMutation::parse_op` missing, `GltfDiff` test fixture type errors) — zero
errors ever appeared inside any `png` file across any run. This is the documented "Concurrent Cargo
Workspace Churn" pattern (other sessions' in-progress edits, not a defect in this work). Polled until
those sibling sessions' edits stabilized, then re-ran the full suite: **1060 passed, 0 failed, 0
ignored** — clean, matching the "count only goes up, never down" requirement (baseline before this
session's start, per the ticket's stated prior baseline, was 1033+; the actual pre-png-change
baseline observed this session — before this session's own edits, with siblings' concurrent edits
already in flight — is not directly comparable since those sibling artifacts' own work was also
landing during this window; what matters is that with all sessions' work applied, 1060/0 is clean).
Intermediate/earlier capture kept for the record as `f6-png-full-crate-test.txt`; the authoritative
final capture is `f6-png-full-crate-test-final.txt`.

## Deviations from the recon report

1. **Tri-state field count**: recon table row 20 guessed 12; real count is **8** (`plte`, `trns`,
   `gama`, `chrm`, `srgb`, `phys`, `time`, `bkgd`). Verified by direct grep of the `PngDiff` struct
   body and cross-checked against the real compiler error list.
2. **`PngColorType`/`PngSrgbIntent` already had `to_u8`/`from_u8` helpers** on the type itself
   (pre-existing, not part of this ticket's scope) — reused them directly in the hand-rolled
   grammar instead of writing a fresh ordinal mapping, saving a little code versus a from-scratch
   enum encoder.
3. **`PngChunkMarker` needed its own encoder** not explicitly anticipated by the recon's worked
   examples (gif/svg's hand-rolled codecs don't have an analogous "mixed unit+data enum used as a
   collection-triple payload" case) — modeled it as a literal chunk-name tag
   (`IHDR`/.../`TEXT[idx]`/`UNKN[idx]`) rather than a single-letter tag, since the marker names are
   already short, spec-standard, and self-documenting inside `chunk_order{...}` output.

## Ownership / repo-rules compliance

- Touched only `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  and `.../🧬️mutations/🦀️component.rs` (plus this report and ticket-folder `.txt` scratch files).
- No edits to `📦️glue.rs`, `📜️script.ts`, SDK traits, `schema`/`dsl`/`protocol` modules, or `🏪️store`.
- `POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) — not touched, not added to.
- No `git commit`/`stash`/`checkout`/`reset`/worktree commands used anywhere in this session.
- All temp/scratch files placed in the ticket folder as `.txt`.
- No `ticket_open`/`ticket_close`/`ticket_reopen` calls made.
