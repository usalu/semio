# Dependency Protocol 2 Current Read-Only Audit

No installer, initializer, frontend, native build, registered test or pure probe was run. This is source inspection only. No repository source was edited.

## Finding

The live callback recheck in Bun CJS checks `live` only before `await current.recheck()`. An in-flight recheck can survive callback release if its controlled advance is suspended while body and final outer recheck complete. It needs a second live check after the awaited recheck to refuse completion outside callback lifetime. Existing released-callback assertion checks invocation after release, not this concurrent schedule.

## Source Conclusions

Canonical manifest projection removes exactly top-level semio, sorts recursive object keys by UTF8 bytes and preserves array order. All other JSON fields remain. Generation frames original paths and byte lengths, includes platform/architecture/Bun version/membership and original lock/config/patch bytes. Full original source byte buffers are retained separately from canonical material, with path kind, device, inode, mode and ancestor identity; missing optional own configuration is rechecked. Root fallback config is captured only if own configuration is absent, matching the current precedence branch.

Publisher observes full selected manifest text, directory roster, physical ancestors and policy text; membership recheck replays original observations without selecting again. Owner enumeration invokes scopesPlan without packagesPlan. Preparation enters selected publisher before staging, so optional deleted package manifest refresh precedes capture. Store rechecks, publisher rechecks and source rechecks fence reuse and publication; original Store.control supplies signal and budget/progress continuation checks. Report-only publication remains separate from installation callback custody.

No `stagedProjection` or `matchesHash` assertion/field exists in the inspected dependency and workspace source. The defining hash computes canonical original material; staged root workspaces and patch paths are subsequently remapped. Current runtime assertions check metadata omission and generation equality/drift, not exact staged-byte/hash correspondence. Do not describe that correspondence as executed evidence.

Native metadata classification is by selected first-party ownership, not an explicit per-manifest semio validation: the public projection helper strips semio on every object it receives. The current closed first-party staging caller is consistent with that assumption; third-party manifest use would require an explicit boundary. Native fields (including scripts, nested semio, dependency types and exports) are not otherwise removed. The corpus contains 14 authored cases and independent SQLite/Ajv/SHA comparisons; no assertion was executed in this audit.

## Exact Current Source SHA-256

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/🟦️.ts`: `f0f30c3098f38ecde0d68a42f9c123c4fa20e1b2470fa7473be37ec5c8a6c1a6` (12680 bytes; recorded snapshot matches).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/📜️script.ts`: `7b369b28671bf3d75a521c5a72b1cae4b01fc12b9824183c9c3177e5a8795880` (4137 bytes; recorded snapshot matches).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts`: `7d12fb77b3a4dd50953b1ccc938dabe3a30c41804bf4bdb2e9710c6bf5b36787` (17680 bytes; recorded snapshot matches).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧬️schema/📥️install.json`: `da94f5e818a0ba50a7d5afe81518b11c245fae6986218332b52bd174c9673864` (1766 bytes; recorded snapshot matches).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️install.json`: `3954d08d26264dcc41ff226d886866a1fa24950d8302a7cd3834300ac931e566` (4479 bytes; recorded snapshot matches).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/🟦️.ts`: `5722f0be5ae0b2ce25fcd641aa73c0934d4ef03ab797df14d34b8380afac6d2d` (7152 bytes; recorded snapshot matches).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/💾️storage/🧪️tests/📥️runtime/🟦️.ts`: `b05fb2fde6b4a25ac923a863108c5bd1d13fb3cc8de9d96be7b9612014df90b7` (10521 bytes; recorded snapshot matches).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs`: `7ccb9ec728e3efd94ac382e3bf84b2fdab09f0e468efcadd55a285e7e9c9c3ef` (21363 bytes; recorded snapshot matches).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟦️.ts`: `41d36c8376e65bd1fb31a12abce77d960e17f2db23b4e53717866eb88271f8dc` (5527 bytes; recorded snapshot matches).
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🧬️schema/🔣️.json`: `31bf8fd6f0f6ec80318c8f3212a2a3d3f19fc96ba96c6b3e2de7444a2caa394d` (5399 bytes; recorded snapshot matches).
