# Explore: `s` OS — per-plugin / per-app blocker table

Generated: 2026-09-05 (Sonnet read-only explorer). Method: enumerated all 33 plugin dirs under `✏️s/🔌️plugins/`; for each, every `🗿️artifacts/<kind>/🏅️standards/<ver>/🪆️subsets/<subset>/✏️editor/🦀️.rs` + `👁️viewer/🦀️.rs` pair. Counted `Migrated` / `BatchOnlyPendingRewrite` / `factory_type` / `bounded_first_step_tool_proofs!` / stub markers per file via `grep -c`. Cross-checked against tickets `PROCEDURAL-3D-END-TO-END`, `GIVE-FORMS-APP-AN-OWNED-TOOL-JOB-FACTORY`, `PUZZLE-3D-END-TO-END`, `PROCESS-END-TO-END`, `SOURCING-END-TO-END`, `RUNTIME-DEPENDENCY-ELIMINATION-…`, and the two terra catalog audits in `COMPLETE-SEMIO-END-TO-END`.

Naming: `🪐️space` is the `s` host plugin (package id `s`). `💡️reasoning` = package id `reasoning-mindmap`. `🗟️artifacts/` is not a plugin.

## 0. Repo-wide migration snapshot

427 `Migrated`, 414 `BatchOnlyPendingRewrite`, 7 `Unclassified`, 3 `ForbiddenFromUi` (from `PUZZLE-3D-END-TO-END/📓️why-puzzle3d-is-not-end-to-end.md`). Cleanest references: **lowpoly (48/48)**, sequence (20/0), writer (19/0). Worst: **norm (0 of ~60 across 15 apps)**, puzzle3d (10/69), puzzle5d (11/41), note (9/27), puzzle2d (6/34), flow (26/15).

## 1. Per-plugin / per-app table

Mig/Batch = editor(E)/viewer(V) counts of `Migrated`/`BatchOnlyPendingRewrite`. Fac = `factory_type` present (E/V). Proof = `bounded_first_step_tool_proofs!` present. Ex = `setActiveExample` reachable. Stub = `todo!`/`unimplemented!`/🚧️ count (E+V). Descriptor = pair status per residual audit.

