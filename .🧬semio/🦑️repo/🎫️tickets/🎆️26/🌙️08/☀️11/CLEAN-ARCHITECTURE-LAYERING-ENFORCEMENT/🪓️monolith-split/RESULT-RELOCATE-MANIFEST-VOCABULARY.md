# Result: relocate the manifest product vocabulary (slice A, `relo-manifest`)

Executor: Opus, foreground, no sub-agents, 2026-10-15 14:50–17:10. Brief: `📓️RELOCATION-BRIEF.md` slice A.

## 0. Verdict

`🧰️framework/🔨️modules/🛂️manifest/🧳️product-vocabulary/` is gone, and no compatibility re-export is left. Each item now lives with the module that implements it or whose types need it. History edit, tool run and the gumball audience rule reach apps through a new neutral registration point, `ReservedContribution`; the manifest no longer names them. Introduction and tutorial became the framework module `🎓️onboarding`. Callers, TS twins, fixtures, tests, script filters, source-text readers and taxonomy are updated.

Concurrency, which you need to know to read this report: the wave-4 FRAMEWORK-FACADE carver (`w4fw-tools/`, FLEET.md: "FRAMEWORK-FACADE (plus relocating product vocabulary)") overlaps this slice. It carved the facade while I worked and absorbed my moves into crates without changing them semantically:
- my `🎓️onboarding/🦀️.rs` became `semio-framework-onboarding`;
- `⏪️time-travel/🕹️actions/🦀️.rs` became `semio-framework-time-travel-actions`;
- `⏯️tool-run/🕹️actions/🦀️.rs` became `semio-framework-tool-run-actions`;
- the facade re-exports them as `semio_framework::{onboarding, history_edit, tool_run}`, the same paths my caller rewrites target;
- the manifest children became `semio-framework-manifest-*` crates, and `🛂️manifest/🦀️.rs` was deleted.

The final crate layout of the facade is therefore that carver's work, not mine. This report gives the final locations as they stand on disk at 17:00.

Compile verification is blocked by the launcher ("Finite original maintenance authority required", section 7). Everything below was verified statically or with TS tooling. No Rust test was run.

## 1. Decisions per item (owner evidence)

| Item | Decision | Evidence | Final location |
| --- | --- | --- | --- |
| `ActionKind` | framework contract, not product vocabulary | It is a field of `ActionDefinition`/`CommandDefinition`, and `derive_audience`/`ActionSemantics::for_kind` in the actions module consume it. Variant docs neutralised (no more camera, selection or hover list). The projection doc (type level) is unchanged. | `🛂️manifest/🕹️actions` (crate `semio-framework-manifest-actions`) |
| `InteractiveJobClassification` | framework contract | It is a field of `CapabilityExecution`, set by `resumable_framework`, and validated by `validate_interactive_job_classification` in the same module. The "Phase-8 migration" doc is neutralised. | same |
| `AppRole` | framework contract | `AppDefinition.role`, `surface_app_id`, and `ActionArgControl::{ArtifactKind,SurfaceApp}.roles`. I placed it in app-surface. The carver moved it to the lower arg-schema crate, because arg-schema needs it and the crate DAG points downward. | `🛂️manifest/🧮️arg-schema` (crate `semio-framework-manifest-arg-schema`) |
| `PackageRole` | framework contract | `PackageDescriptor.role` | `🛂️manifest/🧾️plugin-manifest` (crate `semio-framework-manifest-plugin`; I placed it beside `PackageDescriptor`, which the carver moved there) |
| `OsMediaCapability`, `MediaClass`, `MediaForm` | media vocabulary | `MediaType {class, form}` and `ArtifactKindSpec.media_capability` | `🛂️manifest/🎞️media` (crate `semio-framework-manifest-media`, the media owner) |
| `HISTORY_EDIT_*`, `history_edit_action_definitions`, `history_edit_finalize_dialog`, tests | feature of the framework time-travel module | The session reducer is `semio-framework-time-travel` (`role = framework`), and the framework TS kernel and replication twins implement the history edit. | `⏪️time-travel/🕹️actions/🦀️.rs` = `semio_framework::history_edit`, contributes `RESERVED` (app actions + editor dialog) |
| tool run re-export, `app_declares_tool_run`, `tool_run_action_definitions`, tests | feature of the framework tool-run module | `semio-framework-tool-run` (`role = framework`) owns the ids, labels and chords. | `⏯️tool-run/🕹️actions/🦀️.rs` = `semio_framework::tool_run`, contributes `RESERVED` (window actions). The carver dropped my vocabulary re-export, so consumers now depend on `semio-framework-tool-run` directly (section 3). |
| `START_INTRODUCTION_*`, `Introduction*`, `Tutorial*`, tutorial engine, start/record tutorial actions, `TUTORIAL_CONVERGE_MS` | framework UI feature, `🎓️onboarding` | The framework UI implements it (`🖱️ui/🎓️introduction` eligibility, React introduction overlay, TS tutorial engine port). `AppDefinition.introduction/tutorials` carry its declarations, following the precedent of `AppDefinition.interactions` with the `🕹️interaction` module. | new module `🧰️framework/🔨️modules/🎓️onboarding/` = `semio_framework::onboarding` |
| `GUMBALL_CHROME_ACTION_IDS`, `framework_fixed_audience(_violation)` | os rule, enforced by the plugin SDK builder | The gumball is implemented by os interaction hosts (World3dHost, Canvas2dHost, infinite world gumball). Only plugins declare `setTransformGumballFlag`, and only the builder applied the rule. | `🔌️plugin/🏗️builder/🧩️app/🖐️gumball-audience/🦀️.rs` (`gumball_fixed_audience`, `RESERVED` with a fixed audience) |

