STATUS: GREEN (static inference run, log 🗑️generated/1105b/graphinf-run2.txt + graphinf-targets.json). All 9 prepare/activate react-prototype targets are produced; root cause of 'did not resolve' was duplicate dependency keys in 5 Cargo.toml (concurrent semio-framework-* -> semio-* rename sweep), fixed. Wasm/native cargo untouched; no activation run.

## Findings
- Driver: `1105b-tools/graphinf/driver.mjs` (bun; calls playgroundPreparationTargets on the real tree, 4113 targets, ~3 min). Helpers: tomlscan.mjs, dedupe.py, projects.mjs, deps.py.
- Before fix: inference died with `Cannot redefine key` (TOML) in wfc/job, wfc/oracle, wfc/bitmap, lowpoly editor, root Cargo.toml (semio-plugin-host*), later also remodeling and ui-contract text. Withholds the whole @semio-tech/framework-os-dev preparation family. Fixed by removing the duplicate identical lines (root Cargo.toml, 5-6 crate manifests). The rename sweep owner must dedupe after every pass (python3 dedupe.py <file>).
- All 9 variants have playground rows in hub/compositions (demonstrator: generator koordinator aggregator aussuchen bearbeiten verfolgen; energy; fem3d; procedural: generation3d). Statik is not a playground variant.
- The 4 `:wasm` projects (framework-surface-rs, framework-editor-rs, framework-os-node-graph-rs, puzzle-plugin) plus framework-plugin-web:support, semio-os-infinite:fonts, flow browser-package:wasm, canvas:stage-runtime-assets, workspace:deps-browsers are authored. session-<v> comes from plugin-registry playgroundSessionTargets. All 30+ *-component packages referenced exist as Cargo packages.
- project.json needed no change.

## Inferred targets (project @semio-tech/framework-os-dev), per variant
| variant | prepare deps | activate deps |
|---|---|---|
| aggregator | 37 | prepare-aggregator-react-prototype |
| aussuchen | 37 | prepare-aussuchen-react-prototype |
| bearbeiten | 37 | prepare-bearbeiten-react-prototype |
| energy | 7 | prepare-energy-react-prototype |
| fem3d | 7 | prepare-fem3d-react-prototype |
| generation3d | 19 | prepare-generation3d-react-prototype |
| generator | 37 | prepare-generator-react-prototype |
| koordinator | 37 | prepare-koordinator-react-prototype |
| verfolgen | 38 | prepare-verfolgen-react-prototype |

prepare-<v>-react-prototype dependsOn = plugin-registry:session-<v>, framework-plugin-web:support, semio-os-infinite:fonts, renderer engine :wasm (surface, editor, node-graph, puzzle), then the hub/plugin `*-component:component` closure (30 for the demonstrator rows, 1 for energy/fem3d, 13 for generation3d).

## Expected activation order
1. 4 wasm producers + fonts + web:support + session-<v> + component builds (parallel, cargo)
2. prepare-<v>-react-prototype x9 (aggregator aussuchen bearbeiten energy fem3d generation3d generator koordinator verfolgen)
3. demonstrator prepare-prototype (runtime script prepare)
4. activate-{energy,fem3d,generator}-react-prototype (activate-lanes-prototype)
5. demonstrator activate-prototype; then dev/serve (6029), prepare-e2e (+ workspace:deps-browsers), serve-e2e, test-e2e.
Note: activate-prototype does not activate aggregator/aussuchen/bearbeiten/koordinator/verfolgen/generation3d; only their prepare. Coordinator to decide whether that matches the runtime script.

## Test
`inference/🧪️tests/🎪️demonstrator-targets/🟦️.ts` (3 tests, bun 3 pass): all tracked Cargo.toml parse (oracle @iarna/toml), demonstrator variants are declared hub rows (oracle iarna), wasm producers authored. Run: bun test ./<path>.

## Union receipt gate vs lane closures (static, current tree)
STATUS: GREEN, no mismatch found, no owner edits needed. Evidence: `🗑️generated/1105b/graphinf-closure.json` (driver `1105b-tools/graphinf/closure.mjs`, `lanecompare.mjs`).
- Registry source = Cargo `metadata.semio` of 71 plugin/extension components (158 playground rows); the closure is the real `runtimeComponentClosure` (dependsOn + extends-shallow + consumes -> contributes).
- Roots (catalog panes variant + runtimeVariant): generator, generation3d, koordinator, aggregator, aussuchen, bearbeiten, verfolgen, energy, fem3d -> plugins demonstrator, procedural, energy, fem. All 9 variants have rows. Union = 32 components.
- Lanes: generator (root demonstrator, appScoped) = 30; energy = 1; fem3d (root fem) = 1. Disjoint, together exactly the 32; missing 0, extra 0. The gate's derived lanes (generator, energy, fem3d) equal `activate-lanes-prototype`. Procedural/generation3d is covered through demonstrator, hence no own lane.
- Inferred `prepare-<lane>-react-prototype` component deps (30/1/1) map 1:1 onto these closures (no missing, extra or unknown crate).
- No dangling `depends-on` ids (no stale renamed ids), no `consumes` topic without contributor. Only host component is `space` (not reached by any root).
- Caveat: the gate reads the generated, gitignored `@semio-tech/s-dev-catalog-ts/generated/plugins` (stale since 10/19 in the tree: `s-dev-composition:generate` must run first; `test-runtime-catalog` does). I did not verify the generated rows, only their source.
- Test: `inference/🧪️tests/🧾️demonstrator-union/🟦️.ts` (3 pass; oracle: independent BFS on @iarna/toml).
