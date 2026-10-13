# Build Mode Integration

Exact vocabulary changes remove dev/ship build-mode aliases and wasm-iterate compiler-profile identity. No compatibility adapters remain. Cargo manifests and native compiler policy are owned by the native agent.

## Changed Source Owners

- `📜️script.ts`
- `🌎️hub/📦️packages/🦀️rust/📜️script.ts`
- `🌎️hub/🧫️fixtures/🌳️unit-census/🔣️.json`
- `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧫️fixtures/🧾️cargo-provenance/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json`
- `🧰️framework/🛍️products/💻️os/🧪️testing/⚖️scale/📤️publication/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧪️tests/✅️catalog-complete/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔄️refresh/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️browser-host/🏗️staging/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🩺️readiness/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/👁️watch-policy.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🦀️wasm-profile-policy/🧬️v1/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📦️materialization/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/👁️watch/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🛂️descriptor/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🌐️server-pool/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️export-path-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧪️ticket-owned-browser-host-staging/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️host-handle-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️capability-policy/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/✅️verification/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📊️benchmarks/🔌️plugins/📋️plan/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📊️size/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🛂️descriptor-emission/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧫️fixtures/🔏️document-execution-target-lease-v1/🔣️.json`

## Environment Bindings

- `📋️project.json`
- `🌎️hub/🧪️tests/🤝️dev-collaboration/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧫️fixtures/🎮️registry/🏗️workspace.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️browser-serving/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️native-runtime/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/⚡️cache-contracts/🟦️.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`

## Coordinator Runtime Validation

- Added schema and language-neutral mode/policy vectors before implementation. The first Nx run exposed a missing explicit test artifact directory; the properly owned red run then failed on the absent implementation module, as intended.
- `bun nx run @semio-tech/framework-process:test-build-mode --configuration=prototype` executed the owned implementation against Ajv and TypeScript transpilation: 3 tests passed, 68 assertions, no failures. Revalidated after shared infrastructure integration: 3 passed; task time 5.0 s under parallel fleet load. This is task runtime, not total launcher latency.
- Extracted workspace BuildScript and Storybook discovery into direct command owners, with root routing loaded lazily. Root build/build-storybook/build-canvas-font-assets targets invoke the dedicated canonical script and no longer load the root audit monolith.
- Added language-neutral workspace build ownership vectors before extraction. Red Nx execution failed on the absent owner entrypoint. After correcting the independent bundler test's wildcard and literal filename matching, the prototype Nx suite passed 2 tests with esbuild import-boundary and actual mode dispatch observations. Task runtime 1.8 s; no root audit or interactivity test implementation is included by the independent bundler.
- New prototype/release workspace build and all added test commands are registered in launch.json. Parsed launch.json, root project metadata, and nx.json successfully.
- Real release execution and root prototype graph probes exposed two integration defects: native original-task selection ignored --configuration, and generic OS build aggregators depended on fixed release targets. Both have been assigned to and corrected by the Nx workstream; independent revalidation remains required before declaring acceptance.
- New process-build, process-build-mode, repository build, Storybook build, and CMake owner taxonomy declarations are explicit.

All generated command logs remain below the ticket's generated directory during ongoing work. No full monorepo build, Windows/macOS/Linux parity, or goal completion is claimed by these focused checks.
