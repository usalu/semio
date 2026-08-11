# F6 — 📄txt utf-8: OpText/OpBinary + DiffCodec Report

**Status: DONE. Real, verified, both `cargo test` runs below actually executed (not estimated).**

## STEP 1 — Classification (verified for real, not trusted from the recon table)

Recon's guess: "DERIVE probable — simple lines: Vec<String> shape." Confirmed correct for BOTH
sides, by adding the derive and reading `cargo check` output (no compile errors ever attributed
to any `📄txt` file, across 7 separate `cargo check -p semio-s-plugin-stdio --lib` runs taken at
different points while other sessions' concurrent, unrelated breakage came and went around it —
see below).

**Diff side (`TxtDiff`)**: `dsl::DslDiff` — DERIVE. Walked every field: `trailing_newline:
Option<bool>`, `line_ending: Option<LineEnding>`, `lines: Option<TxtLinesDiff>`. None is
`Option<Option<_>>` (3b does not apply — `lines` composes via a `removed`/`modified`/`added`
triple, it never itself carries a tri-state "removed vs absent" flag). The only enum in the walk
is `LineEnding` (`Lf`/`CrLf`, both unit variants) — unit-only, so it binds via `DslScalar`
rather than blocking the derive (3a does not apply either, since `DslScalar` is one of the two
legitimate `DslField` derive sources per `f6-recon-report.md` §3a).

**Mutation side (`TxtMutation`)**: `dsl::DslOps` — DERIVE. Walked every variant's payload,
including `SetSnapshot`'s whole `TxtSnapshot` (`schema: String, lines: Vec<String>,
trailing_newline: bool, line_ending: LineEnding`). No data-carrying enum anywhere in the tree —
same `LineEnding` unit-enum reused directly as a bare (non-`Option`) field
(`SetLineEnding{value: LineEnding}`), which the gif 89a pilot's `SetFrameDisposal{index, disposal:
GifDisposal}` precedent already proved compiles cleanly for `DslOps`.

Both sides landed on the DERIVE path — no hand-rolled grammar needed anywhere in this artifact.

## STEP 2a — Derive path implementation

Cascading `#[derive(dsl::DslRecord)]` / `#[derive(dsl::DslScalar)]` additions, innermost-out:

- `LineEnding` (📸️snapshot/component.rs): `dsl::DslScalar` added (unit-variant-only enum →
  `DslField` directly).
