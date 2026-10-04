# Surface Upward Public API Audit

The actual General Surface Rust package root exposes `pub use dsl::os_dsl` after declaring the product OS kernel as the private extern alias dsl. Its normal manifest dependency resolves to the actual OS package. The alias creates a genuine public General-to-product namespace independent of whether a current caller uses it.

Bounded current searches across Framework, S and the actual Hub root (`🌎️hub`) found no direct or grouped-use os_dsl callers through semio_framework_surface, surface, framework_surface_node_graph or the renderer alias framework_surface_tiled_map. Actual Surface callers found in GIS and renderer use terrain, tiled_map, paint and node_graph. Hub has no matching Surface Rust call/dependency occurrence. This is a current source census, not a statement about future or dynamically authored consumers.

The existing rust-source-direction path laws do not cover this public namespace edge. Executing inspectRustCompileReferences on the exact Surface root yields only its four authored General module path mounts, so rustSourceDirectionEdges has no OS public-use reference to classify. The separate binding facts parser correctly retains both extern aliases and the dsl::os_dsl import, but RustImportFact has no visibility field: pub use is indistinguishable from private use in that contract. Existing binding fixtures cover extern alias resolution, including hoisted and child imports; that proves provider identity, not an explicit public upward-export refusal law.

A bounded correction can retire the unused public os_dsl forwarding item and its now-unused dsl extern alias, and add a language-neutral law for public upward namespace exports, with a private-import negative-control and renamed provider alias. Source/provider direction rules may independently catch the declared OS dependency, but that does not replace the missing public-export assertion.

Do not infer complete Surface-to-OS dependency retirement from this unused alias. The current Surface node_graph production source still exposes PackError in NodeGraphError and invokes store::pack_rt helpers, with store also aliasing the same OS kernel; Surface also mounts the product infinite_canvas provider. Those real defining-owner obligations require separate consumer/domain work. No production source was changed and no Native execution was delayed.

Evidence: Surface package `🦀️.rs` lines5–7 and Cargo.toml; source direction resolver `rustSourceDirectionEdges`; binding parser `RustImportFact`/inspectRustBindingFacts; current rust-source-direction and binding fixture corpora.

## Exact Publication Review

Independent current physical/body/hash checks admit `surface-public-entry-1.json` and `surface-public-entry-published-1.json`: the after body is exactly the before body with only the unused DSL extern alias and its public os_dsl export removed. Store alias, documentation, whitespace outside those lines, and all four original domain mounts remain exact. Actual source currently equals the published after body. This closes the concrete upward public os_dsl alias; no owning Rust compile/runtime or full Kernel dependency removal is claimed.
