# Registered Renderer Profiles, Activation and Future Mount Readiness

Read-only current source inventory; no new build/server/HTTP execution, mounted input or output claim is supplied by this document. Root holds production until its final source checkpoint. The last actual WGPU run 48039 stopped at S locked fetch before Trunk; bounded current S lock and normal dependency-coherence proofs have since passed separately.

## Existing finite native renderer owners

Project `@semio-tech/framework-renderer-wgpu` owns `wasm` and `wasm-release`. Both current authored targets call the existing `🏗️compiler/🌐️wasm/📜️script.ts build <dev|release>` from its TypeScript package directory. Both depend on `workspace:deps-cargo`, `workspace:deps-trunk` and `workspace:deps-wasm-opt`; inferred native prerequisites remain owned by normal Nx. The development retry command is `bun nx run @semio-tech/framework-renderer-wgpu:wasm --skip-nx-cache --outputStyle=static`; the corresponding release target is `wasm-release`. No flags, native target, grants, feature selection or budgets are changed by this readiness record.

The compiler uses the current Rust package `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml`. Development output is its `dist/wasm-dev`; release output is `dist/wasm-release`. Trunk's generated configuration selects `release = false|true`; its actual raw compiler query has the same `wasm32-unknown-unknown`, `--offline --frozen --locked --message-format=json` arguments and adds only the existing `--release` for release. The original build budget comes from the existing `buildBudgetMs()` and cancellation owns only this subtree.

Actual Cargo build roots remain the previously authorized absolute repository cache `cargo/fixture-boundary-production/{target,build}`. Trunk owns a disposable target beneath its normal staging directory while inheriting the durable build directory and observer root. Completed proof is discovered under actual build directory `semio-trunk-provenance/<observation>/trunk.json`; that receipt binds original producer source SHA, profile, manifest, same completed raw/query WASM SHA, compiler receipt and staged output SHA. Source/roster/currentness must be revalidated rather than crediting existing dist files. Development and release have distinct observation and output owners. Their compiled/resource evidence can legitimately become historical when original source or observed directory rosters change.

## Existing activation owners

Canonical `playgroundActivationTargetV1(variant, profile, renderer)` returns `@semio-tech/framework-os-dev:activate-<variant>-<renderer>-<profile>` only for an admitted current generated playground. The actual inferred target generator is library `🟨️.mjs` `playgroundPreparationTargets`, not a hand-copied project table. For admitted `s`, the four normal activation targets are react/wgpu × dev/release. Each owns its matching runtime completion under Dev TypeScript package `dist/runtime/<renderer>/<profile>/s/activation`.

WGPU activation depends on matching prepare. Its owned closure includes current registry session, browser support, font producer, Flow browser producer, the selected WGPU `wasm|wasm-release`, browser boot/frame worker/renderer boot generators and the declared boot component materializations. React preparation owns its corresponding normal session/support/fonts/Flow/declared engine and boot component closure. The current generator still explicitly names `semio-framework-os-infinite:fonts`; retain that actual source selection and independently prove its current original asset inputs after any genuine Canvas relocation. This record supplies no successful activation claim and does not rewrite that ownership.

The original activation implementation names healthy actually staged components, checks session identity and source/content custody, then publishes its normal completion receipt. Module staging is owned by `pluginModulesRoot(profile)` under plugin TypeScript package `dist/<profile>/🔌️plugin-modules`; per-renderer extension and activation directories come from `developmentRuntimeRoot`. Original producer receipts and mounted profile inputs must remain current through later registry/materialization/activation. Execute the normal ownership chain serially around the finite native compiler lane; do not duplicate native acquisition from the default guard while another owned compiler is running.

## Future raw mount receipt derivation

WGPU server config exports `wgpuCompletedFrameworkRootsV1(profile)`: compiler root `dist/wasm-<profile>`, boot root `🚀️browser-boot/🤖️generated`, renderer root `🎬️renderer-boot/🤖️generated`, worker root `🎞️frame-worker/🤖️generated`. `wgpuBrowserMounts` owns the literal compiler route `/renderer-modules/wgpu` plus those generated routes and current plugin/extension roots. The browser host itself requests `/renderer-modules/wgpu/semio-framework-os-renderer-wgpu.js` and the corresponding `_bg.wasm`. The WGPU serve owner sets `SEMIO_BUILD_MODE=dev|ship` from its actual selected profile, so config picks dev/release consistently.

