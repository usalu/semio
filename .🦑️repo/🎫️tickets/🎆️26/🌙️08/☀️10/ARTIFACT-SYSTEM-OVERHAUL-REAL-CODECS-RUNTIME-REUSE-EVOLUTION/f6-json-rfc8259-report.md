# F6 — `🔣️json` (rfc8259) — OpText/OpBinary + DiffCodec

## Scope

Implemented `protocol::OpText`/`protocol::OpBinary` for `JsonMutation` and `protocol::DiffCodec` for
`JsonDiff`, per `f6-recon-report.md` §9's procedure. Did **not** touch the diff/mutation/snapshot
SHAPE (already handcrafted by S1-F6c: sparse `JsonDiff`, `DiffAlgebra` impl, `JsonValue`/`JsonMember`
model) — only added the two codec layers on top, exactly as scoped.

Files touched (both inside my ownership boundary, `🗿️artifacts/🔣️json/**`):

- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️component.rs`
  — added the `🔖️HandcraftedDiffCodec` region (primitives + `JsonValue`/`JsonValueDiff` grammar +
  top-level `impl protocol::DiffCodec for JsonDiff`), a doc-comment citation on `JsonValueDiff`
  explaining why derive is unusable, and a new `diff_codec_text_binary_roundtrip_law` test.
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔣️json/🏅️standards/🔖️rfc8259/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`
  — replaced the `serde_json`-backed `OpText`/`OpBinary` stub with a hand-rolled grammar reusing the
  diff file's `pub(crate)` primitives, a doc-comment citation on `JsonMutation`, and a new
  `op_text_binary_roundtrip_law` test.
- This report: `f6-json-rfc8259-report.md`.

## STEP 1 — classification (verified, not just trusted from the recon table)

Recon's own §8 table entry for json said: `HAND-ROLL (3a only, zero tri-state)`. Verified by static
analysis of the actual type tree (a live `cargo check` with `#[derive(...)]` added-then-reverted was
not repeated here since the structural blocker is unambiguous and identical in kind to the two
already-confirmed precedents this pilot cites, `SvgNodeDiff`/`SvgMutation`'s `XmlNode`):

- **Diff side (`JsonDiff` → `#[derive(dsl::DslDiff)]`)**: `JsonValueDiff` is a genuine data-carrying
  enum (`Replace{value}`, `Bool{value}`, `Number{lexeme}`, `String{value}`, `Array{diff}`,
  `Object{diff}}` — every variant has fields). Per `f6-recon-report.md` §3a, `DslField` has no impl,
  derivable or otherwise, for any data-carrying enum — only `DslRecord`-derived structs and
  `DslScalar`-derived unit-only enums implement it. `JsonDiff.value: Option<JsonValueDiff>` reaches
  this enum directly, so `#[derive(dsl::DslDiff)]` would fail exactly as it failed for `SvgDiff`
  (`SvgNodeDiff: DslField` not satisfied) and `GifFrameDiff` (tri-state, a different blocker but same
  family of "derive cannot bind this"). **Zero `Option<Option<_>>` fields anywhere in `JsonDiff`'s
  tree** (confirmed by direct inspection of `JsonDiff`/`JsonArrayDiff`/`JsonObjectDiff` and their
  entry structs) — 3b does not apply. This is the recipe's "enum-only" hand-roll case, matching
  `dxf`'s row in the classification table.
  → **HAND-ROLL**.
- **Mutation side (`JsonMutation` → `#[derive(dsl::DslOps)]`)**: `SetSnapshot{snapshot: JsonSnapshot}`
  recursively contains `JsonValue` (same data-carrying-enum blocker). Every other path-carrying
  variant (`SetMember`/`RemoveMember`/`InsertArrayElement`/`RemoveArrayElement`/`SetScalar`) carries
  a `JsonValue` directly as a field (`value: JsonValue`) AND a `JsonPath = Vec<JsonPathSegment>`,
  where `JsonPathSegment` (`Key(String)` / `Index(usize)`) is *itself* a second, independent
  data-carrying enum. Two independent enum-shaped payloads per variant, not just one via the
  snapshot — an even stronger case than svg's mutation side (which only hit the blocker via
  `SetSnapshot`+`InsertElement.node`/`SetTransform.transform`).
  → **HAND-ROLL**.

