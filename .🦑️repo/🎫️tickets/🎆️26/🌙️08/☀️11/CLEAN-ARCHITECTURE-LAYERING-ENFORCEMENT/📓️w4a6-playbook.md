# w4a6 — playbook plugin: delete closed `Contribution::PlaybookBlockKind` path

Wave 4a6 of CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT. Removes the closed `Contribution` enum
producer call + both closed-shape consumer fallbacks in the playbook subtree, per the
`w4a5-verify-summary.md` confirmation that every producer/consumer now also has the open
`TopicContribution`/`topic_contribution(s)` path.

## Files edited

1. `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs` (producer)
   - `module_extension_bundle()`: removed the `.contributes(Contribution::PlaybookBlockKind{...})` call;
     `.contributes_topic("playbook.blockKind", ...)` is now the sole contribution declaration.
   - Dropped now-unused `Contribution` from the `semio_framework_plugin::{...}` import list.
   - Test `module_manifest_contributes_building_component`: rewrote to assert against
     `manifest.topic_contributions[0]` (`topic == "playbook.blockKind"`, decoded JSON payload fields
     `blockKind`/`paramsBodyKey`/`previewBodyKey`) instead of destructuring `manifest.contributions[0]`
     as `Contribution::PlaybookBlockKind` (field no longer exists on `PluginManifest`).

2. `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs` (consumer)
   - `extension_palette_entries()`: removed the closed-enum fallback branch (`entry.contribution` match
     on `Contribution::PlaybookBlockKind`). Now the sole path is: filter `topic_contribution` by topic
     `"playbook.blockKind"`, decode `PlaybookBlockKindTopicPayload`, map to the triple. No match → skipped.
   - Dropped now-unused `Contribution` from the `use semio_framework::{...}` import.
   - Updated doc comments (function + payload struct) that described the "reads both shapes" behavior.
   - Tests: deleted `render_builder_palette_includes_contributed_block_kinds` (exercised the closed-only
     path — dead code once the fallback is gone) and
     `render_builder_palette_prefers_topic_contribution_over_closed_contribution` (nothing left to prefer
     over). Kept `render_builder_palette_includes_topic_contributed_block_kinds`, simplified its
     `ProgramContributionEntry` literal to drop the `contribution:` field (field itself was deleted
     upstream by the framework agent — struct now only has `plugin_id`/`topic_contribution`).

3. `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs` (os-side consumer,
   `builder_kit::resolve_block_kind_extensions`, added by prior wave w4a5)
   - Removed the closed-enum fallback branch (previously: try open `topic_contribution` first, then
     match `entry.contribution` against `Contribution::PlaybookBlockKind`). Now only reads
     `topic_contribution` filtered by topic `"playbook.blockKind"`, decodes `BlockKindPayload`, or skips.
   - Dropped now-unused `Contribution` from the `use semio_framework::{...}` import (kept
     `ProgramContributionEntry`).
   - Updated doc comments on `BlockKindPayload`, `resolve_block_kind_extensions`, `build_palette`, and
     the file header that referenced the closed enum.
   - Tests (`builder_kit_tests`): replaced `closed_only_entry()` helper with `open_topic_entry()`
     (struct literal now only has `plugin_id`/`topic_contribution`, matching the framework's already-landed
     field deletion on `ProgramContributionEntry`). Deleted
     `resolve_block_kind_extensions_falls_back_to_closed_contribution` (dead: no more fallback) and
     folded `resolve_block_kind_extensions_prefers_open_topic_contribution_over_closed` /
     `resolve_block_kind_extensions_ignores_unrelated_topics_and_kinds` into two focused replacements:
     `resolve_block_kind_extensions_reads_open_topic_contribution` (happy path) and
     `resolve_block_kind_extensions_ignores_unrelated_topics` (no match → empty vec).

Confirmed via grep across the whole `✏️s/🔌️plugins/📖️playbook` subtree afterward: zero remaining hits for
`Contribution::PlaybookBlockKind` or `.contribution` field access. The closed shape is fully gone from
all three assigned files.

Also confirmed `PluginManifest`/`ProgramContributionEntry`/`TopicContribution` in
`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` already reflect the framework agent's parallel type
deletion: `PluginManifest` has no `contributions: Vec<Contribution>` field anymore (only
`topic_contributions: Vec<TopicContribution>`), and `ProgramContributionEntry` has only
`plugin_id`/`topic_contribution` fields — matches what this wave's task briefing said to assume.

## Verify

`cargo check -p semio-s-plugin-playbook -p semio-s-plugin-playbook-procedural -p semio-framework-os-flow`
— **blocked by an unrelated, out-of-scope compile error**, not by anything in this wave's edits:

```
error[E0432]: unresolved import `semio_framework::Contribution`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:4
error[E0599]: no method named `contributes` found for struct `component::app::Plugin`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs:166
```

That file (`framework/.../🔌️plugin/🏗️builder/🦀️component.rs`) is the generic `Plugin`/`ExtensionBundle`
builder itself — it still defines `pub fn contributes(mut self, contribution: Contribution) -> Self` and
a `contributions: Vec<Contribution>` field, i.e. the framework-side `Contribution` type deletion (owned
by a different, parallel agent per this wave's briefing) hasn't landed there yet. It is not in my
assigned files/directories and I did not touch it, per operational rules ("unrelated compile error → note,
don't fix, move on"). `semio-framework-plugin` (which that file compiles into) is a transitive dependency
of all three of my target crates, so the `cargo check` above fails to build at all right now — reran once
after a 20s wait, same two errors, unchanged.

Manually verified my 3 edited files instead:
- Grepped for any remaining `Contribution`/`.contribution` references in the playbook plugin subtree and
  the os playbook module file — none found (aside from the doc-comment prose I already updated).
- Cross-checked `ProgramContributionEntry`/`TopicContribution`/`PluginManifest` field shapes directly
  against `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (lines ~2690-2750) to make sure every struct
  literal I edited/kept matches the already-landed field set.
- Confirmed crate names via each `Cargo.toml`: `semio-s-plugin-playbook`,
  `semio-s-plugin-playbook-procedural` — both match the task's `cargo check -p` targets exactly.
  `semio-framework-os-flow` for the os module confirmed by the prior wave's
  `📓️w4a5-os-playbook.md` note (mounted via `os/modules/flow/packages/rust/📦️glue.rs`).

**Re-run `cargo check -p semio-s-plugin-playbook -p semio-s-plugin-playbook-procedural -p
semio-framework-os-flow` once `framework/.../🔌️plugin/🏗️builder/🦀️component.rs`'s `Contribution`
deletion lands** — expect a clean pass given the manual field/name verification above; flag if not.

## Files touched (summary)

- `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs`
- `✏️s/🔌️plugins/📖️playbook/🎛️apps/📖️playbook/🎭️modes/🏗️builder/🪟️windows/🏗️builder/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🦀️component.rs`