After an actual current receipt passes the strict Trunk/unit/resource verifier, derive raw candidate rows from its exact staged output entries and the actual server mount root, never guess digests or accept arbitrary dist presence. The JS and WASM owner paths must be the exact current receipt outputs under the selected compiler root. Read their actual lengths/SHA only after producer handoff and bind them to the receipt outputs; recheck before/after HTTP. Then provide those real route/path/SHA/length rows to Root's existing `mounted-runtime-http` input. No such input is populated here.

React's current Vite config also chooses that same profile-specific compiler root and aliases `/renderer-modules/wgpu`, with an explicit filesystem allow list. It does **not** install the WGPU raw static-dir mount for that route: its static-dir list covers plugin/vendor/extension roots. Thus source alias declaration alone cannot establish React raw JS digest equality; actual Vite JS responses may be transformed. Root's HTTP verification must use actual mount semantics and distinguish transformed JS from raw staged bytes. A direct WASM route or genuine static production asset is a source-backed candidate pending actual response, not a claimed positive. Four renderer/profile contexts may require different positive raw-byte owner rows; do not force the WGPU raw-JS expectation onto React.

Root's existing retained command uses the normal `ensureDevServe` owner, reserves a fresh port, refuses reuse, checks bounded raw response MIME/length/independent hashes, and stops only its owned server. It requires a nonempty real current context roster and does not replace the normal server. Actual HTTP and protected selected actor execution remain separate pending proofs.

## Current source identities

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json`: `0c590ac70431047f7cfd361ef55c43414711b50b37f942cc88eb1e08df954ac4`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📜️script.ts`: `8568e1966f732075a9b64102a708f4458102b24452702b2c980f12d4b8d65823`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🟦️.ts`: `0b02718910e80504f1510f8fd2acf64626edff7632f7d0024faaea187b1434ff`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts`: `dc3bf23c90bbb1aeafb53f3aaa2962eee5ee3a5d6fa32b943f6262bc108367fb`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts`: `e5ebd36fd84bd7b95064ca202e6eb34da8b655cdcc18439a9069b21f9407cb0b`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts`: `772ce682e52d65cfa6a004c006b27676ea0e93036f7a51dd4ad106e1757dc15c`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts`: `4535dd63d439bfdd1ccb33976cae6222f4777ef99aa4fa118585540f5c742e46`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts`: `9642c4350b58fe21989038eee0d8069e99af6d1d946c033ae9f0289d8fc943e5`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts`: `89112ae534c71bc767ec40dd70e93898032b4982570c5fa09ee9e28f8a8f4e27`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`: `e9a2e397bbc9bfba254b6cf999e14e3c151db19fa02d1cac0fbc6400dfd9097a`

## Current Asset Dispatch Observation

Read-only source observation on 2026-10-08: the current React Vite owner selects all declared playground assets for the normal host filter, including `s`, at `resolvedPlaygroundAssets`; individual variants select only their own declaration. The WGPU server selects only the exact current playground row's `assets`. Both now use the genuine framework asset dispatch owners; the current CAD and Infinite declarations still select canonical asset roots and no fixture route. This is source selection evidence only. Future actual HTTP contexts can use normal React `s` to check both genuine assets; WGPU needs the corresponding actual variant declaration for an asset positive. Its compiler JS/WASM mounts remain separate raw positive candidates. No source edits, server execution, input placeholder or successful byte response is supplied by this observation.

Observed source SHA-256 identities:

- 🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts: `89112ae534c71bc767ec40dd70e93898032b4982570c5fa09ee9e28f8a8f4e27`
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/🎚️config/🟦️.ts: `dc3bf23c90bbb1aeafb53f3aaa2962eee5ee3a5d6fa32b943f6262bc108367fb`
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts: `33f192ede1108816c35e97c3ef72ed2f2b5614d397b56f59dd635b675fa860d6`
