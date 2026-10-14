# Polygon Mesh Contract And Complete Plugin Direction Cut

## Ownership

Moved the full real TypeScript polygon parser, owned channel types, geometry analysis, component expansion, transforms, triangulation, loop/knife cut and inspection implementation from Flow BREP into framework mesh-engine/🥽️polygon. All generic geometry schemas, portable shared fixtures and the independent Three.js/Ajv/OBJ oracle suite move with it. The existing polygon source JSON schema and native Rust PolygonMeshSource/parser are already owned by framework mesh-engine; they stay at that neutral owner. Flow Rust remains a neural operator/widget adapter over those existing native public APIs rather than being copied into a fake neutral abstraction.

The authored framework SDK subpath `@semio-tech/framework/mesh/polygon` explicitly exports every contract/type and operation from this exact implementation. The three original production Procedural callers (selection, widget input edit, and IO conversions) consume that export. No compatibility path remains at Flow; its old TypeScript geometry module is removed. Flow retains only its packaged neural channel integration test and native operator adapter tests. All 25 moved-fixture include_str paths in Flow Rust unit/standalone tests resolve to the neutral owner. Native execution was not run here.

## Enforcement

The crossPluginRules constructor emits every distinct ordered plugin pair, including every destination Flow. It uses resolved source ownership and authored workspace package aliases, applies all dependency kinds, and has no test/tooling exemption. The actual source census has 36 plugin owners and 1260 distinct ordered rules. The portable closed-schema matrix covers physical sources, installed/uninstalled aliases, subpaths and nested installed aliases on both ends, production/test/script contexts, local/npm dependency kinds, same-owner allowance and package-prefix lookalike exclusion. Verdicts are independently checked by dependency-cruiser's own normalized matcher. Parent-owned pattern-safety child tests are imported in this suite; the parent changed only literalPrefixPattern and registered the combined suite in the existing repo-lib `test dependency-direction` route.

## Test-Driven Evidence And Runtime

Before moving geometry, the actual old Flow suite ran through the ticket Nx source harness: 107 passed and one failed because the packaged descriptor test referred to the stale BREP root descriptor. The retained adapter test now refers to the current 🧩️component descriptor. The full current plugin census failed before rule implementation: expected1260, observed1225, proving all35 missing Flow destination pairs.

After relocation the actual permanent framework router was run through the ticket Nx harness with `run --files=🧰️framework/📦️packages/🟦️typescript/📜️script.ts --arguments=["test-polygon-mesh"] --group=mesh-route`: 108 tests passed, zero failed, 19950 assertions. Runtime output included `[DEBUG] Polygon public SDK { vertices:4, faces:2, selection:[0,1,2] }`; parser and component selectors are the same function identities as the neutral implementation.

Actual Procedural selection runtime tests initially exposed13 stale quick-action test payloads lacking required angle/pullDirection/neutralPoint fields. Authored a portable complete quickActionParameters fixture and used it in the existing schema oracle tests without weakening the schema. The final original-caller suite passed82 tests, zero failed. It now explicitly executes the existing widget-input self-tests and the existing IO mesh surface/polygon import oracles; diagnostic output confirmed glTF admission refusals8/modes7, authored palette import cases2/refusals7, surface bridge cases2/refusals4, polygon import cases2/refusals2, with independent Three/Earcut/glTF Transform results. A direct Bun invocation of widget-input helper alone registered0 tests; it was not treated as validation and was subsequently invoked through the retained integration suite.

The neutral implementation, public SDK and pair-constructor suite passed strict TypeScript checking through ticket Nx. Final combined current-source test and repeated strict results will be appended below.

## Registrations And Limits

Registered framework `test-polygon-mesh` in its existing 📜️script.ts, 📋️project.json and package.json, and one ordered entry in both launch seed and launch.json. Three.js and Ajv are explicit test-only framework dev dependencies, with no external runtime dependency added. Flow already consumes the framework runtime SDK elsewhere; its manifest now owns that workspace dependency as a dependency. Procedural already declares framework workspace dependency. Newly authored neutral file paths have maximum122 characters.

Full current-source dependency graph acceptance remains parent-owned. This scoped report does not claim native Cargo builds, end-to-end UI activation, full constructor-policy parity, or full-goal acceptance. Procedural IO still has existing sibling Stdio and private IEEE754 imports, unrelated to the removed Flow bypass; those existed under previously enforced non-Flow pair rules and require the broader direction census to classify. No AGENTS.md, Git state, worktree, Jack or registry/composition ownership files were changed.

## Files

- `.vscode/launch.json`
- `.vscode/🧩️launch.seed.jsonc`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🎚️set-widget-input/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧪️tests/🔬️unit/🟦️.ts`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎯️selection/🧪️tests/🧫️fixtures/🔣️.json`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/package.json`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🔬️unit/🟦️.ts`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🥽️mesh/🧪️tests/🔬️unit/🦀️.rs`
- `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️extension-guest-standalone/🦀️.rs`
- `🧰️framework/📦️packages/🟦️typescript/package.json`
- `🧰️framework/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/📦️packages/🟦️typescript/📜️script.ts`
- `🧰️framework/📦️packages/🟦️typescript/🥽️mesh/🥽️polygon/🟦️.ts`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🟦️.ts`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🔬️unit/🟦️.ts`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🧫️fixtures/⏱️modeling-budget/🔣️.json`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🧫️fixtures/⏱️output-budget/🔣️.json`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🧫️fixtures/🔎️inspection/🔣️.json`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🧫️fixtures/🧭️component-transform/🔣️.json`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧪️tests/🧫️fixtures/🧵️merge-faces/🔣️.json`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧬️schema/🔎️inspection/🔣️.json`
- `🧰️framework/🔨️modules/🏗️mesh-engine/🥽️polygon/🧬️schema/🧭️component-transform/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🟨️.cjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧬️schema/🔣️.json`

Removed paths are the old Flow 🟦️.ts and the old Flow geometry schemas/fixtures at corresponding relative paths now listed under framework mesh-engine/🥽️polygon. The old Flow unit TypeScript file remains as its adapter integration test.

## Final Current-Source Verification

- Combined ticket Nx source harness `test`, group mesh, actual neutral polygon suite + constructor matrix/safety child + Flow adapter suite + original Procedural consumer integration suite: **196 passed, 0 failed, 38113 expect calls**, four files,26.51s. The portable pair oracle covers864 source/target/context/dependency-kind combinations; actual authored plugin inventory covers all1260 ordered distinct pairs.
- Final `strict`, group mesh, actual neutral implementation + public polygon SDK wrapper + constructor suite (including safety child): **passed**,4.5s.
- Permanent framework router `test-polygon-mesh`, exercised through Nx `run` fixture: **108 passed,0 failed,19950 expect calls**,1.83s.
- Original Procedural selection/widget-edit/IO integration suite: **82 passed,0 failed**,3.45s. Original portable schema payload staleness is repaired by complete authored fixture fields, not admission relaxation.
- Removed only this agent's ticket generated verification groups mesh, mesh-route and mesh-consumers after recording results. Preserved all source test fixtures, ticket inputs, reports and other agents' generated groups.