Onboarding uses direct calls instead of the registry. The builder is the authoring API: it has `.introduction()`/`.tutorial()` and validates onboarding by name, and its app-roster lane runs before an `AppDefinition` exists. So it calls `semio_framework::onboarding::*` directly, which is a downward os → framework edge. The manifest no longer names onboarding vocabulary. Only `AppDefinition`'s typed fields import its declaration types, as they do for interactions.

## 2. Registration point

`ReservedContribution { id, app_actions: fn() -> Vec<ActionDefinition>, window_actions: fn(&AppDefinition) -> Vec<ActionDefinition>, editor_dialogs: fn() -> Vec<DialogDefinition>, fixed_audience: fn(&str) -> Option<CapabilityAudience> }`. It is built with const builders (`new`, `with_app_actions`, …) and consumed through `reserved_app_actions`, `reserved_window_actions`, `reserved_editor_dialogs`, `reserved_fixed_audience` and `reserved_audience_violation`. I wrote it in `🕹️actions`; the carver placed it in the app-surface crate, because it needs `AppDefinition`/`DialogDefinition`.

Hosts:
- The facade's `FRAMEWORK_RESERVED_CONTRIBUTIONS = [history_edit::RESERVED, tool_run::RESERVED]`.
- The plugin builder (`builder_reserved_contributions()` = framework list + `gumball_audience::RESERVED`) uses it for:
  - the audience check (former `framework_fixed_audience_violation`);
  - the history roster: `history_action_definitions()` is now the 7 lane verbs, followed by `reserved_app_actions`, in the same order as before;
  - editor dialogs: the finalize dialog is added only for editors, as before;
  - window actions: tool run, in the same position as before.
- The mcp catalog `framework_action_definitions` uses the framework list, in identical order. It still adds only `startIntroduction` from onboarding, so its published semantics are unchanged.

Tests:
- `🛂️manifest/🧪️tests/🧩️reserved-contributions/🦀️.rs` proves that non-default contributions are honoured: empty contribution, app-action order, window actions computed from the app, editor dialogs, first owner of a fixed audience, and violation with a declared override.
- Per-feature tests: `⏪️time-travel/🕹️actions/🧪️tests` (contribution reserves verbs + dialog), `⏯️tool-run/🕹️actions/🧪️tests` (window-only contribution), and the builder gumball test, which drives `reserved_audience_violation(&[RESERVED])` against the language-agnostic fixture.
- The carver added `🛂️manifest/🧪️tests/🧩️framework-reserved` for facade hosting.

