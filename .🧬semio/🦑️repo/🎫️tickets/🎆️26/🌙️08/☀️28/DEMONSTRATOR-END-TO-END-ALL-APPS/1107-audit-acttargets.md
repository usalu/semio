# 🧭️ 1107 Audit: acttargets (inferred react-prototype targets)

Read-only audit. No edits, no cargo, no activate/serve/lock/sync, no modifying git commands. The only write is this file.

## 1. Where the targets come from

The 12 targets are dependsOn strings, all literal in the demonstrator project:

- `♻️mit-bestand/🧺️demonstrator/📋️project.json:34-42`: nine `@semio-tech/framework-os-dev:prepare-<v>-react-prototype` (aggregator, aussuchen, bearbeiten, energy, fem3d, generation3d, generator, koordinator, verfolgen), dependency of `prepare-prototype`.
- `♻️mit-bestand/🧺️demonstrator/📋️project.json:299-301`: three `activate-<v>-react-prototype` (energy, fem3d, generator), dependency of `activate-lanes-prototype`.
- `activate-prototype` (`📋️project.json:272-276`) depends on `prepare-prototype` and `activate-lanes-prototype`.

So the earlier note "not literal in any project.json" is accurate only for the target definitions. The names are literal in dependsOn, and their producers are inferred.

**Producer rule.** `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs`:

- `playgroundPreparationTargets`, lines 137-306.
- Emits `activate-${variant}-react-${profile}` at line 226 and `prepare-${variant}-react-${profile}` at line 234, for profile `prototype` and `release`.
- Inputs are one row per `[[package.metadata.semio.playground]]` in component Cargo.toml files (collected at lines 146-157), gated by `catalogBuildContributionForManifestV1`.
- Owner: `🔎️owner-discovery/🟨️.mjs:175` and `📦️project-nodes/🟨️.mjs:190` merge these targets into `@semio-tech/framework-os-dev` (project `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📋️project.json`).
- Refusal path, lines 186-197: if runtime-input admission is refused, the same names are still emitted as a `runtime-input-check` diagnostic target. That is "present but failing", not missing.

**Variant rows (Cargo.toml).** All hub component crates are `component-kind = "plugin"` with a `[package.metadata.component]` block:

| variant | file:line |
|---|---|
| energy | `🌎️hub/🧩️compositions/🔋️energy/🧩️component/📦️packages/🦀️rust/Cargo.toml:21` |
| fem3d | `🌎️hub/🧩️compositions/🏗️fem/🧩️component/📦️packages/🦀️rust/Cargo.toml:28` |
| generation3d | `🌎️hub/🧩️compositions/🌀️procedural/🧩️component/📦️packages/🦀️rust/Cargo.toml:40` |
| generator | `🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/📦️packages/🦀️rust/Cargo.toml:44` |
| koordinator | same file:53 |
| aggregator | same file:62 |
| aussuchen | same file:71 |
| bearbeiten | same file:80 |
| verfolgen | same file:89 (also `engines` at :96) |

**Catalog gate.** Admission of the rule's input needs:

- `📋️build-owners.json:28224-28234` lists `catalogProviders` with `sourcePath` `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🟦️.ts`. That file exists. Its `catalogBuildContributionV1` (lines 6-7) names project `@semio-tech/s-dev-composition`, target `generate`, and inputs `✏️s/🧑‍💻dev/🧩️catalog/📣️publication/🔣️.json` and `🌎️hub/📇️catalog/🔣️.json`.
- `🌎️hub/📇️catalog/🔣️.json` has `packageRoots: ["🌎️hub/🧩️compositions"]`, which covers all four hub crates above.
- `✏️s/🧑‍💻dev/📋️project.json:2` names `@semio-tech/s-dev-composition`, and `:225` defines `generate`.
- The rule also needs the wgpu renderer producer with prototype and release `SEMIO_BUILD_MODE` configs (`🧰️framework/.../🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`): `wasm` config at `:1266` and `:1274`; the project is `@semio-tech/framework-renderer-wgpu`.
- The flow producer `🧰️framework/.../🌊️flow/🎁️browser-package/📦️packages/🦀️rust/📋️project.json` has `wasm` at `:242`.
- The `verfolgen` engine `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/📋️project.json` has `wasm` at `:55`.

