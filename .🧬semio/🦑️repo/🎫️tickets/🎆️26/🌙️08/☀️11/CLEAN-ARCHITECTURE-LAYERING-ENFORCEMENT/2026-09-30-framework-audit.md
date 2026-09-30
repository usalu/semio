# Framework Dependency Audit

Read-only inspection on 2026-09-30. No implementation or test execution was performed. Root and framework products/OS AGENTS.md instructions were inspected. Searches excluded build outputs where appropriate.

## Existing Enforcement

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧹️lint/🕸️dependency-boundaries/🟨️.cjs` defines `framework-no-implementation` using taxonomy area layers. Its stated scope includes type imports, dynamic imports, tests, tooling, and package subpaths.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🟦️.ts` inventories TypeScript and JavaScript roots independently of dependency-cruiser reports.
- Its sibling `🦀️cargo/🏃️execution/🟦️.ts` checks authored Cargo declarations against offline locked Cargo metadata. Neither inspected direction mechanism scans Rust compile-time source or fixture inclusions.
- Library package `📜️script.ts` exposes `lint dependency-direction`, `lint cargo-dependency-direction`, and `test canonical-architecture`. Root script invokes dependency direction from `verify layering`.
- No authored production Cargo dependency on `✏️s` or `🌎️hub` was found in framework manifests; textual matches were comments. This is evidence about manifest edges only, not proof of independence.

## Confirmed Delete-Specific Blockers

Framework Rust test compilation depends directly on specific trees through `include_str!`:

| Framework consumer | Specific owner | Location |
| --- | --- | --- |
| Surface paint unit tests | s raster protection fixture | `🧰️framework/🔨️modules/🗺️surface/🎨️paint/🧪️tests/🔬️unit/🦀️.rs:464` |
| OS directory client tests | Hub execution target lease fixture | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧪️tests/🔬️unit/🦀️.rs:411`, also 429 and 1024 |
| OS MCP schema quick tests | Hub inference schema | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🧪️tests/🔬️quick/🦀️.rs:171` |
| OS MCP inference tests | Hub GIS approval and undo fixtures | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/💡️inference/🧪️tests/🔬️inference-jobs/🦀️.rs:5`, also 461 |
| OS flow slider label tests | s flow editor fixture | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧬️schema/📸️snapshot/🧪️tests/🔬️slider-label/🦀️.rs:5` |

The MCP test `os_mirror_of_the_hub_approval_request_is_structurally_identical` explicitly treats the Hub schema as authority and the framework schema as its mirror. The correct ownership is a shared framework contract used by Hub. Neutral lease fixtures also belong next to the framework lease contract. Domain GIS approval scenarios should be tested in the specific owner; framework inference tests need domain-neutral scenarios.

Other concrete references bypass package/import graphs:

- Presentation React Vitest config `🧰️framework/🛍️products/🎤️presentation/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts:22` aliases `@semio-tech/animate-presentation-core` to a literal s animate app path. Inspect usage and remove the domain alias or relocate its domain test.
- Framework OS TypeScript package script line 385 reads Hub GIS approval fixture.
- Framework OS Rust package script lines 1299 and 2046 read Hub bootstrap source.
- Framework OS host Rust package script lines 177 and 896 read the s stdio semio artifact source.
- OS run unit tests line 607 reads the s note descriptor; nearby code checks s note Wasm build output.

## Recommended Independent Slice

1. Add schema-first language-neutral boundary vectors and a Rust source-edge inventory for literal `include_str!`, `include_bytes!`, `include!`, and `#[path]` dependencies. Resolve edges relative to authored source owners and enforce taxonomy dependency direction, including tests. Verify fixture edges against a third-party parsing implementation.
2. Relocate the execution target lease corpus to the framework directory contract owner, updating both framework and Hub consumers. Do not duplicate the fixture or create compatibility paths.
3. Extract shared inference approval contract ownership into framework and use it from Hub; replace domain fixtures in general tests with neutral framework fixtures.
4. Remove the presentation test alias if unused. Otherwise move the domain test with its alias to s.

The Rust compile-time edge inventory is the clearest missing enforcement mechanism. Import and Cargo checks alone cannot establish the requested deletability rule. Full framework independence remains unverified and requires the other source/config/script edges to be resolved.
