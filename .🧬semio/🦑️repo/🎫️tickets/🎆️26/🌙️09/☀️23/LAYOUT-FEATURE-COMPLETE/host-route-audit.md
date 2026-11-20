# WGPU Host Route Audit

Source inspection, 2026-11-20. No server was started, and no live endpoint success is claimed.

The framework-os-dev activation serve owner accepts only React; WGPU has a separate canonical server owner at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts. It accepts variant, profile, explicit port and composition; it consumes completed renderer artifact inputs. Direct script invocation lacks the task-owned Original process admission, so use the generated repository serve target, not direct fallback. Activation publishes the selected guest receipt; it does not itself listen.

The authored Specific WGPU composition is ✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts. It selects SEMIO_PLUGIN from generated PLAYGROUND_BUILD_TARGETS, composes catalog rows using the actual S_OS_PORT, loads generated native catalog, and supplies playground assets. The generated catalog directory was absent at inspection; session-layout must publish it. The server default config instead explicitly uses undefined variant and empty catalog and assets; a plugin query alone cannot furnish the real Layout catalogue.

Server/🟦️.ts mounts completed renderer files under /renderer-modules/wgpu, staged guest modules under /🔌️plugin-modules, and extensions under /🧩️extension-modules. The registry session constructs each guest bridge URL from the authoritative inventory directory and 🌉️bridge.js. The server boundary returns 404 for missing mounted artifacts, avoiding SPA fallback. Once the canonical listener is live, verify the injected semio-plugin-catalog contains Layout, request its exact moduleUrl and dependent generated WASM, and check the renderer JS and WASM routes for successful responses. Static source and generated files do not establish fresh HTTP responses or interactive guest startup.

## Task capability route

The live-activation build test at 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🧪️tests/🧊️live-activation/🟦️.ts:28–44 verifies generated WGPU serve and dev targets depend on activation, activation depends on preparation, and preparation includes the matching renderer build. Its separate temporary Nx fixture uses controlled publishers and is not a route for issuing the real host provider/store capabilities. No source-supported owner-only route bypassing the fact inventory while retaining these capabilities was identified. Direct owner-script invocation has already refused the missing Original parent. Scoped task retries remain the supported route.
