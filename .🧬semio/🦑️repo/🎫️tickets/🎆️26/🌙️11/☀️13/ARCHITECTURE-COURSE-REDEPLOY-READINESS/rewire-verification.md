# Rewire verification

## Leftover imports and pins
- No import of `@semio-tech/framework-kernel/testing/*`, `@semio-tech/framework/kernel/testing/fixtures` or `@semio-tech/framework/raster/video/testing/unit` remains outside tickets (grep of all removed keys).
- No pin test or fixture outside tickets mentions the removed export keys; nothing to update.
- Resolution check of the 14 rewired files: 69 relative imports, 0 missing; every named fixture import exists in the kernel `🧪️tests/🧫️fixtures` module.

## Runs
| command | exit | result |
| --- | --- | --- |
| `@semio-tech/framework-kernel:verify-media-export-encoding` / `verify-file-open-import` / `verify-spawned-job-drive` | 0 / 0 / 0 | rewired kernel owned-verify imports |
| `@semio-tech/framework-kernel:test` | 0 | 11 files, 94 tests |
| `@semio-tech/framework-os:verify-video-render-export` | 0 | rewired os owned-verify (4 imports incl. VideoRenderHost unit) |
| `@semio-tech/framework-os:test` | 1 | 35/38 files, 762/765 tests; 3 failures unrelated (see below) |
| `@semio-tech/framework-os:typecheck` | 1 | blocked by an unrelated Rust compile error (see below) |
| `@semio-tech/framework-renderer-react:test` | 0 | 1 file, 37 tests |
| `@semio-tech/framework-renderer-wgpu:test` | 1 | unrelated wasm output pin (see below); vitest never ran |
| `@semio-tech/print:check` | 0 | after fix |
| `@semio-tech/print:test` | 0 | after fix; viz-inference 631/631 |
| `@teaching/architecture-course:check` | 0 | |
| `@teaching/architecture-course:typecheck` | 0 | first run exit 1 = graph service timeout, rerun exit 0 |

## Fixes
- `print:check`: `CheckScript` used `runArtifactTypeScriptPackageMain`, which needs a `metadata.semio.artifactPackage` producer that publishes `dist`. Print is not an artifact package (its `build` compiles TeX documents), so the artifact check was the wrong tool. `CheckScript` now typechecks each registered suite with the no-output primitive `checkArtifactTypeScriptSourceV1` (same compiler flags, same suite list). File: `🧰️framework/🛍️products/📓️print/🧪️tests/🔬️script-commands/🟦️.ts`. 42.8 s real check.
- `print:test` pin drift (pre-existing): the tectonic command-boundary graph now legitimately imports `📖️pdf/🗜️representation` and `🖨️tectonic-template-compilation/🖼️resources` (which uses `sharp`, a declared print dependency). Added both sources and `sharp` to the two tectonic entries in `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️command-boundaries.json` (4 added lines, regenerated from the esbuild metafile, owners unchanged).

## Unrelated failures found (not touched)
- Rust: `semio-plugin-decl-app` fails to compile (`OriginalConstructionAdmission::has_failure` missing, `🛬️lifted/🎬️media-export/🦀️.rs:107,116`); this blocks `framework-os:typecheck` through `semio-framework-os-flow-browser-package:wasm`.
- `framework-os:test`: `store/👷️worker` suite `default.cases is not iterable` (folder-write-acknowledgment fixture shape), `app-channel-codec` media-export checkpoint_id mismatch (2), `actor-transport` codec bridge (1).
- `renderer-wgpu` script asserts wasm `outputs` is `dist/wasm-<profile>` but the project declares `{projectRoot}/.🧾️store/wasm.json`.

## Unverified
- The rewired vitest files outside any include list I could run (ShellHelpers and store-sync `command-rejection`, renderer engine `renderer-types-and-forms`, `runtime-admission`, `router-plugin-faults`, plugin `shardclient-*`, plugin poll composition, docklayoutstore plugin-source) were not executed: the owning vitest configs (`renderer-wgpu:test`, os include list) are blocked or do not list them. They are verified only by import resolution and export-name checks above, plus that the fixtures module itself is exercised by the kernel tests.
- `framework-os:typecheck` (would typecheck the rewired TS) did not run.
