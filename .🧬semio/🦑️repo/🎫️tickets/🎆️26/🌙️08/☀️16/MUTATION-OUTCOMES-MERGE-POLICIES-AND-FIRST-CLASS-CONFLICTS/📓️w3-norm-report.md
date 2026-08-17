# Lane R2 (resumed W3-A/B/C) — norm plugin — report

**Lease:** `✏️s/🔌️plugins/📕️norm/**` only.

## Scope covered

All 111 `🔺️diff` leaves originally listed as missing `MutationOutcome` (verified via the brief's
exact discovery command, saved to `🧪️w3-norm-r2-*` scripts + `norm-leaves.txt` equivalent) are now
converted, across facets: din4108 (21), din18599 (13), en1993 (17), en1995 (20), iso16757 (21),
vdi3805 (19). A final full-tree rescan of `✏️s/🔌️plugins/📕️norm` (using `find -print0 | xargs -0
grep -L MutationOutcome`, safe against the emoji-heavy filenames — an earlier `$(find …)` word-split
scan produced a spurious non-empty result and was discarded) confirms **zero** `🔺️diff` leaves in the
whole plugin still lack `MutationOutcome`.

Each leaf follows the frozen verb-family table: `change`/`update`/`set` → Error absent / Warning
`mutation.no-op` same / Fatal `mutation.invariant` domain; `create` → Fatal `mutation.duplicate-id` /
Warning `mutation.clamped` on an out-of-range explicit insert index; `delete`/`remove` → Error
`mutation.target-missing`; `rename` → Error / no-op / (Fatal collision where applicable); `add` → Error
owner-absent / Warning no-op-present; `resize` → Fatal `mutation.invariant` on non-finite/inverted
extent; `reorder`/`insert` → Warning `mutation.clamped`; `replace`/`edit` → Error / no-op; cascade
deletes keep their `mutation.cascade` Info. Root-scoped whole-facet updates with no addressable target
(e.g. en1993's 17 `update-*-inputs` leaves, `update-script-limits`, `update-climate`) got Fatal-on-domain
+ Warning-no-op but no Error (no missing-target case applies), per the fan-out recipe's allowance.

Also converted (hand-written, non-generated):
- `✏️s/🔌️plugins/📕️norm/🎚️config/🦀️component.rs` — `NormConfigMutation` (`impl Mutation<NormConfig>`).
- `✏️s/🔌️plugins/📕️norm/👥️presence/🦀️component.rs` — `NormPresenceMutation`.
- `✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️component.rs` — generic `SetArtifactMutation<D>` shared by all
  15 norm families.

All three whole-record self-diff types follow the CAD-plugin precedent (`MutationOutcome::new(base
.clone()).warn("mutation.no-op", …)` instead of `::empty()`, since their `MutationDiff::apply` is a
non-sparse full replace where an all-default diff would wrongly reset state) — except
`SetArtifactMutation<D>`, whose `ArtifactDiff<D>` **is** sparse (`document: Option<D>`, `apply` falls
back to the current projection on `None`), so `::empty()` is safe and used there.

No `fn validate` overrides existed on `Mutation`/`MutationKind` anywhere in the lease (the only
`validate*` hits are unrelated domain-structure validators — `validate_structure`,
`validate_catalogue_structure`, `validate_dictionary`, IO codec `validate_payload` — left untouched).
No CRDT vocabulary (`merge_strategy`, `ConflictRule`, `merge_concurrent_diffs`, `ResolutionPlan`,
`assert_crdt_*`) or `Severity::Hint` remains anywhere in the lease. Every `mutation.*` code used is
one of the frozen 7 (`mutation.apply` also appears, but it is a `dsl::Diagnostic` code in the
existing `mutate()` glue, an unrelated diagnostic namespace, not a `MutationMessage` code).

## Call-site fixes (inside the lease, beyond the 111)

The 111-leaf conversion left several call sites across the **whole plugin** (not just my 6 facets —
some in facets an earlier lane had already converted: din16798, en1998, en1999, en1997) still treating
`Mutation::diff()`'s return as a bare `Diff` instead of `MutationOutcome<Diff>`. Fixed:
- `🔺️diff/📝️text/🦀️component.rs` unit tests in en1995, en1998, en1999, en1997 (`let diff = ….diff(&base); diff.apply(&base)` → `let outcome = ….diff(&base); outcome.diff().apply(&base)`).
- `🧬️mutations/🦀️component.rs` dispatch-enum `round_trip` test helpers in din4108, vdi3805, iso16757, en1993 (`operation.diff(base).apply(base)` → `.diff(base).diff().apply(base)`).
- `🧬️mutations/🦀️component.rs` `assert_mutation_diff_absorb_law` call sites in din18599, din16798, en1995, en1998, en1999, en1997 (`let d1 = ….diff(&base);` → `.diff(&base).diff().clone();`, matching the pattern already used by en1990/en1991/en1992/en1994/en1996).
- en1993's `change_annex_diff_is_sparse` test (direct field access on the outcome → `outcome.diff()` first).
- `🎚️config/🦀️component.rs`'s own inverse test.

A broad final re-grep (`find … -name '🦀️component.rs' | xargs grep '\.diff(&'`, filtered for anything
not already `.diff().clone()`/`.diff().apply(`/`let outcome =`) confirms no remaining unconverted call
site in the lease.

## Verify

Crate name confirmed from Cargo.toml: **`semio-s-plugin-norm`**.

```
cargo check -p semio-s-plugin-norm
cargo test -p semio-s-plugin-norm --lib
```

**Both commands are blocked before reaching the norm crate at all** — `semio-s-plugin-norm` depends on
`semio-s-plugin-stdio` (`📦️packages/🦀️rust/Cargo.toml:101`), and `semio-s-plugin-stdio` itself fails to
compile: **197 errors, 0 of them inside `✏️s/🔌️plugins/📕️norm/**`** (verified: `grep "^error\[" … | grep
-v 🗄️stdio` → empty; the sole `error: could not compile …` line names only `semio-s-plugin-stdio`).
Both `cargo check` and `cargo test` produce the identical 197-error / 228-warning stdio failure
(full logs: `🧪️w3-norm-check.txt`, `🧪️w3-norm-test.txt`).

Representative failures, all inside stdio, all pre-existing and outside this lease:
- `✏️s/🔌️plugins/🗄️stdio/…/💾️binary/…/🧬️mutations/🦀️component.rs:109` — `error[E0405]: cannot find trait
  OpText in this scope` (`impl OpText for BinaryMutation`, missing import).
- `✏️s/🔌️plugins/🗄️stdio/…/📄️pdf/…/✏️editor/🦀️component.rs:102` — `error[E0277]: the trait bound
  PdfMutation: OpBinary is not satisfied`.
- Same shape repeated across ~30 stdio artifact families (svg, step, xlsx, docx, pptx, tiff, gif,
  html, epw, ply, mp4, …) — `OpText`/`OpBinary` not implemented/not in scope for their mutation enums.

This is exactly the cross-lane dependency the worker brief calls out: "stdio's legacy enums are
FULL-STDIO's charter — our lane wraps them minimally". The `26/08/16/FULL-STDIO-ARTIFACT-STANDARDS-
CODECS-INFERENCES-AND-MUTATIONS` ticket is confirmed **open** (mid-flight) via its `🎫️ticket.json`, and
the blocking files' `git log --date=iso` shows the same very-recent commit as the rest of the live
tree — consistent with an in-progress concurrent lane, not a regression I introduced. I did not touch
any file under `✏️s/🔌️plugins/🗄️stdio/**`. A retry (`🧪️w3-norm-check-retry.txt`) minutes later shows the
stdio error count dropping from 197 → 176 (warnings 228 → 229) with the exact same error shapes —
confirming FULL-STDIO is actively converging live, not stuck; re-running `cargo check -p
semio-s-plugin-norm` once it lands should be a clean pass for this lane's own code.

**I cannot paste a passing cargo check/test count for `semio-s-plugin-norm` itself** — the crate is
never reached. What I can and did verify: `grep -L MutationOutcome` returns nothing for any `🔺️diff`
leaf in the lease (0/111 remaining), and a repo-wide search for stray un-migrated `.diff(&base)` call
sites in the lease returns nothing. This is a static/textual verification, not a compiler-verified one
— report to the coordinator that norm's `cargo check` is blocked on FULL-STDIO and needs re-running
once that ticket lands.

## Files touched (all inside `✏️s/🔌️plugins/📕️norm/**`)

111 `🔺️diff` + sibling `🦠️mutation` leaves across din4108, din18599, en1993, en1995, iso16757, vdi3805
(see `🧪️w3-norm-r2-scalar-convert.ts`, `🧪️w3-norm-r2-wrap-mutation-signatures.ts`,
`🧪️w3-norm-r2-en1993-inputs.ts`, `🧪️w3-norm-r2-en1993-label-cleanup.ts`, `🧪️w3-norm-survey.ts` for the
scripted pass; ~55 leaves were hand-converted for non-uniform verbs), plus:
- `🎚️config/🦀️component.rs`, `👥️presence/🦀️component.rs`, `📄️artifact/🦀️component.rs`
- `🗿️artifacts/{din4108,din16798,din18599,en1993,en1995,en1997,en1998,en1999,vdi3805,iso16757}/🏅️standards/🔖️…/🧬️schema/🧬️mutations/🦀️component.rs` (dispatch-enum test call-site fixes)
- `🗿️artifacts/{en1995,en1997,en1998,en1999}/🏅️standards/🔖️…/🧬️schema/🔺️diff/📝️text/🦀️component.rs` (unit-test call-site fixes)

Scratch scripts/logs (kept per CLAUDE.md, all in this ticket folder): `🧪️w3-norm-r2-scalar-convert.ts`,
`🧪️w3-norm-r2-wrap-mutation-signatures.ts`, `🧪️w3-norm-r2-en1993-inputs.ts`,
`🧪️w3-norm-r2-en1993-label-cleanup.ts`, `🧪️w3-norm-survey.ts`, `🧪️w3-norm-check.txt`,
`🧪️w3-norm-test.txt`.