| Plugin | App (kind) | Path root | Mig E/V | Batch E/V | Fac E/V | Proof | Ex | Stub | Descriptor | Ticket status | Concrete blocker |
|---|---|---|---|---|---|---|---|---|---|---|---|
| lowpoly | lowpoly | `💠️lowpoly/🗿️artifacts/💠️lowpoly` | 48/0 | 0/0 | y/n | n/y | n | 0+2 | OK | reference | None; copy its factory pattern. |
| sequence | sequence | `🎬️sequence/🗿️artifacts/🎬️sequence` | 20/0 | 0/0 | y/y | y/n | n | 0 | OK | - | Fully migrated. |
| writer | writer | `✒️writer/🗿️artifacts/✒️writer` | 19/0 | 0/0 | n/n | y/y | y | 0 | **DIVERGENT** (pack drops `interactiveJob:"migrated"`) | - | Regenerate descriptor via producer fix. |
| sourcing | curation | `🪵️sourcing/🗿️artifacts/🗂️curation` | 18/0 | 0/0 | n/y | y/n | n | 1+1 | **DIVERGENT** (`sourcing.curation` vs `sourcing.curate`) | `☀️01/SOURCING-END-TO-END/📓️status.md` — all 8 Batch commands migrated, 115/126 tests | Blocked by **stdio** (2196 `ToValue`/`FromValue` derive errors) + grid-window scene > 32 KiB payload. |
| procedural | generation3d | `🌀️procedural/🗿️artifacts/🧊️generation3d` | 31/0 | 0/0 | y/n | y/n | y(8) | 1 | **DIVERGENT + STALE** (`2d.generation` vs `2d.procedural`; committed `🔣️.json` still shows 6/47 batchOnly) | `☀️03/PROCEDURAL-3D-END-TO-END/📓️status.md` (2026-09-05) | Descriptor never regenerated after `Generation3dBoundedCommandJobFactory`; runtime hard-rejects 6 actions until `describe` + registry generate on green stdio. |
| procedural | generation2d | `🌀️procedural/🗿️artifacts/🌀️generation2d` | 8/0 | 0/0 | n/n | n/n | y(5) | 1 | same | in-flight by peers | `Generation2dBoundedCommandJobFactory` precedent (editor `🦀️.rs:115-203`). |
| procedural | assembly | `🌀️procedural/🗿️artifacts/🧩️assembly` | 0/0 | 0/0 | y/n | n/n | n | 0 | same | - | No actions at all. |
| forms | forms | `📋️forms/🗿️artifacts/📋️forms` | 29/0 | 0/0 | y/n | y/n | n | 2+1 | OK | `☀️03/GIVE-FORMS-APP-AN-OWNED-TOOL-JOB-FACTORY/📓️implementation-app-owned-factory.md` | `FormsBoundedCommandJobFactory` (28 tools) landed but never compile-verified (workspace corruption). |
| puzzle | 3d | `🧩️puzzle/🗿️artifacts/🧊️3d` | 10/59 | 1/3 | n/n | y | n | 0+1 | OK | `☀️02/PUZZLE-3D-END-TO-END/📓️status.md`, `📓️why-puzzle3d-is-not-end-to-end.md` | 59/69 Batch; `fillBuildTick` cannot be migrated without `PuzzleCommandWork::step` app-instance param. |
| puzzle | 5d | `🧩️puzzle/🗿️artifacts/🖐️5d` | 11/41 | 1/1 | n/n | y | n | 0+1 | OK | same | 21% migrated. |
| puzzle | 2d | `🧩️puzzle/🗿️artifacts/◻️2d` | 6/34 | 1/0 | n/y | y | y(viewer) | 0+1 | OK | same | 15% migrated. |
| block | 3d | `🧱️block/🗿️artifacts/🧊️3d` | 0/0 | 0/0 | n/n | n/n | y | 1+2 | **MISSING** | catalog audit: 1,569 `E0433`; `◻️2d/…/🎥️move-camera2d/↩️inverse/🦀️.rs:8` resolves `super::super::move_camera2d::mutation` | Plugin doesn't compile; no descriptor. |
| block | 5d | `🧱️block/🗿️artifacts/🖐️5d` | 8/0 | 0/0 | y/n | y/n | n | 0+1 | MISSING | same | Gated by plugin compile. |
| block | 2d | `🧱️block/🗿️artifacts/◻️2d` | 0/0 | 0/9 | n/n | n/n | n | 1+0 | MISSING | same | 9 viewer actions dead. |
| cad | cad | `📐️cad/🗿️artifacts/📐️cad` | 32/0 | 0/17 | y/n | n/y | n | 1+2 | OK | catalog audit `E0599` missing mutation module (stale) | 17 viewer Batch. |
| gis | gismap | `🌍️gis/🗿️artifacts/🗺️gismap` | 16/0 | 0/13 | n/n | y/y | n | 1+1 | OK | - | 13 viewer Batch. |
| gis | gisterrain | `🌍️gis/🗿️artifacts/🏔️gisterrain` | 6/0 | 0/0 | n/n | n/y | n | 2+0 | OK | - | Under-built (6 actions). |
| process | process3d | `🏭️process/🗿️artifacts/🧊️process3d` | 7/0 | 0/4 | y/n | y/n | y | 0+1 | OK | `☀️01/PROCESS-END-TO-END/📓️status.md` (2026-09-05): 260/261 tests | Red test + wasm export trace to stdio gltf codec/schema mismatch (`🗄️stdio/📇️registry/🦀️.rs:466-468`, "s.stdio.gltf executable mapping keys diverge"). |
| norm | 15 apps (din4108, din16798, din18599, en1990–en1999, iso16757, vdi3805) | `📕️norm/🗿️artifacts/<kind>` | **0/0 all** | ~4/0 or 0/4 | n/n | n/n | n | 1-3 each | OK | - | Zero actions migrated in all 15; one shared factory + classification flip unblocks 15 apps. |
| playbook | playbook | `📖️playbook/🗿️artifacts/📖️playbook` | 3/0 | 7/0 | y/n | y/n | n | 0+1 | **MISSING** | catalog audit | No descriptor; depends on stdio. |
| trinity | jack | `🔱️trinity/🗿️artifacts/🔌️jack` | 9/0 | 0/1 | n/n | n/y | y | 0 | **MISSING** | catalog audit: `framework_editor` re-export unavailable (`🔌️jack/…/✏️editor/🌉️wasm/🦀️.rs:7`) | Doesn't compile; no descriptor. |
| trinity | rewriting | `🔱️trinity/🗿️artifacts/♻️rewriting` | 0/0 | 0/0 | n/n | n/n | n | 0 | same | same | Stub app. |
| dag | dag | `🕸️dag/🗿️artifacts/🕸️dag` | 3/11 | - | n/y | n/y | n | 2 | OK | - | 11/14 dead; factory on viewer only. |
| draw | drawing | `🖍️draw/🗿️artifacts/🖍️drawing` | 7/0 | 0/0 | n/y | y/n | n | 0+1 | OK | catalog audit: absent stdio `drawing` snapshot names (`🖍️draw/…/🚪️io/🦀️.rs:16-19`); no cache dir | Doesn't build against stdio drawing snapshot API. |
| animate | presentation | `🎞️animate/🗿️artifacts/🎬️presentation` | 6/0 | 0/0 | n/n | y/n | n | 0+2 | OK | catalog audit: missing presentation mutation module | First failure in full-catalog rebuild order. |
| shooting | shooting | `🎥️shooting/🗿️artifacts/🎥️shooting` | 3/0 | 0/0 | y/n | y/n | n | 0+1 | OK | - | Healthy, small. |
| mathematical | equation | `➗️mathematical/🗿️artifacts/➗️equation` | 8/0 | 0/0 | y/n | y/n | n | 0+1 | **DIVERGENT** (`equation` vs `mathematical`) | catalog audit: stale stdio types | Descriptor identity mismatch. |
| vcs | vcs | `🌿️vcs/🗿️artifacts/🌿️vcs` | 12/0 | 0/0 | y/n | n/y | n | 1+1 | OK | - | Healthy. |
| imperative | procedure | `📜️imperative/🗿️artifacts/📜️procedure` | 2/0 | 0/10 | y/n | n/y | n | 0+1 | **DIVERGENT** (`procedure` vs `imperative`) | - | 10 viewer dead + identity mismatch. |
| architect | program | `🏛️architect/🗿️artifacts/🏛️program` | 0/0 | 0/0 | n/n | n/n | n | 3+1 | **DIVERGENT** (`data.program` vs `data.🏛️program`) | catalog audit: stale stdio types | Unstarted migration, 4 stubs. |
| energy | model | `🔋️energy/🗿️artifacts/🔋️model` | 0/0 | 0/0 | n/n | n/n | n | 0 | **DIVERGENT** (`data.model` vs `data.🔋️model`) | catalog audit: `semio_framework` import unresolved, 279 errors (`🔋️model/…/🧵️simulation-session/🦀️.rs:7`); no core wasm | Does not compile. |
| remodel | remodeling | `📸️remodel/🗿️artifacts/📸️remodeling` | 3/0 | 0/0 | n/y | n/y | n | 1+0 | OK | catalog audit: missing mutation module | Mutation module ownership compile error family. |
| fem | 3d | `🏗️fem/🗿️artifacts/🧊️3d` | 4/0 | 0/16 | y/n | n/y | y | 0+2 | OK | catalog audit: missing mutation module | 16 viewer dead. |
| fem | 2d | `🏗️fem/🗿️artifacts/◻️2d` | 4/0 | 0/16 | y/n | y/n | n | 1+0 | OK | same | Same. |
| note | note | `🗒️note/🗿️artifacts/🗒️note` | 9/27 | - | y/n | n/y | n | 0 | OK | - | 27 Batch. |
| layout | layout | `📏️layout/🗿️artifacts/📏️layout` | 14/7 | - | n/y | y/y | n | 0+1 | OK | catalog audit: no cache dir, no recorded diagnostic | Needs fresh isolated `cargo check`. |
| raster | raster | `🖨️raster/🗿️artifacts/🖨️raster` | 0/0 | 0/0 | n/n | n/n | n | 2+0 | OK | - | No actions yet. |
| reasoning-mindmap | wires | `💡️reasoning/🗿️artifacts/🔌️wires` | 3/8 | - | n/n | n/n | n | 1+0 | OK | - | 8/11 dead. |
| s (space) | home | `🪐️space/🗿️artifacts/🏠️home` | 13/0 | 0/3 | n/y | n/y | n | 0 | OK | host | 3 viewer dead on the shell's own home. |
| s (space) | space | `🪐️space/🗿️artifacts/🪐️space` | 14/0 | 0/6 | n/n | n/n | n | 0 | OK | host | 6 viewer dead. |
| demonstrator | playground | `🎪️demonstrator/🗿️artifacts/🎪️playground` | 2/0 | 0/0 | n/y | n/y | n | 1+0 | **DIVERGENT** (`s.sourcing.curation` vs `.curate`) | - | Borrows procedural/cad/puzzle3d/sourcing/process/gis apps (`🪪️manifest/🎪️demonstrator/🦀️.rs:57-71`), inherits their blockers. |
| flow | flow | `🌊️flow/🗿️artifacts/🌊️flow` | 26/0 | - | n/y | y/y | n | 0 | OK | - | 15 viewer Batch. |
| **stdio** | ~37 format kinds × editor+viewer | `🗄️stdio/🗿️artifacts/*` | not tallied | - | - | - | - | - | **MISSING** | all four END-TO-END tickets hit stdio breakage this week | Direct dependency of all 33 plugins. Recent failures: wasm link "functions count exceeds limit of 1000000"; 2196 `ToValue`/`FromValue` derive errors; gltf codec/schema id mismatch (`🗄️stdio/📇️registry/🦀️.rs:466-468`); 2× `E0023` BREP errors; no descriptor pair. |

