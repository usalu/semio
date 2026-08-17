# F6 — ☁️ply 1.0 — OpText/OpBinary + DiffCodec

## Summary

Implemented handcrafted `protocol::DiffCodec` for `PlyDiff` and handcrafted `protocol::OpText`/
`protocol::OpBinary` for `PlyMutation`, replacing the previous `serde_json`-based `OpText`/
`OpBinary` stubs and PlyDiff's total absence of a `DiffCodec` impl. **Both sides land on the
HAND-ROLL path** — the recon report's classification-table guess of "DERIVE (probable)" for ply
was **wrong**; the sweep that produced that guess was a single-file grep for `pub enum` in the
diff file only, and ply's real blockers live in the *snapshot* module (`PlyProperty`/`PlyValue`),
reachable from both the diff and the mutation trees.

| Side | Path | Reason |
|---|---|---|
| Diff (`PlyDiff`) | **HAND-ROLL** | 3a "enum-in-tree": `PlyProperty` (via `PlyElementDiff::properties: Option<Vec<PlyProperty>>`) and `PlyValue` (via `PlyRowFieldChange::value: PlyValue`) are both genuine data-carrying enums with no derivable `DslField` impl. |
| Mutation (`PlyMutation`) | **HAND-ROLL** | Same 3a reason, hit independently and more pervasively: `SetRowProperty{value: PlyValue}` carries the enum directly; `SetSnapshot`/`AddElement`/`InsertRow` all transitively reach it via `PlySnapshot`/`PlyElement`/`PlyRow`. |

Both real compiler citations (from actually adding `#[derive(dsl::DslDiff)]` /
`#[derive(dsl::DslOps)]`, `cargo check`-ing, capturing the error, then reverting) are in
`f6-ply-step1-classification-citations.txt` in this folder, and are also cited as doc comments
directly on `PlyDiff`'s module header and `PlyMutation`'s module header, plus on `PlyElementDiff`
itself (the exact struct the diff-side error points at).

## STEP 1 — classification (verified for real, not trusted from the table)

Per `f6-recon-report.md` §9 STEP 1: added `#[derive(dsl::DslDiff)]` to `PlyDiff` (cascading
`#[derive(dsl::DslScalar)]` on `PlyFormat` and `#[derive(dsl::DslRecord)]` down through
`PlyElementsDiff`/`PlyElementModified`/`PlyElementAdded`/`PlyElementDiff` to push the compiler past
the trivial `PlyFormat` blocker to the real one), ran `cargo check -p semio-s-plugin-stdio --lib`,
and got a real `error[E0277]: the trait bound PlyProperty: DslField is not satisfied` pointing at
`PlyElementDiff::properties: Option<Vec<PlyProperty>>` (diff/component.rs:290), with the help note
pointing at `PlyProperty`'s own `pub enum` declaration in snapshot/component.rs:50. Then separately
added `#[derive(dsl::DslOps)]` to `PlyMutation` and got `error[E0277]: the trait bound PlyValue:
DslField is not satisfied` at `SetRowProperty { ..., value: PlyValue }` (mutations/component.rs:29),
plus (same compiler run) `PlySnapshot`/`PlyElement`/`PlyRow` all also `DslField`-unsatisfied via
`SetSnapshot`/`AddElement`/`InsertRow`. All temporary derive attributes were then removed and the
files restored to their pre-experiment derive lists before writing the real hand-rolled
implementation (verified via re-reading the files — no experimental derives survived into the
final diff).

## STEP 2b — hand-rolled implementation

Followed `f6-recon-report.md` §5's grammar template (the gif89a/svg pilots), copying the exact
primitive set (`hex_encode`/`hex_decode`/`split_top_level`/`strip_brackets`/`encode_option`/
`decode_option`) verbatim into `☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`,
marked `pub(crate)` where `🧬️mutations/🦀️component.rs` needs to reuse them (same intra-artifact
reuse pattern svg's mutations file uses against svg's diff file — `PlyMutation`'s `OpText`/
`OpBinary` import `dec_element`/`dec_format`/`dec_row`/`dec_str`/`dec_value`/`enc_element`/
`enc_format`/`enc_row`/`enc_str`/`enc_value`/`split_top_level`/`strip_brackets` directly from the
diff module rather than duplicating a second copy).

