# Plugin Registry and Loading Report

## Findings (static analysis, no source changes made)
1. `🧰️framework/.../🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json` is `[]` and gitignored (`**/🤖️generated/`), dated Nov 2. The framework registry is regenerated output, not authored. Plugins are discovered only through Cargo manifests carrying `[package.metadata.component]` (`🔎️discovery/🟦️.ts` `findPluginCargoFiles`), so an empty framework registry is not itself a norm defect.
2. The "s" instance catalog is `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🔌️plugins.json` (stale, Oct 29). It already lists `norm` (cratePath `🌎️hub/🧩️compositions/📕️norm/🧩️component/📦️packages/🦀️rust`, wasm `semio_hub_norm_component.wasm`, activation events `computation.norm.*` and `s.norm.*` for all 15 standards). Norm therefore loads via the Rust hub composition (component wasm), not via a `🎛️apps` crate. It is a stale snapshot and needs regeneration after the compile fixes.
3. `✏️s/🔌️plugins/🔒️policy-allowlist.json` (key `semantic-vocabulary`, consumed by `policyDiscoveredAllowlist`, lane `workspace:semantic-vocabulary`) has 219 of 342 entries pointing at nonexistent paths across ~35 plugins (norm: 30 `🎛️apps/...` + 48 `🗿️artifacts/...`), i.e. the whole file predates taxonomy renames. Stale entries only exempt nothing; the correct fix is to resync with actual lane breaches, which needs the lane run.
4. Proof-of-load test for s: `workspace`-level `@semio-tech/s-dev-composition:catalog-check` (vitest in `✏️s/🧑‍💻dev/🧩️catalog/🧪️tests`: plugin-identity, host-activation, selected-runtime, projection) plus `verify-catalog` and `test-catalog-publication`.

## Commands run
- `repo run @semio-tech/plugin-registry:preview-generated` (killed by my 560 s timeout under host load).
- `repo run @semio-tech/plugin-registry:generate` and `repo run workspace:semantic-vocabulary`: first attempt hit a transient `graph.plan-refused` catalog error (another agent mid-edit of build texts, later self-resolved); later attempts stalled for 10+ min.

## Blocker
Graph service deadlock: `Reading the project graph slice ... @semio-tech/gis-gismap-js (project, 495 s, 0 rows); waiting for memory headroom (954 of ~509 MB reserved)` never progresses; swap 6.7/8.9 GB used. No generated output or lane result obtained. Files changed: none.

## Next steps
Rerun once the host is healthy: `repo run @semio-tech/plugin-registry:generate`, `repo run @semio-tech/s-dev-composition:generate`-equivalent for the s catalog (check `✏️s/🧑‍💻dev/🧩️catalog/📣️publication`), `repo run workspace:semantic-vocabulary` (then sync allowlist), `repo run @semio-tech/s-dev-composition:catalog-check`.
