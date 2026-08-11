# W4a6 — Imperative: Delete Closed `Contribution` Path

## Scope
- `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️component.rs`
- `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️component.rs`
- `✏️s/🔌️plugins/📜️imperative/🧩️extensions/` (logic, effect, math, control, text)
- `✏️s/🔌️plugins/🎬️sequence/` subtree (checked, no direct edits needed)

## Changes

### `🧩️extension_sdk/🦀️component.rs` (producer, shared builder)
- `imperative_module_contribution()`: removed the `contribution: Contribution::ImperativeModule{...}` field
  from the returned `ProgramContributionEntry` literal. It now only sets `plugin_id` and
  `topic_contribution: Some(imperative_module_topic_contribution(...))`.
- Dropped the now-unused `Contribution` import.
- Updated 3 docstrings that referenced `Contribution::ImperativeModule` (manifest doc, app-id const,
  the `imperative_module_topic_contribution` region banner) to describe the open `"imperative.module"`
  topic shape instead — no more mention of the deleted enum variant.

### `📇️registry/🦀️component.rs` (consumer, `imperative_module_fields`)
- Removed the closed-enum fallback branch (`let Contribution::ImperativeModule { app_id, manifest_json, .. } = &entry.contribution ...`).
  The open `topic_contribution` (topic `"imperative.module"`) read is now the only path; if absent,
  wrong topic, or undecodable, the entry is skipped (`None`) — same behavior as a malformed entry today.
- Dropped the now-unused `Contribution` import.
- Updated the two docstrings that referenced the closed enum as a comparison point.

### 5 extension wrapper files — `🔌️plugins/📜️imperative/🧩️extensions/{🧠️logic,📣️effect,🧮️math,🎮️control,📝️text}/🦀️component.rs`
Contrary to the task's expectation ("thin wrappers... verify they don't need direct edits"), all 5
`bundle()` functions built their `ProgramContributionEntry` via `imperative_module_contribution()` and
then called `.contributes(entry.contribution)` — a direct read of the field being deleted
framework-side. Fixed all 5 identically:
- `bundle()` now calls `imperative_module_topic_contribution()` directly (already existed in each file)
  and wires `.contributes_topic(topic_contribution.topic, topic_contribution.payload)` instead of
  `.contributes(entry.contribution)`. No more `entry`/`ProgramContributionEntry` dependency in `bundle()`.
- `📣️effect/🦀️component.rs` additionally had a test (`bundle_contributes_core_module_for_imperative_play`)
  pattern-matching `entry.contribution` against `semio_framework::Contribution::ImperativeModule{...}`.
  This wasn't a fallback-only test (it exercised the real bundle-building path, now open-shape-only), so
  it was adapted rather than deleted: it now asserts on `entry.topic_contribution` (topic
  `"imperative.module"`, `payload["appId"|"moduleId"|"manifestJson"]`) instead of the closed enum.
- Confirmed (grep) none of the 5 files construct their own `Contribution::*` literal — the only
  `Contribution` references were the `entry.contribution` field reads described above, now removed.

### `🔌️plugins/🎬️sequence/` subtree
- Grepped for `Contribution` (capital-C, the enum/type) — zero hits, confirmed empty as the task
  expected.
- `🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/⚙️engine/🦀️component.rs` calls
  `imperative_module_contribution()` (math/text/effect/control) and feeds the results into
  `contributions_json_from_entries(&[...])` for test bootstrap. It never touches the `.contribution`
  field directly (only `contributions_json_from_entries`, which serializes whatever fields
  `ProgramContributionEntry` has), so it needed no edit and is unaffected by the field's removal.

## Verify
- `cargo check -p semio-s-imperative` — **clean**, no errors. Only 3 pre-existing warnings (unused
  `extern crate` x2 in `imperative`'s `glue.rs`, unused `OperatorInfo` import in registry
  `component.rs`) — none introduced by this wave, not touched.
- `cargo check -p semio-s-plugin-sequence` — **3 errors, all outside my assigned files**, both in
  `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` (framework's plugin
  builder, not owned by this wave):
  - `error[E0432]: unresolved import semio_framework::Contribution` (builder still imports the type)
  - `error[E0599]: no method named contributes found for struct component::app::Plugin`
  This is the parallel framework-side `Contribution` type-deletion agent's work still in flight (per
  task background: "Once ProgramContributionEntry.contribution field is deleted framework-side (a
  parallel agent's job)"). No error references any imperative or sequence file — confirmed by grepping
  the full error log for `imperative`/`sequence`, zero matches. Per operational rules, noted and not
  touched; not caused by and not fixable from within this wave's file scope.

## Result
Closed `Contribution`/`contribution` shape is fully gone from all files in scope: `extension_sdk`,
imperative `registry`, and all 5 imperative extension wrappers. Open `TopicContribution` /
`topic_contribution` under the `"imperative.module"` topic is now the sole path, producer and
consumer. `semio-s-imperative` compiles clean. `semio-s-plugin-sequence` is blocked only by the
still-in-progress framework-side deletion in `🔌️plugin/🏗️builder/🦀️component.rs`, outside this
wave's scope.
