# WGPU Single-Enqueue Dependency Check R17

Read-only Cargo metadata --offline --no-deps produced the actual workspace package/dependency declarations. A transitive superset traversal from semio-framework-os-renderer-wgpu includes all declared target/feature/dev/build workspace edges. It reaches52 workspace packages and does not reach semio-framework-value-resident. This proves the unfinished resident package is absent even from this overinclusive workspace graph; it is not an exact enabled-feature compilation closure.

The first displayed metadata output was truncated; the second read was captured completely below. Neither invocation compiled Rust. External registry packages are not enumerated by --no-deps and this report does not claim their source capture.

```json
{
  "kind": "Cargo metadata workspace dependency superset, including all declared targets/features/dev/build edges",
  "root": "semio-framework-os-renderer-wgpu",
  "residentReached": false,
  "packages": [
    "semio-framework",
    "semio-framework-2d",
    "semio-framework-3d",
    "semio-framework-actor",
    "semio-framework-async",
    "semio-framework-async-macros",
    "semio-framework-compiler",
    "semio-framework-dispatch-macros",
    "semio-framework-editor",
    "semio-framework-geometry",
    "semio-framework-graph",
    "semio-framework-hash",
    "semio-framework-job",
    "semio-framework-math",
    "semio-framework-mesh-engine",
    "semio-framework-number",
    "semio-framework-os",
    "semio-framework-os-flow",
    "semio-framework-os-infinite",
    "semio-framework-os-kernel",
    "semio-framework-os-kernel-dsl-derive",
    "semio-framework-os-kernel-neural-engine",
    "semio-framework-os-renderer-wgpu",
    "semio-framework-os-services",
    "semio-framework-pack",
    "semio-framework-plugin",
    "semio-framework-plugin-host",
    "semio-framework-replication",
    "semio-framework-schema",
    "semio-framework-schema-derive",
    "semio-framework-surface",
    "semio-framework-trace",
    "semio-framework-ui",
    "semio-framework-ui-backend-d3d12",
    "semio-framework-ui-backend-metal",
    "semio-framework-ui-backend-vulkan",
    "semio-framework-ui-backend-webgpu",
    "semio-framework-ui-contract",
    "semio-framework-ui-host",
    "semio-framework-ui-render",
    "semio-framework-ui-runtime",
    "semio-framework-ui-scene",
    "semio-framework-ui-styling",
    "semio-s-plugin-flow-extension-brep",
    "semio-s-plugin-flow-extension-dictionary",
    "semio-s-plugin-flow-extension-list",
    "semio-s-plugin-flow-extension-logic",
    "semio-s-plugin-flow-extension-math",
    "semio-s-plugin-flow-extension-primitive",
    "semio-s-plugin-flow-extension-text",
    "semio-s-plugin-puzzle",
    "semio-s-plugin-stdio"
  ],
  "edges": [
    {
      "from": "semio-framework",
      "to": "semio-framework-actor",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework",
      "to": "semio-framework-hash",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework",
      "to": "semio-framework-mesh-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework",
      "to": "semio-framework-ui",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-2d",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-3d",
      "to": "semio-framework-geometry",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-3d",
      "to": "semio-framework-number",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-3d",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-3d",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-actor",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-actor",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-actor",
      "to": "semio-framework-async",
      "kind": null,
      "target": "cfg(target_arch = \"wasm32\")"
    },
    {
      "from": "semio-framework-async",
      "to": "semio-framework-trace",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-async",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-compiler",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-editor",
      "to": "semio-framework-os-infinite",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-editor",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-editor",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-editor",
      "to": "semio-framework-ui-styling",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-geometry",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-graph",
      "to": "semio-framework-geometry",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-graph",
      "to": "semio-framework-os-kernel-neural-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-graph",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-graph",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-job",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-job",
      "to": "semio-framework-trace",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-job",
      "to": "semio-framework-async",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-math",
      "to": "semio-framework-dispatch-macros",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-math",
      "to": "semio-framework-geometry",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-math",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-mesh-engine",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os",
      "to": "semio-s-plugin-stdio",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os",
      "to": "semio-framework-ui",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os",
      "to": "semio-framework-actor",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os",
      "to": "semio-framework-async",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os",
      "to": "semio-framework-plugin-host",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-graph",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-math",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-os-kernel-neural-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-2d",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-3d",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-os-infinite",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-replication",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-s-plugin-stdio",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-ui-styling",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-ui-backend-webgpu",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-ui",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-s-plugin-flow-extension-brep",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-s-plugin-flow-extension-dictionary",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-s-plugin-flow-extension-list",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-s-plugin-flow-extension-logic",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-s-plugin-flow-extension-math",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-s-plugin-flow-extension-primitive",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-flow",
      "to": "semio-s-plugin-flow-extension-text",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-compiler",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-geometry",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-graph",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-math",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-3d",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-ui-styling",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-ui",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-infinite",
      "to": "semio-framework-async",
      "kind": null,
      "target": "cfg(target_arch = \"wasm32\")"
    },
    {
      "from": "semio-framework-os-kernel",
      "to": "semio-framework-os-kernel-dsl-derive",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-kernel",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-kernel",
      "to": "semio-framework-hash",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-kernel",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-kernel",
      "to": "semio-framework-pack",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-kernel",
      "to": "semio-framework-replication",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-kernel",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-kernel",
      "to": "semio-framework-actor",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os-kernel",
      "to": "semio-framework-os-services",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os-kernel-dsl-derive",
      "to": "semio-framework-hash",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-kernel-neural-engine",
      "to": "semio-framework-replication",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-actor",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-async",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-os-services",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-plugin-host",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-os-flow",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-editor",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-surface",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-os-infinite",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-s-plugin-puzzle",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-3d",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-job",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-trace",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-ui-host",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-ui-render",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-renderer-wgpu",
      "to": "semio-framework-ui",
      "kind": null,
      "target": "cfg(not(target_os = \"wasi\"))"
    },
    {
      "from": "semio-framework-os-services",
      "to": "semio-framework-actor",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-services",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-services",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-os-services",
      "to": "semio-framework-async",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-os-services",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-pack",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-pack",
      "to": "semio-framework-replication",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-pack",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-dispatch-macros",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-hash",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-schema",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-trace",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-ui-runtime",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-ui-scene",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-ui",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework-actor",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework-os-services",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework-trace",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework-ui",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework-async",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-plugin-host",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-replication",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-schema",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-schema",
      "to": "semio-framework-pack",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-schema",
      "to": "semio-framework-schema-derive",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-schema",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-surface",
      "to": "semio-framework-os-infinite",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-surface",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-surface",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-surface",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-surface",
      "to": "semio-framework-ui-styling",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui",
      "to": "semio-framework-geometry",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui",
      "to": "semio-framework-ui-scene",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui",
      "to": "semio-framework-ui-styling",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-framework-ui-backend-d3d12",
      "to": "semio-framework-ui-render",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-backend-metal",
      "to": "semio-framework-ui-render",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-backend-vulkan",
      "to": "semio-framework-ui-render",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-backend-webgpu",
      "to": "semio-framework-ui-render",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-contract",
      "to": "semio-framework-ui-styling",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-host",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-host",
      "to": "semio-framework-ui-render",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-host",
      "to": "semio-framework-job",
      "kind": null,
      "target": "cfg(not(target_arch = \"wasm32\"))"
    },
    {
      "from": "semio-framework-ui-host",
      "to": "semio-framework-ui-backend-webgpu",
      "kind": null,
      "target": "cfg(target_arch = \"wasm32\")"
    },
    {
      "from": "semio-framework-ui-host",
      "to": "semio-framework-ui-backend-vulkan",
      "kind": null,
      "target": "cfg(target_os = \"linux\")"
    },
    {
      "from": "semio-framework-ui-host",
      "to": "semio-framework-ui-backend-metal",
      "kind": null,
      "target": "cfg(target_os = \"macos\")"
    },
    {
      "from": "semio-framework-ui-host",
      "to": "semio-framework-ui-backend-d3d12",
      "kind": null,
      "target": "cfg(target_os = \"windows\")"
    },
    {
      "from": "semio-framework-ui-render",
      "to": "semio-framework-geometry",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-render",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-render",
      "to": "semio-framework-ui-styling",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-runtime",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-runtime",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-scene",
      "to": "semio-framework-geometry",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-framework-ui-scene",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-brep",
      "to": "semio-framework-os-flow",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-brep",
      "to": "semio-framework-os-kernel-neural-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-brep",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-brep",
      "to": "semio-framework-3d",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-brep",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-brep",
      "to": "semio-s-plugin-stdio",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-brep",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-dictionary",
      "to": "semio-framework-os-flow",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-dictionary",
      "to": "semio-framework-os-kernel-neural-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-dictionary",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-dictionary",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-dictionary",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-list",
      "to": "semio-framework-os-flow",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-list",
      "to": "semio-framework-os-kernel-neural-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-list",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-list",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-list",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-logic",
      "to": "semio-framework-os-flow",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-logic",
      "to": "semio-framework-os-kernel-neural-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-logic",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-logic",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-logic",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-math",
      "to": "semio-framework-os-flow",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-math",
      "to": "semio-framework-os-kernel-neural-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-math",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-math",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-math",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-primitive",
      "to": "semio-framework-os-flow",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-primitive",
      "to": "semio-framework-os-kernel-neural-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-primitive",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-primitive",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-primitive",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-text",
      "to": "semio-framework-os-flow",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-text",
      "to": "semio-framework-os-kernel-neural-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-text",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-text",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-flow-extension-text",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-geometry",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-graph",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-os-infinite",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-async",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-dispatch-macros",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-hash",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-schema",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-ui-scene",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-s-plugin-stdio",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-s-plugin-puzzle",
      "to": "semio-framework-os",
      "kind": null,
      "target": "cfg(not(all(target_arch = \"wasm32\", target_env = \"p2\")))"
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-3d",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-dispatch-macros",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-geometry",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-graph",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-hash",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-job",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-math",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-mesh-engine",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-number",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-os-kernel",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-plugin",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-schema",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-ui-contract",
      "kind": null,
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-async-macros",
      "kind": "dev",
      "target": null
    },
    {
      "from": "semio-s-plugin-stdio",
      "to": "semio-framework-ui-scene",
      "kind": "dev",
      "target": null
    }
  ]
}
```