## 3. Files changed by me

Moves (plain `mv`):
- `🛂️manifest/🧪️tests/🧪️history-edit-actions/{🦀️.rs,🟦️.ts}` and `🧫️fixtures/🧫️history-edit-actions/🔣️.json` → `⏪️time-travel/🕹️actions/🧪️tests/{🦀️.rs,🟦️.ts,🧫️fixtures/🔣️.json}`
- `🔬️tool-run-actions/{🦀️.rs,🟦️.ts}` and `⏯️tool-run-actions.json` → `⏯️tool-run/🕹️actions/🧪️tests/…`
- `🖐️gumball-verb-audience/🦀️.rs` and its fixture → `🔌️plugin/🏗️builder/🧩️app/🖐️gumball-audience/🧪️tests/…`
- `🎞️tutorial-document-track.json`, `🖱️tutorial-local-interaction.json` → `🎓️onboarding/🧪️tests/🧫️fixtures/`
- `🛂️manifest/🎬️tutorial/🏠️local-interaction/{🟦️.ts,🧬️contract.json}` → `🎓️onboarding/🏠️local-interaction/`

New files:
- `🎓️onboarding/{🦀️.rs,🟦️.ts,🧪️tests/🦀️.rs}`; the tests were carved out of `🔬️app-label`.
- `⏪️time-travel/🕹️actions/{🦀️.rs,🟦️.ts}`, `⏯️tool-run/🕹️actions/{🦀️.rs,🟦️.ts}`.
- `🛂️manifest/🗺️window-layout/🟦️.ts`: the hand-refined `WindowLayout` family extracted, to break a new type-only onboarding↔manifest cycle.
- `🛂️manifest/🧪️tests/🧩️reserved-contributions/🦀️.rs`, `🖐️gumball-audience/🦀️.rs`.

Framework edits:
- manifest children: `🕹️actions`, `🧱️app-surface`, `🖼️view-model`, `🧮️arg-schema` (`with_schema`/`plain_string` → `pub(crate)`), parent mounts and imports. The parent was later deleted by the carver.
- `🔬️app-label` test: onboarding tests removed, plus a history-lane and `switchAlternative` test.
- TS twin `🛂️manifest/🟦️.ts`: blocks moved out, imports added, dangling `PluginRuntime` region closed.
- the framework barrel `📦️packages/🟦️typescript/🟦️.ts` exports the four new twins.
- `🎠️kernel/🟦️.ts`, `🖥️platform/🟦️.ts`, `💻️os/🎚️config/🧬️schema/🟦️.ts` (`WindowLayout` import).
- `🧬️schema/📽️projection/🦀️.rs` and `🤖️generated/🪪️manifest/🟦️.ts`: identical doc text for `run`.
- `📦️packages/🦀️rust/📜️script.ts` test paths and filters; the carver later re-pointed the packages to the new crates.
- root `📜️script.ts` bracket-verb allow path; `🛠️tool-machine/🧪️tests/🧪️history-closure-policy/🟦️.ts` NEGATIVE path; `🔣️taxonomy.json` (`🎓️onboarding` in members-of-modules).

Callers (import and path edits only):
- **Plugin SDK:**
  - `🏗️builder/🧩️app` (injection sites);
  - `⏪️time-travel/🦀️.rs`, `⏪️time-travel/🧵️instance/🦀️.rs` (slice E files, import lines only);
  - `⚛️reactor/🔌️app`, `🎮️command`, `👷️job/🏛️reserved`, `📜️declaration/🧩️app`, `📜️declaration/🪟️surface`;
  - `🧵️instance/🦀️.rs`, `🧵️instance/📨️dispatch` (import and path only);
  - `🪟️window-kits`;
  - tests `🔬️app-app-builder`, `🔬️plugin-runtime-plugin-builder-contract`, `🧪️folder-reload-route`, `🧪️history-edit-acceptance`, `🧪️time-travel`.
