# Implementation — app-owned factory for all 29 generation3d tools

## Summary

Gave `generation3d`'s editor app (`Generation3dPlayApp`) a real app-owned tool-job factory
(`Generation3dBoundedCommandJobFactory`) covering **all 29** declared commands (the 23 that already
claimed `Migrated` plus the six that were `BatchOnlyPendingRewrite`), plus Artifact- and Config-lane
`ArtifactStoreOneItemPreparationFactory` implementations. Per the task's instruction this is the
**complete** fix (plan `📓️plan-migrate-six-actions.md` §7 explicitly scoped itself to only the six and
left the other 23 on the bare `"BoundedFirstStepCommandJobFactory"` sentinel — that scoped option was
NOT taken; every one of the 29 tool-proof rows now shares one `factory_type:
Generation3dBoundedCommandJobFactory`).

## Files changed

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs`
  (main change — see anchors below)
- `…/✏️editor/🎮️commands/🕸️node-graph-edit/🦀️.rs` — added `pub(crate) fn apply_selected`
- `…/✏️editor/🎮️commands/🧩️delete-selection/🦀️.rs` — added `pub(crate) fn apply_selected`
- `…/✏️editor/🎮️commands/🧭️translate-selection/🦀️.rs` — added `pub(crate) fn apply_selected`
- `…/✏️editor/🎮️commands/🧭️rotate-selection/🦀️.rs` — added `pub(crate) fn apply_selected`
- `…/✏️editor/🎮️commands/🧭️scale-selection/🦀️.rs` — added `pub(crate) fn apply_selected`

### Editor `🦀️.rs` anchors (post-edit line numbers)

- Imports: lines 26-34 — added `semio_framework::{ToolExecutionContract, ToolFactoryKey,
  ToolJobFactoryError}`, `semio_framework_plugin::retained_command::{ArtifactRetainedCommandJob,
  ArtifactRetainedCommandPayload, BoundedArtifactCommandWork}`, and `AppOperationContext`,
  `ArtifactOwnedToolJobRequest`, `ArtifactToolFactoryRegistry`, `ArtifactToolPublicationContract`,
  `ArtifactToolPublicationLane`, `EditorApp` into the existing `semio_framework_plugin::{...}` block.
- `//#region 🧵️RetainedCommands` (~161-320): `GENERATION3D_RETAINED_TOOL_IDS` (all 29 ids),
  `generation3d_bounded_contract()`, `generation3d_bounded_extent()`, `generation3d_retained_reduce()`
  (the retained-command reducer — routes `nodeGraphEdit`/`deleteSelection`/`translateSelection`/
  `rotateSelection`/`scaleSelection` through their new `apply_selected` real-interaction-selection
  entry points, everything else through `command.dispatch(&doc, &cfg, &mut session)`),
  `Generation3dBoundedCommandJobFactory` (`ToolJobFactory` + `ArtifactOwnedToolJobFactory` impls, 29
  `PUBLICATION_CONTRACTS` entries — see lane table below).
- `//#region 📬️ArtifactStorePreparation` (~322-450): `generation3d_next_edit` (shared `protocol::Edit<M>`
  builder for both lanes), `Generation3dArtifactStorePreparationFactory` /
  `Generation3dArtifactStorePreparation` — **generic** over `Generation3dMutation` via
  `protocol::Mutation::diff`/`inverse` + `protocol::MutationDiff::apply` (modeled on lowpoly's
  `LowpolyArtifactStorePreparation`, `✏️s/🔌️plugins/💠️lowpoly/…/✏️editor/🦀️.rs:1246-1383`, confirmed
  byte-for-byte identical `advance()`/`begin()` gating logic; `close_step` uses a simpler
  unconditional-release strategy rather than lowpoly's per-field budget check — a deliberate
  simplification, not a correctness gap, since `close_step` only runs during store teardown).
