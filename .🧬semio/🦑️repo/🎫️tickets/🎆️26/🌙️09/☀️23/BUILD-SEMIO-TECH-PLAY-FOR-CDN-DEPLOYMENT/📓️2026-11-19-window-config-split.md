# Window config crate split under the source ceiling

`semio-plugin-window-config` read 18 files / 262,467 bytes (ceiling 262,144). The modules `🗄️store-owners`, `📥️retained`, `🧬️preview` and `🧭️owner-registry` form one mutually dependent cluster (all use `super::*`, retained and store-owners reference each other, every one uses the owner registry), so none of them can leave without a cycle. The only self-contained lower layer is `📐️owner-contract-and-mutation` (owner trait, mutation, snapshot, native pair receiving) plus `🔏️pack-authority-and-publication` (pack, authority, publication); the latter uses only the former.

## Change
- New crate `semio-plugin-window-config-owner` at `🪟️window/🎚️config/📐️owner-contract-and-mutation/📦️packages/🦀️rust` (Cargo.toml, 📋️project.json), lib root = the existing `📐️owner-contract-and-mutation/🦀️.rs`; pack-authority is mounted by `#[path]` from that root. No source file moved or reformatted; only the root header (imports that `use super::*` used to supply) changed.
- Parent crate: dependency added, the two `#[path]` modules replaced by explicit `pub use semio_plugin_window_config_owner::{...}` of the same names (public API unchanged except the hidden `owner_contract_and_mutation` / `pack_authority_and_publication` module paths); three internal `use crate::owner_contract_and_mutation::X` became `use crate::X`.
- Registered in root `Cargo.toml`, `🧰️framework/Cargo.toml` members and `📋️build-owners.json`.
- Result: parent closure 262,467 -> about 231 KB; new crate about 31 KB.

Helper: `window-config-closure.ts` prints a crate's source closure with sizes.

## Verification
- Preflight: no OVER lines; parent closure 230,882 bytes / 14 files.
- `repo cargo check` (release) passed for the new crate, the parent `semio-plugin-window-config` and the dependent `⏪️time-travel` crate.

## Outcome note
While verifying, a concurrent fleet agent landed the same split as `semio-plugin-window-config-contract` (`🪟️window/🎚️config/📜️contract`), moving the same two modules. That supersedes my `semio-plugin-window-config-owner` crate, so I removed my now-stale remnant (`📐️owner-contract-and-mutation/📦️packages` stub and its two workspace-member lines). My check results above were taken before that move; the old/new crate checks were not re-run against the `📜️contract` layout. Open item (not mine): the in-progress `🔏️pack-authority-and-publication/📦️source/📦️packages/🦀️rust` crate currently makes the preflight report `missing-input` because its source file is not there yet.