- **mcp:** `🗂️catalog`.
- **os renderer** (16 files): EngineCanvas tutorial-surface-geometry, engine-surface-attach, wgpu-node-graph test; Shell time-travel, tutorial-runtime, shell-actions, chrome-dialog-and-tooltips, chrome-overlays-and-tour, panel-records, and 7 Shell tests; plus `engine-contract/🟦️.ts` and the ui test `owned-locale-detector-retirement/🟦️.tsx`. In the ui test this also fixes a fixture path that was broken before.
- **Plugins:** procedural 2d editor test, reasoning and dag reorganize tests, puzzle 2d fill test, puzzle 3d editor and 4 tests, remodel editor and example tests, energy editor test, trinity graph test, 6 composition tests.

Cargo (minimal consumer edits; locks NOT refreshed, section 7):
- `semio-framework-tool-run` added to os-renderer-wgpu (non-wasi deps);
- `semio-framework-tool-run` added to the dev-deps of `semio-s-artifact-remodel-remodeling` (owner of the editor/example `[[test]]`) and `semio-s-artifact-energy-model-editor`.

Schemas: the manifest `🧬️schema/{🔣️.json,🔗️.graphql,🛰️.proto,🟦️.ts}` hold only `AppRole`/`AppRef`/`ToolDefinition`/`UtilityDefinition`. These stay manifest wire types, so no schema edit was needed.

## 4. Verification (commands run, real results)

Tools are in `relo-manifest-tools/`: `oracle.py`, `privacy_scan.py`, `classify_callers.py`/`flag_callers.py`, `ts_sandbox.py`, `rewrite_callers.py`. Logs are `🗑️generated/relo-manifest-*`, and the baseline tarball is `🗑️generated/relo-manifest-baseline.tar.gz` (147 pristine files).

1. **Full-resolution differential** (`oracle.py`, analysis-only `rustc +nightly-2026-07-20 --emit=metadata -o /dev/null`, coordinator patches, baseline sandbox vs current):
   - `semio_framework_plugin`, 421-file closure via the oracle's module walker, coordinator patches applied (14 baseline / 20 current files patched; the extra 6 are my split `use` lines and new imports of externals):
     - native: 9256 / 9266. The 14 new keys are all unresolved *external* crates: 9 `semio_framework` and 4 `semio_framework_tool_run` path and import errors, from split brace imports and `semio_framework::onboarding::…`-style qualified paths, plus the abort line. 4 removed keys are of the same kind.
     - wasip2: 9547 / 9557, the identical pattern.
     - Neither has a new E0412/E0422/E0423/E0425/E0603/E0616/E0624, and no non-external E0432/E0433.
     - `--test` (live tree): 9365 / 6833, 0 new critical keys. The totals are confounded: the two sequential runs were about 10 minutes apart while slice E edited plugin files, so live files the baseline shares changed in between.
     - A `--freeze` rerun (both trees from one frozen copy) aborted asymmetrically: the baseline stopped at 11372 errors and the current run reached 21561. It is inconclusive, and the plugin `--test` configuration must be confirmed by the real `cargo test` once the gate admits it.
   - `semio_framework_os_mcp` native / `--test`: 131 / 131 and 486 / 486, 0 new keys. The catalog file is mounted by the carved crate `semio_framework_os_mcp_catalog` (`📚️catalog-compiler`), which gives native 104 / 104 and `--test` 104 / 104, 0 new and 0 removed keys.
   - `semio_framework_os_renderer_wgpu` (285-file closure) native 379 / 379, `--test` 585 / 585, wasm32-unknown-unknown 333 / 333, 0 new and 0 removed keys. The patched-file count is 3 baseline vs 4 current: one more external glob neutralised identically by the patch rule.
   - facade `semio_framework` (run 16:00, mid-carve): native 1790 / 1798, `--test` 2360 / 2703. Every new key is an unresolved *external* crate import (`semio_framework_tool_run`, `semio_framework_ui_locale`, `serde`, …) from the new modules, or the carver's in-flight crate edges. The one E0603 (`InteractionDefinition` private import) sits in carver-rewritten test text. The facade was no longer a monolith when the oracle ran, so this differential cannot be attributed and is superseded by the carver's per-crate oracle.
   - Plugin crates whose edits are external-crate paths only (puzzle, remodel, energy, dag, reasoning, trinity, composition): an external path is unresolved on both sides by construction, so the differential cannot see it. These edits were proved instead by the reference scan (item 2b).