- `//#region 📬️ConfigStorePreparation` (~452-580): `Generation3dConfigPreparationFactory` /
  `Generation3dConfigPreparation` — **also generic**, not hand-matched. This deviates from the plan's
  §4.3 (which hand-matched only `SetGeneration` because the plan was scoped to 6 tools). Once all 29
  tools share one factory, the Config lane is exercised by **9** of the 10 `Generation3dConfigMutation`
  variants (`SetLodMode`, `SetShowMode`, `SetCamera`, `SetPreviewCamera`, `SetSun`, `SetGeneration`,
  `SetActiveUtility`, `SetLocale`, `SetPreviewEval`) — hand-matching would have meant duplicating the
  match arm from `Generation3dConfigMutation`'s own `Mutation::diff` impl
  (`…/✏️editor/🎚️config/🦀️.rs:264-322`, which already implements `protocol::Mutation<Generation3dConfig>`
  with a real `inverse`/`diff`, verified before writing this). The generic form reuses that existing
  impl instead, so there is exactly one place that knows what each config mutation does.
- Four `ArtifactEditor` overrides (`build_artifact_store_one_item_preparation_factory`,
  `build_config_store_one_item_preparation_factory`, `register_tool_job_factories`, `build_tool_job`)
  inserted immediately before the `bounded_first_step_tool_proofs!` invocation (~690-735).
- `bounded_first_step_tool_proofs!` block (~737-767): `factory: "Generation3dBoundedCommandJobFactory"`
  (was `"BoundedFirstStepCommandJobFactory"`), added `factory_type: Generation3dBoundedCommandJobFactory,`,
  added six new tool rows (`nodeGraphEdit`, `addGeneration`, `removeGeneration`, `renameGeneration`,
  `updateGenerationValues`, `selectGeneration`) — same literal contract as the pre-existing 23 rows
  (byte-for-byte, `bounded_first_step(8_192, 32, 32, 16_384, 7_500)`), so nothing about the 23 changed
  except which factory owns them.
- Six classification flips (~1136-1170): `nodeGraphEdit`, `addGeneration`, `removeGeneration`,
  `renameGeneration`, `updateGenerationValues`, `selectGeneration` — `BatchOnlyPendingRewrite` →
  `Migrated`. Zero `BatchOnlyPendingRewrite` occurrences remain in the file.
- New test `retained_route_dispositions_are_exact_and_exhaustive` (inserted right after
  `command_ids_are_unique_and_cover_every_row`, in the `#region 🔖️CommandSurface` test block): asserts
  `GENERATION3D_RETAINED_TOOL_IDS.len() == 29`, `bounded_first_step_tool_proofs().len() == 29`,
  `PUBLICATION_CONTRACTS.len() == 29`, contract shape/cancellation policy, no duplicate tool ids, and
  that every `every_command()` row's `command_id()` is in `GENERATION3D_RETAINED_TOOL_IDS`.

### Command-handler anchors

Each of the five interaction-aware handlers got one new `pub(crate) fn apply_selected(...)` appended
right after its existing `pub fn apply(...)` — same body as `apply`, but taking `selected: &[String]`
directly instead of `interaction: &app::InteractionView<'_>` (plugin code cannot construct an
`InteractionView` itself — its fields are `pub(crate)` to `semio_framework_plugin`). This lets
`generation3d_retained_reduce` preserve the exact same real-`graph`-selection behavior
`Generation3dPlayApp::handle` already gives these five commands, reading selection straight off
`protocol::InteractionState` (`interaction.selection.get("graph").map(|s| s.ids.clone())`).

- `🕸️node-graph-edit/🦀️.rs:78-83` — `apply_selected(payload, doc, selected)` → `apply_operations(...)`
- `🧩️delete-selection/🦀️.rs:45-50` — `apply_selected(doc, selected)` → `delete_selected(...)`
- `🧭️translate-selection/🦀️.rs:100-105` — `apply_selected(payload, doc, selected)` → `translate_ids(...)`
- `🧭️rotate-selection/🦀️.rs:98-103` — `apply_selected(payload, doc, selected)` → `rotate_ids(...)`
- `🧭️scale-selection/🦀️.rs:97-102` — `apply_selected(payload, doc, selected)` → `scale_ids(...)`

## Lane table — all 29 tools, from each handler's actual `Emit` construction

Every row below was read directly from its handler's `handle`/`apply` function body (not guessed);
file:line citations are into the pre-edit handler files.

