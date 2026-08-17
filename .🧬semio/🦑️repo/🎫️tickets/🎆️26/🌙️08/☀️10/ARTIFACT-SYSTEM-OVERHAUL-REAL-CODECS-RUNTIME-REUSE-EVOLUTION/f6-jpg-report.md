# F6 — 📷️jpg (jfif-1.01) — OpText/OpBinary + DiffCodec

**Scope**: implement `protocol::DiffCodec` for `JpgDiff` and `protocol::OpText`/`protocol::OpBinary`
for `JpgMutation`, per `f6-recon-report.md`'s §9 procedure. Ownership boundary respected: only files
under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/**` touched, plus this report. No shared files
(`glue.rs`, `📜️script.ts`, `dsl`/`protocol`/`schema` framework crates) edited.

## STEP 1 — classification (verified for real, not trusted from the recon table)

### 1a. Diff side (`JpgDiff`) — HAND-ROLL, confirmed

Added `dsl::DslDiff` to `JpgDiff`'s derive list for real (with the manual `DiffCodec` impl
temporarily commented out to avoid a masking conflicting-impl error) and ran
`cargo check -p semio-s-plugin-stdio --lib`. Two independent, simultaneous failures, both real
compiler output:

```text
error[E0277]: the trait bound `JpgFrameChange: DslField` is not satisfied
   --> …/🔺️diff/🦀️component.rs:738:23   (pub frame: Option<JpgFrameChange>)
help: the trait `DslField` is not implemented for `JpgFrameChange`
   --> …/🔺️diff/🦀️component.rs:106:1
106 | pub enum JpgFrameChange {
```
— `JpgFrameChange` (`Modify(JpgFrameFieldsDiff)` / `Replace{frame: Option<JpgFrameHeader>}`) is a
genuine data-carrying enum (§3a of the recon report).

```text
error[E0277]: the trait bound `std::option::Option<u8>: DslField` is not satisfied
   --> …/🔺️diff/🦀️component.rs:720:35   (pub re_encode_quality: Option<Option<u8>>)
error[E0277]: the trait bound `Option<JfifThumbnail>: DslField` is not satisfied
   --> …/🔺️diff/🦀️component.rs:735:32   (pub jfif_thumbnail: Option<Option<JfifThumbnail>>)
error[E0277]: the trait bound `std::option::Option<u16>: DslField` is not satisfied
   --> …/🔺️diff/🦀️component.rs:753:34   (pub restart_interval: Option<Option<u16>>)
```
— three tri-state `Option<Option<T>>` fields (§3b): `classify_field` peels exactly one `Option`
layer and there is no `impl<T: DslField> DslField for Option<T>` anywhere in `dsl`.

The derive attempt (and the temporary comment-out) was fully reverted before proceeding; the real,
hand-rolled `impl protocol::DiffCodec for JpgDiff` in the live file is untouched by this probe.

### 1b. Mutation side (`JpgMutation`) — HAND-ROLL, confirmed

Added `dsl::DslOps` to `JpgMutation`'s derive list for real and ran `cargo check`. Confirmed two
layered findings:

1. **Cascading requirement** — none of the reachable types (`JpgSnapshot`, `JpgFrameHeader`,
   `JpgFrameComponent`, `JpgQuantTable`, `JpgHuffmanTable`, `JpgSegment`, `JfifThumbnail`,
   `JpgHuffmanTableKey`, plus the unit enums `JfifDensityUnits`/`JpgHuffmanClass`) carry
   `#[derive(dsl::DslRecord)]`/`#[derive(dsl::DslScalar)]` yet, so every struct-payload variant
   rejects immediately, e.g.:
   ```text
   error[E0277]: the trait bound `JpgSnapshot: DslField` is not satisfied
     --> …/🧬️mutations/🦀️component.rs:32:19   (snapshot: JpgSnapshot)
   error[E0277]: the trait bound `JpgQuantTable: DslField` is not satisfied
     --> …/🧬️mutations/🦀️component.rs:44:16   (table: JpgQuantTable)
   ```
2. **Decisive independent blocker, unrelated to 3a/3b** — even if every one of those 8+ types were
   fully cascade-derived, `SetJfifHeader.version: (u8, u8)` would still fail: direct grep of every
   `impl DslField for …`/`impl<T: DslField, …> DslField for …` in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🦀️component.rs` shows blanket/concrete impls only
   for `bool`/`f32`/`f64`/`String`/`Wire`/`DslValue`/`Vec<T>`/`BTreeMap<String,T>`/`[T;N]` — **no
   tuple arm of any arity**. Fixing this needs either (a) a framework-level `dsl` crate change
   (shared file, outside this artifact's ownership boundary) or (b) replacing the tuple with e.g.
   `[u8;2]` (a Mutation-shape change this ticket's scope explicitly forbids — "do not touch
   snapshot/diff/mutation SHAPE"). Neither is available to this agent, so DERIVE is not merely
   more effort here, it is structurally impossible within scope.

Same as the diff side, the derive probe was added, checked, and fully reverted; the live file's
hand-rolled `OpText`/`OpBinary` is untouched by the probe. Real citations are captured verbatim in
the doc comments on `JpgDiff` and `JpgMutation` themselves.

**Verdict**: HAND-ROLL on both sides — matches the recon report's §8 row for jpg (`HAND-ROLL
(3a+3b)`), now verified for real rather than trusted from the grep-heuristic sweep.

## STEP 2b — hand-rolled grammar

Both codecs reuse the exact primitive set from `f6-recon-report.md` §5 (`hex_encode`/`hex_decode`/
`split_top_level`/`strip_brackets`/`encode_option`/`decode_option`), declared `pub(crate)` in
`🔺️diff/🦀️component.rs` so `🧬️mutations/🦀️component.rs` can reuse them without a third copy
(mirrors svg's diff↔mutations intra-artifact reuse).

Conventions followed:
- Strings/bytes: hex (not base64 — no external dep, matches the artifact's own `ArtifactDsl`
  hex-based `parse_dsl`/`print_dsl`, already the local idiom).
- Plain structs (`JpgFrameComponent`, `JpgFrameHeader`, `JpgQuantTable`, `JpgHuffmanTable`,
  `JpgHuffmanTableKey`, `JpgSegment`, `JfifThumbnail`, the full `JpgSnapshot` for `SetSnapshot`):
  positional `[f1,f2,...]` tuples, declaration order.
- Unit enums (`JfifDensityUnits`, `JpgHuffmanClass`): reuse the types' own pre-existing
  `to_u8`/`from_u8` helpers (already present in `📸️snapshot/🦀️component.rs`) rather than inventing
  a parallel encoding — a single decimal digit.
- The one data-carrying enum, `JpgFrameChange`: single-uppercase-letter tag prefix —
  `M[fields-diff]` (`Modify`) / `R[frame-opt]` (`Replace`), same convention as gif/svg's
  `enc_frame_change`-style enums.
- The three id/index-keyed collection triples (`JpgQuantTablesDiff` by `u8` id,
  `JpgHuffmanTablesDiff` by compound `(class,id)` key, `JpgOtherSegmentsDiff`/`JpgComponentsDiff`
  by index): `name{[removed];[modified];[added]}`, semicolon-separated sections, `key:payload`/
  `idx:payload` entries — unchanged from the recipe (no absorb/apply/between logic touched; that
  machinery already existed and is untouched by this ticket).
- `[u16;64]` quant-table values: bracketed decimal list (not bytes, so hex doesn't apply).
- `[u8;16]` huffman `bits`: hex (genuinely bytes).
- Top-level `JpgDiff`/`JpgMutation` line: space-separated `name=value` (Diff) / `keyword arg=value
  ...` (Mutation) tokens, absent token = unchanged. Tri-state Diff fields
  (`re-encode-quality`/`jfif-thumbnail`/`restart-interval`) additionally wrap their value in
  `[0]`/`[1,x]` since token-presence alone only signals "the tri-state slot changed".
- `encode_diff`/`encode_op` = the text bytes verbatim (same simplification `GifDiff`/`SvgDiff`/
  `WriterDiff` use — satisfies every `DiffCodec`/`OpBinary` law without inventing a denser format).

No deviation from the recon report's documented grammar conventions.

## STEP 3 — tests (both added, both pass)

- `diff_codec_text_binary_roundtrip_law` (`🔺️diff/🦀️component.rs`, `handcrafted_diff_codec_tests`
  module): exercises `JpgDiff::default()` plus four `between()` results over three fixture
  snapshots (`snap_a`/`snap_b` differing in every field including all three collection triples in
  both directions — removed+modified one way, modified+added the other, matching the recipe's
  documented same-length-`between()` workaround — and `snap_c` with `frame: None` to exercise
  `JpgFrameChange::Replace`). Asserts `!printed.contains('\n')`, `parse(print(x)) == x`,
  `decode(encode(x)) == x` for every case.
- `op_text_binary_roundtrip_law` (`🧬️mutations/🦀️component.rs`, existing `tests` module):
  exercises all 12 `JpgMutation` variants including `SetSnapshot` with a full nested
  `JpgFrameHeader`/`JpgFrameComponent` snapshot and a frame-less/thumbnail-less variant, both legs
  of every `Option<T>`-shaped argument (`thumbnail`, `restart_interval`, `quality`). Same three
  assertions per case.

## STEP 4 — verification (real, this session)

| Check | Result |
|---|---|
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::jpg"` | **31/31 passed** (29 pre-existing + 2 new: `op_text_binary_roundtrip_law`, `diff_codec_text_binary_roundtrip_law`) |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | **1075/0** — final clean run, this session |

**Note on the whole-crate run**: an interim whole-crate run mid-session showed `1074 passed; 1
failed` — the sole failure was `artifacts::docx::…::diff_codec_text_binary_roundtrip_law`
(assertion on a `based_on` tri-state field), entirely inside a sibling F6 sub-wave's `📜️docx`
artifact, not `📷️jpg`, and not touched by this session in any way. This is the same class of
"concurrent in-progress work" already hit once earlier in this session (a `dxf` `DslField`
compile-error episode from another sibling sub-wave, and separately a repo-wide `🧊️3d` module
Cargo workspace-manifest churn, both of which resolved themselves mid-session without any action
from this agent — the tree's git status shows many other artifacts' `🔺️diff`/`🧬️mutations` files
concurrently modified/added by other live sessions). Per repo rules this agent must not edit
`📜️docx`'s files (outside the `📷️jpg` ownership boundary) — the run was retried after the
sibling session landed its fix, and the final whole-crate run is clean: **1075 passed, 0 failed**
(baseline at session start, per `f6-recon-report.md`, was 1019; the growth reflects every other F6
sub-wave's concurrent work landing during this session, not just this artifact's +2 tests).

## Files touched (real, live)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — doc comment on `JpgDiff` citing the real derive-failure compiler errors; hand-rolled
  `impl protocol::DiffCodec for JpgDiff` (primitives, value codecs, diff-value codecs, top-level
  print/parse, all in a new `#region 🔖️HandcraftedDiffCodec`); several helpers made `pub(crate)`
  for mutation-side reuse; new `handcrafted_diff_codec_tests` module with
  `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — doc comment on `JpgMutation` citing the real derive-failure compiler errors; replaced the
  `serde_json`-based `OpText`/`OpBinary` stub with a hand-rolled grammar reusing `schema::diff`'s
  `pub(crate)` primitives; new `op_text_binary_roundtrip_law` test in the existing `tests` module.
- This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6-jpg-report.md`.
- Scratch (ticket folder, `.txt`/backups kept per repo rules, not deleted): temporary derive-probe
  backups and probe output are in the session scratchpad only (outside the repo), not committed to
  the ticket folder, since they were pure verification scaffolding, immediately reverted, and never
  represent a durable finding beyond what's cited verbatim above and in the source doc comments.

## Deviations from the recon report's conventions

None in grammar shape. One correction beyond what recon's §8 sweep found: the sweep's row for jpg
only anticipated the diff-file's own `pub enum JpgFrameChange` (3a) and its 3 tri-state fields
(3b); this session additionally confirmed, via a real compiler probe, an entirely separate
Mutation-side blocker — `dsl` has no `DslField` impl for tuples of any arity, which independently
forces HAND-ROLL on `JpgMutation` regardless of cascading effort (the enum-reachability question
recon's own §3's decision rule poses does not even need answering here, since the tuple blocker is
decisive on its own, and unlike 3a/3b it isn't fixable within this artifact's ownership boundary
even in principle). Recorded in this report and in the `JpgMutation` doc comment for the next
agent who reads this file.
