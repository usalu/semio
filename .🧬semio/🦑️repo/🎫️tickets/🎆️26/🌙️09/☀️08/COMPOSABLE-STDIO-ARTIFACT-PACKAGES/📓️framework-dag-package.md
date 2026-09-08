# Framework DAG Artifact Package

The schema audit found persisted DAG documents bundled inside the Infinite interactive canvas host. The snapshot, node and port data, fixture projection, mutation leaves, diff algebra, and codecs now live in an independent artifact package below `♾️infinite/🗿️artifacts/🕸️dag`. Source remains in the neutral taxonomy. It uses graph and styling value contracts without depending on Infinite. The host composes the data package.

AJV passed the language-neutral schema fixture before the missing-package assertion produced the expected red. Existing direct-mutation and VCS laws moved with their implementation. Dependency registration, consumers, and runtime verification are pending.

The direct data import migration updated 29 Rust source files and 9 owning manifests. Framework and plugin consumers now reference the DAG data package for persisted types and functions. Host rendering and interaction symbols continue to belong to Infinite.

- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🌀️generation2d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📦️packages/🦀️rust/Cargo.toml`
- `✏️s/🔌️plugins/🪐️space/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust/Cargo.toml`

Mounted-source inspection showed the artifact does not use the semio-framework umbrella. The async test macro was inspected and generates only standard-library thread-park executor code. The unused umbrella dependency was removed from DAG and Flow declarations; DAG now reaches only its required lower-level data, graph, styling, and codec dependencies.
