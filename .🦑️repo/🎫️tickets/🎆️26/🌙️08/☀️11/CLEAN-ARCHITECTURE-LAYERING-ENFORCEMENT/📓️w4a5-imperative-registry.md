# w4a5 — Imperative Registry: Open `topic_contribution` Consumer

## Scope
File: `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️component.rs`

Task: make the `Contribution::ImperativeModule` consumer in `compose_registry` also read the
open `topic_contribution` (topic `"imperative.module"`), preferring it when present, falling
back to the closed `Contribution::ImperativeModule` variant. Additive dual-read only — the
closed-enum path is left in place for a later wave to delete.

## What I found

- Producer side (prior wave, already done): `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️component.rs`
  `imperative_module_contribution()` builds a `ProgramContributionEntry` with both
  `contribution: Contribution::ImperativeModule { app_id, module_id, label, icon_id, manifest_json }`
  and `topic_contribution: Some(imperative_module_topic_contribution(...))`.
  `imperative_module_topic_contribution()` builds a `TopicContribution { topic: "imperative.module",
  payload: { appId, moduleId, label, iconId, manifestJson } }` (camelCase JSON keys).
- `ProgramContributionEntry` (framework manifest component, `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`)
  carries `topic_contribution: Option<TopicContribution>` **per entry** — singular, not the
  `Vec<TopicContribution>` the task brief mentioned (that `Vec` form is `PluginManifest.topic_contributions`,
  a different, manifest-level field not touched by this consumer). `parse_contributions()` returns
  `Vec<ProgramContributionEntry>`, which is what `compose_registry` iterates.
- Only two fields of `Contribution::ImperativeModule` are actually read by the consumer:
  `app_id` (filtered against `imperative_extension_sdk::IMPERATIVE_PLAY_APP_ID`) and `manifest_json`
  (parsed into `ImperativeExtensionManifest`). `module_id`/`label`/`icon_id` are ignored (`..`), so the
  open-shape decode only needs to reconstruct those two fields.

## What I changed

In `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️component.rs`, added a `//#region 🗂️TopicContribution`
block near the top:

- `IMPERATIVE_MODULE_TOPIC` const = `"imperative.module"`.
- `ImperativeModuleTopicPayload` — a private `#[derive(Deserialize)] #[serde(rename_all = "camelCase")]`
  struct with just `app_id: String, manifest_json: String`, matching the producer's payload shape.
- `imperative_module_fields(entry: &ProgramContributionEntry) -> Option<(String, String)>` — the merge:
  checks `entry.topic_contribution`, and if its `topic == "imperative.module"` and
  `.decode::<ImperativeModuleTopicPayload>()` succeeds, returns those fields; otherwise falls back to
  destructuring `entry.contribution` as `Contribution::ImperativeModule { app_id, manifest_json, .. }`;
  returns `None` if neither shape matches (i.e. this entry isn't an imperative-module contribution at all).

In `compose_registry`, replaced the old:
```rust
let Contribution::ImperativeModule { app_id, manifest_json, .. } = entry.contribution else { continue };
```
with:
```rust
let Some((app_id, manifest_json)) = imperative_module_fields(&entry) else { continue };
```
Everything downstream (the `IMPERATIVE_PLAY_APP_ID` filter, manifest parsing, native-registrar
dispatch, catalogue merge) is unchanged — it now just runs on fields sourced from whichever shape
was present, open preferred.

This completes the loop described in the task brief for all 5 imperative extensions
(logic/effect/math/control/text): each extension's manifest build already populates
`topic_contribution` via the shared `imperative_module_contribution()` helper, and this registry
now reads it.

## Verification

- `cargo check -p semio-s-imperative` — **clean** (3 pre-existing warnings, none introduced by this
  change: two `unused extern crate` in that crate's own `glue.rs`, one `unused import: OperatorInfo`
  in the registry file that predates this edit).
- Also confirmed `cargo check -p semio-framework` (defines `Contribution`/`TopicContribution`) compiles
  clean standalone.
- Attempted `cargo check -p semio-s-plugin-sequence` (a real consumer plugin, per the task's fallback
  instruction) as an additional sanity pass — it currently fails, but the failure is unrelated to this
  change: `semio-framework-math`'s `🕸️graph/🗣️dsl/🦀️component.rs:849` has a non-exhaustive match over
  `TokenKind` (`Lt`/`Gt`/`Amp`/3 more not covered). This matches the operational note about "a math
  tokenizer TokenKind mid-edit" by another concurrent session — not something I touched or fixed,
  per instructions not to chase unrelated breakage. `semio-s-imperative` itself, which is the crate my
  file actually belongs to, checked clean.
- Also saw one transient unrelated failure earlier in `🧰️framework/🔨️modules/🗣️dsl/📖️grammar/🦀️component.rs`
  (missing field `lex` in `GrammarFile` initializer) on a first `cargo check -p semio-s-imperative` run
  that self-resolved on retry a few minutes later — consistent with concurrent live edits to shared dsl
  files, not caused by or related to this change.

## Files touched

- `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️component.rs` (edited)

No other files touched. No git-state-modifying commands run (read-only `git log`/`git diff`/`git status`
only, to distinguish pre-existing vs. transient-concurrent errors from anything of mine).
