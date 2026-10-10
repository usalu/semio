# Selected Preparation and Launch JSONC Validation

2026-10-09 read-only inspection. No Cargo, synchronization, preparation, builds, cache cleanup or process changes were executed.

## Genuine Preparation Owner

The legal lock-refresh operation is `bun nx run @semio-tech/cargo-workspaces:synchronize -- --manifest <selected-owner> --package semio-framework-os-kernel`, retaining the original caller's SEMIO_CARGO_PREPARATION_STORAGE environment. The original Core snapshot-receiving route in ticket `📥️inputs/canonical-schema-law-owner/📜️script.ts:1903-1910` selects `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`, package semio-framework-os-kernel, and its actual package `test-native long io::control::tests:: -- --nocapture` command. Resolve the selected manifest through the existing cargoWorkspacePreparationInvocationV1 rather than guessing its workspace owner or losing its package selection.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts:318-328` derives the workspace manifest and explicit package selectors from the actual native Cargo invocation and emits the preparation subscript's synchronize command with inherited storage and NX_WORKSPACE_ROOT. Replaying the original native package GUI route automatically invokes this owner; no manual lock repair is required. The actual synchronize owner (preparation 📜️script.ts:40-52) acquires the original exclusive cargo-preparation lease and calls withPreparedCargoDependencyPairV1. Its update --workspace operation intentionally owns lock mutation, followed by fetch --locked and lock/input custody checks. Final native execution remains locked. `prepare` alone only prepares recipes/membership and does not own this update/fetch pair.

The Nx target is verified in cargo/📋️project.json: synchronize is uncached, forwardAllArgs true, and calls the respective 📜️script.ts synchronize. The main Cargo script explicitly registers SynchronizeScript. Existing GUI configuration in both launch files is named `🧰️framework 🦑️repo 🦀️workspace 🔄️synchronize`; its command is `bun nx run @semio-tech/cargo-workspaces:synchronize -- --manifest "${input:cargoSynchronizationManifest}"`. The input is a Cargo owner manifest prompt defaulting to Cargo.toml. This generic GUI target has no package input and therefore does not by itself preserve the original selected package set. Prefer the original native selected route's automatic owner invocation for exact custody, or supply explicit original --package through the existing Nx synchronize target. Do not silently broaden the operation to all workspace packages.

The logged locked failure remains pre-compilation evidence, with no new Core Native assertions. An original selected synchronize replay owns legitimate regeneration; if its subsequent locked fetch refuses again, inspect authentic preparation argv/lock/input observations. Do not bypass locked policy or overwrite lock manually.

## Existing Third-Party Parser Check

Executed only a read-only Bun inline command importing installed jsonc-parser (bun.lock version 3.3.1), reading both files and parsing with allowTrailingComma:true. Both returned errors:[]:

| File | Configurations | Inputs | JSONC errors |
|---|---:|---:|---:|
| .vscode/launch.json | 6782 | 97 | 0 |
| .vscode/🧩️launch.seed.jsonc | 4994 | 37 | 0 |

This is actual syntax validation of the files as read, not launch execution, semantic command qualification, or proof they remain unchanged during concurrent edits. A later read extracted the synchronize name/command/input successfully with the same third-party parser. No source or lock mutation was made by this audit.
