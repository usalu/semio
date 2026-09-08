# Mounted Consumer Dependency Audit

Used Nx’s Rust module/include traversal over external consumer library, binary and explicit test targets. It includes cfg-gated source, and this is a structural check rather than a compilation result. Initial traversal stopped at a dynamic puzzle include; the following table records unsupported includes per owner explicitly.

| Package | Mounted Rust Files | Missing Direct Artifacts | Unused Direct Artifacts | Limitation |
| --- | ---: | --- | --- | --- |
| semio-s-plugin-procedural | 364 |  |  |  |
| semio-s-plugin-energy | 1502 |  |  |  |
| semio-hub | 38 |  |  |  |
| semio-s-plugin-animate | 120 |  |  |  |
| semio-s-plugin-norm | 0 |  |  | Error: Generated Rust modules require conservative source inputs: /Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🏭️vdi3805/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs |
| semio-s-plugin-writer | 110 |  |  |  |
| semio-s-plugin-dag | 125 |  |  |  |
| semio-s-plugin-sequence | 96 |  |  |  |
| semio-s-plugin-process | 134 |  |  |  |
| semio-s-plugin-lowpoly | 146 |  |  |  |
| semio-s-plugin-block | 602 |  |  |  |
| semio-s-plugin-flow-extension-brep | 2 |  |  |  |
| semio-s-plugin-flow | 141 |  |  |  |
| semio-s-plugin-reasoning-mindmap | 104 |  |  |  |
| semio-s-plugin-playbook | 93 |  |  |  |
| semio-s-plugin-raster | 126 |  |  |  |
| semio-s-plugin-draw | 131 |  |  |  |
| semio-s-plugin-space | 179 |  |  |  |
| semio-s-plugin-remodel | 373 |  |  |  |
| semio-s-plugin-fem | 487 |  |  |  |
| semio-s-plugin-demonstrator | 40 |  |  |  |
| semio-s-plugin-cad | 154 |  |  |  |
| semio-s-plugin-trinity | 253 |  |  |  |
| semio-s-plugin-puzzle | 0 |  |  | Error: Dynamic Rust include requires explicit source inputs: /Users/ueli/Documents/semio/✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🔣️icons/🦀️.rs |
| semio-s-plugin-layout | 133 |  |  |  |
| semio-framework-os-flow | 40 |  |  |  |
| semio-s-plugin-mathematical | 114 |  |  |  |
| semio-s-plugin-shooting | 213 |  |  |  |
| semio-s-plugin-architect | 1130 |  |  |  |
| semio-s-plugin-forms | 126 |  |  |  |
| semio-s-plugin-imperative | 92 |  |  |  |
| semio-s-plugin-gis | 195 |  |  |  |
| semio-s-plugin-note | 233 |  |  |  |
| semio-s-plugin-vcs | 84 |  |  |  |
| semio-s-plugin-sourcing | 84 |  |  |  |
