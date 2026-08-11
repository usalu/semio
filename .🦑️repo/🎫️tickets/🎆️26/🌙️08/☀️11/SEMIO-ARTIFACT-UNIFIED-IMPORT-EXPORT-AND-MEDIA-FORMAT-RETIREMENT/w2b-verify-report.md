# W2b Independent Verification Report

Verifier re-read the actual code for all 7 W2b subsets (document, image, video, audio, animation,
presentation, workflow) rather than trusting the agent reports. Confirmed: **the crate does not
currently compile**, and this is caused by real bugs inside 3 of the 7 W2b subsets themselves
(document, image, workflow) — not purely "foreign concurrent churn" as most reports claim.

## 0. Report inventory — CRITICAL GAP

Only 4 of the 7 required `w2b-<subset>-report.md` files exist in the ticket folder:
`w2b-audio-report.md`, `w2b-presentation-report.md`, `w2b-video-report.md`,
`w2b-workflow-report.md`. **`w2b-document-report.md`, `w2b-image-report.md`, and
`w2b-animation-report.md` do not exist anywhere in the ticket folder or repo.** Per CLAUDE.md
("You MUST create a markdown file inside the ticket folder for every research or summary you
do"), this is itself a process violation. The code for all three subsets does exist on disk and
is clearly real, substantial work (not stubs) — so the agents did the work but never wrote/lost
their closing report. Verification below covers all 7 subsets by reading code directly regardless
of report presence.

## 1–4, 6. Per-subset code checks (diff/mutation hand-rolling, catch-alls, DIALECT/WRITES,
SubsetValidator, cross-reuse legitimacy)

| Subset | Apply-and-capture (`diff()` re-derives via `.apply()`+re-diff) | `snapshot: Option<Snapshot>` full-replace escape hatch | Bare catch-all hiding real diffs | `DIALECT`/`WRITES` matches own path | Real (non-stub) `SubsetValidator` registered |
|---|---|---|---|---|---|
| document | None found — every `diff()`/`inverse()` hand-written | None (only in doc-comment prose disclaiming it) | `_ =>` at `diff_block:633` is legit (cross-*variant* fallback to `Replace`, e.g. Paragraph→Heading; same-variant pairs are field-by-field) | ✅ `subset: SubsetId("document")` | ✅ `SemioDocumentValidator`, real checks |
| image | None found | None (doc-comment only) | `other =>` arms are decode-error formatters only | ✅ `SubsetId("image")` | ✅ `SemioImageValidator` |
| video | None found | None (doc-comment only) | decode-error formatters only | ✅ `SubsetId("video")` | ✅ `SemioVideoValidator`, checks `rate.den==0`, dims==0, pts monotonicity |
| audio | None found | None (doc-comment only) | decode-error formatters only | ✅ `SubsetId("audio")` | ✅ `SemioAudioValidator` |
| animation | None found | None | decode-error formatters only | ✅ `SubsetId("animation")` | ✅ `SemioAnimationValidator` |
| presentation | None found | None (doc-comment only) | `_ =>` at `diff_shape:614` is the same legit cross-variant fallback pattern as document | ✅ `SubsetId("presentation")` | ✅ `SemioPresentationValidator`, checks dangling master/layout refs + duplicate ids |
| workflow | None found | None (doc-comment only) | decode-error formatters only | ✅ `SubsetId("workflow")` | ✅ `SemioWorkflowValidator`, node/edge id uniqueness + edge endpoint existence |

