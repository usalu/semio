# w4a6 — Process Extensions: Delete the Closed `Contribution::ProcessMachines` Path

## Scope

Final destructive cut for the `Contribution` → `TopicContribution` migration, scoped to
`✏️s/🔌️plugins/🏭️process/` subtree: the 4 process extensions, the `🎛️apps/🧊️3d/🦀️component.rs`
test-seed helper, and the `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` consumer.
Remove `.contributes(Contribution::ProcessMachines{...})` / the closed-enum fallback / closed-shape test
fixtures entirely — `.contributes_topic("process.machines", ...)` / `topic_contribution` becomes the sole
path everywhere.

## Pre-check

Grepped `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` for `enum Contribution` / `ProcessMachines` /
`SourcingModule` — zero hits. Confirmed the parallel framework-side type-deletion agent has already removed
the closed `Contribution` enum from its defining module entirely (only `TopicContribution` and
`ProgramContributionEntry { plugin_id, topic_contribution: Option<TopicContribution> }` remain there — no
`contribution` field). Proceeded on the assumption stated in the task: edit every file as if the closed
shape is already gone, regardless of whether the wider workspace currently compiles.

## Changes

### Producer sites — 4 process extensions

Removed the `.contributes(Contribution::ProcessMachines{...})` call (kept `.contributes_topic("process.machines", ...)`
as sole contribution) and the now-unused `use semio_framework::Contribution;` import from:
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/🦀️component.rs`

`🪵️wood`'s test `bundle_contributes_wood_machines_for_process3d_play` specifically asserted on
`manifest.contributions[0]` matched as `Contribution::ProcessMachines{...}` — closed-shape-only coverage
that would otherwise become dead/uncompilable. Rewrote it (didn't delete — CLAUDE.md requires extending
existing test files rather than dropping coverage) to assert the equivalent facts against
`manifest.topic_contributions[0]` (`topic == "process.machines"`, decoded JSON payload fields
`appId`/`moduleId`/`label`/`machinesJson`). Metal/robotic/concrete had no such test; nothing else to change
there.

### `🎛️apps/🧊️3d/🦀️component.rs` — `seed_domain_catalog_contributions()`

Removed the `contribution: Contribution::ProcessMachines{...}` field from both `ProgramContributionEntry`
literals (`process-wood`, `process-metal`), keeping only `topic_contribution: Some(TopicContribution::new(...))`
(already wired by w3.5). Dropped `Contribution` from the local
`use semio_framework::{Contribution, ProgramContributionEntry, TopicContribution};` import.

### Engine consumer — `🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

- `sync_process_machine_contributions`: removed the closed-enum fallback branch entirely. The function now
  does `let Some(payload) = entry.topic_contribution.filter(topic == "process.machines").and_then(decode) else { continue };`
  — open-shape read is the only path; an entry with no matching open contribution is skipped, same as a
  malformed one would be today. Removed `Contribution` from the `use semio_framework::{parse_contributions,
  Contribution, IconName};` import. Updated the two doc comments that referenced `Contribution::ProcessMachines`
  to describe the topic-payload shape instead (no behavioral change, just stale-reference cleanup).
- Test fixture `sync_process_machine_contributions_merges_hot_installed_catalogs`: its one
  `ProgramContributionEntry` literal was `contribution: Contribution::ProcessMachines{...}, topic_contribution: None`
  — exactly the "only way to exercise the closed path" case the task flagged. Converted it to the open shape:
  `topic_contribution: Some(TopicContribution::new("process.machines", serde_json::json!({...})))` with the
  same field values (camelCase JSON keys), dropped the `contribution` field and the `Contribution` import
  (now `use semio_framework::{ProgramContributionEntry, TopicContribution};`). Test still exercises the sole
  remaining code path unchanged in intent (hot-installed catalog merge).

## Verification

Ran:
```
cargo check -p semio-s-plugin-process -p semio-s-plugin-process-metal -p semio-s-plugin-process-wood \
  -p semio-s-plugin-process-robotic -p semio-s-plugin-process-concrete
```

Full output saved to scratchpad (not ticket-relevant beyond this summary):
`/private/tmp/claude-501/-Users-ueli-Documents-semio/9aab7911-976f-4f65-8d12-1fcd4d6fd73b/scratchpad/w4a6-cargo-check.txt`

Result: **2 errors, both entirely outside my assigned files**:

```
error[E0432]: unresolved import `semio_framework::Contribution`
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:4:73

error[E0599]: no method named `contributes` found for struct `component::app::Plugin` in the current scope
 --> 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🏗️builder/🦀️component.rs:166:29
```

Both errors are in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` — the
parallel framework type-deletion agent's own file (it removed the `Contribution` enum and the `Plugin`
builder's closed `.contributes()` method, but that builder file's own `use` and one call site haven't been
updated yet by that agent). Confirmed via grep of the full check output: zero errors trace to any path under
`✏️s/🔌️plugins/🏭️process/`. Per operational rules ("compile error unrelated to what you touched — note it
and move on"), not investigated or touched further; not my file, outside assigned scope.

Also grepped the entire `✏️s/🔌️plugins/🏭️process/` subtree for `Contribution::` and bare `\bContribution\b`
(excluding `TopicContribution`/`ProgramContributionEntry`/`topic_contribution`/`*_contributions`/
`contributions_json`) after all edits — zero hits. The closed shape is fully gone from every assigned file.

## Files Touched (updated only, none created/removed)

- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🔩️metal/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🤖️robotic/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🧩️extensions/🧱️concrete/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🎛️apps/🧊️3d/🦀️component.rs`
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

Ticket not closed (subagent scope — instructed not to close/reopen shared tickets).
