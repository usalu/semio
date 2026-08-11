# W2b — video subset — real implementation report

Scope: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️video/**` only.

## What changed

Replaced the W1b full-replace scaffold with a real, spec-complete implementation:

- **Snapshot** (`🧬️schema/📸️snapshot/🦀️component.rs`): `SemioVideoSnapshot{schema, streams}`,
  `SemioVideoStream{kind: SemioVideoStreamKind, codec, width, height, rate: SemioRational,
  samples: Vec<SemioVideoSample>}`, `SemioVideoSample{pts, key, data}` — matches the master plan's
  spec line exactly (`streams{kind enum(video/audio/subtitle), codec, width, height,
  rate:Rational{num,den}, samples{pts,key,opaque data}}`). `SemioVideoStreamKind` is a named enum
  (Video/Audio/Subtitle); `SemioRational{num,den}` is a video-subset-owned named struct (not a
  bare tuple, not one of the shared `engine::geometry` types — `Rational` is video-specific).
  Sample `data` stays an opaque `Vec<u8>` — honest boundary, never decoded here.
- **Diff** (`🧬️schema/🔺️diff/🦀️component.rs`, 768 lines): hand-rolled sparse diff. Both
  collections (`streams`, and within a stream its `samples`) are plain ordered lists with no
  spec-mandated key, so both are diffed via the shared `engine::triples::IndexedTripleDiff<D,T>`
  (imported directly, not redefined). Local generic `between_indexed`/`apply_indexed`/
  `inverse_indexed`/`absorb_indexed` (ported from docx's own hand-rolled indexed engine) are reused
  at BOTH nesting levels. Real `protocol::MutationDiff`/`protocol::command::DiffAlgebra` impls, and
  a hand-rolled `protocol::DiffCodec` (bracket-depth-aware text grammar via the shared
  `engine::triples` split/strip helpers; binary = the text bytes verbatim, matching
  gif/svg/docx's own hand-rolled codecs).
- **Mutations** (`🧬️schema/🧬️mutations/🦀️component.rs`, 600 lines): 9-variant named enum
  (`NoMutation`, `SetSnapshot`, `InsertStream`, `RemoveStream`, `SetStreamMeta`, `InsertSample`,
  `RemoveSample`, `SetSampleData`, `SetSampleFlags`). Every variant's `diff()`/`inverse()` is
  hand-written (never apply-and-capture). Hand-rolled `protocol::OpText`/`protocol::OpBinary`
  (`<keyword> arg=value ...` grammar, reusing the diff module's `pub(crate)` value codecs). The
  pre-existing `📄set-snapshot/{🦠️mutation,🔺️diff,↩️inverse}` triad dir needed no changes (its
  generic delegation to `apply_semio_video_mutation`/`diff_set_snapshot`/`Mutation::inverse` still
  compiles against the new types unchanged).
- **Builder/Analyzer** (`🏗️builder`, `🧐️analyzer`): unchanged — both were already generic over the
  whole snapshot and needed no edits for the new field shape.
- **Composer** (`🎹️composer/🦀️component.rs`, 207 lines): unchanged compose path (analyzer-only,
  native dialect — matches the ✳️any-subset pattern every other semio subset composer follows).
  **Real `SubsetValidator`**: `check_semio_video_invariants` — `rate.den == 0` is a hard `Error`
  (divisor everywhere downstream frame timing is computed); a `Video`-kind stream with
  `width == 0 || height == 0` is a hard `Error` (not decodable by any real container reader);
  non-monotonic `pts` within a stream is a soft `Warning` (real containers legitimately reorder
  decode vs. presentation order for B-frames — honestly modeled as advisory, never a hard failure).
  Registration (`register()`) unchanged: schema descriptor + `register_document_codec::<SemioVideoSnapshot,
  SemioVideoMutation>("stdio.semio.video")` (repo-wide-unique id, per `store::register_document_codec`'s
  duplicate-id panic) + `register_subset_validator`.
- **Grammar leaves**: all 8 text (`.g4`/`.ebnf`/`.grammar.semio`/`.graphql`/`.json`/`.proto`/`.ts`/`.rs`)
  and 6 binary (`.abnf`/`.protocol.semio`/`.ksy`/`.spicy`/`.ts`/`.rs`) leaves handcrafted honest for
  all 3 facets (snapshot/diff/mutations) — no `*OCTET`/size-eos catch-alls; snapshot's
  envelope+hex(JSON) shape is genuinely described (the JSON body's own structure lives in the
  sibling JSON Schema, not duplicated); diff/mutations grammars fully spell out the `streams=`
  triple / `<keyword> arg=value` shapes field-by-field. Facet-level (snapshot/diff/mutations root)
  `.ts`/`.json`/`.graphql`/`.proto` mirrors rewritten to the real field shape (previously a
  copy-pasted generic `entries: {key,value}[]` placeholder). The root `SemioVideoArtifact` mirror
  (`🧬️schema/{🟦️.ts,🔣️.json,🔗️.graphql,🛰️.proto}`) also rewritten to match.
- **`io/component.rs`**: left as-is (structure-only doc comment) — real import/export leaves are
  explicitly W4 scope per the master plan, not this subset agent's job.

## Honest-boundary compliance

Per the master plan: video ships schema-complete for its own shape (streams/samples metadata) but
never attempts to decode compressed sample payloads — `SemioVideoSample.data: Vec<u8>` is always
carried as typed opaque bytes, whole-value replace only in the diff (matching dwg's precedent).

## Files touched (all within write scope)

- `🧬️schema/📸️snapshot/🦀️component.rs` — real snapshot types
- `🧬️schema/🟦️component.ts`, `🔣️component.json`, `🔗️component.graphql`, `🛰️component.proto` —
  real `SemioVideoArtifact` mirrors
- `🧬️schema/🔺️diff/🦀️component.rs` — real sparse diff + hand-rolled `DiffCodec`
- `🧬️schema/🧬️mutations/🦀️component.rs` — real mutation vocabulary + hand-rolled `OpText`/`OpBinary`
- `🎹️composer/🦀️component.rs` — real `SubsetValidator` invariant checks
- All `📸️snapshot/`, `🔺️diff/`, `🧬️mutations/` facet-level `.ts`/`.json`/`.graphql`/`.proto` mirrors
- All `📸️snapshot/📝️text/`, `📸️snapshot/💾️binary/`, `🔺️diff/📝️text/`, `🔺️diff/💾️binary/`,
  `🧬️mutations/📝️text/`, `🧬️mutations/💾️binary/` grammar leaves (8+6 per facet × 3 facets)

Not touched (out of write scope, correctly left to their owners): `🏗️builder/🦀️component.rs`,
`🧐️analyzer/🦀️component.rs`, `🚪️io/🦀️component.rs` (no changes needed), the `📄set-snapshot` triad
dir (no changes needed), `📦️glue.rs`, `📇️catalog.json`, `📜️script.ts` (all hot/closer-only files).

## Verification

`cargo check -p semio-s-plugin-stdio --lib` / `cargo test -p semio-s-plugin-stdio --lib
"artifacts::semio.*video"`: **zero errors under the `✳️video` path**, confirmed across FIVE
independent full-crate compile attempts during this session (raw output saved at
`w2b-video-check-poll1.txt`, `w2b-video-check-poll2.txt`, `w2b-video-test-poll1.txt`,
`w2b-video-test-poll2.txt`, `w2b-video-test-poll3.txt`) — total crate error counts across the 5
runs: 68, 63, 74, 56, 53, every single one entirely under SIBLING subsets' paths (`document`,
`object`, `presentation`, `mp4`, `image`, `audio`, `animation`, `cad`, `json`, …) — other
W2a/W2b/W3 agents' concurrent work-in-progress, monotonically shrinking as they land fixes.
**Zero of these errors were ever under `✳️video`, in any of the 5 runs.** This is a live instance
of this ticket's own
documented hazard ("Concurrent Cargo Workspace Churn" / hazard management §"Concurrent sessions") —
`semio-s-plugin-stdio` is one shared crate, and ~15-25 other wave agents were compiling it
concurrently throughout this session (confirmed via `ps aux`), each attempt genuinely queuing on
the shared `target/` build-directory file lock (`cargo`'s own reported status: "Blocking waiting
for file lock on build directory") for several minutes before getting a turn.

`cargo test -p semio-s-plugin-stdio --lib "artifacts::semio.*video"`: could not complete to a
GREEN state this session — three separate full attempts (`w2b-video-test-poll1.txt`,
`w2b-video-test-poll2.txt`, `w2b-video-test-poll3.txt`; error counts 74 → 56 → 53, monotonically
shrinking as sibling agents land fixes) every time compiled the ENTIRE crate cleanly through the
video subset and then failed on sibling-subset errors before test *execution* could begin (Rust
requires the whole crate to compile before any test binary runs — no way to "skip" unrelated
modules). The recurring offenders across all 3 attempts are `image`'s and `audio`'s
`SemioImageDiff`/`SemioImageMutation`/`SemioAudioDiff` missing `use crate::dsl::{DiffCodec,
OpText}` imports and `animation`'s `SemioAnimationDiff` missing `use crate::dsl::MutationDiff` —
all genuinely other agents' in-progress subsets (confirmed zero relation to anything video
touches), several still mid-edit per repeated `git status` checks across the session. I cannot fix
those myself (outside `✳️video/**`). **This is a real, currently-unmet exit-checklist item** — not
because the video implementation is wrong (5/5 independent full-crate compiles — 2 `cargo check` +
3 `cargo test` — confirm zero errors and zero warnings-as-errors anywhere under `✳️video/**`, only
2 harmless lint-level warnings matching the exact style docx/other real subsets already carry) but
because the shared crate as a whole isn't green while sibling waves are still landing. Recommend
the W2b verify/closer step re-run `cargo test -p semio-s-plugin-stdio --lib "artifacts::semio.*video"`
once W2a/W3 finish landing — the shrinking error trend (74→56→53) across this session's own 3
attempts suggests this should go green soon after.

`bun ./📜️script.ts policy` (full repo, raw output `w2b-video-policy-poll1.txt`): total
21524 breaches (W0 baseline was 21564 — net improvement, other waves' fixes outweighing this
wave's additions). Grepped exhaustively for `✳️video` — exactly 2 hits, both explained, neither a
defect in this implementation:
1. `taxonomy/emoji-prefix` on the `📄set-snapshot` mutation-triad dir name (missing U+FE0F on
   `📄`) — inherited verbatim from the W1b scaffold (I never renamed this dir); the identical
   `📄set-snapshot` name (same missing-variation-selector shape) is the repo-wide standard triad
   dir name used by gif and every other artifact with a `SetSnapshot` mutation, part of this
   category's pre-existing 454-entry (now 486-entry) baseline, not something scoped to this ticket
   or this subset to fix.
2. `os-state-authority/item-scope-global` on `🎹️composer/🦀️component.rs:116` (the
   `static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry>`) — this is the EXACT pattern this
   ticket's own brief instructed copying from `📄️pdf/🏅️standards/🔖️1.7/🪆️subsets/✳️a/🎹️composer/
   🦀️component.rs`, which independently verified trips this identical rule at its own
   `composer.rs:98` (same OnceLock shape) — confirmed pre-existing/sanctioned, not new pattern
   introduced by this subset. This category's repo total rose 240→269 (+29) this session, matching
   ~12 semio subsets' composers all landing the same mandated SubsetValidator registration pattern
   in parallel this wave — a known, designed consequence of the master plan's own "mandatory
   SubsetValidator, pdf/a composer is the copy template" instruction, not an error.

## Known gap requiring closer action (script.ts, out of my write scope)

- `📜️script.ts`'s `POLICY_DIFF_COMPLETENESS_ALLOWLIST` still lists
  `"stdio/semio/standards#v1-subsets-video-schema-diff-component"` — per `w1b-type-ownership.md`'s
  own instruction, this entry should come OFF the allowlist now that a real `🧰️triples`-backed
  sparse diff has landed (this ticket's own `SemioVideoDiff` is no longer full-replace). I cannot
  edit `script.ts` (hot file, closer-only per this ticket's hazard management) — flagging for the
  W2b closer.
- `📜️script.ts` defines `POLICY_FACET_MIRROR_DRIFT`/`POLICY_GRAMMAR_HONESTY`/`POLICY_DIFF_ALGEBRA`/
  `POLICY_FIELD_SWEEP_PRESENCE` (the S-8 rules from `🧬️schema-design.md`), but a full
  `bun ./📜️script.ts policy` run this session (below) confirms NONE of these 4 rule names appear
  anywhere in the current aggregate breach report (its 25 active rules are a different set) — they
  are evidently not yet wired into the default `policy` command's output, so despite writing real
  field-accurate facet mirrors for every video facet anyway (best practice regardless), there was
  no live gate to satisfy or risk tripping here today. Flagging only so a future wave that DOES
  wire these rules in doesn't mistake "zero breaches today" for "verified compliant."

## Shared infra gaps

None found specific to video — the `engine::triples`/`engine::geometry` shared infrastructure
worked exactly as documented (`IndexedTripleDiff<D,T>`'s `#[serde(default)]`-per-field shape does
require `D: Default` and `T: Default` for its `Deserialize` impl — video's own
`SemioVideoStreamDiff`/`SemioVideoStream`/`SemioVideoSampleDiff`/`SemioVideoSample` all derive
`Default`, which is why this bound was satisfied cleanly here — sibling subsets whose collection
item types don't derive `Default` hit exactly this as a real compile error during this session,
confirming the bound is real and this was a live pitfall, not hypothetical).
