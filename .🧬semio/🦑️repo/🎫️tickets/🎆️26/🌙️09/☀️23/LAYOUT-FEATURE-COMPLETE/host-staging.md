# WGPU Browser Staging Inventory

The WGPU browser command requires a completed renderer compiler publication, three generated browser bundles, a published font asset, a Specific OS catalog and selected guest staging. On this checkout the owned products area `.cache/b/p/m` was absent. The renderer build receipt, all three generated JS directories, the generic registry generation, the Specific catalog and the Layout activation receipt were absent when checked. The UI axes directory contained its generated Rust unit; the browser expansion still has a declared UI generation dependency.

## Canonical Producers

All commands begin at the repository root, with repository language and ordinary resource admission.

| Input | Authoritative producer | Expected output or dependency |
| --- | --- | --- |
| Renderer core | `repo run @semio-tech/framework-renderer-wgpu:wasm:prototype --parallel=1 --output=stream` | Products `renderer/<owner SHA prefix>/prototype/wasm`, containing `semio-renderer-wgpu-component.js` and `semio-renderer-wgpu-component_bg.wasm`; the supported build owns wasm-bindgen, artifact hashes and publication. |
| Browser boot | `repo run @semio-tech/framework-renderer-wgpu:generate-browser-boot` | WGPU `🚀️browser-boot/🤖️generated/🟨️.js`; depends on the UI axes producer `@semio-tech/ui-rs:generate`. |
| Renderer library boot | `repo run @semio-tech/framework-renderer-wgpu:generate-renderer-boot` | WGPU `🎬️renderer-boot/🤖️generated/🟨️.js`; depends on the UI axes producer `@semio-tech/ui-rs:generate`. |
| Browser frame worker | `repo run @semio-tech/framework-renderer-wgpu:generate-frame-worker` | WGPU `🎞️frame-worker/🤖️generated/🟨️.js`; depends on the UI axes producer `@semio-tech/ui-rs:generate`. |
| Generic registry | `repo run @semio-tech/plugin-registry:generate` | Own generated registry data; depends on the authenticated catalog-inventory producer `repo:generator-inputs`. |
| Specific OS catalog/session | `repo run @semio-tech/plugin-registry:session-layout` | Depends on `@semio-tech/s-dev-composition:generate`, the Specific output at `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`, and provider descriptor/version checks. |
| Browser guest bridge | `repo run @semio-tech/framework-plugin-web:support` | Owned products publication and `.🧾️store/support.json` for the browser bridge and preview2 shim. |
| WGPU font bundle | `repo run semio-os-infinite:fonts` | Font products publication and font receipt; depends on `semio-framework-canvas-font-assets:build` for the actual font packer. |
| Layout module and activation | `repo run @semio-tech/framework-os-dev:activate-layout-wgpu-prototype --output=stream` | Stages the selected guest closure and publishes the activation receipt. Serve consumes the staged closure and exact host inputs. |

The renderer core’s checked manifest belongs to `semio-renderer-wgpu-component`. It uses `wasm32-unknown-unknown`, unlike the Layout guest’s `wasm32-wasip2`. Browser Vite verifies existence of the two core files, all three JS bundles, and the canonical font file before it starts.

A bare `repo <renderer-owner-script> build prototype` failed immediately with `Original parent Script process invocation required`. The compiler, browser publication and Vite scripts require the task-owned issued process. The scoped target is running as session 1871 and is waiting on the shared graph. The exact native prewarm (session 20223) is separate and writes raw Cargo outputs only:

```text
repo cargo --store=shared --configuration=prototype --parallelism=1 -- build --manifest-path "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧩️component/📦️packages/🦀️rust/Cargo.toml" -p semio-renderer-wgpu-component --target=wasm32-unknown-unknown --lib --offline --frozen
```

The production Layout WASI build and checkout activation were already running when this prewarm started. The prewarm can share admitted compiler outputs with the owned renderer producer. The scoped target retains responsibility for the bindgen wrapper, products publication and receipts.

## Browser Bridge Preflight

The browser support owner materialized all 15 pinned Preview2 shims in ticket scratch and generated the real browser shard worker. The CLI shim contained both the complete-line release and guest log classification patches. An independent esbuild transform parsed each shim and the worker as ECMAScript modules. This exercised the same source builders as the support command. Publication still requires the running scoped repository target, its Original caller and destination receipt.

The live graph maps WGPU prepare and activation through the common renderer-aware operators and maps WGPU serve directly to the dedicated Vite server. The WGPU server receives the composition path from the inferred play target and binds the declared port 6179. The former React-only serve operator is a separate target. The old shared runner received SIGINT while selecting source inputs, and each attached activation, support, fonts and UI-axes client then closed under reference 26b06666. The Semio owner-only activation plan and independent support producer are queued behind current fact-store hydration.

## Current Prototype Bring-Up

The normal development target is `@semio-tech/framework-os-dev:dev-layout-wgpu-prototype`. The actual Specific inference test passed its Layout serve and activation contracts. The target serves port 6179 and selects the Specific WGPU composition.

The normal catalogue script published the inventory. The Layout session stage published one plugin under the prototype browser products. The activation owner reached its staged component check after reading usable session, browser support and font inputs. The live endpoint depends on the linked guest module and the raw WGPU renderer output.

The active source-staging commands use the repository's ordinary owner scripts with the workspace Script bootstrap. After the guest publishes, the semantic-pair verifier checks the compiled descriptor against the authored linked-asset fixture, and the catalogue owner refreshes its projection. The production activation and WGPU serve owners follow. The HTTP probe then checks the isolated-origin headers and real JavaScript and WASM assets. The extended Playwright journey uses the Chromium profile with real WebGPU and records the editor state and its exported files under this ticket's generated directory.

The executable journey is maintained in ticket `26/09/16/LAYOUT-PLUGIN-END-TO-END`.
