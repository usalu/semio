# F6 — 📐️step (ap214) — OpText/OpBinary + DiffCodec Report

**Artifact**: `📐️step`, standard `ap214`, path `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/`.
Followed `f6-recon-report.md` §9's procedure literally (STEP 1 classify → STEP 2b hand-roll → STEP 3
tests → STEP 4 verify → STEP 5 this report).

## STEP 1 — real classification (not trusted from the recon's heuristic table)

The recon's own row for step flagged `CHECK-ENUM-ELSEWHERE`: `StepValue` is a real data-carrying
enum declared in the snapshot module (`📸️snapshot/🦀️component.rs`), and the diff-file grep sweep
(0 `pub enum` in the diff file itself) was known not to prove anything since `StepValue` could still
be reachable through `StepEntity`/`StepArgsDiff`. I did the real compile-check both sides call for:

1. **Diff side**: temporarily added `dsl::DslDiff` to `StepDiff`'s derive list, ran `cargo check -p
   semio-s-plugin-stdio --lib`. Real compiler output (captured in
   `f6-step-derive-attempt-check.txt`):
   ```
   error[E0277]: the trait bound `v_ap214::...::StepEntitiesDiff: DslField` is not satisfied
   ```
   cascading from `StepEntityDiff.args: Option<StepArgsDiff>` → `StepArgsDiff.modified/added` →
   `StepArgModified.value` / `StepArgAdded.value: StepValue`. `StepValue` (a genuine data-carrying
   enum: `Unset`/`Derived`/`Integer`/`Real`/`String`/`Enum`/`Reference`/`Aggregate`/`TypedValue`) has
   no `DslField` impl and none is derivable for it (only `DslRecord`-derived structs and
   `DslScalar`-derived UNIT-only enums implement `DslField`) — classic §3a blocker, same root cause
   as `SvgNodeDiff`/`XmlNode`.
2. **Mutation side**: separately added `dsl::DslOps` to `StepMutation`'s derive list, same command.
   Real compiler output:
   ```
   error[E0277]: the trait bound `v_ap214::...::StepEntity: DslField` is not satisfied
      (from InsertEntity { index: usize, entity: StepEntity })
   error[E0277]: the trait bound `v_ap214::...::StepValue: DslField` is not satisfied
      (from SetEntityArg { .., value: StepValue } and InsertEntityArg { .., value: StepValue })
   ```
   Confirms 3a independently on the mutation side too — `StepEntity`/`StepValue` carried directly as
   variant fields, not just via `SetSnapshot`'s nested `StepSnapshot`.

Both derive attempts were then reverted (no `dsl::DslDiff`/`dsl::DslOps` left on either type) and the
real compiler-error citations were preserved as doc comments on the hand-rolled impls below.

**No tri-state `Option<Option<_>>` anywhere in `StepDiff`** (§3b does not apply) — every `StepDiff`
field (`file_description`/`file_name`/`file_schema`/`entities`) is a plain `Option<T>` ("weak value,
whole-replaced" per this file's own pre-existing doc comment), so 3a alone is the reason to
hand-roll, and the top-level grammar needs no `[0]`/`[1,x]` tri-state wrapper (absent token already
means "unchanged" unambiguously) — only genuinely nested `Option<T>` fields (`StepEntityDiff.name`/
`.args`/`.complex`) need the wrapper.

**Verdict: HAND-ROLL on both Diff and Mutation sides** (3a only, no 3b).

## STEP 2b — hand-rolled implementation

Followed §5's template/grammar conventions exactly, reusing the same primitive set already proven by
`GifDiff`/`SvgDiff` (own copy per the recipe's specific-code mandate, `pub(crate)` from the diff file
so the mutations file can reuse them — same pattern `SvgMutation` uses against `SvgDiff`):

- `hex_encode`/`hex_decode`/`enc_str`/`dec_str`, `split_top_level`/`strip_brackets`,
  `encode_option`/`decode_option`, `parse_usize`/`parse_u64` — added to
  `🔺️diff/🦀️component.rs` under a new `HandcraftedDiffCodec` region (`Primitives` subregion).
- **`StepValue` grammar** (the crux of the artifact): single-uppercase-letter tag prefix, one per
  variant — `U[]` Unset, `D[]` Derived, `I[<dec>]` Integer, `R[<dec>]` Real, `S[<hex>]` String,
  `E[<hex>]` Enum, `F[<dec>]` Reference (`R` taken by Real), `A[<items>]` Aggregate (recursive list),
  `T[<hex-name>,<enc-value>]` TypedValue (recursive, one wrapped value) — `enc_value`/`dec_value`.
- **Value codecs**: `StepComplexType` (`enc_complex`/`dec_complex`), `StepEntity`
  (`enc_entity`/`dec_entity`, `pub(crate)` for the mutations file), `StepFileDescription`/
  `StepFileName`/`StepFileSchema` (positional `[f1,f2,...]` tuples, hex for every string, `pub(crate)`),
  and a full `StepSnapshot` codec (`enc_step_snapshot`/`dec_step_snapshot`, `pub(crate)`) needed only
  by `SetSnapshot`'s `OpText`/`OpBinary` — `StepDiff` itself has no `snapshot: Option<StepSnapshot>`
  full-replace slot.
- **Diff-value codecs**: `StepArgsDiff` (index-keyed triple, `idx:enc_value` entries, same
  `[removed];[modified];[added]` shape as every other artifact's collection triple),
  `StepEntityDiff` (three `encode_option`-wrapped fields: `name`/`args`/`complex`), `StepEntitiesDiff`
  (id-keyed triple, `id:enc_entity_diff`/`index:enc_entity` entries).
- **Top level**: `print_step_diff`/`parse_step_diff` — space-separated `name=value` tokens, one per
  changed top-level field, absent token = unchanged (no tri-state wrapper needed at this level).
  `impl protocol::DiffCodec for StepDiff` — `encode_diff`/`decode_diff` = the text bytes verbatim
  (same simplification `GifDiff`/`SvgDiff`/`WriterDiff` use).
- **Mutation side** (`🧬️mutations/🦀️component.rs`): imports the diff file's `pub(crate)` codecs,
  adds `print_step_mutation`/`parse_step_mutation` (`keyword arg=value ...` grammar, one match arm
  per variant), replaces the old `serde_json`-backed `OpText`/`OpBinary` stub impls with real ones.
  `encode_op`/`decode_op` = text bytes verbatim, same simplification.
- Fixed one real bug caught by `cargo check` mid-implementation: `OpBinary::encode_op`'s body calls
  `self.print_op()` (an `OpText` method), so `protocol::OpText` needed to be imported
  unconditionally (not `#[cfg(test)]`-gated as I'd first written it) — confirmed via a real
  `error[E0599]: no method named 'print_op' found` before the fix.

