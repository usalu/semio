# w4a5 — Process + Sourcing Consumers Read Open `topic_contribution`

## Scope

Wire the CONSUMER side of the `Contribution` → `TopicContribution` migration for the two assigned files.
Additive only: both the closed `Contribution` variant and the open `topic_contribution`/topic string are
still read; the closed path is the fallback, not removed.

## Definitions Checked First

Read `Contribution`, `ProgramContributionEntry`, `TopicContribution` in
`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` (lines ~2686-2831):

- `ProgramContributionEntry { plugin_id, contribution: Contribution, topic_contribution: Option<TopicContribution> }`
  — one entry carries *both* shapes side by side (not a list to merge/de-dupe — a single optional sibling
  field), so the merge strategy here is simply: prefer `topic_contribution` when present and its `topic`
  matches, else fall back to `entry.contribution`.
- `TopicContribution { topic: String, payload: serde_json::Value }` with `.decode::<T>() -> Result<T, serde_json::Error>`.
- `Contribution::ProcessMachines { app_id, module_id, label, icon_id: IconName, machines_json }` and
  `Contribution::SourcingModule { app_id, module_id, label, icon_id: IconName, typology_json, kinds_json }`
  — both `#[serde(rename_all = "camelCase")]` at the enum level, so the topic payload's JSON keys
  (`appId`, `moduleId`, …) line up with a local `#[serde(rename_all = "camelCase")]` decode-target struct.

Also confirmed via `📓️w3.5-process-sourcing.md` that a prior wave already populates
`topic_contribution: Some(TopicContribution::new("process.machines"/"sourcing.module", ...))` at the real
producer sites (extension crates + `🏭️process/🎛️apps/🧊️3d/🦀️component.rs`'s `seed_domain_catalog_contributions`
test helper), with payload JSON shaped exactly like the sibling `Contribution` literal — confirming the
decode-target struct field set below.

## Changes

### `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

- Added `IconName` to the existing `use semio_framework::{parse_contributions, Contribution, ...}` import.
- Added a private `ProcessMachinesTopicPayload` decode-target struct (region `🔖️ProcessMachinesTopicPayload`)
  mirroring `Contribution::ProcessMachines`'s fields.
- `sync_process_machine_contributions`: for each parsed entry, first tries
  `entry.topic_contribution` filtered to `topic == "process.machines"` and decoded via
  `.decode::<ProcessMachinesTopicPayload>()`; falls back to the existing
  `Contribution::ProcessMachines` pattern-match (`entry.contribution`) when the topic entry is absent,
  mismatched, or fails to decode. Downstream logic (app-id filter, `machines_json` parse, catalog push)
  is unchanged — only the field-extraction step gained the open-shape-first branch.

### `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

- Added a private `SourcingModuleTopicPayload` decode-target struct (region `🔖️SourcingModuleTopicPayload`)
  mirroring `Contribution::SourcingModule`'s fields (`icon_id` intentionally omitted — the existing consumer
  never reads it either, since `ContributedSourcingModule` has no icon field; extra JSON keys in the topic
  payload are ignored by serde by default).
- `sync_sourcing_module_contributions`: same prefer-open-then-fallback-to-closed pattern, keyed on
  `topic == "sourcing.module"`.

No test-fixture `ProgramContributionEntry { topic_contribution: None, .. }` literals were touched in either
file (lines ~810/832 in process, ~832 in sourcing) — those are producer-side test data, out of scope for a
consumer-only wave.

## Verification

Ran:
```
cargo check -p semio-s-plugin-process -p semio-s-plugin-sourcing
```

Result: **blocked before reaching either target crate** — `semio-framework-math` fails first with

```
error[E0004]: non-exhaustive patterns: `TokenKind::Lt`, `TokenKind::Gt`, `TokenKind::Amp` and 3 more not covered
  --> 🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust/./../../🕸️graph/🗣️dsl/🦀️component.rs:849:15
error: could not compile `semio-framework-math` (lib) due to 1 previous error
```

This is the concurrent "math tokenizer `TokenKind` mid-edit" churn flagged in this session's own operational
rules (not the "document" module error the task description anticipated — that one has apparently since been
resolved by another session, and this is what surfaced next). `semio-s-plugin-process`/`semio-s-plugin-sourcing`
both depend on `semio-framework`, which depends on `semio-framework-math`, so the failure cascades and blocks
`cargo check` from ever compiling either target crate — confirmed unrelated to anything in scope here (also
independently cross-checked against `📓️w4a-verify-orchestrator.md`, which hit and logged the identical
`TokenKind` E0004 blocker for this same wave). Per operational rules ("if you hit a compile error unrelated to
what you touched, do not fix it — note it and move on"), not investigated or touched further.

Because a real `cargo check` couldn't reach the target crates, sanity-checked both edited files standalone
with `rustc --edition 2021 --crate-type lib --emit=metadata <file>` and filtered the error list: every error
in both files is E0432/E0433 unresolved-crate/import noise (expected — no dependency graph when compiling a
single file outside its crate) or `cannot find attribute serde` (same cause, `#[derive(serde::Deserialize)]`
needs the resolved `serde` crate). No syntax errors, no errors on any of the newly added lines specifically
(new `ProcessMachinesTopicPayload`/`SourcingModuleTopicPayload` structs and the `topic_payload` match blocks
inside both `sync_*_contributions` functions) beyond that same expected-noise class. Both files are
structurally valid Rust; the only path to a real type-check is once the `semio-framework-math` blocker
clears.

**Did not verify** the "prior wave's producer-side data flows through end to end" claim from the task
description at runtime (no test run — `cargo test` is blocked by the same `semio-framework-math` compile
error as `cargo check`). The wiring is logically correct by inspection: `sync_process_machine_contributions`'s
existing unit test (`sync_process_machine_contributions_merges_hot_installed_catalogs`, uses
`topic_contribution: None`) and `sync_sourcing_module_contributions`'s equivalent test are both untouched and
still exercise the closed-shape fallback path unchanged.

## Files Touched

Updated only (both files, no others):
- `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`
- `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/⚙️engine/🦀️component.rs`

No files created or removed. Ticket not closed — subagent scope is consumer-file edits only.
