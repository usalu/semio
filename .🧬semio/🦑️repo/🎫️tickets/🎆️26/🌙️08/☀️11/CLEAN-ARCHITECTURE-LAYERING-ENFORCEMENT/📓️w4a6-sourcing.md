# w4a6 — Sourcing: Delete the Closed `Contribution::SourcingModule` Path

## Scope

Final destructive cut for the `Contribution` → `TopicContribution` migration, sourcing subtree only:
3 extension crates + the engine consumer (`sync_sourcing_module_contributions`) migrated to
prefer-open-fallback-to-closed in w4a5. This wave removes the closed path entirely — no fallback,
no dead branches.

Read `📓️w4a5-process-sourcing.md` first per task instructions for exactly what the fallback shape
looked like.

## Changes

### Producer sites (3 extension crates)

`✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/{🪵️beams,🧱️slabs,🪟️windows}/🦀️component.rs`

- Removed the `.contributes(Contribution::SourcingModule { .. })` call from `bundle()` in each file;
  `.contributes_topic("sourcing.module", ...)` is now the sole contribution declaration.
- Removed the now-unused `use semio_framework::Contribution;` import from each file.
- Rewrote each file's `bundle_contributes_module_for_sourcing_curate` test: was asserting
  `manifest.contributions.len() == 1` and pattern-matching `Contribution::SourcingModule` out of
  `manifest.contributions[0]`. Now asserts `manifest.contributions.len() == 0` and
  `manifest.topic_contributions.len() == 1`, and reads the same fields (`appId`, `moduleId`,
  `typologyJson`, `kindsJson`) off `manifest.topic_contributions[0].payload` (a `serde_json::Value`)
  instead. Same assertions on the decoded typology/kinds JSON, just sourced from the open shape.

### Consumer site (engine)

`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

- `sync_sourcing_module_contributions`: removed the `None => { let Contribution::SourcingModule { .. }
  = entry.contribution else { continue }; ... }` fallback arm entirely. The function now does a single
  `let Some(payload) = entry.topic_contribution.as_ref().filter(topic == "sourcing.module").and_then(decode)
  else { continue }` and reads fields off `payload` directly — no `match`, no closed-enum branch, no
  `entry.contribution` access anywhere (that field is already gone from `ProgramContributionEntry` per
  framework's own type deletion — confirmed by reading
  `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, which now only has `plugin_id` +
  `topic_contribution: Option<TopicContribution>` on that struct).
- Removed the now-unused `Contribution` import (`use semio_framework::{parse_contributions,
  Contribution};` → `use semio_framework::parse_contributions;`).
- Updated the `ContributedSourcingModule` doc comment and the `SourcingModuleTopicPayload` region
  doc comment to drop references to the now-nonexistent `Contribution::SourcingModule`.
- Two test-fixture `ProgramContributionEntry` literals (both previously `topic_contribution: None,
  contribution: Contribution::SourcingModule { .. }` — closed-shape-only tests):
  - `available_modules_tracks_contributed_modules` (was ~line 826)
  - `sync_sourcing_module_contributions_adds_hot_installed_modules` (was ~line 850)

  Both converted to construct `topic_contribution: Some(TopicContribution::new("sourcing.module",
  serde_json::json!({ "appId": .., "moduleId": .., "label": .., "iconId": .., "typologyJson": ..,
  "kindsJson": .. })))` instead — the `contribution` field literal is gone (it no longer exists on the
  struct). Neither test was specifically about closed-shape-fallback *behavior* (both are "does a
  hot-installed module show up in `available_modules`/`sourcing_modules`" tests), so both were
  converted rather than deleted — they keep exercising the now-sole open-shape code path end to end.

Confirmed via grep that `Contribution::SourcingModule` and any `Contribution` reference no longer
exist anywhere in the 4 assigned files.

## Verification

Ran the requested command:
```
cargo check -p semio-s-plugin-sourcing -p semio-s-plugin-sourcing-beams -p semio-s-plugin-sourcing-slabs -p semio-s-plugin-sourcing-windows
```

Result: **blocked before reaching any target crate.** All 5 errors are inside
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️component.rs` (framework's own plugin
builder, not owned by this wave):

```
error[E0432]: unresolved import `semio_framework::Contribution`
error[E0425]: cannot find type `Contribution` in this scope   (x3)
error[E0599]: no method named `contributes` found for struct `component::app::Plugin`
```

This is the framework side of the SAME migration — a parallel/concurrent session is mid-deletion of
`Contribution` from `semio-framework` itself (confirmed: `Contribution::SourcingModule` is already gone
from `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`, and `ProgramContributionEntry` there already
has no `contribution` field — matches this wave's own instructions that framework's deletion lands in
parallel). `🏗️builder/🦀️component.rs`'s `Plugin::contributes()` method / import haven't caught up yet.
Every `semio-s-plugin-sourcing*` crate depends on `semio-framework-plugin`, which depends on this
builder file, so the failure cascades and blocks `cargo check` from ever reaching the 4 target crates —
unrelated to anything touched in this wave, not fixed (out of scope: not my assigned files, per
operational rules for unrelated concurrent churn).

Because a real `cargo check` couldn't reach the target crates, sanity-checked all 4 edited files
standalone with `rustc --edition 2021 --crate-name check_tmp --crate-type lib --emit=metadata <file>`
and filtered the error list: every error in all 4 files is E0432/E0433 unresolved-crate/import noise
(expected — no dependency graph when compiling a single file outside its crate/workspace). Zero errors
of any other kind in any of the 4 files — no E0308/E0560/E0063/etc that would indicate a struct-literal
or type mismatch from the edits. All 4 files are structurally valid Rust; the only path to a real
type-check is once the concurrent `🏗️builder/🦀️component.rs` blocker clears.

**Did not run** `cargo test` for the same reason (same upstream blocker). The two converted test
fixtures are logically correct by inspection: same field values as the removed `Contribution::SourcingModule`
literals, now carried as JSON keys inside `TopicContribution::new("sourcing.module", ...)`'s payload,
matching the camelCase field names (`appId`, `moduleId`, `label`, `iconId`, `typologyJson`, `kindsJson`)
that `SourcingModuleTopicPayload`'s `#[serde(rename_all = "camelCase")]` decode target expects.

## Files Touched

Updated only (4 files, no others created or removed):
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪵️beams/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🧱️slabs/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/🪟️windows/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

Ticket not closed — subagent scope is edit-only, per assignment.
