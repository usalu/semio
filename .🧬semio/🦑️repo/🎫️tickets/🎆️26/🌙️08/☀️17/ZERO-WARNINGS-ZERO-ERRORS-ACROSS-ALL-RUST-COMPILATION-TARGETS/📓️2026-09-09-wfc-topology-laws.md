# WFC Topology Laws

Extended existing grid tests to enumerate every coordinate and validate reported dimension boundaries. Added neutral JSON vectors for paired/unpaired/duplicate/self custom 3D stencils and planar/spatial half-turn groups; Serde parses the vectors and compares transformed numeric outputs. Existing cube-group assertions now traverse the declared group selectors. Sparse-volume membership is checked against known occupied and absent coordinates. Registered seven exact runtime laws.

Four Rust test sources, JSON and runner syntax parsed. Runtime and full compiler verification pending.

- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧫️fixtures/🔀️topology-contracts/🔣️.json
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🔲️grid-2d/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🧊️grid-3d/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🕳️sparse-3d/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧩️assembly/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧩️wfc-engine/🪞️symmetry/🧪️tests/🔬️unit/🦀️.rs
- /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/ZERO-WARNINGS-ZERO-ERRORS-ACROSS-ALL-RUST-COMPILATION-TARGETS/📜️script.ts