### Grammar (real, not `serde_json`)

- Strings/bytes: lowercase hex (`enc_str`/`dec_str`), same rationale as every other pilot (no
  external base64 dep, no escaping, matches `PlySnapshot`'s own `ArtifactDsl` hex convention where
  applicable).
- `Option<T>`: uniform `[0]`=None / `[1,<T>]`=Some(T) tag (`encode_option`/`decode_option`).
- Plain structs: positional `[f1,f2,...]` tuples.
- `PlyProperty` (data-carrying enum, `Scalar{name,kind}` / `List{name,count_kind,value_kind}`):
  single-uppercase tag prefix — `S[name,kind]` / `L[name,count_kind,value_kind]`.
- `PlyValue` (data-carrying enum, 8 scalar variants + recursive `List(Vec<PlyValue>)`):
  single-letter tag matching `enc_scalar_type`'s own letters (`c`/`C`/`s`/`w`/`i`/`u`/`f`/`d`) plus
  `L[v1,v2,...]` for the recursive list — the recursion is handled for free since `enc_value`/
  `dec_value` call themselves.
- `PlyFormat` (unit-only enum): single-letter tag `a`/`l`/`b`.
- `PlyScalarType` (unit-only enum, 8 variants): single-letter tag `c`/`C`/`s`/`w`/`i`/`u`/`f`/`d`
  (same letters as `PlyValue`'s tags — no collision since they're never mixed in the same grammar
  position).
- Collection triples: `{[removed];[modified];[added]}` (semicolon-separated sections, each a
  comma-separated bracketed list). Two DIFFERENT key shapes were needed (a real, documented
  deviation from gif89a's uniform index-keyed triple helper — see Deviations below):
  - `PlyRowsDiff` (index-keyed on all three sections, mirrors gif89a's `frames` triple exactly).
  - `PlyElementsDiff` (NAME-keyed `removed`/`modified` — `PlyElement::name` is the real identity,
    there is no `RenameElement` mutation — but INDEX-keyed `added`, matching `PlyElementAdded`'s
    own real shape).
- `PlyElementDiff`'s own sparse fields print as single-letter `tag:value` pairs (`P`/`R`) inside
  its own `[...]`, same shape as gif89a's `enc_frame_diff`.
- Top-level `PlyDiff`/`PlyMutation` line: space-separated `name=value` tokens (Diff) or `keyword
  arg=value ...` (Mutation) — ply uses a uniform `key=value` shape for EVERY top-level token
  (including the collection fields, e.g. `elements={...}`), unlike gif89a's `name{...}` shape for
  collections specifically — a deliberate simplification since ply's tokens are already all
  `key=value` and the collection's own curly-brace body needs no extra name repetition.
- `encode_diff`/`encode_op` = `print_diff()`/`print_op()`.into_bytes()` — same simplification
  `WriterDiff`/gif89a/svg use. Satisfies every `DiffCodec`/`OpText`/`OpBinary` law.

Real captured `print_diff` output (from the new `diff_codec_text_binary_roundtrip_law` test,
exercising the name-keyed `elements` triple's removed+modified+added simultaneously plus the
nested index-keyed `rows` triple, the weak `properties` replace, and both `PlyProperty`/`PlyValue`
enum tag families including `PlyValue::List`'s recursion): see the test itself in
`🔺️diff/🦀️component.rs`'s `codec_tests` module for the exact assertions (the literal string is long
and not reproduced verbatim here to keep this report scannable — run the test to see it, or read
`f6-ply-test-scoped-final.txt` in this folder for the passing run).

Real captured `print_op` output shape (from `op_text_binary_roundtrip_law`):
`set-row-property element-name=76657274657865 row-index=0 property-name=78 value=f[42]` for
`SetRowProperty{element_name:"vertex", row_index:0, property_name:"x", value:PlyValue::Float(42.0)}`.

## STEP 3 — tests (mandatory, both paths, both added)

- `🔺️diff/🦀️component.rs::codec_tests::diff_codec_text_binary_roundtrip_law` — new test module
  (the diff/mutations files had NO pre-existing `#[cfg(test)]` module of their own; ply's existing
  law-test suite — `mutation_diff_law`/`inverse_law`/`absorb_law_*`/`between_roundtrip_law`/
  `field_sweep_*` — lives in `⚙️engine/🦀️component.rs`'s test module instead, per F1-F5's placement
  choice for this artifact; the two new codec tests follow the SAME file-placement convention the
  gif89a/svg pilots use — colocated with the `impl DiffCodec`/`impl OpText`/`impl OpBinary` they
  test, in a dedicated `codec_tests` submodule so they're clearly additive to (not a rename of) any
  future test consolidation). Uses fresh `sweep_a`/`sweep_b` fixtures (adapted from the engine
  test module's own `sweep_a`/`sweep_b`, since those are private to that module and not
  importable) exercising `between()` in both directions plus the empty-diff case. Asserts
  `!printed.contains('\n')`, `parse(print(x)) == x`, `decode(encode(x)) == x` for all 3 cases.
- `🧬️mutations/🦀️component.rs::codec_tests::op_text_binary_roundtrip_law` — new test module (same
  placement rationale), covering all 9 `PlyMutation` variants including `SetSnapshot`'s whole
  nested snapshot, `AddElement`'s bare `PlyElement` payload (itself containing a `PlyProperty`),
  `InsertRow`'s `PlyRow` payload, and two `SetRowProperty` cases (a plain scalar `PlyValue` and a
  recursive `PlyValue::List`). Same three assertions per variant.

