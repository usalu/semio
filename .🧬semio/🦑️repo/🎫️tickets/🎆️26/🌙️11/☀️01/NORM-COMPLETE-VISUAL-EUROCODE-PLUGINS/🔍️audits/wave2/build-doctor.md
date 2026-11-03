# Build doctor: Cargo member authority

Authority = a crate manifest is found by `membersPlan` of its owner workspace: `package.workspace` must resolve to the owner root, the path must match the owner's `member-manifests` glob (`**/📦️packages/🦀️rust/Cargo.toml` for norm), and not be excluded. Path dependencies must hit such a member.

Root cause: ndp-model had `workspace` with 7 `..` instead of 6 (resolved to `✏️s/🔌️plugins`). Sync then also dropped it from `📕️norm/Cargo.toml` members; the ten tables and the facade were fine.

Fixes: corrected the key in `ndp/🧱️model/📦️packages/🦀️rust/Cargo.toml`; re-added the member line in `📕️norm/Cargo.toml`.

Check: `🧰️scripts/check-cargo-authority.py` validates workspace key, members, glob, unique names, workspace inheritance, lib path and path deps for all 28 norm crates (excluding artifacts/bridge): 0 errors. `repo ./📜️script.ts verify build-owners sync` passed. No `📋️project.json` is needed for ndp crates (siblings like compliance-core have none).
Ready markers appended to integrate and build-en* latest.md.
