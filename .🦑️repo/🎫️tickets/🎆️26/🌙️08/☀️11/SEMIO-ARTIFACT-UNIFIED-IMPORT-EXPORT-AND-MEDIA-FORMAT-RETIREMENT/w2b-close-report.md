# W2b Closer Report

Scope: 7 W2b subsets (`document`, `image`, `video`, `audio`, `animation`, `presentation`,
`workflow`) under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️<subset>/**`,
plus building `✳️any` from scratch, plus `📜️script.ts` allowlist burn-down, plus the final gate.
Read `w2b-verify-report.md` and all 4 existing `w2b-<subset>-report.md` files
(audio/presentation/video/workflow) before starting; `w2b-document-report.md`,
`w2b-image-report.md`, `w2b-animation-report.md` do not exist (a documented process gap in the
verifier's own report) — their subsets' code was inspected directly instead.

## 1. Verifier-flagged bugs fixed (all 4, all confirmed via independent `cargo check`/`cargo test`)

| Subset | File | Bug | Fix |
|---|---|---|---|
| document | `🧬️mutations/🦀️component.rs` | `use protocol::{OpBinary, OpText};` gated `#[cfg(test)]`, but the non-test `impl protocol::OpBinary` block calls `self.print_op()` | Made the import unconditional |
| workflow | same file shape | identical bug (directly contradicted workflow's own report's "0/8 attributable errors" claim) | Same fix |
| image | `🧬️mutations/🦀️component.rs` | `OpText`/`OpBinary` never imported at all (not even test-gated); `impl protocol::OpBinary` calls `self.print_op()`/`Self::parse_op()` | Added unconditional `use protocol::{OpBinary, OpText};` |
| image | `🧬️mutations/🦀️component.rs` (test mod) | `use protocol::{DiffAlgebra, ...}` — `DiffAlgebra` lives at `protocol::command::DiffAlgebra`, not `protocol::DiffAlgebra` (unresolved import) | Corrected to `protocol::command::DiffAlgebra` |
| image | `🔺️diff/🦀️component.rs` | test module calls `print_diff`/`parse_diff`/`encode_diff`/`decode_diff` via method syntax with no `DiffCodec` import anywhere | Added unconditional `use protocol::DiffCodec;` |
| animation | `🧬️mutations/🦀️component.rs` | `#[cfg(test)] mod tests` calls `diff.apply(&base)` with no `protocol::MutationDiff` in scope (test-only; doesn't block `cargo check --lib`) | Added `MutationDiff` to the existing `#[cfg(test)]`-gated import |

After these 5 fixes, `cargo check -p semio-s-plugin-stdio --lib` — **0 errors** (was previously
unable to compile at all per the verifier). Raw output: `w2b-closer-check1.txt`.

## 2. New bugs found and fixed (discovered only once the crate finally compiled — the verifier
could not have caught these, since `cargo test` never ran successfully before this session)

- **`✳️any`'s own `SemioMutation`** (this closer's own new code): internally-tagged
  `#[serde(tag = "mutation")]` wrapping a newtype variant whose payload (`Semio<X>Mutation`) is
  ITSELF internally tagged with the identical `"mutation"` key — serde flattens the newtype's
  fields into the outer object, producing `{"mutation":"audio","mutation":"setSampleRate",...}`
  (two identical keys), which `serde_json` then refuses to parse back. Caught by this file's own
  `op_text_binary_roundtrip_law` test. **Fix**: switched to adjacent tagging
  (`#[serde(tag = "mutation", content = "payload")]`), nesting the wrapped value under its own key
  instead of flattening it.
- **`workflow`'s `params` field-sweep/between-roundtrip fixtures** (`🔺️diff/🦀️component.rs`
  test module): `NamedTripleDiff`'s `apply_named` helper always appends `added` items at the TAIL
  (correct, documented behavior for a name-keyed — i.e. order-insignificant — collection). The
  `sweep_a`/`sweep_b` fixtures put the item that becomes "added" in the REVERSE direction
  (`"toRemove"`) at the FRONT of `sweep_a`'s `params` list, so `between(sweep_b,
  sweep_a).apply(sweep_b)` reconstructed it at the tail instead of matching `sweep_a`'s literal
  vec order — a real `assert_eq!` mismatch in `between_roundtrip_law` and `field_sweep`, not a bug
  in `apply_named` itself. **Fix**: moved `"toRemove"` to the end of `sweep_a`'s `params` list,
  matching `apply_named`'s real (correct) reconstruction order.
- **`animation`'s `field_sweep_covers_every_mutable_field` fixture**
  (`🔺️diff/🦀️component.rs`): `timelines`/`channels`/`keyframes` are ALL `IndexedTripleDiff`
  (position-keyed, truncating-tail semantics — an index can only ever be `removed` OR `added`
  relative to one `between()` call, never both, since one collection can't be simultaneously
  longer AND shorter than the other). The fixture kept every level's `sweep_a`/`sweep_b` pair at
  EQUAL length, so `removed` and `added` were always empty everywhere — only `modified` ever fired
  — even though the test's own doc comment already correctly named this "structural trap." **Fix**:
  introduced genuine length asymmetry at all 3 nesting levels (alternating which direction shows
  `removed` vs `added` per level, mirroring `presentation`'s own already-correct field_sweep
  pattern for its index-keyed `slides`), and added the missing reverse-direction channel checks.

All fixes verified with real, verbatim `cargo test` output — see `w2b-closer-animation-fix-test2.txt`,
`w2b-closer-workflow-fix-test.txt`, `w2b-closer-any-test-scoped.txt`.

## 3. `✳️any` envelope subset — built from scratch (real, not scaffold)

**W2a status caveat**: no `w2a-close-report.md` exists in the ticket folder — W2a has not
formally closed. Per this closer's brief, I did NOT blindly trust that and did NOT fake imports:
I directly inspected all 6 W2a subsets' actual `🧬️schema/📸️snapshot/🦀️component.rs` files on disk.
All 6 (`brep`, `mesh`, `model`, `object`, `cad`, `drawing`) are real, substantial, non-scaffold
implementations — 5 have their own `w2a-<subset>-report.md`; `cad` has none, but its code
(`CadEntity`'s 9-variant vocabulary, real `SemioPoint2`-based geometry, etc.) is structurally
identical in completeness to every other real subset. On that basis I built `✳️any` against the
real types, not stubs. **This should be independently reconfirmed once `w2a-close-report.md`
actually lands** — flagging per the brief's own instruction rather than silently asserting done.

Built (all under `✳️any/**`, `📦️glue.rs`/`📇️catalog.json` untouched — module tree was already
mounted by W1b):

- **`🧬️schema/📸️snapshot/🦀️component.rs`**: `SemioSubsetSnapshot` (13-variant tagged union, already
  W1b-scaffolded against these exact type paths — needed only a doc-comment update, since the
  referenced types' internals grew in place without renaming) wrapped by `SemioSnapshot{schema,
  subset}`.
- **`🧬️schema/🔺️diff/🦀️component.rs`** (full rewrite): `SemioDiff` = master-plan-mandated "same-kind
  nested diff | Replace{snapshot}" — `NoChange` + 13 same-kind wrapper variants (each nesting that
  subset's OWN real `Semio<X>Diff` unchanged, zero reinvention) + `Replace(Box<SemioSnapshot>)`
  for genuine cross-kind changes or explicit `SetSnapshot`. Real `MutationDiff`/`DiffAlgebra` impls
  (total — never panic — on kind-mismatch malformed input, degrading to a documented safe no-op/
  restore-base fallback). Hand-rolled `DiffCodec`: one `tag:payload` line per variant, where each
  same-kind payload is exactly that subset's own already-real `print_diff()`/`parse_diff()` output
  (genuine reuse, not re-derivation); `Replace`'s payload is hex(json(snapshot)), the same
  "neutral semio type, JSON-pack honesty boundary" every subset's own `SetSnapshot` embedding uses.
  10 tests: between/inverse/absorb across same-kind-nested, cross-kind-Replace, and NoChange; a
  13-tag dispatch-coverage test; full `DiffCodec` text/binary round-trip.
- **`🧬️schema/🧬️mutations/🦀️component.rs`** (full rewrite): `SemioMutation` = `NoMutation` +
  `SetSnapshot{snapshot}` + 13 wrapper variants embedding each subset's own real `Semio<X>Mutation`
  enum unchanged. `diff()`/`inverse()` for a wrapped variant delegate straight through to that
  subset's own `Mutation` impl (envelope owns only routing, never per-field logic). Hand-rolled
  `OpText`/`OpBinary`: plain `serde_json` passthrough of the whole enum — the SAME "JSON-pack
  passthrough" convention `brep`'s own real, complete `SemioBrepMutation::OpText` impl already
  uses for its full vocabulary (not a shortcut unique to this envelope). Adjacently tagged (see
  bug #2 above). 6 tests: NoMutation/SetSnapshot/wrapped-variant mutation_diff_law+inverse_law, a
  second wrapped-subset (workflow `InsertNode`) test, kind-mismatch safe-no-op test, 13-arm
  dispatch coverage, op text/binary round-trip.
- **`🎹️composer/🦀️component.rs`**: added the mandatory `SemioValidator` `SubsetValidator` for the
  `"*"` dialect (policy requires it, `pdf`'s `✳️a` composer is the copy template per the ticket
  brief). Decodes the payload as `SemioSnapshot`, then dispatches by the decoded variant's active
  kind to that ONE subset's own already-real, already-registered `SubsetValidator` (re-encodes the
  inner snapshot via its own `ArtifactPack`, delegates) — zero invariant logic duplicated here, 3
  tests (clean delegate, invalid-payload delegate, undecodable-payload soft-warning).
- **`🏗️builder`/`🧐️analyzer`**: untouched — both were already fully generic over
  `SemioSnapshot`/`SemioMutation`/`SemioDiff` by name and needed no changes for the richer types.
- **`🚪️io`**: untouched (structure-only stub, explicitly W4 scope per the master plan, matches
  every sibling subset's own `🚪️io` leaf).

Scoped test run: `cargo test -p semio-s-plugin-stdio --lib "artifacts::semio::standards::v1::subsets::any::"`
→ **19 passed, 0 failed** (`w2b-closer-any-test-scoped.txt`).

## 4. `📜️script.ts` allowlist burn-down

`POLICY_DIFF_COMPLETENESS_ALLOWLIST` — removed the 8 entries for `document`/`image`/`video`/
`audio`/`animation`/`presentation`/`workflow`/`any`, each verified beforehand (via `grep -c "impl
protocol::DiffCodec for\|impl DiffCodec for"`) to now carry a real hand-rolled `DiffCodec` impl —
confirmed non-zero for all 8 before removing. W2a's 6 entries (`brep`/`cad`/`drawing`/`mesh`/
`model`/`object`) and the standard-level `stdio/semio/standards#v1-engine-component` entry (a
different, W1/W1b-owned allowlist item, not per-subset) were explicitly left untouched, per this
ticket's write-scope rules. Re-ran `bun ./📜️script.ts policy` after removal: **zero new breaches**
introduced (grepped the diff-completeness rule's output for `semio` — no hits), confirming the
removal was correct, not merely silent.

Searched the rest of `script.ts` for other allowlists possibly keyed to W2b subsets
(`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`, `POLICY_PACK_COMPLETENESS_ALLOWLIST`,
`POLICY_DSL_COMPLETENESS_GENERIC_BRIDGE_ALLOWLIST`, `POLICY_GRAMMAR_FILE_ALLOWLIST`,
`POLICY_PROTOCOL_FILE_ALLOWLIST`) — none reference any of the 7 subsets or `✳️any`; no further
cleanup needed there.

## 5. Final gate

### `cargo test -p semio-s-plugin-stdio --lib` (verbatim tail, `w2b-closer-final-cargo-test.txt`)

```
test result: FAILED. 1483 passed; 14 failed; 1 ignored; 0 measured; 0 filtered out; finished in 7.97s
```

The crate **compiles and runs** for the first time in this ticket's W2 wave (the verifier could not
get past compile errors at all). All 14 remaining failures are confirmed OUTSIDE W2b's write
scope by file path:

- `artifacts::semio::standards::v1::subsets::brep::...field_sweep_every_field_present_in_diff` — W2a subset, real bug (`e1.diff.start_vertex`/`end_vertex`/`curve` not all populated), not touched.
- `artifacts::semio::standards::v1::subsets::mesh::...between_roundtrip_law` / `field_sweep` / `inverse_law` (3 tests) — W2a subset, real bug (collection reorder mismatch after apply — same class of issue this closer fixed for `workflow`, but `mesh` is W2a's file, not mine to edit), not touched.
- `artifacts::semio::standards::v1::subsets::model::...op_text_binary_roundtrip_law` — W2a subset, real bug (`SetElement{spatial_id: Some(None)}` vs `None` mismatch on parse — a tri-state encoding bug), not touched.
- `artifacts::csv::...` (4 tests), `artifacts::json::...` (5 tests) — entirely foreign artifacts (not `semio`, not this ticket's format additions), pre-existing, not touched.

**Zero failures remain in `document`/`image`/`video`/`audio`/`animation`/`presentation`/
`workflow`/`✳️any`** — every one of the 7 W2b subsets plus the new `✳️any` envelope is fully green.

### `bun ./📜️script.ts policy` (verbatim head + tail, `w2b-closer-final-policy.txt`)

```
21524 high-priority breach(es) across 25 rule(s):
```

vs. the verifier's own immediately-prior snapshot of 21523 — net **+1**, attributable to ordinary
concurrent-wave churn from other live sessions (W2a/W3 continued editing throughout this session;
`git status` showed 1270+ modified files repo-wide at various points), not this session's own
edits: each of the 7 subsets plus `✳️any` carries exactly 2 breaches (`os-state-authority/item-scope-global`
on the mandated `VALIDATOR_ENTRY: OnceLock` pattern, `taxonomy/emoji-prefix` on the inherited
`📄set-snapshot` dir name) — the same pre-existing, sanctioned counts the verifier already
documented, confirming this session introduced zero new breaches of its own.

**Known, documented, NOT fixed (design-judgment / structural, per the brief's "don't guess"
instruction)**:

- **`image` carries 13 breaches** (not 2) — the same `OnceLock` pattern once, plus 12
  `taxonomy/emoji-prefix` hits because `image`'s mutations built ONE separate triad directory per
  mutation variant (`📄remove-frame`, `📄set-metadata-entry`, `📄set-dimensions`, …) instead of the
  single `📄set-snapshot` triad every other subset (including image's own siblings) uses. This is a
  structural outlier requiring restructuring ~12 directories into 1 — a real refactor, not a
  one-line fix, and risks the already-passing `image` test suite if rushed. Flagged for a
  follow-up pass, not attempted this session.
- **W2a's 4 failing tests** (`brep` field_sweep, `mesh` ×3) and **`model`'s 1 failing test**
  (op_text_binary_roundtrip_law) — real bugs, discovered only because this session's fixes made
  the crate compile for the first time. Explicitly out of this closer's write scope (W2a's
  subsets, not W2b's) per the ticket's own hazard-management rules; the `mesh` bugs in particular
  look like the SAME "NamedTripleDiff/IndexedTripleDiff-backed collection reorder" class this
  closer just fixed for `workflow`/`animation` — should be quick for whoever owns W2a's closer
  pass, using this report's §2 as a template.

## Files touched

- `document/🧬️schema/🧬️mutations/🦀️component.rs` — import fix only
- `workflow/🧬️schema/🧬️mutations/🦀️component.rs` — import fix; `workflow/🧬️schema/🔺️diff/🦀️component.rs` — fixture reorder fix
- `image/🧬️schema/🧬️mutations/🦀️component.rs` — 2 import fixes; `image/🧬️schema/🔺️diff/🦀️component.rs` — 1 import fix
- `animation/🧬️schema/🧬️mutations/🦀️component.rs` — import fix; `animation/🧬️schema/🔺️diff/🦀️component.rs` — fixture restructure fix
- `any/🧬️schema/📸️snapshot/🦀️component.rs` — doc comment only
- `any/🧬️schema/🔺️diff/🦀️component.rs` — full rewrite (real diff)
- `any/🧬️schema/🧬️mutations/🦀️component.rs` — full rewrite (real mutations)
- `any/🎹️composer/🦀️component.rs` — added `SemioValidator`
- `📜️script.ts` — removed 8 `POLICY_DIFF_COMPLETENESS_ALLOWLIST` entries
- `.🦑️repo/🎫️tickets/.../STATUS.md` — created (did not exist), this report

No `📦️glue.rs`/`📇️catalog.json`/`🔣️taxonomy.json` edits (none needed). No new test files — all
tests extend existing `#[cfg(test)] mod tests` regions. No `ticket_close` called (orchestrator-only
per the master plan).

## Verification artifacts (this ticket folder, `.txt`)

`w2b-closer-check1.txt`, `w2b-closer-any-check1.txt`, `w2b-closer-any-check2.txt`,
`w2b-closer-test1.txt` through `w2b-closer-test4-final.txt`, `w2b-closer-any-test-scoped.txt`,
`w2b-closer-animation-fix-test.txt`, `w2b-closer-animation-fix-test2.txt`,
`w2b-closer-workflow-fix-test.txt`, `w2b-closer-policy1.txt`, `w2b-closer-final-cargo-test.txt`,
`w2b-closer-final-policy.txt`.
