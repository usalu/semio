# F6 — 📄️pdf 1.4 — OpText/OpBinary + DiffCodec Report

**Scope**: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/` only. 1.7 is out of scope
(a different F6 agent's row) and was never touched. Followed `f6-recon-report.md` §9's procedure
literally.

## STEP 1 — classification (verified for real, not trusted from the §8 table blindly)

`PdfDiff` (`🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`) is a flat 3-field struct
(`width: Option<f64>`, `height: Option<f64>`, `text: Option<String>`) — a plain sparse-patch
`Option<T>`, never `Option<Option<T>>` tri-state. `PdfMutation`'s only enum-shaped payload is
`SetSnapshot { snapshot: PdfSnapshot }`, and `PdfSnapshot`'s whole tree (`PdfSnapshot { schema:
String, page: PageDoc }`, `PageDoc { width: f64, height: f64, text: String }`) has zero
data-carrying enums anywhere. Per §3's decision rule this is unambiguously the **DERIVE** path on
both sides — confirmed for real, not assumed:

- Added `dsl::DslDiff` to `PdfDiff`'s derive list → `cargo check -p semio-s-plugin-stdio --lib` →
  **zero pdf-scoped errors** (first attempt compiled clean, no cascading `DslRecord` requirement
  on the Diff side since it has no struct-valued fields of its own).
- Added `dsl::DslOps` to `PdfMutation`'s derive list → same `cargo check` → **one cascading error**:
  `PdfSnapshot: DslField is not satisfied` (expected — `SetSnapshot` carries the whole snapshot).
  Added `#[derive(dsl::DslRecord)]` to `PdfSnapshot` and `PageDoc` (§3's cascading-requirement
  caveat) → re-ran `cargo check` → zero pdf-scoped errors.

