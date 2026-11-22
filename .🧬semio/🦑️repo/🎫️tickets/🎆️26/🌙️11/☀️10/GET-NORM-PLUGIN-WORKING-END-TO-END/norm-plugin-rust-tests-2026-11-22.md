# Norm plugin Rust tests, 2026-11-22

## Result
`@semio-tech/norm-plugin:test` does NOT pass yet. The Cargo resolution failure is gone, the stale Norm test code is fixed, and the run is now blocked by a concurrent framework migration outside Norm.

## Causes and fixes

1. Cargo preparation failure (`cargo:selected-dependencies failed (101)`).
   - The first lines of Cargo's error never reached us. The captured child output was only the last two `... which satisfies path dependency` lines, which is the tail of a "failed to select a version" / out-of-date-lock chain.
   - Another agent had added `semio-framework-value` to four stdio extension manifests (media, binary, mesh, image) after the hub `Cargo.lock` was last written.
   - By the time of the retry the lock was current (hub `Cargo.lock` rewritten at 04:31 by repo preparation) and `metadata --locked --offline` succeeded.
   - `repo dependencies lock --all` only regenerated the JS store locks. It did not touch any `Cargo.lock`.
   - A transient `ENOENT .../🔏️index/🧊️stamps` came from another agent mid-edit in the repo SDK source index. It resolved itself.

2. Error-message truncation.
   - `prepareCargoWorkspaceInvocation` kept only `.slice(-2048)` of the output.
   - New `boundCargoDiagnosticV1(text, head=2048, tail=2048)` in the custody module keeps the head, an omitted-characters marker and the tail. The head is where Cargo's `error:` line and first causes sit.
   - Test added: "bounded Cargo diagnostics keep the error head and the tail".
   - That test was not run: `@semio-tech/cargo-workspaces` has no `test` target. Verify it with the owner's test target.
   - Remaining gap: the repo captures a child's stderr as the last 64 KiB (`slice(-65536)`) in `📥️capture/🟦️.ts`. A very long Cargo error can still lose its head before it reaches the bounded formatter.

3. Stale Norm tests (`semio-hub-norm` lib test: 44 compile errors).
   - The surface law called `assert_viewer_never_mutates`, which only accepts viewers with `NoTransient`. Norm viewers now carry `NormLaneTransient`. The law is also type-guaranteed, because viewer dispatch returns `ViewEmit`.
   - The law call is removed. The editor/viewer dialect law stays.
   - The `set_active_example_case` tests referenced a deleted per-family `editor::<family>::commands::set_active_example` command module. `setActiveExample` is now a framework action driven by `ArtifactEditor::examples()`.
   - Those cases are removed. The roster law now loads every example, plus the empty id and an unknown id, through `semio_s_plugin_norm_app_surface::roster_example_snapshot`. It keeps the check that the picker roster equals the example modules on disk.

4. Current blocker (not Norm).
   - Another agent is replacing the framework window-layout API (`WindowLayoutRoot`, `WindowLayoutStackNode`, `WindowLayoutWindowNode`, `WindowLayoutChild` and `WindowLayoutAxisNode` become `WindowLayoutNode` and `Axis`). Files under `🧰️framework/🔨️modules/🖱️ui/...📐️layout` and `🔌️plugin/🦀️.rs` were changing at 04:39.
   - `semio-s-plugin-norm-app-surface-foundation` and `-viewport3d` fail with E0432 on these old names. 8 Norm app-surface files still use them, among them `📋️prelude`, `🏷️ids-chrome-and-labels` and `🪟️manifest-and-media`.
   - I did not migrate these, to avoid racing that agent.
   - Once the framework migration settles, Norm's app-surface must be ported to `WindowLayoutNode::Window { window_kind_id, .. }` and the new stack form.

Not reached yet: the EN 1998 `seismic-rc-frame` golden twin and the DIN/ISO/VDI example tests. No `SEMIO_UPDATE_GOLDEN` regeneration was done. The empty `cases` tables were left alone.

## Changed files
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🟦️.ts`
- `🌎️hub/🧩️compositions/📕️norm/🧪️tests/🧰️laws/🔬️surface/🦀️.rs`
- `🌎️hub/🧩️compositions/📕️norm/🧪️tests/🔬️surface/🦀️.rs`

Logs are in `🗑️generated/norm-plugin-test-2026-11-22*.log`.

## Rebuild
No browser component needs a rebuild from these changes. They touch only repo tooling and Rust test code.
