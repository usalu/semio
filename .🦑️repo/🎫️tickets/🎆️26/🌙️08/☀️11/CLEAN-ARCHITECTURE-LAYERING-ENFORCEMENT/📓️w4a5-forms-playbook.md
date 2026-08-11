# w4a5 — forms + playbook: open `TopicContribution` consumer wave

## Scope
Additive consumer-side migration (wave 4a, does NOT remove the closed `Contribution` path — a later
wave deletes it once every producer+consumer is confirmed migrated).

Files touched:
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🦀️component.rs`
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🎚️config/🦀️component.rs`
- `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs`

Reference read first: `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` — `Contribution::FormsQuestionKind`/
`Contribution::PlaybookBlockKind` variant fields (region `🔖️Contributions`), `TopicContribution`/`decode`
(region `🔖️TopicContribution`), `ProgramContributionEntry` (carries both `contribution` and an optional
`topic_contribution` per entry — this is the shape both consumer files actually operate on, not
`PluginManifest.topic_contributions`).

Style precedent followed: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🦀️component.rs`'s
`sync_host_flow_extension_contributions` — same idiom (a `XxxTopicPayload` deserialize struct + a
`XXX_TOPIC` const + `entry.topic_contribution.as_ref().filter(topic == X).and_then(decode)` preferred over
the closed match) was already established there for `flow.extension`; reused verbatim for
`forms.questionKind` and `playbook.blockKind`.

## forms/apps/forms/component.rs
- Added `FormsQuestionKindTopicPayload` (camelCase decode target: appId/questionKind/label/iconId/
  paramsBodyKey/previewBodyKey) and `const FORMS_QUESTION_KIND_TOPIC = "forms.questionKind"`.
- Replaced `question_kind_match` (borrowed-tuple return, closed-only) with:
  - `question_kind_route_from_contribution` — same closed-shape match as before (both
    `FormsQuestionKind` and legacy `PlaybookBlockKind`), now returns an owned `QuestionKindRoute`.
  - `question_kind_route_from_topic` — topic-gated decode + kind filter, same owned `QuestionKindRoute`.
  - `find_question_kind_contribution` — per entry, tries the open shape first
    (`entry.topic_contribution`), falls back to the closed shape (`entry.contribution`). Preferring open
    per entry, not merging fields across the two.
  - Signature changed from `Option<(&'a str, &'a Contribution)>` to `Option<(&'a str, QuestionKindRoute)>`
    — verified this fn has no callers outside this file (grepped the whole repo), so the signature change
    is contained. This also let `render_extension_question` drop its previously-redundant second
    `question_kind_match` call (was calling the matcher twice: once inside `find_question_kind_contribution`,
    once again on the returned `&Contribution` to extract fields) — now resolves once.
- `catalogue_kinds`: per entry, tries the topic shape first (decode + push), `continue`s past the closed
  match on success; unchanged closed-match fallback otherwise (still handles both `FormsQuestionKind` and
  legacy `PlaybookBlockKind`).
- Tests added (extended the existing `mod tests`, no new files):
  - `extension_question_emits_external_slot_when_topic_contribution_registered` — topic-only entry (closed
    side carries a non-matching kind) still resolves.
  - `extension_question_prefers_topic_contribution_over_closed_contribution` — both shapes present, distinct
    `appId` per shape, asserts the rendered slot carries the open shape's `appId` and not the closed one's.
  - `catalogue_kinds_includes_topic_contributed_kinds` — topic-only entry surfaces in `catalogue_kinds`.

## forms/apps/forms/config/component.rs
No consumer code here (confirmed by reading the whole file) — `contributions_json` is an opaque JSON blob
field with no direct `Contribution::FormsQuestionKind` match. Updated its two doc comments (module doc +
field doc) to describe the entry shape as carrying both the closed and open payloads and to state the
open-preferred-when-present rule, so the doc stays accurate now that `🦀️component.rs`'s consumers read both.

## playbook/…/builder/component.rs
- Added `PlaybookBlockKindTopicPayload` (camelCase: blockKind/label/iconId) and
  `const PLAYBOOK_BLOCK_KIND_TOPIC = "playbook.blockKind"`.
- `extension_palette_entries` (feeds `build_palette` → the block-list builder's palette): per entry, tries
  the topic shape first (decode + map to the same `(String, String, String)` tuple the closed path already
  produced), falls back to the pre-existing closed `Contribution::PlaybookBlockKind` match unchanged.
- Tests added (extended existing `mod tests`):
  - `render_builder_palette_includes_topic_contributed_block_kinds` — topic-only entry surfaces in the
    palette.
  - `render_builder_palette_prefers_topic_contribution_over_closed_contribution` — both shapes present with
    distinct labels, asserts the palette entry's label is the open shape's.

## Merge strategy (judgment call, per instructions)
Per-entry "check topic first, fall back to closed" — not a de-dupe-by-key merge across the whole list.
`ProgramContributionEntry` already pairs one optional `topic_contribution` with one `contribution` on the
SAME entry (see the struct def), so there is no cross-entry de-dupe to do: each entry's own open field is
authoritative over its own closed field when present. This matches the flow/registry precedent exactly.

## Verification
`cargo check -p semio-s-plugin-forms -p semio-s-plugin-playbook --tests` was run twice (a few minutes apart).
Both runs abort before reaching either target crate:

```
error[E0004]: non-exhaustive patterns: `TokenKind::Lt`, `TokenKind::Gt`, `TokenKind::Amp` and 3 more not covered
   --> 🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/./../../🕸️graph/🗣️dsl/🦀️component.rs:849:15
error: could not compile `semio-framework-math` (lib) due to 1 previous error
```

This is the known unrelated concurrent churn (a math tokenizer `TokenKind` mid-edit, per the session's
background note) — `semio-framework-math` is a transitive dependency reached before cargo gets to either
`semio-s-plugin-forms` or `semio-s-plugin-playbook`; neither target crate's own compilation was attempted in
either run (no `Checking semio-s-plugin-forms`/`Checking semio-s-plugin-playbook` line in either log). Did
NOT touch `🕸️graph/🗣️dsl/🦀️component.rs` or `🔤️token/🦀️component.rs` (outside assigned files; another
session's in-progress edit).

Could not obtain a real compiler confirmation as a result. In lieu of that: manually traced every type used
in the diff against its actual definition in the framework source (not assumed) —
`TopicContribution::{topic, payload, new, decode}`, `ProgramContributionEntry::topic_contribution`,
`IconName`'s `Deserialize`/`Display`/`From<&str>` impls, `semio_framework_plugin`'s `pub use
semio_framework::*` re-export (confirms `semio_framework_plugin::TopicContribution` resolves), and
`crate::playbook::build_palette`/`BlockPaletteEntry`'s fields — all confirmed to match what the new code
calls. No cargo output backs this up; flagging that explicitly rather than claiming a green check.

## Files worked on
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🦀️component.rs` (consumer logic + tests)
- `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🎚️config/🦀️component.rs` (doc comments only, no code change)
- `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs`
  (consumer logic + tests)
