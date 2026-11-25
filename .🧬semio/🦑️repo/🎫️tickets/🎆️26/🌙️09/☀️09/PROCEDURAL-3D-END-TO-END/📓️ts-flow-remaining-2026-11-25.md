# TypeScript Flow remainder and audit, 2026-11-25

No Rust was edited and no wasm was built. No Rust root cause was found, so there is no diff for main.

## Results (bun/vitest run one process at a time, swap below 2.8 GiB)

| Suite | Before | After | Verdict |
|---|---|---|---|
| Flow wasm extension `🧩️extensions/🕸️wasm/🧪️tests/🔬️unit` | 24 pass, 10 fail | 34 pass, 0 fail | all expectations stale (plus one stale TS entry, see 5) |
| `🌐️browser/🏷️ownership/🏷️browser-ownership` | fail | pass (5 owners, 20 consumers, 45 inputs) | stale fixture, stale package.json, stale preview inputs |
| `node-graph-hosts` | fail | still fails | needs generated `🎁️browser-package/🕸️bindings/flow_core.js` (wasm build); import path itself is correct |
| registry `✅️catalog-complete`, `🧩️composition/🎚️config` | not run | not run | need `SEMIO_SCRIPT_PROVIDERS` and the vitest policy issued by `repo run @semio-tech/plugin-registry:test`; plan phase needs 2.3 GB, refused by the memory gate |

## Unit suite, per test

1. Normal failed turn: `binding.step(effect)` replaced `step(grant)`; failure returned unwrapped. Expectation updated.
2. Plugin worker policy: close grant is now passed as `cx.with_retained_loan(state.mounted_policy.close, ...)`. Expectation updated.
3. Inference close: debit moved into `close_original_retained_effect_step` and `receive_original_work` (`consume_retained` before `consume_fuel`). The test now asserts that ordering in the framework job source instead of the removed local pieces.
4. Preview payload: 2D and 3D take `retained_grant` or `effect.grant()`; the `"retained"` payload key is gone. Tokens updated, `not "retained".to_string()` kept.
5. Typed failure: `evaluation_admit_original(&mut self.fault,&mut self.active,effect)`. Updated.
6. Gateway causes: causes retire through `ControlledRetirement::new(source)`. Updated.
7, 8. MCP: the public schema no longer has `turn`, `InferenceRetainedAuthority`, `InferenceRetainedTurnInput`; the request carries `retainedPolicy` and Rust rejects `turn` as private native authority (`inference_call_request`). The schema test `🧬️schema/🧪️tests/💡️inference-request` asserts the same. Both tests rewritten to the public contract (five-axis `InferenceRetainedPolicy`, every required field, `turn` refused). Root cause in source: `📦️packages/🟦️typescript/🧬️schema/🟦️.ts` still re-exported removed names and lacked `parseInferenceRetainedPolicy` and `HubInferenceApprovalRequestV1`; fixed.
9. Domain refusal: `admit_evaluation_failure(&mut self.fault,&mut self.fault_place,cx)`. Updated.
10. Registry admission: schema is draft-07 (changed on purpose); test used `ajv/dist/2020`. Now `ajv`.

## Ownership fixture

Consumers moved: ABI schema token lives in `🌊️flow/🧪️tests/🧫️fixtures/🟦️.ts` (hidden-tab test imports it from there); draw-list fixture is under `🖼️canvas/🧪️tests/🧫️fixtures`; layout names `@engine/flow/browser`; `publishFlowBrowserPackage` is in `🎁️browser-package/🏗️build/📜️script.ts`, `testFlowBrowserOwnership` in `🧪️tests/🔬️rust-package-commands`. Nx inputs list synchronised with project.json. Preview target now inherits default inputs (as every other target; committed in 723). Real defect: `package.json` scripts pointed at nonexistent project `semio-framework-os-flow-core`; corrected to `semio-framework-os-flow-browser-package` (fixture too).

## Audit of the previous agent's edits

1. procedural-descriptors `setLodMode`: correct (published descriptor lists main, preview and generate-preview).
2. node-graph-gestures: coverage was lost (the no-op gesture guard that retires the baseline). Restored: asserts `note_leaves(journalled)`, `pending_leaves.is_empty()` retiring the baseline, `record_history_edit(baseline)`. Vitest 28/28 with descriptors.
3. channel-hover: correct (fixture `incoming:false` is authoritative, Rust unit test reads the same field).
4. procedural-formats `exportSourceChannels`: partly wrong (set-grouping reordered channels compared with Rust `add_geometry_channel`). Rewritten as a faithful port. Laws 7/7.
5. replacement schema and evaluation-source test: correct (fixture and Rust law use 3 rounds, tolerances and admission rows); added `toleranceRounds`/`sourceEpochs` length coupling to the TS test. 11/11.
6. ui resource schema `maximumCopyBytes` 4096: correct (matches fixture and Rust `TURN`).
7. geometry caller token: correct (token present in the actions-b source; passes).
8. retirement source-contract token: correct (`close_step(&mut self.scene,body,effect)` present).
9. browser ownership fixture: was incomplete; completed above.
10. host-activation: new `dependsOn` expectation matches the generator (preparation is a cached verdict over `:component` targets). Lost coverage that prepare targets compile nothing: restored `outputs` equals `[]` for react, wgpu and alpha-react prepare targets. Not run (needs registry environment).
11. composition `devServeCommandV1`: correct (`execution` source builds `run @semio-tech/framework-os-dev:dev-<variant>-<renderer>-<profile>`).
