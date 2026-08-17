# W2 — Open Contribution (additive `TopicContribution` alongside closed `Contribution`)

## Goal
Add an open, generic replacement shape for the closed `Contribution` enum / `PluginContribution` union
(7 hardcoded s-plugin-specific variants: `PlaybookBlockKind`, `SourcingModule`, `ProcessMachines`,
`FlowExtension`, `FormsQuestionKind`, `CadComputer`, `ImperativeModule`) that the generic
`🧰️framework/🔨️modules/🛂️manifest` module should not know by name. Additive only — the closed enum/union
is untouched and stays live; a later wave removes it once every producer/consumer has migrated.

## Pre-read
Read `📓️w2-catalog-injection.md` first (parallel agent's prior edit to this same `🟦️component.ts` file,
removing its upward import of the OS product's generated plugin/playground registry). Confirmed my edit
is orthogonal — it touches a completely different region of the file (the `PluginContribution`/
`PluginManifest` types near the bottom, not the deleted import at the top) and does not conflict with
or revert that change.

## Type names chosen
- Rust: `TopicContribution` (`🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`)
- TS: `TopicContribution` (`🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`)
Same name both sides, per the task's own suggestion, kept consistent for greppability.

## What changed

### 1. `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`
- Read the existing `Contribution` enum (lines ~2685–2782, 7 variants) and `PluginManifest` struct
  (~2801–2817) fully first.
- Added a new `//#region 🔖️TopicContribution` / `//#endregion` block right after
  `parse_contributions()` and before `PluginManifest`, containing:
  ```rust
  #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
  #[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
  #[serde(rename_all = "camelCase")]
  pub struct TopicContribution {
      pub topic: String,
      #[cfg_attr(feature = "typegen", ts(type = "unknown"))]
      pub payload: serde_json::Value,
  }
  impl TopicContribution {
      pub fn new(topic: impl Into<String>, payload: serde_json::Value) -> Self { .. }
      pub fn decode<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> { .. }
  }
  ```
  (Same shape the task specified; added the `typegen`/ts-rs derive + `#[cfg_attr(.., ts(type =
  "unknown"))]` on `payload` to match this file's existing convention for every other manifest type —
  the task's literal snippet omitted those since it isn't ts-rs-aware, but every sibling type in this
  file carries them.)
- Doc comment on `TopicContribution` states the topic-id convention: reuse the existing
  `contributes`/`consumes` string vocabulary already used elsewhere in this codebase's crate metadata
  (e.g. `"flow.extension"`, `"playbook.blockKind"`, `"cad.computer"`) — no topics enumerated here, each
  future producer/consumer wave picks its own.
- Added a new field to `PluginManifest`, alongside the existing `contributions: Vec<Contribution>`:
  ```rust
  #[serde(default, skip_serializing_if = "Vec::is_empty")]
  pub topic_contributions: Vec<TopicContribution>,
  ```

### 2. `🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`
- Added `export type TopicContribution = { readonly topic: string; readonly payload: unknown };` right
  before `PluginManifest`, with the same topic-vocabulary doc comment as the Rust side.
- Added `readonly topicContributions?: readonly TopicContribution[];` to `PluginManifest`, optional —
  matches this interface's existing convention for `contributions?`/`commands?` (both already optional
  here, unlike the Rust struct's `#[serde(default)]`-but-required-field convention). Since it's optional
  and the type is a plain `type` alias (not a `class`), no existing object literal anywhere needs
  updating — TS excess-property checks never flag a *missing* optional field.

## Call sites touched to keep things compiling
**TS: none needed.** `topicContributions` is optional; every existing `PluginManifest`-shaped object
literal keeps compiling unchanged.

**Rust: none touched (out of ownership) — full compile-break survey below.** `PluginManifest` has no
`Default` impl and none of its construction sites use `..Default::default()`/struct-update syntax, so
every exhaustive struct-literal site needs `topic_contributions: vec![]` (or `Vec::new()`) added by
hand once this field lands. My file ownership for this wave is `🛂️manifest/🦀️component.rs` and
`🟦️component.ts` only — I did not touch any of the sites below. Ran `cargo check` scoped per-crate to
enumerate exactly what breaks:

- `cargo check -p semio-framework` (the crate `🛂️manifest` itself compiles into): **clean, zero errors**
  (only pre-existing warnings from `semio-framework-os-kernel`, unrelated to this change).
- `cargo check -p semio-framework-plugin`: **2 errors**, both `E0063: missing field
  'topic_contributions'`:
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:5884`
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6120`
- `cargo check -p semio-framework-plugin-host`: **1 error**, same kind:
  - `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️component.rs:816`
- `cargo check -p semio-framework-os-renderer-wgpu`: pulls in `semio-framework-plugin` as a dependency,
  surfaces the same 2 errors above (transitively blocked, no independent errors of its own reached).
- `cargo check -p semio-framework-os` (the `🖥️host` product crate): also depends on
  `semio-framework-plugin`, blocked by the same 2 errors before reaching its own compilation unit — but
  **that unit itself also has matching literal sites that will need the same fix once unblocked**,
  found by grep (not yet cargo-confirmed since the build never got that far):
  - `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs:974, 1121, 1128, 1187, 1194, 1219, 1235`
    (7 sites — `#[path = "../../🦀️component.rs"]` in this crate's `📦️glue.rs` maps to this exact file)
- Renderer `Shell` component (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs:5466`)
  also constructs a `PluginManifest {}` literal — same fix will be needed there, whichever crate that
  compiles into (did not resolve which crate before running low on scope; grep-confirmed site only).
- **`🧰️framework/🛍️products/💻️os/🦀️component.rs`** (products/os root, 7 near-duplicate `PluginManifest {}`
  sites at lines 971, 1118, 1125, 1184, 1191, 1216, 1232 — text is almost byte-identical to the
  `🖥️host/🦀️component.rs` file above): grepped every glue.rs `#[path = ...]` in the tree and found **no
  crate currently wires this file in** (only `🖥️host/🦀️component.rs`, a sibling file with a different
  path, is wired via `#[path = "../../🦀️component.rs"]` from `🖥️host/📦️packages/🦀️rust/📦️glue.rs`). This
  file is very likely a live in-progress artifact of the concurrent cross-session refactor mentioned in
  my briefing (not yet spliced into any crate) — did not touch it, did not investigate further, per
  instruction to leave unrelated concurrent-refactor debris alone.

None of the above files are in my ownership for this wave, so none were edited. All are narrow,
single-line, mechanical additions (`topic_contributions: vec![]` inserted into each literal) — flagging
for whichever wave/agent owns those files next, mirroring how the prior `w2-catalog-injection` wave
left its one out-of-ownership `resolvePlaygroundBoot` caller as a documented follow-up rather than
reaching outside its file list.

## Verification
- **Rust**: `cargo check -p semio-framework` (the crate containing `🛂️manifest`) — **0 errors, 0 new
  warnings** attributable to this change. Confirmed the 3 downstream crates that construct
  `PluginManifest` via exhaustive literals (`semio-framework-plugin`,
  `semio-framework-plugin-host`, transitively `semio-framework-os-renderer-wgpu`) now fail with exactly
  the expected `E0063: missing field 'topic_contributions'` at the exact literal sites — proof the new
  field is wired correctly and additively (no other diagnostic kind, no fallout beyond the expected
  field-count mismatch).
- **TS**: no framework-wide `typecheck` nx target exists (confirmed same absence the prior wave found).
  Built a scoped `tsconfig` (`extends` root `tsconfig.json`, `include` = only `🟦️component.ts`) and ran
  `bunx tsc --noEmit --incremental false --allowImportingTsExtensions true` against it. Zero diagnostics
  anywhere in `🟦️component.ts` itself; the diagnostics tsc did emit (95 lines total) are all pre-existing,
  in `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🟦️manifest.ts` — `Cannot find name 'UiMenuRef'/'Label'/
  'Locale'/'Terminology'/'UiTreeActionPlacement'` — the same latent "type used without local import, only
  ever re-exported via `export *`" debt the prior wave's `glue.ts` scoped check also hit, unconnected to
  this edit.

## Files touched
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🛂️manifest/🟦️component.ts`

No other files edited.