| # | Tool id | Emits | Lanes | Citation |
|---|---|---|---|---|
| 1 | `setActiveExample` | `artifact_mutations` + `config_mutations` (`Snapshot{config_after_example_load}`), or `Emit::default()` | Artifact + Config | `🎨️set-active-example/🦀️.rs:41-53` |
| 2 | `nodeGraphEdit` | `Emit{artifact_mutations,..}` via `apply_operations` | Artifact | `🕸️node-graph-edit/🦀️.rs:23-51` |
| 3 | `deleteSelection` | `Emit{artifact_mutations,..}` via `delete_selected` | Artifact | `🧩️delete-selection/🦀️.rs:15-22` |
| 4 | `removeWidget` | `Emit{artifact_mutations,..}` or `Emit::default()` | Artifact | `🧩️remove-widget/🦀️.rs:19-27` |
| 5 | `moveMediaNode` | `Emit::mutations(...)` or `Emit::default()` | Artifact | `🕸️move-media-node/🦀️.rs:19-25` |
| 6 | `addWidget` | `Emit{artifact_mutations,..}` or `Emit::default()` | Artifact | `🧩️add-widget/🦀️.rs:21-41` |
| 7 | `patchFlowWidgets` | `Emit::mutations(...)` | Artifact | `🧩️patch-flow-widgets/🦀️.rs:19-33` |
| 8 | `reorganize` | `Emit::mutations(...)` or `Emit::default()` | Artifact | `🕸️reorganize/🦀️.rs:15-21` |
| 9 | `translateSelection` | `Emit{artifact_mutations, coalesce_key,..}` or `Emit::default()` | Artifact | `🧭️translate-selection/🦀️.rs:64-72` |
| 10 | `rotateSelection` | same shape | Artifact | `🧭️rotate-selection/🦀️.rs:65-73` |
| 11 | `scaleSelection` | same shape | Artifact | `🧭️scale-selection/🦀️.rs:64-72` |
| 12 | `addGeneration` | `artifact_mutations` (`CreateGeneration`) + `config_mutations` (`SetGeneration`) | Artifact + Config | `🧬️add-generation/🦀️.rs:17-35` |
| 13 | `removeGeneration` | `artifact_mutations` (`DeleteGeneration`) + `config_mutations` (`SetGeneration`) | Artifact + Config | `🧬️remove-generation/🦀️.rs:17-35` |
| 14 | `renameGeneration` | `artifact_mutations` (`RenameGeneration`) + `config_mutations` (`SetGeneration`) | Artifact + Config | `🧬️rename-generation/🦀️.rs:17-35` |
| 15 | `updateGenerationValues` | `artifact_mutations` (`ChangeGenerationValue`) + `config_mutations` (`SetGeneration`) | Artifact + Config | `🧬️update-generation-values/🦀️.rs:17-35` |
| 16 | `nodeGraphViewport` | `Emit::config([SetCamera])` | Config | `🕸️node-graph-viewport/🦀️.rs:17-18` |
| 17 | `worldPointerDown` | `Emit::default()` always | HostOnly | `🗂️world-pointer-down/🦀️.rs:14-15` |
| 18 | `graphPointerDown` | `Emit::default()` always | HostOnly | `🕸️graph-pointer-down/🦀️.rs:14-15` |
| 19 | `setLodMode` | `Emit::config([SetLodMode])` | Config | `👁️set-lod-mode/🦀️.rs:16-17` |
| 20 | `setShowMode` | `Emit::config([SetShowMode])` | Config | `👁️set-show-mode/🦀️.rs:16-17` |
| 21 | `toggleSun` | `Emit::config([SetSun])` | Config | `🌞️toggle-sun/🦀️.rs:14-17` |
| 22 | `setSunAzimuth` | `Emit::config([SetSun])` | Config | `🌞️set-sun-azimuth/🦀️.rs:16-19` |
| 23 | `setSunElevation` | `Emit::config([SetSun])` | Config | `🌞️set-sun-elevation/🦀️.rs:16-19` |
| 24 | `setSunIntensity` | `Emit::config([SetSun])` | Config | `🌞️set-sun-intensity/🦀️.rs:16-19` |
| 25 | `setCamera` | `Emit::config([SetPreviewCamera])` | Config | `👁️set-camera/🦀️.rs:17-18` |
| 26 | `selectGeneration` | `Emit::config([SetGeneration])` only | Config | `🧬️select-generation/🦀️.rs:18-24` |
| 27 | `setActiveUtility` | `Emit::config([SetActiveUtility])` | Config | `👁️set-active-utility/🦀️.rs:20-21` |
| 28 | `setLocale` | `Emit::config([SetLocale])` | Config | `🗣️set-locale/🦀️.rs:16-17` |
| 29 | `flowEvalTick` | `Emit{effects, config_mutations: [SetPreviewEval],..}` | Config | `🧮️flow-eval-tick/🦀️.rs:14-32` (the `effects` field is not lane-gated — only `artifact_mutations`/`config_mutations`/`draft_mutations`/ephemeral presence/transient/child are, per `require_complete_tool_operation_pipeline` `🧰️framework/…/🔌️plugin/🦀️.rs:22273-22278`) |