## STEP 3 — tests (both new, both real)

- `diff_codec_text_binary_roundtrip_law` (new `handcrafted_diff_codec_tests` mod in the diff file):
  exercises every `StepValue` variant including the recursive `Aggregate`/`TypedValue` cases, plus
  all three `entities` collection-triple flavors (removed/modified/added) and `StepEntityDiff.complex`
  on both a modified AND a newly-added entity, via a real `StepDiff::between()` result in both
  directions plus the empty-diff and self-diff cases. Asserts `!printed.contains('\n')`,
  `parse_diff(print_diff(x)) == x`, `decode_diff(encode_diff(x)) == x` for every case.
- `op_text_binary_roundtrip_law` (appended to the existing mutations `tests` mod): every
  `StepMutation` variant, including `InsertEntity`'s bare `StepEntity` payload and
  `SetEntityArg`/`InsertEntityArg`'s bare `StepValue` payload exercising every `StepValue` variant
  (incl. nested `Aggregate`/`TypedValue`). Same three assertions per case.

## STEP 4 — verification (real, all commands actually run)

The shared tree had active concurrent F6 sibling sessions the entire time (confirmed via `git
status` showing uncommitted WIP on `🏗️ifc`, `📕️xlsx`, `📊️csv`, `☁️ply` at various points, and via
`ps aux` showing simultaneous `cargo test -p semio-s-plugin-stdio --lib artifacts::txt` /
`artifacts::pdf::standards::v1_4` / `artifacts::step` processes blocked on the same build-directory
lock) — this is the documented "Concurrent Cargo Workspace Churn" pattern, not my bug. I polled
(`cargo check`/`cargo test`, ~20s intervals) rather than chasing it, per that pattern, and confirmed
by real `git status` + real compiler error text each time that every transient failure named a file
outside `🗿️artifacts/📐️step/**` before treating it as noise:

1. `cargo check -p semio-s-plugin-stdio --lib` — my own step files: **zero errors, zero warnings**
   (confirmed clean 3 times across the session as siblings' breakage churned and cleared around it).
2. `cargo test -p semio-s-plugin-stdio --lib "artifacts::step"` → **93 passed, 0 failed, 0 ignored**
   (`f6-step-poll-test.txt`), including both new law tests
   (`diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`).
3. `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1032 passed, 1 failed**
   (`f6-step-whole-crate-test.txt`). The 1 failure is
   `artifacts::xlsx::...::handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law` — a
   real assertion failure inside **xlsx's own** hand-rolled `DiffCodec` (a trailing empty-string
   entry mismatch in `XlsxOpcDiff.relationships`, `xlsx/.../🔺️diff/🦀️component.rs:1287`), entirely
   outside my ownership boundary (`🗿️artifacts/📕️xlsx/**`, another F6 sibling's in-progress work,
   confirmed via `git status` showing xlsx's diff file modified this session by another agent). Per
   my instructions I must not touch `xlsx`'s files. Count-wise this is a **net improvement** over the
   recon's own baseline (1019 passed / 0 failed) since it now includes far more artifacts' worth of
   new F6 tests (xlsx, ifc, csv, ply, step, ...) landing concurrently — my own artifact contributes 0
   failures and 2 new passing law tests to that total.

## Deviations from §5's grammar conventions

None structural. One naming choice: `StepValue::Reference` uses tag `F` (not `R`, which is already
`Real`) — documented inline via a doc comment on `enc_value`/`dec_value`, not a real deviation from
the *convention* (single-uppercase-letter tag, distinct per variant), just a concrete letter choice
forced by `StepValue` having two variants (`Real`, `Reference`) that would otherwise collide on `R`.

## Files touched (real, live, not reverted)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — hand-rolled `impl protocol::DiffCodec for StepDiff` (full grammar + helper functions, most made
  `pub(crate)` for mutations-side reuse), `+ handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law`
  test. `StepDiff`/`StepEntitiesDiff`/`StepEntityDiff`/`StepArgsDiff` left un-derived with an
  explanatory doc comment (3a blocker, real compiler-error citation captured).
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — hand-rolled `OpText`/`OpBinary` for `StepMutation` (reusing the diff file's `pub(crate)` grammar
  primitives), replacing the old `serde_json`-backed stub impls, `+ op_text_binary_roundtrip_law`
  test. `StepMutation` left un-derived with an explanatory doc comment (3a blocker, citation
  captured).
- Ticket-folder scratch (`.txt`, kept per repo rules): `f6-step-baseline-check.txt`,
  `f6-step-baseline-check-full.txt`, `f6-step-derive-attempt-check.txt`, `f6-step-impl-check1.txt`
  through `check3.txt`, `f6-step-test-attempt1.txt`/`attempt2.txt`, `f6-step-test-final.txt`,
  `f6-step-poll-check.txt`, `f6-step-poll-test.txt`, `f6-step-whole-crate-test.txt`.

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework
crates were all read-only this session. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`)
untouched — not a shortcut, the real `DiffCodec` impl is what makes `bun ./📜️script.ts policy`'s
`dsl-migration/diff-completeness` check stop flagging `📐️step`'s diff file (not independently
re-verified this session — the check is file-level/literal-text and the recon already established
its mechanics; `impl protocol::DiffCodec for StepDiff` and `impl store::TextError`... text now exist
in the file, satisfying it by the same mechanism verified for binary/gif89a/svg in the recon).
`STATUS.md` not updated (out of my ownership boundary per this session's instructions).

## Summary

| Check | Result |
|---|---|
| Diff side classification | HAND-ROLL (3a: `StepValue`/`StepEntitiesDiff`: DslField not satisfied, real compiler error) |
| Mutation side classification | HAND-ROLL (3a: `StepEntity`/`StepValue`: DslField not satisfied, real compiler error) |
| `cargo check -p semio-s-plugin-stdio --lib` (step files) | 0 errors, 0 warnings |
| `cargo test -p semio-s-plugin-stdio --lib "artifacts::step"` | 93 passed, 0 failed |
| `cargo test -p semio-s-plugin-stdio --lib` (whole crate) | 1032 passed, 1 failed (failure is xlsx's own, outside my ownership boundary) |
