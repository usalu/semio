STATUS: GREEN. tsc clean over the wide tree except the 2 expected `@semio-tech/puzzle-wasm` not-found; bun + renderer vitest tests pass.

## Real bugs fixed
- `PluginRuntime/🧺️handle-ui-intake/🟦️.ts` (`createUiSurfaceProjection`): the local `const reads = uiReadsByInstance.get(...)` (a `Set<Promise>`) shadowed the `reads` lease registry parameter, so `reads.issueTarget(...)` in `project`/`projectSections` hit the Set (TS2339 x4) and, being declared after the `read` IIFE starts, also a TDZ ReferenceError at runtime before the first `await`. Renamed the in-flight set to `inflight`; `reads.issueTarget` now resolves to the lease registry (`lease.ts`). This affected every `refreshUi` of a retained surface in the browser (all 8 panes).
- `…/🏪️store/📬️paged-one-item/🧬️mutation-apply/🧪️tests/🧫️fixtures/🔣️.json`: a peer (commit 705) overwrote the structural-mutation cases fixture (base/cases/cancelAt/maximumTurns, used by `tests/🦀️.rs` `fixture()` and `tests/🟦️.ts`) with the issuer-stage law. Restored the cases fixture from blob 4be74a6bb0e; moved the stage law to `tests/🧫️fixtures/📋️stages/🔣️.json` (+ new `🧬️schema/🔣️.json`) and repointed `tests/📋️stages/🦀️.rs` (`include_str!`). SHARED CHANGE (Rust test input; Rust not re-run by me).
- `…/🏪️store/🕰️history/🏁️replay/🧪️tests/🟦️.ts`: `Object.entries(law.modes)` inferred `unknown` -> `Object.entries<number>(…)`.
- `…/📺️renderer/🧑‍🎨engine/🧪️tests/🛠️plugin-runtime-helpers/🟦️.tsx`: `fakeHandle` lacked `acquireOwnedUiSurface` (required by `PluginWasmHandle`) -> added `acquireOwnedUiSurface: () => undefined` like the other fakes.

## Wide typecheck
- `C/1029-tools/tsfront/tsconfig.json` (renderer, plugin browser-bundle + materialization, os shell + host, plugin-registry (module + package), demonstrator, both ambient dirs; types bun,node,react,react-dom,vite/client like os/tsconfig.json).
- `node_modules/.bin/tsc -p …` -> `C/🗑️generated/1029/tsfront-tsc1.txt` (2 puzzle-wasm + 1 lodash-es: ambient decl dir was missing from my include), `tsfront-tsc2.txt`: only the 2 expected `@semio-tech/puzzle-wasm` not-found errors.
- bridge-tsc1's 9 errors: all 9 resolved (4 issueTarget, 3 mutation-apply, 1 replay, 1 helpers).

## Tests
- `bun test ./…/mutation-apply/🧪️tests/🟦️.ts ./…/replay/🧪️tests/🟦️.ts` -> 2 pass, 0 fail.
- `bun test ./…/handle-ui-intake/lease.test.ts` -> 7 pass (lease registry incl. `issueTarget`).
- Renderer vitest (long level, plugin-runtime-suite which registers typedwire/owned-facade/ui-close/instance-open/transaction-coordinator/view-model-surfaces suites): 1 file, 140 tests pass (`C/🗑️generated/1029/tsfront-vt3.txt`).
- `repo run @semio-tech/framework-renderer-react:test -- <paths>` is not usable for targeted runs: default level is `quick` (fixed include list, paths -> "No test files found") and the nx graph wait took ~47 min (vt2). Targeted runner: `C/1029-tools/tsfront/vitest.ts` (`bun <it> <relative paths from the react package dir>`; builds the repository vitest policy, level long).
- Not run: Rust tests for mutation-apply/stages (fixture move above); the coordinator's next mutation-apply cargo test should confirm.

## Follow-up (coordinator request)
- Rust: `repo cargo --store=shared --configuration=prototype -- test --manifest-path …/📬️one-item-owners/📦️packages/🦀️rust/Cargo.toml --lib stage_contract` (crate `semio-framework-os-store-one-item-batch`) -> rc=0, `stage_contract::neutral_stage_contract_has_every_retained_frontier` 1 passed (`C/🗑️generated/1029/tsfront-cargo1.txt`). Repointed `include_str!` works.
- Schema registration: sibling fixture schemas are registered by being loaded and Ajv-validated next to their fixture (admission/inverse-* in `source-and-native-tests/🟦️.ts`; cases fixture in `mutation-apply/🧪️tests/🟦️.ts`). There is no separate taxonomy/fixture registry entry for them. The new `📋️stages/🧬️schema/🔣️.json` is now loaded in `mutation-apply/🧪️tests/🟦️.ts` (new test: Ajv strict validation + stage order/Commit-exclusion laws); bun test 2 pass; wide tsc unchanged (only puzzle-wasm).