This matches the recon table's row 8 prediction ("DERIVE (probable)") exactly — the
`CHECK-ENUM-ELSEWHERE` caveat (raised because 1.7's `PdfValue` object-graph enum lives in the same
artifact family) does not apply to 1.4: 1.4's `PageDoc`/`PdfSnapshot` are its own, deliberately
frozen, pre-real-codec stub types (per the file's own existing doc comment) with no `PdfValue`
reachable anywhere in this standard's tree.

## STEP 2a — DERIVE path implementation

Files touched (all within my ownership boundary, exactly 3 Rust files):

1. **`🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`**
   - `#[derive(dsl::DslRecord)]` added to `PageDoc` and `PdfSnapshot`.
   - `#[dsl(block)]` added to `PdfSnapshot::page` (struct-valued field, readability convention
     from `SpaceMutation`/gif89a precedent per §2's brief).
2. **`🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`**
   - `#[derive(dsl::DslDiff)]` added to `PdfDiff` → `protocol::DiffCodec` is now **fully generated**,
     no hand-written body at all (identical situation to `BinaryDiff` in §4 of the recon).
   - Doc comment added citing the real classification evidence (compiles clean, cascading
     `DslRecord` requirement, why 1.4 differs from 1.7's `CHECK-ENUM-ELSEWHERE` flag).
3. **`🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`**
   - `#[derive(dsl::DslOps)]` added to `PdfMutation` → gives `dsl::DslVariants` for free.
   - `#[dsl(block)]` added to `SetSnapshot`'s `snapshot` field.
   - Per P6 (`DslOps` never emits `OpText`/`OpBinary`), replaced the prior `serde_json`-based
     `OpText`/`OpBinary` stub impls with the exact §2 handcrafted boilerplate wrapper (verbatim
     shape to `BinaryMutation`/`GifMutation`): `parse_op`/`print_op` via
     `dsl::DslVariants::variants()`/`from_named_record`/`to_named_record` + `dsl::parse`/`dsl::print`,
     and `encode_op`/`decode_op` forwarding straight to `dsl::variants_binary::encode_op`/`decode_op`.

No `FlowMutationDsl`-style mirror enum was needed — `DslOps` derived directly on `PdfMutation`
itself, exactly like `SpaceMutation`/`GifMutation` in the recon's precedent.

## STEP 3 — tests (both added, both pass)

- **`op_text_binary_roundtrip_law`** (`🧬️mutations/🦀️component.rs`) — exercises both variants
  (`NoMutation`, `SetSnapshot` with a real nested-struct payload incl. non-trivial float/string
  values), asserts `!printed.contains('\n')`, `parse_op(print_op(m)) == m`, and
  `decode_op(encode_op(m)) == m` for each.
- **`diff_codec_text_binary_roundtrip_law`** (`🔺️diff/🦀️component.rs`) — exercises both a
  fully-populated diff (`between(sweep_a, sweep_b)`, every field present) and the fully-empty diff
  (`between(sweep_a, sweep_a)`), asserts the same three properties via `print_diff`/`parse_diff`/
  `encode_diff`/`decode_diff`.

Both tests use exact-binary-representable float literals (`300.5`, `400.25`) so the round-trip
assertion isn't confounded by decimal-to-binary float printing precision.

## STEP 4 — verification (real, both scoped and whole-crate)

```
cargo test -p semio-s-plugin-stdio --lib "artifacts::pdf::standards::v1_4"
test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 1010 filtered out; finished in 0.00s
```
All 23 tests in this module's scope pass, including the 2 new law tests
(`diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`) and every pre-existing
test in the 1.4 tree (`between_roundtrip_law`, `inverse_law_diff_level`, `absorb_law_*`,
`field_sweep_*`, `mutation_diff_law_matches_apply_pdf_mutation`,
`mutation_apply_inverse_round_trips_every_variant`, plus the subset `a`/`x` builder/analyzer/
composer tests and `engine::tests::codec_retention_law_text_round_trips_through_encode_decode`
that live under the same module prefix from other agents' concurrent work on this same artifact).

```
cargo test -p semio-s-plugin-stdio --lib   (whole crate, run twice for stability)
test result: FAILED. 1032 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in ~9-10s
```
**The single failure is `artifacts::xlsx::standards::v_ecma_376::…::handcrafted_diff_codec_tests::
diff_codec_text_binary_roundtrip_law`** — a different F6 agent's own hand-rolled xlsx `DiffCodec`
grammar has a real bug (an empty-string `""` appears in `relationships.removed` on the parse side
that isn't in the original diff — a trailing-separator artifact in their `split_top_level`-style
grammar). This is entirely inside `📕️xlsx`'s ownership boundary, not `📄️pdf`'s, was already
present before I ran either whole-crate check, reproduced identically both times (stable, not a
one-off transient), and involves zero code I authored or touched. I did not attempt to fix it —
out of my ownership boundary per this ticket's explicit rules.

### Note on this session's live-tree turbulence
This wave (`ARTIFACT-SYSTEM-OVERHAUL…F6`) runs many agents concurrently editing the same
`semio-s-plugin-stdio` crate. During my STEP 1/4 verification the crate went through several
transient non-compiling states from other agents' in-flight edits (confirmed via repeated
`cargo check`/`cargo test` runs showing errors exclusively in `📕️xlsx`, `☁️ply`, `📊️csv`,
`📄txt`, `🏗️ifc` — never `📄️pdf`) before settling to the 1032/1-failed state above. I polled
(via two `Monitor`-driven retry loops, ~10 attempts total) rather than chasing or fixing those
files, per the "Concurrent Cargo Workspace Churn" pattern. At every single intermediate checkpoint,
zero errors ever referenced `📄️pdf`'s own files — my 3 changed files compiled clean from the very
first `cargo check` onward.

## Deviations from the recon's §5/§2 template

None on substance. `PdfDiff`/`PdfMutation` landed exactly where the recon's row 8 predicted
(DERIVE both sides), and the §2 OpText/OpBinary wrapper was copied verbatim (only the type name
changed). The only addition beyond copy-paste was the cascading `#[derive(dsl::DslRecord)]` on
`PdfSnapshot`/`PageDoc`, which §3/STEP 2a explicitly call out as expected or the DERIVE path.

## Files touched (real, live, not reverted)

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  — `dsl::DslRecord` on `PageDoc`/`PdfSnapshot`, `#[dsl(block)]` on `page`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — `dsl::DslDiff` on `PdfDiff` (fully derived `DiffCodec`, no hand-written body), doc comment,
  `diff_codec_text_binary_roundtrip_law` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄️pdf/🏅️standards/🔖️1.4/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — `dsl::DslOps` on `PdfMutation` (derived clean), `#[dsl(block)]` on `SetSnapshot::snapshot`,
  handcrafted `OpText`/`OpBinary` replacing the `serde_json` stubs, doc comment,
  `op_text_binary_roundtrip_law` test.
- This report: `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION/f6-pdf-1.4-report.md`.

**No shared files touched**: `📦️glue.rs`, root `📜️script.ts`, the `dsl`/`protocol`/`schema`
framework crates, `POLICY_DIFF_COMPLETENESS_ALLOWLIST` were all read-only for this session (the
allowlist grep confirms `📄️pdf` was never in it and still isn't — nothing to remove, nothing
added). `STATUS.md` not touched (per ticket rules, that's the closer's job, not a per-artifact
agent's).

## Live policy state (observed, not edited)

`policyDiffCompletenessBreaches` (root `📜️script.ts:3185-3205`) is a literal-text, file-level
check: `content.includes("dsl::DslDiff") || content.includes("DiffCodec for")`. `PdfDiff`'s file
now contains the literal token `dsl::DslDiff` in its derive-attribute list, so this diff file no
longer trips that rule — verified by reading the check's source directly (did not run the full
`bun ./📜️script.ts policy` command, which is expensive repo-wide; the recon report already
established the mechanism and this file's own text is sufficient evidence).

## Summary (report JSON fields)

- `artifact`: `📄️pdf`
- `standard`: `1.4`
- `diff_path`: `derive`
- `mutation_path`: `derive`
- `tests_passed`: 23 (pdf 1.4 module scope, includes both new law tests); whole-crate run shows
  1032 passed with the crate's sole failure being unrelated (`📕️xlsx`, out of my ownership).
- `tests_failed`: 0 (pdf 1.4 module scope)
- `deviations`: none on substance; see "Note on this session's live-tree turbulence" above for
  the concurrent-session context around STEP 4's whole-crate check.