- `TxtSnapshot` (📸️snapshot/component.rs): `dsl::DslRecord` added — alongside, not replacing,
  the existing hand-rolled `store::ArtifactDsl`/`store::ArtifactPack` (same non-destructive
  pattern as the pilot's `BinarySnapshot`). Lets `TxtSnapshot` embed as `TxtMutation::SetSnapshot`'s
  payload.
- `TxtLineAdded`, `TxtLineModified`, `TxtLinesDiff` (🔺️diff/component.rs): `dsl::DslRecord`
  added to all three, bottom-up (`Vec<usize>`/`Vec<TxtLineModified>`/`Vec<TxtLineAdded>` bind via
  the `dsl` crate's blanket `Vec<T>` impl once their element types are `DslField`).
- `TxtDiff` (🔺️diff/component.rs): `dsl::DslDiff` added — fully generates `protocol::DiffCodec`
  (`print_diff`/`parse_diff`/`encode_diff`/`decode_diff`). No hand-written `DiffCodec` impl
  exists or is needed.
- `TxtMutation` (🧬️mutations/component.rs): `dsl::DslOps` added, plus `#[dsl(block)]` on
  `SetSnapshot`'s `snapshot: TxtSnapshot` field for readability (matches
  `BinaryMutation`/`GifMutation` precedent). Emits `dsl::DslVariants` only (P6) — the §2
  handcrafted `OpText`/`OpBinary` wrapper was then written verbatim from the recon report's
  template, replacing the pre-existing `serde_json`-based stub impls.

No `#[dsl(base64)]` was needed anywhere — `📄txt` has zero `Vec<u8>` fields (it's a pure
line/text artifact, unlike binary/gif).

## STEP 3 — Tests added (both mandatory laws)

- `op_text_binary_roundtrip_law` (🧬️mutations/component.rs, `mod tests`): reuses the file's
  existing `all_variants(&b)` fixture (already covers every one of `TxtMutation`'s 7 variants,
  including `SetSnapshot`'s full nested-record payload). For each variant: asserts
  `print_op()` is one line, `parse_op(print_op(m)) == m`, and `decode_op(encode_op(m)) == m`.
- `diff_codec_text_binary_roundtrip_law` (🔺️diff/component.rs, `mod tests`): four cases —
  `TxtDiff::default()` (empty), a scalar-only diff (`trailing_newline`+`line_ending` both
  `Some`, `lines: None`), a directly-constructed diff with `TxtLinesDiff`'s `removed`+`modified`+
  `added` ALL populated simultaneously (a real `between()` result can only ever populate
  `modified`+`removed` XOR `modified`+`added`, since `TxtLinesDiff::between`'s own
  pairwise-then-tail algorithm makes the two tails mutually exclusive — documented in the test's
  own doc comment), and a genuine `TxtDiff::between(&a, &b)` result reusing the file's existing
  `between_roundtrip_synthetic` fixture shape. Same three assertions per case
  (`!printed.contains('\n')`, print/parse round-trip, encode/decode round-trip).

Also required a one-line fix beyond the recon template: added `#[cfg(test)] use
protocol::{OpBinary, OpText};` to 🧬️mutations/component.rs's top-level imports — without it,
`m.print_op()`/`TxtMutation::parse_op`/`m.encode_op()`/`TxtMutation::decode_op` in the new test
don't resolve (the trait impls are written against fully-qualified `protocol::OpText`/
`protocol::OpBinary` paths, which doesn't bring the trait into method-call scope elsewhere in
the file). Caught via a real `cargo check` error (`E0599: no method named 'print_op' found`),
not assumed.

## STEP 4 — Verification

**Concurrent compile churn (real, observed, not this artifact's fault):** this is a single-crate
lib (`semio-s-plugin-stdio`), so `cargo test` cannot execute at all — for any artifact, including
this one — while ANY file in the crate fails to compile. Across this session, `📄txt`-owned files
individually never once appeared in an `error[...]` in ~9 separate `cargo check`/`cargo test`
invocations; every transient error belonged to `📕️xlsx`, `📊️csv`, `📐️step`, `☁️ply`, or `🏗️ifc` —
other F6 agents' own artifacts, confirmed actively mid-edit concurrently in real time (their
files' mtimes moved within the same minutes these checks ran; matches
`.🦑️repo/…/feedback-concurrent-cargo-workspace-churn.md`'s precedent). Polled with `Monitor`
rather than blocking synchronously (per that same guidance) until the crate compiled clean, then
ran the real test commands immediately:

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::txt"` → **21 passed, 0 failed** (0.01s).
  Includes both new law tests: `schema::diff::component::tests::diff_codec_text_binary_roundtrip_law`
  and `schema::mutations::component::tests::op_text_binary_roundtrip_law`, plus every pre-existing
  `📄txt` test (`engine`, `analyzer`, `examples::demo`, the absorb/inverse/field-sweep suite) —
  all green.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1032 passed, 1 failed** (10.25s;
  baseline per `f6-recon-report.md` was 1019/0, so the crate's pass count went up, never down, as
  required by STEP 4). The 1 failure is
  `artifacts::xlsx::standards::v_ecma_376::subsets::any::schema::diff::component::handcrafted_diff_codec_tests::diff_codec_text_binary_roundtrip_law`
  — a different F6 agent's own in-flight `📕️xlsx` work, entirely outside `📄txt`'s ownership
  boundary (confirmed by name/path: it's xlsx's own hand-rolled `DiffCodec` test, not anything
  this session touched). Every one of the 21 `artifacts::txt::` tests in this same full run
  reported `ok`.

## Deviations from §5/§9 template

- None on the grammar/derive side — this artifact needed zero hand-rolling, matching the
  recon's "DERIVE probable" guess exactly on both sides.
- One addition beyond the copy-paste template: the `#[cfg(test)] use protocol::{OpBinary,
  OpText};` import fix above, needed because `📄txt`'s mutations file didn't already import
  those traits (binary's file happened to already have this import for a different pre-existing
  test; txt's didn't, since its previous `OpText`/`OpBinary` impls were called only via the
  `protocol::Mutation`/`store::register_document_codec` trait-object path, never by bare method
  call in its own tests, until now).

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs`
  — `dsl::DslScalar` on `LineEnding`, `dsl::DslRecord` on `TxtSnapshot`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — `dsl::DslRecord` on `TxtLineAdded`/`TxtLineModified`/`TxtLinesDiff`, `dsl::DslDiff` on
  `TxtDiff` (fully derived `DiffCodec`), + `diff_codec_text_binary_roundtrip_law` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📄txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — `dsl::DslOps` on `TxtMutation` (`#[dsl(block)]` on `SetSnapshot::snapshot`), handcrafted
  `OpText`/`OpBinary` replacing the `serde_json` stubs, `#[cfg(test)] use protocol::{OpBinary,
  OpText};` import fix, + `op_text_binary_roundtrip_law` test.
- Ticket-folder scratch (`.txt`): none needed — all `cargo check` outputs were kept in the
  session's own scratchpad directory (outside the ticket folder, per instructions to use the
  scratchpad for temp files), not the ticket folder, since the ticket-folder-`.txt` rule is
  specifically about durable ticket-scoped artifacts and none of these intermediate check
  captures needed to persist past this session.

No shared files touched: `📦️glue.rs`, `📜️script.ts`, `dsl`/`protocol`/`schema` framework crates
all read-only for this session. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` untouched (correct — the
goal is for the real check to stop flagging `📄txt`'s diff file once it compiles, not to
allowlist around it; `TxtDiff`'s file already contains the literal text `dsl::DslDiff`, which is
exactly what `policyDiffCompletenessBreaches` (`📜️script.ts:3185-3205`) greps for).
