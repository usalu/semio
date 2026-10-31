# Current Cargo Local Path Audit

Read-only parsed-manifest audit at 2026-10-29T18:24:49.846446+00:00. No Cargo/compiler/native/test/daemon/lease process was launched. Python standard-library tomllib parsed current authored TOML; no dependency was added.

## Result

Inspected **3645 Cargo manifests** and **46687 explicit local path dependency edges**, including ordinary/dev/build, workspace.dependencies and target-specific variants. No TOML parse failures and no existing-target package-name mismatch were found. Exactly **one missing local referent** was found.

Enumeration excluded policy-generated directories, hidden infrastructure, symlink directories/manifests and reserved preservation trees. Workspace-inherited edges are represented by their actual workspace.dependencies path declaration rather than counted once per inheritor. Package feature or external registry resolution was not evaluated.

## Concrete Blocking Edge

- Consumer: 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🛠️tool-machine/⌨️typing/✍️edit/📦️packages/🦀️rust/Cargo.toml:25
- Dependency: semio-framework-tool-typing-edit
- Missing target: 🧰️framework/🔨️modules/🛠️tool-machine/⌨️typing/✍️edit/📦️packages/🦀️rust/Cargo.toml
- Expected package name: semio-framework-tool-typing-edit
- Existing same-name package candidates: [].
- Target existence rechecked after census: False.

The domain Source already exists at framework tool-machine/typing/edit/🦀️.rs with corresponding neutral JSON and Rust tests, but no package adapter Cargo.toml exists at the referenced packages/rust endpoint. The OS plugin imports the new typing-edit crate, so changing only the dependency name to the parent tool-machine package is not justified by this audit. No unambiguous existing canonical replacement package exists.

Smallest clean fix is for the owning typing-edit implementation to complete its intended Rust package adapter and exact local dependencies/tests, or deliberately refactor the domain and all consumers together. Do not add an alias/legacy adapter or delete the dependency just to advance exporter. Package Source ownership and compilation remain the owning peer's responsibility.

This is a proven missing authored referent at two observations, not proof that a peer is inactive or that the incomplete adapter is permanent. Coordinate the owner before editing the area; no other broken local edge was found to prioritize in parallel.

## Evidence

Raw census is ticket-generated cargo-current-local-paths.json (temporary generated evidence only). The prior actual73042 failure was supplied by the coordinator; this audit independently corroborates the missing dependency in current Source, without rerunning the exporter. Recheck the same missing endpoint and manifest graph after owner completion before launching the one existing canonical exporter route.