No tool needed the `Draft`, `Presence`, `Transient`, or `Child` lanes.

## Size limits

- `GENERATION3D_RETAINED_RAW_BYTES = 8_192` — matches the wire-byte limit already in every one of the
  original 23 contracts.
- `GENERATION3D_ARTIFACT_STORE_MAXIMUM_BYTES = 65_536` — checked against the real fixtures: all 8 example
  DSLs under `📚️examples/*/🖼️assets/*/🗣️.dsl.semio` are 586-1,610 bytes of text
  (`hexagonal-mushroom-column` is the largest at 1,494 bytes); `setActiveExample`'s full-fixture replace
  is the single largest Artifact mutation any tool emits, so 64 KiB is generous but real.
- `GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES = 262_144` — sized for `flowEvalTick`'s `SetPreviewEval`
  (`FlowEvalSession::eval_json()`), the largest Config payload; this is a per-node scalar/string
  evaluation summary, not mesh geometry (full mesh export goes through `export_media("geometry:out", ..)`
  as a separate media port, never through config — confirmed by reading `Generation3dPlayApp::export_media`
  and `flow_eval_tick::handle`). Not empirically measured (the crate would not build — see Verification),
  so this is a documented heuristic bound, not a measured one; flagged as open below.

## Verification

### Baseline (before any edit)

```
cargo test --package semio-s-plugin-procedural --lib generation3d:: -- --nocapture
```
Did **not** reach test execution at all: the crate failed to compile with **606 pre-existing errors**
(`error[E0277]`: 454, `E0308`: 50, `E0599`: 40, `E0433`: 21, `E0423`: 14, `E0425`: 6, `E0422`: 5, others).
These are NOT `interactive-job.missing-owned-reducer` failures (the expected signature per the ticket's
own root-cause analysis) — they are a much larger, unrelated, in-flight concurrent refactor spanning the
whole `procedural` plugin: `serde::Serialize`/`DeserializeOwned` not satisfied for `Generation3dSnapshot`/
`MeshData`/`PluginAssemblyError` (matches the in-progress
`RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/serde-replacement-surface.md` ticket referenced
in `Generation3dMutation`'s own doc comment), plus dozens of `impl Future<Output = ...>` "no method found"
errors (an in-progress async-conversion of `ArtifactApp`/`VcsArtifactApp` methods whose call sites haven't
been updated with `.await` yet), plus `expected function, found module` errors on every mutation-leaf module
(`create_widget`, `update_widget`, etc.) repeated identically across `assembly`, `generation2d`, and
`generation3d`'s own mutation test files. Full output: `🗑️generated/gen3d-tests-baseline.txt` (47,904 lines).
**Result: 0 passed, 0 failed — did not compile, entirely pre-existing, unrelated to this ticket's files.**

### After the edits

```
cargo check --package semio-s-plugin-procedural
```
Also did not reach `semio-s-plugin-procedural` itself: `cargo tree -p semio-s-plugin-procedural` confirms
it depends on `semio-s-plugin-stdio`, which now fails to compile first (topological build order) with 7
errors, all inside `✏️s/🔌️plugins/🗄️stdio/…/🧿️semio/…/🧬️schema/⚙️engine/🦀️.rs` and
`…/💡️inferences/📏mass-properties/🦀️.rs` — a BREP CAD kernel engine file I never touched (`curve_ops::closest_point`
not found, `impl BrepKernel for Brep` missing 4 trait methods, 5 borrow-checker errors) — another session's
in-progress work landing mid-build. Full output: `🗑️generated/gen3d-check-after.txt`.
**Result: could not determine pass/fail for `semio-s-plugin-procedural` itself — blocked one dependency
crate upstream by unrelated concurrent breakage.**

Per the coordinator's live guidance during this session (native `target/` lock contention, then
`wasm32-wasip2` blocked "waiting for file lock on build directory" since 17:05 by another session's stdio
build), the `wasm32-wasip2` check and `bun ./📜️script.ts describe` + descriptor diff were **not run** in
this session — the coordinator will run them as part of their own dev-boot pass. Marked PENDING below with
the exact commands.