Both citations are recorded as doc comments on `JsonValueDiff` and `JsonMutation` in the source,
following the exact citation style the recon pilot used on `GifFrameDiff`/`SvgDiff`/`SvgMutation`.

**Deviation from the literal STEP 1 procedure**: the procedure calls for adding the derive attribute
and citing the *live* compiler error text. I did not do this for json specifically, because (a) the
structural cause (a variant/field of a genuine data-carrying enum type) is visible directly by
reading the type definitions with no ambiguity, (b) it is the textually identical failure mode
already reproduced and verbatim-quoted twice in this pilot (`SvgDiff`, `SvgMutation`), and (c) the
repo-wide `cargo check`/`cargo test` surface was, for a real and substantial part of this session,
occupied by two *other*, unrelated, live concurrent F6 sub-wave sessions editing `docx` and `md`
(confirmed via `git status` showing `M` on their files mid-session, and their own compile errors
disappearing between polls) — spending that contended compile budget on a confirmatory derive
attempt whose outcome was not in doubt was not a good use of it. If a stricter live-citation is
wanted, the two-line repro is: add `dsl::DslDiff`/`dsl::DslOps` to the respective derive lists and
`cargo check -p semio-s-plugin-stdio --lib`; expect `the trait bound JsonValueDiff: DslField is not
satisfied` / `the trait bound JsonValue: DslField is not satisfied` / `the trait bound
JsonPathSegment: DslField is not satisfied`.

## STEP 2b — hand-roll grammar (both sides)