**All 7 PASS** on the red-flag greps (#1, #2) and on DIALECT/WRITES + SubsetValidator (#3, #4).

## 5. Video/audio honest-boundary framing

**PASS for both.** Read `SemioVideoSnapshot`/`SemioVideoSample` directly: `data: Vec<u8>` is a
genuinely typed opaque byte payload, doc-commented as "never decoded here," diffed whole-value —
no fabricated decode claim anywhere in the diff/mutation/composer code. Read
`SemioAudioSnapshot`/`SemioAudioChannel` directly: `samples: Vec<f32>` are real decoded samples
(this subset is explicitly NOT payload-opaque, per its own doc comment, matching the master
plan's `audio: sample_rate/format + channels{f32 samples}` spec line), diffed per-index via
`IndexedTripleDiff`. Both framings match the master plan's own distinction exactly.

## 6. Presentation's document-block reuse

**PASS — confirmed spec-mandated, not an illegal import.** Presentation's
`🧬️schema/{📸️snapshot,🔺️diff,🧬️mutations}/🦀️component.rs` all directly `use
...subsets::document::schema::snapshot::{DocBlock, DocListItem, DocRun, DocTableCell,
DocTableRow, RunStyle};`. This IS a cross-subset type import, but `w1b-type-ownership.md` (the
authoritative type-ownership doc, not the looser master-plan.md prose) explicitly sanctions
exactly this: *"presentation mirrors document's block shape with its own type names —
`SlideShape::TextBox` reuses `document::DocBlock` directly (**not** a presentation-local copy)."*
This is the same class of sanctioned exception as model embedding brep/mesh snapshots by id. Not
a violation.

## 7. Independent re-run: `cargo test -p semio-s-plugin-stdio --lib "artifacts::semio" 2>&1 | tail -80`

**FAIL — the filtered test target does not compile, so no pass/fail numbers exist to verify.**
Ran twice (concurrent repo activity from ~27 other live `cargo` processes changed the exact error
count between runs — 62 then 55 — but the crate never compiled in either run). The compile
errors are **not all foreign** as the reports variously claim:

- `document/🧬️schema/🧬️mutations/🦀️component.rs`: real bug, **currently still present**. Line
  24 gates `use protocol::{OpBinary, OpText};` behind `#[cfg(test)]`, but the non-test `impl
  protocol::OpBinary for SemioDocumentMutation` block (line 630) calls `self.print_op()` — a
  production code path that needs the trait in scope unconditionally. `cargo check --lib`
  (no test cfg) reproduces this independently of the `cargo test` run.
- `workflow/🧬️schema/🧬️mutations/🦀️component.rs`: **identical bug**, line 17
  `#[cfg(test)] use protocol::{OpBinary, OpText};`, called from the non-test `impl
  protocol::OpBinary` block. This directly **contradicts workflow's own report**, which claims
  "Every single run showed zero compile errors attributable to any ✳️workflow file (0/8)" — that
  claim does not hold against my independent `cargo check`.
- `image/🧬️schema/🧬️mutations/🦀️component.rs`: same `OpText`/`print_op`/`parse_op` bug (import
  missing entirely outside its `#[cfg(test)] mod tests`, not even gated — never present at
  module scope), **plus** `image/🧬️schema/🔺️diff/🦀️component.rs`'s test module lacks a
  `DiffCodec` import so `print_diff`/`parse_diff`/`encode_diff`/`decode_diff` all fail to resolve
  as methods on `SemioImageDiff` (the trait impl itself, lines 536–553, is real and correct — it's
  purely a missing `use` at the call site), **plus** an unresolved `protocol::DiffAlgebra` import
  in the mutations test module (should be `protocol::command::DiffAlgebra`). Image is the most
  broken of the 7 subsets and, notably, the one with no closing report.
- `animation/🧬️schema/🧬️mutations/🦀️component.rs`: one real (test-only) bug — the `#[cfg(test)]
  mod tests` block calls `diff.apply(&base)` without `protocol::MutationDiff` in scope anywhere in
  the file (not even fully-qualified there, unlike its own `OpText`/`OpBinary` impls which
  correctly use `<Self as protocol::OpText>::print_op(...)` fully-qualified syntax and so don't
  need the import). This is narrower than document/image/workflow's bug — it only blocks `cargo
  test`, not `cargo check --lib` (confirmed: animation does not appear in the plain `cargo check
  --lib` error list).
- audio, video, presentation, brep, mesh, model, cad, drawing: **zero errors attributable to
  these** in either run — confirmed by grepping every `error[...]`'s `-->` path across both full
  transcripts.

Net: **3 of 7 W2b subsets (document, image, workflow) currently ship a genuine, reproducible
production-code compile error**, not merely "blocked by siblings." The video/audio/presentation
reports' framing of this as pure foreign concurrent-churn is **partially wrong** — for workflow
specifically it's contradicted outright by its own report's "0/8 attributable errors" claim.

## Full gate

### `cargo test -p semio-s-plugin-stdio --lib 2>&1 | tail -20`

Not separately re-run — the scoped `artifacts::semio` filter above already proves the whole crate
fails to compile (Rust compiles the entire lib+test target before any filter is applied), so a
full unscoped run would show the identical 3 subset bugs plus whatever unrelated W2a/W3 breakage
exists concurrently. **Cannot report pass/fail counts — none exist.** This is a real regression
against the W1b baseline (`w1b-close-report.md`): **1231 passed; 0 failed** on a clean compile.
No `w2a-close-report.md` exists yet either (W2a is apparently still in flight, consistent with the
heavy concurrent `cargo` activity observed: ~27 processes, 810 modified files under `git status`
at check time).

### `bun ./📜️script.ts policy 2>&1 | tail -40`

**21523 high-priority breaches across 25 rules**, vs. W1b baseline's 21513 — net **+10**, in line
with (slightly worse than) the individual reports' self-measured deltas (workflow: 0 new; audio: 0
new; video: 0 new but reported repo total 21524; presentation: unmeasured, blocked). Policy is a
static-analysis pass (does not require compilation), so this number is trustworthy independent of
the compile failure above. Breach breakdown for the 7 subsets specifically: document/video/audio/
animation/presentation/workflow each carry exactly 2 (both pre-existing, sanctioned patterns —
`os-state-authority/item-scope-global` on the mandated `VALIDATOR_ENTRY: OnceLock` pattern copied
from pdf's `✳️a` composer, and `taxonomy/emoji-prefix` on the inherited `📄set-snapshot` dir name
missing a variation selector). **image carries 13** — the same `OnceLock` pattern once, plus 12
`taxonomy/emoji-prefix` hits because image's mutations built **one separate triad directory per
mutation variant** (`📄remove-frame`, `📄set-metadata-entry`, `📄set-dimensions`, …) instead of the
single `📄set-snapshot` triad every other subset (including image's own siblings) uses — a
structural outlier, each instance is the same pre-existing missing-U+FE0F-selector taxonomy issue
multiplied by extra directories, not a new class of bug, but worth the closer's attention since no
other subset did this.

## Verdict table

| Subset | Report exists | Diff/mutation hand-rolled, no apply-and-capture | No catch-all hiding real diffs | DIALECT/WRITES correct | Real SubsetValidator | Honest boundary (video/audio) | Cross-reuse legit (presentation) | Compiles clean | **Verdict** |
|---|---|---|---|---|---|---|---|---|---|
| document | ❌ missing | ✅ | ✅ | ✅ | ✅ | n/a | n/a | ❌ real bug (OpText scope) | **FAIL** |
| image | ❌ missing | ✅ | ✅ | ✅ | ✅ | n/a | n/a | ❌ 3 distinct real bugs, worst of the 7 | **FAIL** |
| video | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | n/a | ✅ (own files) | **PASS** (own scope) |
| audio | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | n/a | ✅ (own files) | **PASS** (own scope) |
| animation | ❌ missing | ✅ | ✅ | ✅ | ✅ | n/a | n/a | ❌ real bug (MutationDiff scope, test-only) | **FAIL** |
| presentation | ✅ | ✅ | ✅ | ✅ | ✅ | n/a | ✅ | ✅ (own files) | **PASS** (own scope) |
| workflow | ✅ | ✅ | ✅ | ✅ | ✅ | n/a | n/a | ❌ real bug (OpText scope) — **contradicts own report** | **FAIL** |

## Overall verdict: **FAIL**

4 of 7 subsets (document, image, animation, workflow) have real, independently-reproduced compile
errors in their own files — not foreign churn — so `semio-s-plugin-stdio` does not compile and no
`cargo test` pass/fail numbers exist to check against the W1b baseline's 1231/0. video, audio, and
presentation's own files are individually clean (`cargo check` isolates cleanly to their scope)
but cannot be certified via `cargo test` either, since the shared crate as a whole must compile
first. Additionally, 3 of 7 required closing reports (document, image, animation) were never
written, a CLAUDE.md process violation independent of the code defects. Policy is roughly flat
(+10 breaches, non-blocking) and not the limiting factor. **The W2b wave is not ready to close**
— document, image, workflow need their `OpText`/`OpBinary` scope bug fixed (move the import out
from behind `#[cfg(test)]`, or switch the two `self.print_op()`/`Self::parse_op()` call sites to
animation's own fully-qualified-syntax pattern, which already avoids the issue correctly), image
additionally needs its `DiffCodec`/`DiffAlgebra` test-module imports fixed, and animation needs
`protocol::MutationDiff` imported for its test module. All four fixes are each a 1–2 line import
change, not a design problem — every other law/architecture check above passes.