### What was verified instead (no working compiler available)

- `rustfmt --edition 2021 --check` on the full edited editor file and all five edited command-handler
  files: **zero parse errors** on every file (only cosmetic re-wrapping diffs from rustfmt's default
  100-col width vs. the repo's wider `rustfmt.toml`) — rustfmt fully parses a file before it can diff it,
  so this rules out any unclosed delimiter / malformed syntax across the entire ~2,770-line editor file
  and all five handler files, start to finish.
- Brace/paren/bracket counts balance across the whole edited editor file (704/704, 2607/2607, 234/234).
- `GENERATION3D_RETAINED_TOOL_IDS` (29), `Generation3dBoundedCommandJobFactory::PUBLICATION_CONTRACTS`
  tool ids (29), and the `bounded_first_step_tool_proofs!` macro's 29 tool rows are set-identical
  (scripted diff, confirmed empty).
- Cross-checked every generic-preparation-factory assumption against real trait definitions before
  writing code: `Generation3dMutation: protocol::Mutation<Generation3dSnapshot>` (via `#[derive(dsl::Mutations)]`,
  `Diff = Generation3dDiff`, which implements a real structural `protocol::MutationDiff` —
  `🧬️schema/🔺️diff/📝️text/🦀️.rs:154`); `Generation3dConfigMutation: protocol::Mutation<Generation3dConfig>`
  (hand-written, `✏️editor/🎚️config/🦀️.rs:264-322`, `Diff = Generation3dConfig`, which implements
  `protocol::MutationDiff<Generation3dConfig>` as an identity apply via
  `store::impl_whole_record_config!(Generation3dConfig)`); both implement `protocol::OpBinary` (used for
  the retained-byte-size admission check).

## Open items

1. **wasm32-wasip2 check** — not run this session (blocked by another session's build-lock contention on
   the shared `target/`; per coordinator instruction, deferred to their dev-boot pass). Command:
   `cargo check --package semio-s-plugin-procedural --target wasm32-wasip2` (or with
   `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/target-gen3d RUSTC_WRAPPER="" ... -j 4` if the shared
   target dir is still contended).
2. **Descriptor regeneration** — not run this session (needs a clean wasm build first). Command:
   `cd "✏️s/🔌️plugins/🌀️procedural/📦️packages/🦀️rust" && bun ./📜️script.ts describe`, then confirm
   `✏️s/🔌️plugins/🌀️procedural/🔣️.json` reads `"interactiveJob": "migrated"` for `nodeGraphEdit`,
   `addGeneration`, `removeGeneration`, `renameGeneration`, `updateGenerationValues`, `selectGeneration`
   in app `s.procedural.generation3d@1/*#editor`, and diff the rest of the file against
   `git show HEAD:'✏️s/🔌️plugins/🌀️procedural/🔣️.json'` to confirm no other action's value moved.
3. **Real test run** — `cargo test --package semio-s-plugin-procedural --lib generation3d::` needs to be
   re-run once `semio-s-plugin-stdio` (and whatever remains of the serde/async churn seen at baseline)
   is fixed by its owning session(s). This implementation was verified by source reading and rustfmt-level
   syntax checking only, NOT by a passing compile or test run — that is the one thing this report cannot
   honestly claim.
4. `GENERATION3D_CONFIG_STORE_MAXIMUM_BYTES` is a reasoned-but-unmeasured heuristic (see Size limits above)
   — worth confirming empirically against `flowEvalTick`'s real `eval_json()` output for all 8 examples
   once the crate builds.
5. Config-lane `close_step` uses a simpler unconditional-release strategy than lowpoly's per-field
   budget-respecting version (see ArtifactStorePreparation anchor above) — functionally adequate for
   store teardown but not textually identical to the lowpoly precedent; noted for a future pass if strict
   budget parity with lowpoly is wanted.
