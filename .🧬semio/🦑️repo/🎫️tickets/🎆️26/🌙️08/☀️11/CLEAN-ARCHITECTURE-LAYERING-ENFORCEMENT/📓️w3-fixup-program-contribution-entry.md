# Wave 3 fix-up #2 — `ProgramContributionEntry` needed the same open-field treatment

Process's agent independently found a THIRD manifest-adjacent type with the
same gap: `ProgramContributionEntry` (`🛂️manifest/🦀️component.rs:2788`,
`{ plugin_id, contribution: Contribution }`) — used by app-level "seed
contributions into a document/session" helper functions (distinct from both
`PluginManifest` and `ExtensionManifest`). It had no open-topic sibling
field either.

## Fix
- `🛂️manifest/🦀️component.rs`: added `pub topic_contribution:
  Option<TopicContribution>` (`#[serde(default)]`) to
  `ProgramContributionEntry`.
- Repo-wide grep found 16 files matching `ProgramContributionEntry {` —
  investigation showed only some are the REAL global type; several files
  (os core `💻️os/🦀️component.rs`, os host `🖥️host/🦀️component.rs`,
  renderer `Shell/🧊️component.rs`, procedural's engine file) define their
  **own locally-named, unrelated struct** that happens to share the name —
  confirmed via `use`/definition inspection before touching anything, so
  those were correctly left alone.
- Fixed the 9 real construction sites of the actual
  `semio_framework::ProgramContributionEntry`:
  - `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️component.rs` —
    completed properly rather than just unblocking: the imperative-sequence
    wave-3 agent had already built a parallel `imperative_module_topic_contribution()`
    twin function (an "additive adapted" workaround for this exact gap);
    wired it in as `topic_contribution: Some(imperative_module_topic_contribution(...))`
    instead of a hollow `None`, so all 5 imperative extensions (logic,
    effect, math, control, text — all thin wrappers over this one shared
    builder) now genuinely populate the open contribution, not just compile.
  - `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs` (×2),
    `🏭️process/🗿️artifacts/🧊️process3d/⚙️engine/🦀️component.rs` (×1),
    `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/⚙️engine/🦀️component.rs`
    (×2), `✏️s/🔌️plugins/📋️forms/🎛️apps/📋️forms/🦀️component.rs` (×2),
    `✏️s/🔌️plugins/📖️playbook/…/🏗️builder/🦀️component.rs` (×1) — added
    `topic_contribution: None,` (test fixtures / seed-catalog helpers;
    real topic-payload population left as a follow-up since it needs
    per-site payload review, unlike imperative's case where the data
    already existed).

## Verification
- `cargo check -p semio-framework` — clean.
- `cargo check -p semio-s-plugin-imperative-{logic,effect,math,control,text}` — all clean.
- `cargo check -p semio-s-plugin-process` / `-sourcing` — blocked ONLY by
  the known unrelated concurrent "document" module churn (confirmed
  identical error shape to every other plugin hitting it this ticket).
- `cargo check -p semio-s-plugin-playbook` — 3 pre-existing `E0308` errors,
  confirmed byte-identical file:line to what the wave-3 playbook agent
  already flagged as unrelated stdio JSON-codec debt (different files
  entirely, not touched by this fix).
- `cargo check -p semio-s-plugin-forms` — blocked only by the document churn.

## Still open (real Step B completion, follow-up)
cad (4 sites), flow (12 sites), process (3, now unblocked structurally but
still `None`), sourcing (2, `None`), playbook (1, `None`) can now call
`.contributes_topic(topic, payload)` / set `topic_contribution: Some(...)`
— the mechanism is fully wired end to end (3 manifest-adjacent types all
carry the open shape now), only the actual per-site payload construction
remains, deferred to keep this fix-up scoped and reviewable.
