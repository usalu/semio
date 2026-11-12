# Engine-Neutral 3D Models for Every Staffage — Summary

`S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `M` = `🧰️framework/🔨️modules/🧊️3d/🗿️model`. Design and amendments: `design.md`. Every claim below points to the report that carries the evidence.

## Delivered

| Part | Where | Evidence |
|---|---|---|
| Neutral model kernel `semio.model/v1`: nodes with mechanism joints, six declarative solid kinds, meshes, skins, materials, variants, poses, animations; validation, tessellation, pose evaluation, feature edges, solid provenance; progress and cancellation everywhere | `M` | `kernel-core.md`, `kernel-maintenance.md`, `kernel-audit-fixes.md` |
| Owned GLB encoder (skins, animations, material variants, unlit, animation pointer) | `M/📤️export/🧊️glb` | `glb-encoder.md` |
| Owned binary FBX 7.4 encoder with owned deflate (skeleton, skin, bind pose, node and material curves) | `M/📤️export/🎬️fbx` | `fbx-encoder.md` |
| Owned WebGL2 viewer and software rasteriser (orbit, presets, picking, follow camera, root motion) | `M/🖥️viewer` | `model-viewer.md` |
| Staffage `volume` schema, typed twin, validator rules; `glb`, `fbx`, `model`, `model-zip` formats; `axonometric` viewpoint | `S/🧬️schema`, `S/🔨️modules/🗂️catalog` | `staffage-volume-schema.md` |
| Authored `volume` for all 152 assets, 4,903 solids at authoring time, sources kept in `volumes/` | `S/🗂️catalog` | `authoring-*.md`, `volume-agreement.md` |
| `modelOf`: asset → neutral model, the seven views (incl. thermal, forces, moments) as variants of one geometry; GLB per pose, FBX per clip, model JSON, ZIP; CLI | `S/🔨️modules/🧊️volume`, `S/🔨️modules/📤️export` | `staffage-volume-module.md` |
| Batch export of the whole catalog: 1,982 files, 1.73 GB, deterministic, resumable, cancellable, gitignored output | `S/🔨️modules/📤️export/📦️batch` | `verification.md` |
| Axonometric drawing through the existing vector pipeline (SVG, PDF, DXF, sheets, casts, Python consumer) | `S/🔨️modules/🎨️depiction` | `axonometric.md` |
| React target and site: interactive 3D mode, shared transport, downloads, en/de | `S/🎯️targets/⚛️react`, `S/🌐️site` | `react-site.md` |

## Test state (direct vitest runs, last run of each)

| Suite | Result | Source |
|---|---|---|
| Kernel core / GLB / FBX / viewer | 336 / 37 / 43 / 164 passed, 0 failed, 1 GPU test skipped | `final-integration.md` |
| Staffage library | 2,500 of 2,501 passed; `dwg-reopen` fails (wasm not built) | `lateral-ambiguous.md` |
| Catalog-wide model guard | 152 of 152 assets: every pose a GLB accepted by the Khronos validator, every clip an FBX loaded by three and assimp | `verification.md` |
| React target | 212 of 212 passed, exit 0 (coordinator's own run after the `isElement` fix) | this file |
| Site | 5 of 5 passed | `final-integration.md` |
| Python consumer | not run against real manim; 61 + 48 pass with a stubbed `cairo` (diagnostic only) | `dwg-python.md` |

## Not achieved

- **`repo run` has never completed for any target.** Three whole-repo graph builds failed on three different causes: a stale dependency sync (repaired), an invalid cache reason on the new `export-models` target (fixed), and finally `Workspace import is not resolved by current package metadata: @semio-tech/framework/shell/generated` from `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/🧬️schema/🟦️.ts` (committed 2026-10-23, outside this work) at project 1,360 of 4,754 after 50 minutes. See `repo-graph.md`, `repo-graph-2.md`.
- **DWG**: `staffage_dwg.wasm` is not built; the crate's dependency closure includes four framework crates that do not compile in the current tree (`dwg-python.md`).
- **Python consumer on macOS**: `pycairo` has no macOS wheel and the host has no cairo (`dwg-python.md`).
- **Repo MCP server**: cannot be installed on this machine (`tooling.md`); ticket operations went through the built native binary's CLI.
- 15 of 152 assets miss the silhouette gate against their 2D drawings; reasons inferred per asset in `volume-agreement.md`.
- Real-browser checks cover Chromium/WebGL2 only; touch, Firefox, Safari, forced colours and reduced motion were not checked (`react-site.md`).

## Fixes outside the task

- Go compile error in the repo git provider (`model.DiffLines`).
- `isElement` undefined in `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts`.
- Catalog: 351 missing `solid.sweep` values and the silent default, invalid slugs, schema errors, the polygon-union crash, 19 `lateral-ambiguous` findings (`catalog-fix.md`, `final-integration.md`, `lateral-ambiguous.md`).
- Repo tool: memory governor attribution, cargo inference memoisation, named refusal for invalid cache reasons (`repo-graph.md`, `repo-graph-2.md`).
- Test oracles provisioned through pinned `uv` and hash-locked wheels (`tooling.md`, `oracle-tools.md`).
- A stale duplicate `admit_original_construction_settled` was removed from `🧰️framework/🔨️modules/🧵️job/🏭️construction/🦀️.rs`, which belongs to another group's in-progress work (`dwg-python.md`).

## Decisions for the dev

1. How the repo MCP launcher should be installed here (`tooling.md` lists three options).
2. Whether the DWG crate waits for the framework migration or is decoupled from the plugin framework.
3. How macOS gets cairo for manim.
4. Review of two schema relaxations (`Clip.fps` any positive number, `heatW` may be negative) and the regenerated `export-performance` pins.
5. The 1.6 GB batch output under `S/📦️packages/🟦️typescript/📤️dist` (gitignored, regenerable in about 7.5 minutes).