## 2. Ranked top-10 cross-plugin fixes

1. **Get `stdio` compiling clean and described** (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/🦀️.rs`; gate `🔌️plugin/📇️registry/📜️script.ts:2233-2467`).
2. **Fix the shared descriptor producer** `describePluginComponent` (`🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts:56-75`) to take raw+core receipts — prerequisite for re-emitting all 30 residual descriptor rows.
3. **Shared `factory_type` + bounded proof factory for the 15 norm apps**, pattern `🌀️procedural/🗿️artifacts/🧊️generation3d/…/✏️editor/🦀️.rs:165-247`.
4. **Add an app-instance parameter to `PuzzleCommandWork::step`** (`🧩️puzzle/…/🎮️commands/🧵️retained/🦀️.rs:42-49`; symptoms puzzle3d editor `🦀️.rs:6116-6118`, `:2168`).
5. **Make registry `check` fail closed on missing/divergent descriptors** (`📜️script.ts:1947-1953`, `:2069-2079`).
6. **Real `describe` route for the 26 extensions** (`📚️library/📦️packages/🟦️typescript/🟦️.ts:3009-3043` only emits `.sxt`).
7. **Gate on Rust `interactiveJob` classification vs committed `🔣️.json` drift** (procedural, forms, sourcing all shipped source fixes without regenerating).
8. **Move `ArtifactApp::Snapshot` bound off `Serialize + DeserializeOwned` onto `ToValue`/`FromValue`** (`🔌️plugin/🦀️component.rs`, ~78 serde refs).
9. **Relocate ~26 wgpu-tier symbols imported unconditionally by `♾️infinite/🌍️world/🦀️component.rs`** (`🖱️ui/🎯️targets/🧊️wgpu/📦️glue.rs`, `♾️infinite/🌍️world/{draw,action,input}.rs`) so `wasm-bindgen`/`web-sys` stop linking into wasip2 plugins.
10. **Contain the `26/04/08/ENFORCE-UNIQUE-SEMANTIC-EMOJIS-ACROSS-REPOSITORY` Codex applier** (repeated tree-wide corruption; repair scripts `🔨️restore-emoji-corruption.py`, `🔨️revert-codemod-lines.py` under `☀️02/PUZZLE-3D-END-TO-END/`).

## 3. Plugins with no app at all

All 33 core plugins have at least one editor+viewer pair. The 26 extensions declare no artifact/editor/viewer (only `📦️packages/🦀️rust/` with `package`/`test` scripts): flow (9: bim, draw, brep, dictionary, text, primitive, logic, math, list), imperative (5: control, effect, logic, math, text), process (4: concrete, metal, robotic, wood), sourcing (3: beams, slabs, windows), cad (4: aec-building, aec-building-energy, aec-building-structure, spatial-shape — placeholder descriptors `pluginId: "empty"`), playbook (1: procedural). Their only work item is the shared `describe`/descriptor-producer fix.