2. **Definition and reference scans:**
   - (a) `privacy_scan.py` over the moved modules and their tests against the facade's 78 dep-info files: 0 real findings. All flags are trait methods, enum-variant fields or locals.
   - Seeded regression: making `with_schema` private is reported for both history_edit and tool_run.
   - (b) `classify_callers.py` + `flag_callers.py` over all 155 Rust files that mention a moved name: no reference reaches a moved name through `semio_framework::`, `manifest::`, `ui::` or `semio_framework_plugin::` any more. Every remaining bare use resolves through a parent that imports from `semio_framework_tool_run` or from the new module; each one was checked by reading.
   - Losslessness: all 1,482 non-trivial lines (1,090 distinct) of `product-vocabulary` exist in the new owners, except 36 intended lines: neutralised docs, the gumball rule rewritten onto the registry, and old test mounts.
3. **Format, regions, paths, emoji:**
   - `rustfmt +nightly-2026-07-20 --edition 2021 --emit stdout` on 88 touched Rust files: 87 clean. One ("left behind trailing whitespace" in `🔬️plugin-runtime-plugin-builder-contract`) is identical on the baseline copy, so it is pre-existing.
   - Region balance: every file I created is balanced. I closed the `PluginRuntime` region in the manifest TS twin, which has been missing since before the last 12 commits. Imbalances in four peer files are unchanged from baseline: puzzle 3d editor, energy editor test, plugin `📜️declaration/🧩️app`, root `📜️script.ts`.
   - Longest touched path is 224 bytes; longest new file is 184 bytes.
   - Emoji admission (`\p{RGI_Emoji}`/FE0F, bun) passes for `🎓️onboarding`, `🕹️actions`, `🖐️gumball-audience`, `🧩️reserved-contributions`, `🏠️local-interaction`, `🗺️window-layout`; all are unique among their siblings.
4. **Direction:**
   - dependency-cruiser in the repo on the touched TS paths: the first run showed 1 new cycle, `🎓️onboarding ↔ 🛂️manifest` (type-only). Fixed by extracting `🗺️window-layout`. After the fix, 0 violations touch any new file. The remaining 126 (84 no-circular barrel cycles, 41 ui-no-framework-packages, 1 not-to-unlisted) all run through pre-existing files that my edits only shrank.
   - Cargo: all three added edges point downward (os → framework, plugin → framework). The three edited manifests parse (bun TOML).
   - Rust source scan of 95 touched files: 0 upward `use`/`#[path]`/`include*` edges.
5. **TS:**
   - `ts_sandbox.py` (isolated sandboxes, workspace packages re-linked into each sandbox): `tsc --noEmit` scoped to the touched TS closure gives baseline 0 errors (850 files) and current 0 errors.
   - Seeded break (renaming `TutorialDefinition`) gives TS2724, so the check is sensitive.
   - `bun test`: history-edit twin 1 pass and tool-run twin 3 pass, both identical on the baseline sandbox.
   - vitest (`react` config, `-t TutorialLocalInteraction`): 1 passed / 565 skipped.
   - Static resolution of 232 relative imports in the changed TS files: 0 missing apart from one pre-existing `?raw` specifier.
6. **Launcher:** `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework` and the same for `-p semio-framework-plugin` both exit 1 with `error: Finite original maintenance authority required` (`relo-manifest-cargo-*.log`). Not compiled around.

## 5. Residual risks