## 2. Per-target verdict

Verdict is from the static rule and its inputs. Nothing was executed.

| # | target | rule emits it | variant row | in dependsOn of | verdict |
|---|---|---|---|---|---|
| 1 | prepare-aggregator-react-prototype | yes (234) | present (:62) | prepare-prototype | present |
| 2 | prepare-aussuchen-react-prototype | yes | present (:71) | prepare-prototype | present |
| 3 | prepare-bearbeiten-react-prototype | yes | present (:80) | prepare-prototype | present |
| 4 | prepare-energy-react-prototype | yes | present (energy:21) | prepare-prototype | present |
| 5 | prepare-fem3d-react-prototype | yes | present (fem:28) | prepare-prototype | present |
| 6 | prepare-generation3d-react-prototype | yes | present (procedural:40) | prepare-prototype | present |
| 7 | prepare-generator-react-prototype | yes | present (demonstrator:44) | prepare-prototype | present |
| 8 | prepare-koordinator-react-prototype | yes | present (demonstrator:53) | prepare-prototype | present |
| 9 | prepare-verfolgen-react-prototype | yes | present (demonstrator:89) | prepare-prototype | present |
| 10 | activate-energy-react-prototype | yes (226) | present | activate-lanes-prototype (:299) | present |
| 11 | activate-fem3d-react-prototype | yes | present | activate-lanes-prototype (:300) | present |
| 12 | activate-generator-react-prototype | yes | present | activate-lanes-prototype (:301) | present |

Caveats that could turn any row into "present but failing" or "missing":

- `runtimeInputAdmissionV1` and `nativePreparation` (`🟨️.mjs:161-173`) may refuse a playground. The row is then emitted as a diagnostic target. This was not run.
- Any gate exception (wgpu, flow, or engine wasm owner missing) throws for the whole inference, which would drop all 12 at once. The gates checked above are present. `frameworkOsEngineSourceOwnersV1("renderer")` and `("distribution")` owners were not checked.

## 3. Second source: sealed graph

`repo show project @semio-tech/framework-os-dev --graph-policy=current` ran (exit 0, `durationMs` 453943). The result is inconclusive:

- Saved at `/Users/ueli/.claude/projects/-Users-ueli-Documents-semio/81aa81ee-3456-4db7-8ac0-58d6181adc0a/tool-results/mcp-repo-show_project-1794057240085.txt`.
- `stdout` is 91,832 characters and marked truncated. It begins mid-JSON, so it is a tail window.
- Zero occurrences of `react-prototype` in that window. This does not prove absence.
- stderr reports a capacity wait for show (memory), which means the command queued behind other jobs.

To settle it from the sealed graph, a full read of the framework-os-dev project would be needed, not the tail window.

## 4. What each target needs that may be missing

- Playground admission: runtime-input admission for variants energy, fem3d, generator, generation3d, koordinator, aggregator, aussuchen, bearbeiten, verfolgen (not run).
- Native prerequisites: `nativePreparation` for each component closure (not run); any `NATIVE_INPUT_ADMISSION` refusal turns the target into a diagnostic.
- Engine owners for `renderer` and `distribution` (`frameworkOsEngineSourceOwnersV1`), each needing a `wasm` target. Not checked.
- `verfolgen` also needs its engine `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust`. Its `wasm` target is present.
- Catalog providers: the `📋️build-owners.json` entry is present, but its `importRevision` load and the `generate` target of `@semio-tech/s-dev-composition` run at graph time and were not executed here.

## 5. Result

12 present, 0 missing (static evidence from the rule and inputs; sealed-graph check inconclusive; admission and native refusals not verified).
