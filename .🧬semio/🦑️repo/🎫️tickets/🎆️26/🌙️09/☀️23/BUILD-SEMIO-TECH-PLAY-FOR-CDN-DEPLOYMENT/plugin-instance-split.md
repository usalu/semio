# Plugin instance crate split

`semio-plugin-instance-app` (`🧵️instance/📦️packages/🦀️rust`) was refused by the source-boundary preflight at 19 files / 265,230 bytes (hard ceiling 32 files / 262,144 bytes). Its Cargo.toml also carried an invalid `maxSourceBytes = 294912` override (overrides may only tighten); it is now 262144 like its siblings.

## Change

New crate `semio-plugin-instance-members` at `🧵️instance/🪢️members` (bundle crate `MembersCtx`, rank above `instance-ladders`), following the ctx-bundle pattern of the sibling instance crates. Moved out of `🔌️plugin-app-impl` (inherent `VcsArtifactApp` impls became `impl MembersCtx` with unchanged bodies, `pub(super)` became `pub`, `InstanceExtensions::` became `X::`, `self.render_ctx()` became `self.as_render()` and so on):

- 🔀️transactions, 🗃️document-load, 🖼️render-surface, 📨️action-dispatch, 🧾️typed-operation-results, 🎞️media-transfer
- `export_media` (was inline in the `PluginApp` impl) and `loaded_with_app_dialect` (only used by document-load)
- new free fn `document_archive(stores, child_composition)` shared by the composer `PluginApp::document_archive` and media-transfer (previously `PluginApp::document_archive(self)`)
- render-surface renders the reserved sections itself from `RenderCtx` accessors; `semio_plugin_reactor_app::canonical_section_json` became `pub` (`#[doc(hidden)]`) for that

The composer keeps one-line delegations `self.members_ctx().<name>(..)`; `members_ctx()` was added to `📥️staging/🏗️ctx-builders`. `tool_public_contracts` is now inlined in the composer.

## Registration

- root `Cargo.toml` and `🧰️framework/Cargo.toml` workspace members, `📋️build-owners.json`, `📋️project.json` of the new crate, `Cargo.lock` (auto-updated by another process)
- composer `Cargo.toml` depends on the new crate
- `🧵️instance/🧪️tests/🧫️fixtures/🔣️.json`: crate `instance-members`, rank, bridges (render, dispatch, commands, time-travel-turns, ladders), moved files removed from `composerFiles`

## Sizes

Composer crate Rust source after the move: 217,821 bytes in 13 files (was 265,230 / 19 files); preflight no longer lists it. New crate about 53 KB in 8 files.

## Verification

The boundary probe shows no OVER for the instance crate. `repo cargo check -p semio-plugin-instance-members` first failed only on the issues fixed above; later runs are blocked by an unrelated infrastructure error (`Closed host capacity Policy required: /swap/activeMegabytesPerSecond is required`) from a concurrent edit of the host-capacity schema. See the final status below.

## Final status

After the fixes, `repo cargo ... check -p semio-plugin-instance-members` and `-p semio-plugin-instance-app` could not be run for about 20 minutes: every call fails before compiling with the host-capacity policy error above. The last compile that ran reported only the errors fixed afterwards. Re-run both checks once the capacity policy is repaired; expect only unused-import warnings in the new crate root (copied import prelude, to prune).

## Check outcome (01:17)

The reported E0599 on `reserved_mount_construction.terminal_is_empty()` was stale: concurrent edits had since declared the field as a plain `OriginalConstructionAdmission` (components staging), so all call sites (natural-publication, close-terminal, plugin-app-impl) were already consistent. No source edit was needed. `repo cargo --store=shared --configuration=prototype -- check -p semio-plugin-instance-members -p semio-plugin-instance-app` now exits 0 with no warnings in either crate.
