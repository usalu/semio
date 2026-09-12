# UI Generated-Source Audit — 2026-09-12

## Scope and method

This is a read-only independent audit of the completed UI runtime/scene/render move map and the persistent generated-source identities. It covers the 64 UI source moves, the finalized Python and .NET styling owners, selected native checks, launch entries, generator contracts, and the live playground-session authority. It does not modify generators or regenerate persistent sources.

The concurrently completed Storybook consumer-only correction was present before the final plugin-registry freshness check. This audit makes no claim about unrelated source cleanup.

## Current corrective item: one canonical session has two incompatible authorities

'@semio-tech/framework-os-dev:check-playground-session --skip-nx-cache' fails. Its two declared prerequisites, 'repo:generator-inputs' and '@semio-tech/plugin-registry:check-generated', pass. The failure comes from '🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts:115'.

At observation:

| Source | Result |
|---|---|
| DEFAULT_HOST_VARIANT in '.../🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts' | "s" |
| Expected canonical session | 12,139 bytes; variant s, registry plugin space, host mode true, home → studio |
| Actual '.../🧑‍💻dev/🤖️generated/🎮️playground-session/🟦️.ts' | 3,166 bytes; variant flow-extension-brep, registry plugin flow-extension-brep, host mode false |
| First unequal line | 20 |
| Canonical-session mtime | 2026-09-12T13:30:20+0200 |
| Generated catalog mtime | 2026-09-12T13:56:46+0200 |

This is not safely classified as a simple stale generator output. 'ensurePluginRegistry(filterPlugin?)' in the dev script, lines 670–676, computes a selected variant from its explicit filter, SEMIO_PLUGIN, PLAYGROUND_APP_KIND, or the default, then writes that selected variant to the same canonical path. The 'generate playground-session' and 'check-playground-session' routes at lines 109–133 always render and check only DEFAULT_HOST_VARIANT.

The live React consumer does not need this shared mutable source: '📦️packages/🟦️typescript/⚙️vite.config.ts:36,127' maps 'virtual:semio-playground-session' to '🔌️plugin/📇️registry/dist/sessions/<selected variant>/🟦️session.ts'. The registry's 'session <variant>' route already creates exactly that variant-scoped output at '📜️script.ts:3534–3544', with an explicit non-overwrite purpose.

**Corrective recommendation:** reserve '🧑‍💻dev/🤖️generated/🎮️playground-session/🟦️.ts' for the default-host generator only. Remove or constrain the selected-variant write in 'ensurePluginRegistry'; retain the existing 'dist/sessions/<variant>/🟦️session.ts' production route for live variant staging. Refreshing the canonical default alone would hide the failure until the next selected dev variant overwrites it again.

The WGPU cache-input fixture at '🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧊️wgpu-browser-boot-cache-inputs/🟦️.ts:43–48' names the canonical anonymous leaf but tests it with '/session\\.ts$/', which does not match '🟦️.ts'; it consequently asserts zero reads. The test still supports WGPU compiler independence, but cannot act as an input oracle for that renamed canonical leaf. Adjust its predicate only if this canonical artifact is meant to remain part of the test contract.

## Accepted UI source topology

The UI move report supplied 64 rows (63 Rust and one JavaScript). Independent filesystem and mount checks found:

| Check | Result |
|---|---|
| Legacy move coordinates remain present | 0 |
| Canonical anonymous leaves missing | 0 |
| Rust package roots containing #[path] mounts | 7 |
| Moved leaves resolved by those mounts | 64 |
| Broken mount resolutions | 0 |
| Direct moved leaves returned by scoped normalization violations | 0 |

The source-identity comparison resolved rebased relative import paths to their physical canonical target. Sixty-two of 64 moved bodies then matched. The two scoped differences are explained by live source:

* 'ui/🎬️scene/🦀️.rs' contains a concurrent retained 'instances_delta_json' field and its test adjustment, identified as the later Wave B44 functional lane.
* 'ui/🖋️text/🦀️.rs' differs only in a documentation link, rebased font 'include_bytes!' paths, and the test mount.

The WebGPU package root continues to mount the canonical '🧊️surface-adapter/🦀️.rs'; its JavaScript sibling has the corresponding one-level import rebase. The bounded native checks passed:

* 'cargo test -p semio-framework-ui-backend-webgpu --lib': 28 passed, 0 failed.
* 'bun ui/render/tests/webgpu-surface/🟨️.js': passed the surface trace, limits, cancellation/recovery, and callback budget assertions.

## Persistent generated styling owners

The selected finalized owner correction is internally consistent:

* Python semantic source: '🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔤️tokens/🐍️.py'.
* C# semantic source: '.../🎨️styling/🎨️palette/🔷️.cs'.
* '📦️packages/🐍️python/pyproject.toml' force-includes the Python source as '🎨️styling/🐍️.py'; '🎨️styling/__init__.py' is package glue.
* '📦️packages/🔷️dotnet/🔷️.csproj' links the C# source with '<Compile Include="../../🎨️palette/🔷️.cs" Link="🎨️palette/🔷️.cs" />'.
* '🎨️styling/🏗️builder/🟦️.ts' remains the implementation-neutral shared generator tooling.
* The old Python/C# package implementation coordinates are absent. The only intermediate-coordinate occurrence is the historical regression fixture in '📚️library/🧹️normalization/🧫️fixtures/🤖️generated-source-topology/🔣️.json'.

Executed evidence:

| Command or oracle | Result |
|---|---|
| Direct loadTaxonomy / validateTaxonomy / validateGeneratorContractsAgainstWorkspace oracle | no taxonomy or generator-contract problems |
| '@semio-tech/repo-lib:test-generated-source-topology' | 2 passed, 0 failed, 217 expectations |
| '@semio-tech/ui-styling-tokens:check-generated' | passed; generated artifacts fresh |
| '@semio-tech/ui-styling-py:test-quick' | passed; source import and built-wheel import both resolved |
| '@semio-tech/ui-styling-dotnet:build' | passed; Release assembly built with 0 warnings and 0 errors |
| '@semio-tech/plugin-registry:check-generated' | passed after the Storybook consumer correction |
| '@semio-tech/framework-os-dev:preview-playground-session' | passed; returns the canonical semantic output node and no stale removals |

The launch seeds were also compared for the two generated previews. Both '📦️preview🤖plugin-registry' and '📦️preview🤖styling-tokens' occur once with their respective 'bun nx run ...:preview-generated' commands and the same group/order in the duplicate launch sources.

## Scope boundaries and follow-ups

The scoped normalization inventory reports 66 ambient directory violations: runtime 19, scene 8, render 39. None is a leaf from the 64-move set. They are existing surrounding directory/test-fixture coordinates and require a separate taxonomy lane; they do not invalidate the moved implementation leaves or the ownership correction.

The prior UI execution packet records a Metal 'cargo test --all-features --lib' compile failure: five 'Scene::finish' E0599 assertions in 'ui/render/tests/metal-packages-rust-backend-unit/🦀️.rs' (lines 45, 51, 67, 73, 90). This audit did not establish when it began or alter it. The passing focused WebGPU checks above are not evidence that the Metal suite passes.

The generated-source topology oracle validates the 29 persistent identity rows. This audit reran selected producer checks and did not rerun every generator or mutate outputs. The default-session authority conflict remains the sole current generated-source finding from this review.
