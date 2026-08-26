# S-GENERATOR-OWNER-REPAIR

## Outcome

Converted all four generator contracts that were `unsafe` into exact `owned` contracts. Strict taxonomy loading and live owner/target/output validation now report `14 owned / 3 external / 3 unknown / 0 unsafe`.

Every repaired generator has one exact Nx generation target, one exact Nx check target, a shared renderer or output plan, schema-owned inputs, and non-overlapping output roots. Checks compare bytes and, where the output is a directory, exact file membership.

## Repairs

### Styling tokens

- Corrected Python output from the nonexistent double `🎨️styling/🎨️styling` path to `📦️packages/🐍️python/🎨️styling/🤖️generated.py`, matching `🛂️adapters.manifest.json`.
- Added the previously undeclared `🤖️generated/palette-presence.css` output.
- `renderStylingArtifacts` now produces the CSS, TypeScript, C#, Rust, and Python byte plan used by both `generate` and `check-generated`.
- Manifest validation proves the declared adapter outputs equal the rendered output set. Premade-theme validation now reads the actual `🎨️theme` directory.
- Dry check before generation failed only for the corrected missing Python output; generation then refreshed the owned set and the check passed.

### Graph catalog

- Scan roots derive from `taxonomy.pluginAreas`; there is no domain-path literal in the scanner.
- Each schema-owned scan root and every candidate path is tested with `pathIsExcluded` before existence checks, `stat`, reads, or recursion. The opaque `compose/` contract therefore remains lexical and unread.
- `renderGraphArtifacts` is shared by generation and `check-generated`; generation removes only obsolete files inside the one ignored owned root.
- Dry check reported byte freshness but obsolete output membership. Generation reduced the catalog to the exact nine admitted graph manifests; the check then passed.

### Assets

- The build exposes a 286-file deterministic manifest: one tracked README, 254 owned catalog outputs, and 31 metabolism outputs across TypeScript, C#, Rust, Python, JSON-derived TypeScript, and SVG projections.
- All language projections and copied SVGs use the same render plan. `check-generated` verifies bytes and exact membership; pruning is restricted to owned output paths.
- Removed the mutable network fetch from owned generation. The existing `🔣️shortcodes.json` gemoji snapshot is classified separately as the exact `external-emoji-shortcodes` input; the owned renderer validates its catalog before producing `🟦️shortcodes.ts`.
- The old UI duplicate was not deleted or adopted. Four live Rust files still reference `🖱️ui/🖼️assets/🔣️icons/🤖️generated`; `ownerless-ui-icons` remains `unknown` and fail-closed until those consumers have an authorized relocation.

### WGPU frame worker

- Added isolated `generate-frame-worker` and `check-frame-worker` targets. They use one in-memory Bun renderer and do not invoke Trunk, Cargo, WASM generation, or a temporary filesystem.
- `wasm`, `serve`, and `dev` depend on the explicit generation target; their script-side path checks freshness rather than generating the worker as a hidden build side effect.
- Regenerated the tracked `🟨️frame-worker.js` only after the dry check proved it stale.
- Determinism is scoped to one Bun implementation/version. The repository declares `bun@1.2.5`, while validation ran with Bun `1.3.14`; cross-version byte identity is not claimed and remains an environment/toolchain pinning risk.

## Schema contract identities

| Contract | Generate | Check | Output roots |
| --- | --- | --- | ---: |
| `assets-build` | `@semio-tech/assets:build` | `@semio-tech/assets:check-generated` | 8 |
| `graph-catalog` | `@semio-tech/framework-graph:generate` | `@semio-tech/framework-graph:check-generated` | 1 |
| `styling-tokens` | `@semio-tech/ui-styling-tokens:generate` | `@semio-tech/ui-styling-tokens:check-generated` | 5 |
| `wgpu-frame-worker` | `@semio-tech/framework-renderer-wgpu:generate-frame-worker` | `@semio-tech/framework-renderer-wgpu:check-frame-worker` | 1 |

The discovery helpers emit only `['bun', 'nx', 'run', project:target]` for these identities. Live validation proves every generation/check target exists in its declared owner `📋️project.json`, every tracked output exists, and each output has exactly one generator contract owner.

## Validation evidence

Green commands:

```text
bun nx run @semio-tech/ui-styling-tokens:generate
bun nx run @semio-tech/framework-graph:generate
bun nx run @semio-tech/framework-renderer-wgpu:generate-frame-worker
bun nx run @semio-tech/ui-styling-tokens:check-generated
bun nx run @semio-tech/framework-graph:check-generated
bun nx run @semio-tech/assets:check-generated
bun nx run @semio-tech/framework-renderer-wgpu:check-frame-worker
bun -e '<loadTaxonomy + validateGeneratorContractsAgainstWorkspace>'
git diff --check -- <owned paths>
```

Observed assertions:

```text
framework/ui/styling: generated artifacts are fresh
[framework-graph] 9 generated manifests are fresh
@semio-tech/assets 286 deterministic outputs are fresh
framework-renderer-wgpu: 🟨️frame-worker.js is fresh
strict/live generator validation green
ownership counts: {"owned":14,"external":3,"unknown":3}
```

A broad direct `tsc` invocation was not green because it lacks the repository's Bun `ImportMeta` types and also surfaced existing unrelated discriminated-union/type errors in framework actor, machine, styling, and repo-library imports. The four executable owner checks above compiled and ran the changed scripts through Bun/Nx successfully.

Hashes at final validation boundary:

```text
taxonomy.json       11ec0ae52e5b7f86538088892619d53ed7ea1e96ca72dfb02bc2c48dd070604e
discovery component bc7e86947de9513e49d0d3c3cf8c5e2e95a92fabf2567e7a570296cbcb368cd8
assets README       749b8edff24de4f5993224e6c1b92b8f1afe14e4c949a4823b84bfb9591cc06d
styling palette     32b65d62f0b02807ad39c5b69223888207fd8e4526f04951f18f8d6b3ff57ec8
frame worker        e9aacb469938553608d11e6083843e7ddd185e568bc17de1a0b501451b608f8b
```

## Touched paths

- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🛂️adapters.manifest.json`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript/{📜️script.ts,📋️project.json}`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/{📜️script.ts,📋️project.json,🟦️typescript/🟨️frame-worker.js}`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`
- Related ignored outputs under the three owned generator roots, including the corrected Python styling output.

No Git state, AGENTS file, root script/project, normalization engine, shared repo-library test, Compose path, temporary Compose path, or ownerless UI icon tree was modified.

## Residual risks and acceptance checks

- A clean environment must supply the externally classified ignored `🔣️shortcodes.json` snapshot before `assets-build`; absence or catalog drift fails closed.
- The four live consumers of the ownerless UI icon tree must be relocated under a separately authorized consumer migration before that duplicate can be deleted.
- WGPU byte checks are reliable within the executing Bun version; toolchain pin enforcement must resolve the observed `1.2.5` declaration versus `1.3.14` runtime difference before cross-machine byte identity is asserted.
- Acceptance is: strict taxonomy load, live generator target/output validation, all four Nx freshness checks, zero `unsafe` generator contracts, no reads below opaque paths, and no unowned deletion.