- Typeck-phase errors (E0308, E0616/E0624 on external items, trait scope) are unproven until the gate admits a real `cargo check`. Run check and test for the onboarding, time-travel-actions, tool-run-actions, manifest-* crates, the plugin, mcp, os-renderer-wgpu and the touched plugin crates first.
- **Locks:** the root `Cargo.lock` (os-renderer-wgpu edge, plus the carver's new crates), `📸️remodeling/Cargo.lock` and `🔋️model/Cargo.lock` need `update --workspace --offline` (COMPILE-WINDOW-CHECKLIST step 1).
- `semio_framework_plugin::tool_run::…` is avoided everywhere. The remodel example test, whose crate has no `semio-framework` dependency, uses `semio_framework_tool_run` through its crate's new dev-dependency. If slice E adds a crate-root `tool_run`/`time_travel` item in the plugin SDK, nothing of mine depends on the glob.
- The overlap with the wave-4 facade carver: anything they change after 17:00 is outside this report.

## 6. Follow-ups (not done, with plan)

1. **`OsMediaCapability` retirement.** 104 `MeshOnly` + 3 `Brep` constructors in 98 Rust files, 99 Rust files touch `media_capability`, and 39 TS/JSON files carry `mediaCapability` (hub composition descriptors, fixtures, projection). The only reader is `os_resource_media_capability`, which is re-exported but has no caller. Plan:
   - delete the `ArtifactKindSpec.media_capability` field, the enum and the lookup;
   - derive any need from `media_type.form ∈ {Brep, Mesh}`;
   - in one pass, update the projection (version bump), the generated TS, the TS twin, the plugin registrations (mostly removing one field line per `ArtifactKindSpec`), then regenerate the hub composition descriptors and the trusted-catalog fixture.
   
   Not done now because it touches ~140 peer-owned files during an active carve, with no compiler available.
2. **Opening `MediaForm`.** It has 16 variants, 9 of them plugin topics (`Trinity`, `Type`, `Design`, `Kit`, `Flow`, `Sequence`, `Procedure`, `Deck`, `Dag`), across 216 Rust files and 2 TS files. Decision: it should become an open, registered id.
   - Keep framework forms `Any|Vector|Raster|Brep|Mesh|Document|Value` as constants of a `MediaFormId(Cow<'static, str>)`.
   - Plugins register their forms and their one-way conversions (`MEDIA_FORM_CONVERSIONS`) as a `TopicContribution` (`media.form`) with a `📇️space-index`-style registry, so that `media_types_compatible` reads the registered conversion set.
   - Wire form becomes a camelCase string, which keeps today's serde spelling, so the descriptor JSON does not change.
   
   This needs the compile window, because `MediaForm` is `Copy`/`Hash` and is matched in about 200 places.
3. **`InteractiveJobClassification` variant names** (`Migrated`, `BatchOnlyPendingRewrite`) still carry migration wording. Renaming them changes the wire format in 280 Rust and 20 TS files plus root `📜️script.ts` checks. Proposed names: `Interactive`, `BatchOnly`, `Forbidden`, `Retired`.
4. **mcp catalog vs builder:** the catalog publishes only `startIntroduction` from onboarding, while the builder also injects `startTutorial`/`recordTutorial`. Either make onboarding a `ReservedContribution` whose app lane takes the app's declarations, which needs the builder to build its app-lane subject before window resolution, or document that the catalog deliberately leaves out chrome-only tutorial verbs.

## Coordinator addendum (20:40)

The whole-tree dependency-cruiser attribution found 10 new `no-cross-package-relative-*` edges from this slice's TS twins, because `⏯️tool-run` and `⏪️time-travel` are nested package owners. The coordinator fixed them (`RESULT-WRONG-DIRECTION-RELOCATION.md` §2):
- the generated `ToolRun*`/`JobKindId` wire aliases are back in `🛂️manifest/🟦️.ts`;
- `appDeclaresToolRun` now takes `run?: unknown`;
- the manifest twin re-exports the `🗺️window-layout` types, so the kernel and os config import them from it again, and the barrel line was removed;
- the history-edit TS test imports `DIALOG_CHOICE_ARG` from `@semio-tech/framework`.

Scoped tsc: 0 errors, with a seeded probe caught. Both bun twins still pass (1/1 and 3/3).
