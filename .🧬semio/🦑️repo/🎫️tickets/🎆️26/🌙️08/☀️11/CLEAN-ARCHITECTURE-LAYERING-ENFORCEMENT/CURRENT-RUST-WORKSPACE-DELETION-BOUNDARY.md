# Current Rust Workspace Deletion Boundary

The full root manifest and 63 General package manifests are retained in current-cargo-deletion-boundary-1.json. Current root explicit Specific/Hub workspace members: 0. Current General package dependency origins into product/Specific/Hub paths: 7 (normal,development,build and conditional sections are distinguished in evidence).



- `🧰️framework/📦️packages/🦀️rust/Cargo.toml` dependencies / semio-framework-os-kernel → `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust/Cargo.toml` dependencies / semio-framework-os-kernel → `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` dependencies / semio-framework-artifact-infinite-dag → `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` dependencies / infinite_canvas → `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust/Cargo.toml` dependencies / semio-framework-os-kernel → `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml` dependencies / semio-framework-os-kernel → `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust/Cargo.toml` dependencies / infinite_canvas → `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust`

Physical deletion cannot be declared safe from this static census: explicit absent workspace members require workspace ownership repair, and inherited path bindings used by General require defining provider cuts. This report identifies concrete current configuration obligations, without executing Cargo or inferring whole resolution/runtime failure or any proposal equivalence.


Current bounded root workspace observation: actual root has118 explicit members and no ✏️s/🌎️hub members. Editor package.workspace resolves physically to repository root, not a separate General workspace. Direct root-member normal/dev/build dependency table inspection finds no Specific/Hub path edges in that slice. This does not prove physical deletion safety: conditional target dependencies, computed source/runtime reads, nested manifests and transitive Product edges still need complete closure qualification. Editor presently retains direct OSKernel and infinite-canvas dependencies; those exact current manifests are retained under generated/cargo-deletion-boundary/current-root-frames-1.json. No Cargo or deletion experiment performed.


Refreshed actual unconditional normal General→Product Cargo slice has six edges, not a stable assumed seven: ToolRun→OSKernel; Editor→OSKernel/infiniteCanvas; Surface→OSKernel/infiniteCanvas/artifactInfiniteDag. Full manifest frames and target paths are retained in current-general-normal-product-edges-1.json. ToolRun defining component at14 imports OS os_pack body encode/decode and options, while all current value derives already use General Value. The staged Record cohort11 already changes that precise binding to ::pack::record; do not duplicate/drop its owned work or infer it is live. Editor codec cut is separately documented; Canvas/DAG package and provider cuts remain High proposals, not runtime. Surface glue still aliases OSKernel as store. This bounded inventory excludes conditional-target and absent/nonmember manifests, so it does not prove delete-safe workspace closure.
