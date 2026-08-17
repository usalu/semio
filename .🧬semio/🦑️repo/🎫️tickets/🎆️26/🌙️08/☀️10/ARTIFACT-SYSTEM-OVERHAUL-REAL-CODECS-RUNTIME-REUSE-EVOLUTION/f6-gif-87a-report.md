# F6 — gif 87a: OpText/OpBinary + DiffCodec

**Scope**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/**` only. `89a` was NOT touched (already
piloted/done by the recon step; read as template only). No shared files (`glue.rs`, `📜️script.ts`, SDK
traits, schema/dsl/protocol modules, `🏪️store`) were touched.

## Classification (verified for real via `cargo check`, not trusted from the table blindly)

Per `f6-recon-report.md` §9's procedure, both sides were probed independently by temporarily adding the
derive attribute and reading the real compiler output (kept as evidence in the ticket folder), then
reverted before the real implementation:

- **Diff side (`GifDiff`, `GifImageDiff`) → HAND-ROLL.** Confirmed the recon's row (3b, tri-state).
  Adding `#[derive(dsl::DslDiff)]` to `GifDiff` gives:
  ```
  error[E0277]: the trait bound `std::option::Option<GifColorTable>: DslField` is not satisfied
     --> …/🔺️diff/🦀️component.rs:369:21   (pub gct: Option<Option<GifColorTable>>)
  error[E0277]: the trait bound `GifImagesDiff: DslField` is not satisfied
     --> …/🔺️diff/🦀️component.rs:378:24
  ```
  `GifDiff::gct` and `GifImageDiff::lct` (inside the `images` collection's per-item diff) are both
  `Option<Option<GifColorTable>>` — exactly the recon's documented 3b blocker (`classify_field` peels
  one `Option` layer, leaves `Option<GifColorTable>` which has no `DslField` impl anywhere). Full output:
  `f6-gif87a-diff-check1.txt`.
- **Mutation side (`GifMutation`) → DERIVE (clean), matching gif89a's precedent exactly.** First probe
  (`#[derive(dsl::DslOps)]` added, WITHOUT any nested-struct derives yet) failed as expected — cascading
  requirement, not a structural blocker:
  ```
  error[E0277]: the trait bound `GifSnapshot: DslField` is not satisfied  (SetSnapshot { snapshot })
  error[E0277]: the trait bound `GifColorTable: DslField` is not satisfied  (SetGlobalColorTable { gct })
  error[E0277]: the trait bound `GifImage: DslField` is not satisfied  (InsertImage { image })
  ```
  Full output: `f6-gif87a-mutation-check1-derive-attempt.txt`. After adding `#[derive(dsl::DslRecord)]`
  to `GifRgb`/`GifColorTable`/`GifImage`/`GifSnapshot` (the cascading requirement — none of these types
  carry a data-carrying enum or a tri-state field, since a mutation's `Option<T>` means "the new value",
  never a diff tri-state), `#[derive(dsl::DslOps)]` on `GifMutation` compiled with **zero** gif-87a-related
  errors. Full output: `f6-gif87a-mutation-check2-after-cascade-derives.txt` (remaining errors in that
  file are all in `☁️las`, an unrelated sibling artifact mid-edit by another concurrent session — see
  "Deviations/caveats" below).

This exactly matches gif89a's documented split: **Diff hand-rolled (3b only, no enum anywhere in
87a's `GifImage`/`GifColorTable`/`GifSnapshot` tree — confirmed, 0 `pub enum` in either the snapshot or
diff files besides the derive-ineligible tri-state), Mutation derived clean.**

## What was implemented

### Snapshot (`📸️snapshot/🦀️component.rs`) — cascading `dsl::DslRecord`
Added `#[derive(dsl::DslRecord)]` to `GifRgb`, `GifColorTable`, `GifImage`, `GifSnapshot` (required for
`GifMutation`'s `#[derive(dsl::DslOps)]` to bind `SetSnapshot{snapshot: GifSnapshot}`,
`SetGlobalColorTable{gct: Option<GifColorTable>}`, `InsertImage{image: GifImage}`). Added `#[dsl(block)]`
to `GifImage::lct: Option<GifColorTable>` and `GifSnapshot::gct: Option<GifColorTable>`; `#[dsl(base64)]`
to `GifImage::indices: Vec<u8>` — matching gif89a's exact convention for the equivalent fields.

### Mutation side (`🧬️mutations/🦀️component.rs`) — DERIVE + handcrafted OpText/OpBinary
- `#[derive(dsl::DslOps)]` added to `GifMutation` (12 variants: `NoMutation`, `SetSnapshot`,
  `SetScreenSize`, `SetGlobalColorTable`, `SetBackgroundColorIndex`, `SetPixelAspectRatio`,
  `InsertImage`, `RemoveImage`, `MoveImage`, `SetImageGeometry`, `SetImagePixels`,
  `SetImageInterlace`). `#[dsl(block)]` on `SetSnapshot::snapshot`, `SetGlobalColorTable::gct`,
  `InsertImage::image`; `#[dsl(base64)]` on `SetImagePixels::indices`.
- Replaced the `serde_json`-based `OpText`/`OpBinary` stub with the handcrafted wrapper (P6: `DslOps`
  emits `DslVariants` only, never `OpText`/`OpBinary`) — identical ~15-line/~8-line bodies to gif89a's
  `GifMutation` impl, calling `dsl::parse`/`dsl::print` over `DslVariants::variants()` for text and
  `dsl::variants_binary::encode_op`/`decode_op` for binary.
- Added `op_text_binary_roundtrip_law` test exercising all 12 variants (incl. both `SetGlobalColorTable`
  arms: `Some(...)` and `None`), asserting `print_op`/`parse_op` and `encode_op`/`decode_op` round-trip.

### Diff side (`🔺️diff/🦀️component.rs`) — hand-rolled `DiffCodec`
Copied gif89a's grammar template verbatim (per `f6-recon-report.md` §5/§9), trimmed to 87a's smaller
field set (no GCE-derived fields — no `delay_cs`/`disposal`/`transparent_index`/`user_input`/
`plain_text`, no `loop_count`/`comments`/`app_extensions`):
- Primitives: `hex_encode`/`hex_decode`, `parse_u8`/`parse_u32`/`parse_usize`, `split_top_level`
  (bracket-depth-aware), `strip_brackets`, `encode_option`/`decode_option` (uniform `[0]`/`[1,<v>]` tag).
- Value codecs: `enc_rgb`/`dec_rgb`, `enc_color_table`/`dec_color_table`, `enc_image`/`dec_image` (7
  positional fields: left,top,width,height,interlace,lct,indices).
- Diff value codecs: `enc_image_diff`/`dec_image_diff` (single-letter tags `L`/`T`/`W`/`H`/`I`/`C`/`X` —
  a strict subset of gif89a's `GifFrameDiff` tag set, since 87a has no `D`/`S`/`P`/`U`/`Q`-tagged
  fields), `enc_collection_triple`/`dec_collection_triple` (generic `name{[removed];[modified];[added]}`),
  `enc_images_diff`/`dec_images_diff`.
- Top level: `print_gif_diff`/`parse_gif_diff` (space-separated `name=value` tokens: `width`, `height`,
  `gct`, `bg`, `par`, plus the `images{...}` collection section) and
  `impl protocol::DiffCodec for GifDiff` (`encode_diff`/`decode_diff` = `print_diff().into_bytes()`
  verbatim, the same simplification `WriterDiff`/gif89a use).
- Added `diff_codec_text_binary_roundtrip_law` test exercising both tri-states (`gct` at top level,
  `lct` inside a modified image) and the `images` collection triple (removed/modified/added) via real
  `between()` results in both directions, plus the empty-diff case.
- Added a doc comment on `GifImageDiff` citing the confirmed compile error (mirrors the doc comment
  convention on gif89a's `GifFrameDiff`).

## Real captured output

Captured by temporarily adding `eprintln!` to the two roundtrip-law tests, running them with
`--nocapture`, and removing the debug lines afterward (not hand-transcribed).

`print_diff` on the two real `between()` results the test exercises (`a`↔`b`, both directions —
`gct` Some→None and None→Some, `images` collection modified+added in one direction, removed+modified
in the other):
```
width=20 height=16 gct=[0] images{[];[0:[I:1,C:[0]],1:[W:3,H:3,C:[1,[0,[[6,6,6],[6,6,6]]]],X:000000000000000000]];[2:[0,0,3,3,0,[1,[0,[[7,7,7],[7,7,7]]]],000000000000000000]]}
width=10 height=8 gct=[1,[0,[[1,2,3],[1,2,3]]]] images{[2];[0:[I:0,C:[1,[0,[[1,1,1],[1,1,1]]]]],1:[W:2,H:2,C:[1,[0,[[2,2,2],[2,2,2]]]],X:00000000]];[]}
```
`print_op` for every variant in `op_text_binary_roundtrip_law` (12 distinct calls, 13 lines incl. both
`SetGlobalColorTable` arms):
```
no-mutation
set-snapshot snapshot { schema=stdio.gif width=2 height=2 background-color-index=9 pixel-aspect-ratio=0 images=[ left=0 top=0 width=2 height=2 interlace=false indices="AAEBAA==" lct { sorted=false colors=[ r=1 g=1 b=1 r=1 g=1 b=1 ] } ... ] }
set-screen-size width=10 height=10
set-global-color-table gct { sorted=true colors=[ r=0 g=0 b=0 r=0 g=0 b=0 ] }
set-global-color-table
set-background-color-index index=5
set-pixel-aspect-ratio ratio=3
insert-image index=1 image { left=0 top=0 width=2 height=2 interlace=false indices="AAEBAA==" lct { sorted=false colors=[ r=9 g=9 b=9 r=9 g=9 b=9 ] } }
remove-image index=1
move-image from=0 to=2
set-image-geometry index=0 left=1 top=1 width=2 height=2
set-image-pixels index=0 indices="AQEBAQ=="
set-image-interlace index=0 interlace=true
```
Note the `OpText` grammar (`dsl::print`/`dsl::parse`, block-formatted with spaces/braces) is a
genuinely different, more verbose surface syntax than the hand-rolled `DiffCodec` grammar
(compact `[...]`/hex) — expected, since the Mutation side is DERIVED (`dsl::DslOps` +
`dsl::variants_binary`) while the Diff side is HAND-ROLLED (custom `print_gif_diff`/`parse_gif_diff`);
the two codecs are unrelated implementations satisfying two different traits.

## Verification (all real, this session)

| Check | Result |
|---|---|
| `cargo check -p semio-s-plugin-stdio --lib` | 0 errors, gif-87a-related warnings only pre-existing/unrelated (composer lifetime lint, unused engine fields) |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::gif::standards::v87a"` | **27/27 passed**, 0 failed — full output `f6-gif87a-scoped-test-final.txt` |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate, final run) | **1047 passed, 0 failed** — full output `f6-gif87a-full-crate-test-final.txt` |

Note: mid-session, two whole-crate runs briefly showed failures/compile errors unrelated to this
ticket's scope (`🟪️stl` runtime test failures, `🎒️zip`/`☁️las` compile errors) caused by other concurrent
F6 sibling sessions actively editing those artifacts in the shared live tree at that moment (confirmed
by file paths in the error output — none under `🎞️gif/🏅️standards/🔖️87a/**`). Polled with `cargo check`
until those sessions' edits landed cleanly, then re-ran; the final whole-crate run above is 100% green.

## Deviations / caveats

1. **Transient unrelated failures mid-session, all resolved by the final run.** At different points
   this session observed: (a) `🎒️zip`/`☁️las` `cargo check` compile errors (`DslField is not satisfied`
   for `ZipEntriesDiff`/`ZipCompressionMethod`/`LasPointsDiff` etc. — other sessions' in-progress
   `DslDiff`/`DslRecord` derive attempts, not yet cascaded to every nested type), and (b) a `🟪️stl`
   whole-crate test run with 2 runtime failures (`parse_diff`/`parse_op` tuple-arity mismatches in
   `artifacts::stl::standards::v_ascii::…mutations::…`), and (c) a further round of `stl` compile
   errors (`DslField`/`DiffCodec` not satisfied) as that session iterated. None of these ever touched
   `🎞️gif/🏅️standards/🔖️87a/**` — confirmed by grepping the error output for file paths each time. This
   is the expected "Concurrent Cargo Workspace Churn" pattern for a shared live tree with multiple F6
   fan-out sessions editing simultaneously (per this ticket's rules: no worktrees, live shared tree).
   Polled `cargo check` until clean (`f6-recon-report.md`'s and this session's own procedure), then
   re-ran the whole-crate suite — final result is 1047/0, fully green, not just "my tests pass while
   others fail". No fix was applied to `stl`/`zip`/`las` (outside this ticket's ownership boundary).
2. **No grammar deviation from gif89a's template** — 87a's grammar is a strict subset (fewer top-level
   tokens, fewer `GifImageDiff` tags, one collection triple instead of three) but uses the identical
   primitives, tag conventions, and `encode_diff = print_diff().into_bytes()` simplification.
3. Per `f6-recon-report.md` §6, no schema-id work was needed — `#[artifact_schema(id = "s.stdio.gif.diff")]`
   was already correct on `GifDiff` from a prior wave, untouched here.
4. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) was NOT touched — not read as part of this
   session (no need: this artifact's `GifDiff` file now contains a real `impl protocol::DiffCodec`,
   which is what that policy check greps for).

## Files touched (real, live, not reverted)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  — `dsl::DslRecord` on `GifRgb`/`GifColorTable`/`GifImage`/`GifSnapshot`; `#[dsl(block)]` on
  `GifImage::lct`/`GifSnapshot::gct`; `#[dsl(base64)]` on `GifImage::indices`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — hand-rolled `impl protocol::DiffCodec for GifDiff` (full grammar + helper functions, `GifRgb`
  import added), doc comment on `GifImageDiff` citing the confirmed compile error, +
  `diff_codec_text_binary_roundtrip_law` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/🔖️87a/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — `dsl::DslOps` on `GifMutation` (derived clean) with `#[dsl(block)]`/`#[dsl(base64)]` field
  attributes, handcrafted `OpText`/`OpBinary` replacing the `serde_json` stub, +
  `op_text_binary_roundtrip_law` test.
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-gif87a-diff-check1.txt`,
  `f6-gif87a-mutation-check1-derive-attempt.txt`, `f6-gif87a-mutation-check2-after-cascade-derives.txt`,
  `f6-gif87a-check-final.txt`, `f6-gif87a-scoped-test-final.txt`, `f6-gif87a-full-crate-test-final.txt`.
- This report: `f6-gif-87a-report.md`.

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework crates,
`🏪️store`, and gif's `89a` standard were all untouched/read-only for this session.