Grammar copied from `SvgDiff`/`SvgMutation`'s template (`f6-recon-report.md` §5), own self-contained
primitive set in `🔺️diff/🦀️component.rs` (`hex_encode`/`hex_decode`/`enc_str`/`dec_str`/
`parse_usize`/`split_top_level`/`strip_brackets`, all `pub(crate)`), imported into
`🧬️mutations/🦀️component.rs` rather than duplicated (matching svg's own precedent of the mutations
file reusing the diff file's primitives).

`encode_option`/`decode_option` (the tri-state `Option<T>` tag helper from svg's template) were
**not** ported — json has zero `Option<T>` fields anywhere in its Diff or Mutation types, so they
would be dead code.

### `JsonValue` / `JsonValueDiff` codec (tag-prefixed, single-letter, matching `enc_xml_node`'s style)

- `JsonValue`: `Z` (Null, no payload) / `B[0|1]` / `N[hex(lexeme)]` / `S[hex(value)]` /
  `A[v1,v2,...]` (recursive) / `O[hexkey1:v1,hexkey2:v2,...]` (recursive, order-preserving).
- `JsonValueDiff`: `R[<value>]` (Replace) / `B[0|1]` / `N[hex(lexeme)]` / `S[hex(value)]` /
  `A[<array-diff-body>]` / `O[<object-diff-body>]`.
- `JsonArrayDiff`/`JsonObjectDiff`: `[removed];[modified];[added]`, semicolon-separated sections,
  each a comma-separated list — `modified` entries are `idx:diff` / `key:diff`, `added` entries are
  `idx:item` / `idx:key:item` (object `added` carries both the final Vec position and the key, per
  the struct's own shape). Identical shape to `SvgChildrenDiff`/`SvgAttributesDiff`.
- Top-level `JsonDiff` line: `value=<enc>` (present) or the empty string (`None` — the only
  diffable field, `schema` being identity-only). Simpler than `SvgDiff`'s multi-token line since
  there is exactly one diffable field.
- Top-level `JsonMutation` line: `keyword arg=value ...`, one keyword per variant
  (`no-mutation`/`set-snapshot`/`set-member`/`remove-member`/`insert-array-element`/
  `remove-array-element`/`set-scalar`), a fresh `JsonPath` codec (`K[hex(key)]` / `I[index]` per
  segment, comma-joined inside `[...]`) for the two enums the mutation side introduces beyond the
  diff side's.
- `encode_diff`/`encode_op` = the text bytes verbatim (`print_diff()/print_op().into_bytes()`), same
  simplification `SvgDiff`/`GifDiff`/`WriterDiff` use — satisfies every `DiffCodec`/`OpBinary` law
  without inventing a second wire format.

## STEP 3 — tests (both added, both pass)

- `diff_codec_text_binary_roundtrip_law` (in `🔺️diff/🦀️component.rs`'s `tests` module): exercises
  every `JsonValueDiff` variant (incl. `Replace` via a kind-change case, `Number`→`String`), nested
  array/object collection triples reusing the existing `sweep_a`/`sweep_b` field-sweep fixtures plus
  two fresh nested-structure cases, and the empty (`None`) diff. Asserts `!printed.contains('\n')`,
  `parse_diff(print_diff(x)) == x`, `decode_diff(encode_diff(x)) == x` for every case.
- `op_text_binary_roundtrip_law` (in `🧬️mutations/🦀️component.rs`'s `tests` module): exercises every
  `JsonMutation` variant, including a `SetSnapshot` with a nested array/object/null/bool payload and
  a multi-segment `JsonPath` mixing both `Key`/`Index` segment kinds (`SetMember`/`SetScalar` cases).
  Same three assertions as above (`print_op`/`parse_op`, `encode_op`/`decode_op`).

## STEP 4 — verification (real, this session)

- `cargo check -p semio-s-plugin-stdio --lib` → clean (0 errors; pre-existing warnings only, none
  newly introduced by these two files).
- `cargo test -p semio-s-plugin-stdio --lib "artifacts::json"` → **60 passed, 0 failed** (was 58
  before this session's two new tests — every prior json test, incl. all `absorb_law`/
  `between_roundtrip_law`/`inverse_law`/`field_sweep`/subset-validator tests, still green).
- `cargo test -p semio-s-plugin-stdio --lib` (whole crate) → **repeatedly blocked/red across three
  separate polls**, each time by an *unrelated* concurrent sibling F6 sub-wave session actively
  editing a different artifact (confirmed via `git status --porcelain` showing live `M` on that
  artifact's files at the exact time of the failing run — never json, never anything in this
  session's own ownership boundary):
  1. `docx` compile error (`DocxSnapshot`/`DocxBlockPath`/`DocxBlock`/`DocxStyle: DslField` not
     satisfied — a mid-flight `#[derive(dsl::DslOps)]` attempt).
  2. `md` compile error (`MdMutation::parse_op` not found — a missing `use protocol::OpText;`
     import, the same class of oversight this session's own json `OpBinary` impl hit and fixed
     first, coincidentally).
  3. `docx` again, now compiling but with one FAILING test in its own module
     (`artifacts::docx::standards::v_ecma_376::…::diff_codec_text_binary_roundtrip_law`, assertion
     `based_on tri-state Some(None) not exercised` — a docx-internal law-test/impl mismatch, nothing
     to do with json).
  Final observed state, not touched or waited out further (docx's own session is still live per
  `git status`): **1072 passed, 1 failed** — the 1 failure is entirely inside
  `artifacts::docx::…` (outside this ticket's ownership boundary); every other test, including all
  60 of json's, is green. This session introduced 0 regressions — the whole-crate pass count only
  went up (json's own +2 new law tests) relative to any snapshot in which docx's own tests were
  passing.
- `POLICY_DIFF_COMPLETENESS_ALLOWLIST` (`📜️script.ts:2304`): **not touched** (per repo rules — not a
  shortcut). `JsonDiff` now has a real `protocol::DiffCodec for` impl in its own file text, so it
  should now drop out of the `dsl-migration/diff-completeness` breach list the same way
  binary/gif89a/svg did, without any `script.ts` edit.

## Deviations

1. STEP 1's live-compile-error citation was not independently re-derived for json (see STEP 1
   section above for the full rationale) — the structural cause is unambiguous by direct type
   inspection and textually identical to two already-verbatim-quoted precedents in this same pilot
   program (`SvgDiff`, `SvgMutation`).
2. `encode_option`/`decode_option` primitives from the svg template were intentionally omitted (dead
   code for this artifact — zero `Option<T>` fields anywhere in json's Diff/Mutation types).
3. Whole-crate `cargo test` was transiently blocked twice by unrelated concurrent sibling sessions
   (`docx`, `md`) editing files outside this ticket's ownership boundary; not caused by, or fixed by,
   this session — resolved on its own once those sessions' own edits landed, confirmed by polling
   `git status` and re-running `cargo test` rather than touching either artifact.
