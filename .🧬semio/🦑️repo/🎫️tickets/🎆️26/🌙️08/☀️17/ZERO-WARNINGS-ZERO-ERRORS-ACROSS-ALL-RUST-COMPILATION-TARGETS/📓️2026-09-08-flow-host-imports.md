# Flow Host Imports

Pass568 addresses fresh native560 unused-import diagnostics after extraction of the Flow artifact crate. Removed the host crate's unused private `dsl` and `store` aliases, `graph_parameter` import, artifact glob, bridge host glob, and host VCS glob.

The VCS implementation uses the extracted artifact's exports and its retained retirement API. Removed unused collection, derive, host, artifact, and store imports there. Imports needed by the existing VCS tests now live directly in that test module: neural values, `Arc`, identity/edit types, mutation traits, store commands, and snapshot retirement. The host's forms bridge import now lives in its test module. Source inspection confirmed the test calls before relocation.

Files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️bridge/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🧪️tests/🔬️flow-vcs/🦀️.rs`

Compiler and runtime validation remain pending. No test bodies, admission limits, or retirement behavior changed.