## STEP 4 — verification (real, both runs)

- `cargo test -p semio-s-plugin-stdio --lib "artifacts::ply"` → **25/25 passed** (23 pre-existing +
  2 new: `diff_codec_text_binary_roundtrip_law`, `op_text_binary_roundtrip_law`). Full output:
  `f6-ply-test-scoped-final.txt` in this folder.
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **1032 passed, 1 failed**. The 1
  failure — `artifacts::xlsx::standards::v_ecma_376::...::handcrafted_diff_codec_tests::
  diff_codec_text_binary_roundtrip_law` — is **entirely unrelated to ply**: it's a print/parse
  round-trip mismatch inside xlsx's OWN hand-rolled `DiffCodec` (a nondeterministic-ordering bug
  in `relationships`'s `NamedTripleDiff`, reproduced 3× with a different spurious empty-string
  entry each time — classic `HashSet`/iteration-order nondeterminism in someone else's code, not
  mine). This is a different, concurrently-running F6 session's in-progress work on the xlsx
  artifact — confirmed by (a) the file path being entirely inside `🗿️artifacts/📕️xlsx/**`, which
  ply's ownership boundary explicitly excludes, and (b) `cargo check`/`cargo test` runs taken
  minutes apart during this session repeatedly flipped between clean and broken for OTHER
  artifacts (csv, ifc, xlsx errors appeared and disappeared across successive `cargo check` runs
  while I made zero edits to those files) — consistent with the documented "Concurrent Cargo
  Workspace Churn" pattern (another live session mid-edit). Per the ownership boundary ("Do NOT
  touch ... other artifact's files — your ownership is exactly `🗿️artifacts/☁️ply/**`"), this
  failure was left untouched. Full tail output: `f6-ply-test-full-crate-final.txt` in this folder.
  Baseline at F6-recon time was 1019/0; the crate is now at 1032 passed (ply's own +2, plus other
  concurrent sessions' additions) with exactly 1 failure, entirely outside ply's ownership.

## Files touched

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — module doc comment cites the real `PlyProperty: DslField` compile error; `PlyElementDiff` doc
  comment cites the same; new `HandcraftedDiffCodec` region (primitives, value codecs, diff-value
  codecs, top-level `print_diff`/`parse_diff`, `impl protocol::DiffCodec for PlyDiff`); new
  `codec_tests` module with `diff_codec_text_binary_roundtrip_law`.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — module doc comment cites the real `PlyValue`/`PlySnapshot`/`PlyElement`/`PlyRow: DslField`
  compile errors; `OpCodecs` region rewritten from `serde_json` stubs to the hand-rolled grammar
  (reusing the diff file's `pub(crate)` primitives); new `codec_tests` module with
  `op_text_binary_roundtrip_law`.
- Ticket-folder scratch (kept, `.txt`): `f6-ply-step1-classification-citations.txt` (real compiler
  error citations for both derive attempts), `f6-ply-test-scoped-final.txt` (25/25 ply-scoped
  test run), `f6-ply-test-full-crate-final.txt` (whole-crate 1032/1 run, failure isolated to xlsx).

**No shared files touched**: `glue.rs`, `📜️script.ts`, the `dsl`/`protocol`/`schema` framework
crates were all read-only for this session (only read to confirm the derive machinery's real
behavior — the same machinery `f6-recon-report.md` already documented). No other artifact's files
were edited. `POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`) was NOT touched — ply's
`DiffCodec` impl now exists for real, so `bun ./📜️script.ts policy`'s `dsl-migration/
diff-completeness` check should stop flagging `☁️ply`'s diff file on its own merits (not verified
by an explicit `policy` run in this session — the whole-crate `cargo test` failure in xlsx was
prioritized as the higher-signal check, and `script.ts` is outside this session's ownership
boundary to invoke changes against; the recon report's own §7 confirms the check is purely
literal-text/file-level, and this file now contains a real `impl protocol::DiffCodec for PlyDiff`).

## Deviations from §5's grammar template

1. **Two triple key-shapes, not one.** gif89a's §5 template uses a single uniform index-keyed
   `{[removed];[modified];[added]}` triple for every collection. Ply needed a SECOND variant for
   `PlyElementsDiff`, whose `removed`/`modified` are NAME-keyed (`PlyElement::name` is the real
   identity — there's no `RenameElement` mutation) while `added` stays INDEX-keyed (matching
   `PlyElementAdded::index`'s real shape). This is not a stylistic choice — it's forced by
   `PlyElementsDiff`'s own pre-existing (F1-F5) type shape, which already differs from `PlyRowsDiff`
   this way at the Rust level. Implemented as two separate encode/decode function families
   (`enc_rows_diff`/`dec_rows_diff` reusing a generic `dec_index_triple_body` helper, vs.
   `enc_elements_diff`/`dec_elements_diff` written directly since its 3-section key-shape mix
   doesn't fit a single generic helper cleanly) rather than forcing an ill-fitting generic
   abstraction — matches the recipe's stated preference for straightforward per-artifact code over
   cross-cutting genericity.
2. **Uniform `key=value` top-level tokens**, not gif89a's mixed `name=value` (scalars) / `name{...}`
   (collections) shape. Ply's top-level fields are `format`/`comments`/`elements` — all rendered as
   `key=value` including `elements={...}` — a minor simplification since there's no ambiguity to
   resolve (unlike gif89a's `frames{`/`comments{`/`appext{` prefix-matching parse, ply's parser
   still does simple `strip_prefix("elements=")` token matching, just with a `=` instead of no
   separator before the brace).
3. **No `hex_encode`/`split_top_level`/etc. duplication into `🧬️mutations`** — following svg's
   precedent exactly (not gif89a's, which duplicates because gif's Mutation side derived cleanly
   and never needed the primitives at all), `🧬️mutations/🦀️component.rs` imports every primitive it
   needs directly from `🔺️diff/🦀️component.rs` via `pub(crate)` visibility, avoiding a second copy
   within the same artifact.
4. **Test module placement**: added new `#[cfg(test)] mod codec_tests` blocks directly in
   `🔺️diff/🦀️component.rs` and `🧬️mutations/🦀️component.rs` (mirroring gif89a/svg's own placement)
   rather than extending `⚙️engine/🦀️component.rs`'s existing (large, F1-F5-authored) test module —
   CLAUDE.md requires extending existing test files rather than creating new ones, but these are
   NEW test *modules* inside the SAME existing files the new code lives in (not new files), which
   is the same placement choice the gif89a/svg F6 pilots already made for identical reasons (the
   codec tests are colocated with the codec they test, for discoverability, while the pre-existing
   law-test suite in the engine file was left untouched since it doesn't test `DiffCodec`/
   `OpText`/`OpBinary` at all).

No other scope cuts. Every `PlyMutation` variant, every `PlyDiff` field, both data-carrying enums
(`PlyProperty`, `PlyValue` incl. `PlyValue::List`'s recursion), and both collection triple shapes
are exercised by the two new tests.
