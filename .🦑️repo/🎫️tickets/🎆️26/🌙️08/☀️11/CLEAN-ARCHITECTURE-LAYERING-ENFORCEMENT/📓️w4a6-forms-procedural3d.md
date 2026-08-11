# w4a6 — forms + procedural3d: closed `Contribution` deletion

## Scope
Final destructive cut for this mechanism (wave 4a6): delete the closed `Contribution` fallback path and
its producers/tests entirely in the assigned files. No compatibility shims.

Files touched:
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🦀️component.rs`
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

## forms/apps/forms/component.rs
- `question_kind_route_from_contribution` (closed-shape match on both `Contribution::FormsQuestionKind`
  and legacy `Contribution::PlaybookBlockKind`) — deleted entirely.
- `find_question_kind_contribution` — dropped the `.or_else(|| question_kind_route_from_contribution(...))`
  fallback; now the open `topic_contribution` read is the sole path. Doc comment updated to drop the
  "reads both shapes, prefers open" framing.
- `catalogue_kinds` — dropped the `match &entry.contribution { Contribution::FormsQuestionKind {..} =>
  ..., Contribution::PlaybookBlockKind {..} => ..., _ => {} }` fallback block entirely; only the
  `topic_contribution`-decode branch remains (entries with no matching open contribution are now simply
  skipped, same as a malformed one).
- `QuestionKindRoute`'s doc comment updated (was "sourced from either the closed `Contribution` shape or
  the open... shape", now "sourced from the open `forms.questionKind` topic shape").
- `FormsQuestionKindTopicPayload`'s doc comment de-referenced `Contribution::FormsQuestionKind` (type no
  longer exists).
- Removed the `Contribution` import from the `semio_framework_plugin::{...}` use block (nothing left in
  this file needs it).
- `testkit::building_component_contributions()` — was a `ProgramContributionEntry` literal with
  `contribution: Contribution::FormsQuestionKind {..}, topic_contribution: None`; rewritten to
  `topic_contribution: Some(TopicContribution::new("forms.questionKind", json!({...})))` (the `contribution`
  field no longer exists on the real `semio_framework::ProgramContributionEntry` — confirmed by reading its
  current definition, see Verification below). This is the single shared helper every "contribution
  registered" test uses, so fixing it here fixed those call sites too.
- Test changes:
  - `extension_question_accepts_legacy_playbook_block_kind_contributions` — DELETED. It exercised the
    closed-only `PlaybookBlockKind` fallback, which no longer exists; testing dead code.
  - `extension_question_emits_external_slot_when_contribution_registered` — kept as-is; now exercises the
    open path transitively via the rewritten `building_component_contributions()` helper.
  - `extension_question_emits_external_slot_when_topic_contribution_registered` — kept; dropped its
    now-nonexistent `contribution: Contribution::FormsQuestionKind {..}` field from the entry literal
    (was there to prove a non-matching closed shape didn't interfere — moot now), doc comment simplified.
  - `extension_question_prefers_topic_contribution_over_closed_contribution` — DELETED. Its entire premise
    (open shape wins over closed shape on the same entry) is dead now that there is no closed shape to
    prefer over.
  - `catalogue_kinds_includes_topic_contributed_kinds` — kept; dropped its `contribution:
    Contribution::FormsQuestionKind {..}` field the same way, doc comment simplified.

## forms/apps/forms/config/component.rs
No code change (confirmed again: `contributions_json` stays an opaque JSON blob field, no direct
`Contribution` match here). Updated the module doc comment and the `contributions_json` field doc comment
to drop every "closed `Contribution::FormsQuestionKind` (legacy `PlaybookBlockKind`), open shape preferred
when both present" phrasing — now states plainly that entries carry the open `TopicContribution`
(`"forms.questionKind"` topic) shape only. Also swapped a stray "arbitrary `Contribution` list" doc phrase
on the struct itself for "arbitrary `ProgramContributionEntry` list" (that reference had survived from
before this type existed and was already wrong independent of this wave).

## procedural3d/⚙️engine/component.rs (`flow_extension_manifest_json`)
This file defines its OWN LOCAL `ProgramContributionEntry` shadow struct (deserialize-only, built directly
from `contributionsJson` text, not the real framework type):
- Removed the `contribution: Contribution` field, keeping only `plugin_id` and `topic_contribution:
  Option<TopicContribution>` (matches the instruction: "the sole field besides `plugin_id`").
- Removed the `use semio_framework::{Contribution, TopicContribution}` import's `Contribution` half —
  `TopicContribution` is still used, so the import became `use semio_framework::TopicContribution;`.
- `flow_extension_manifest_json` — deleted the `if let Contribution::FlowExtension { manifest_json, .. } =
  &entry.contribution { return Some(manifest_json.clone()); }` fallback block; now returns `None` via `?`
  as soon as either there's no `topic_contribution`, its `topic` doesn't match `FLOW_EXTENSION_TOPIC`, or
  decode fails — same effective behavior as the old fallback's final `None` arm, just without the closed
  branch. Doc comment updated to drop "reads both shapes" framing.
- No test literals in this file construct `ProgramContributionEntry` directly (it's always built via
  `serde_json::from_str` off a `contributions_json` string in `sync_flow_extension_contributions`), so no
  test-fixture edits were needed here.

## Verification
`cargo check -p semio-s-plugin-forms -p semio-s-plugin-procedural` — run three times. Every run aborts
before reaching either target crate, both times on the SAME two errors, both in a file OUTSIDE this wave's
assigned files:

```
error[E0432]: unresolved import `semio_framework::Contribution`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:4:73
error[E0599]: no method named `contributes` found for struct `component::app::Plugin` in the current scope
    --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:166:29
```

Both point at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs`
— a producer-side builder file, not in this wave's assignment. This is the parallel framework-side deletion
(the orchestrator's note: "Every struct literal you touch will have a field that no longer exists once
framework's type deletion lands (parallel agent)") caught mid-flight: the real `Contribution` enum and its
`Plugin::contributes()` builder method have already been removed from `semio_framework`
(confirmed — grepped `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` for `enum Contribution`, zero hits;
the file's `ProgramContributionEntry` there is already the two-field `{ plugin_id, topic_contribution }`
shape with no `contribution` field, which is exactly the shape this wave's `forms/component.rs` edits
already assume), but this one builder call site hasn't been converted to `.contributes_topic(...)`-only
yet. `semio-framework-plugin` (an upstream dependency of both target crates) fails to compile as a result,
so neither `semio-s-plugin-forms` nor `semio-s-plugin-procedural` themselves were ever reached by rustc in
any of the three runs (no `Checking semio-s-plugin-forms`/`Checking semio-s-plugin-procedural` line in any
log) — did not touch that builder file, outside assigned files, another session's in-progress edit per the
operational rules.

In lieu of a green compile: confirmed by direct inspection that the real
`semio_framework::ProgramContributionEntry` (the type `forms/component.rs` imports via `pub use
semio_framework::ProgramContributionEntry;`) is already exactly `{ plugin_id: String, topic_contribution:
Option<TopicContribution> }` — zero `contribution` field, zero `Contribution` enum anywhere in
`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — so every literal and field access left in this wave's
forms edits matches the landed framework shape. `TopicContribution::{topic, payload, new, decode}` and
`ProgramContributionEntry::{plugin_id, topic_contribution}` were re-verified against that same source
(unchanged from wave 4a5's verification). Grepped both target files afterward for any remaining
`Contribution` token outside `TopicContribution`/`ProgramContributionEntry` substrings — zero hits.

## Files worked on
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🦀️component.rs` (deleted closed-shape fallback + producer test
  helper + 2 dead tests, simplified 2 remaining tests)
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🎚️config/🦀️component.rs` (doc comments only, no code change)
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️procedural3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` (removed
  local shadow struct's `contribution` field + `Contribution` import + closed-fallback branch)
