# A2–A4 State Infrastructure

## Scope

This workstream audited and repaired shared framework/OS state infrastructure only. Plugin app
files and the concurrent first-class hover/selection ticket were left untouched.

## Findings

1. PresenceStore and TransientStore were implemented, but the document host exposed only raw
   ArtifactActorMsg::PresenceHeartbeat sends. Every caller could choose its own cadence and flood
   the loss-tolerant lane; there was no host-owned ten-hertz producer/coalescer.
2. Module-local transient helpers were four unrelated process-global maps. Their values had the
   right lifetime but no owning OS transient authority, so a shell/runtime could neither isolate
   nor reset its lane.
3. Named layouts, dock layouts, dock UI state, and window-pane state persisted as four independent
   key-value documents. A single shell configuration had no typed persisted-local authority.
4. The native/wgpu renderer's PREFS_STORE wrote every chrome preference as an independent key,
   outside the shared OS config authority.
5. The app channel's Rust implementation had reached v6 while its TypeScript host twin remained
   at v4. Typed guest presence/transient generations had no object-safe or wire-visible path.
6. React's existing presence action never reached the document actor, and native wgpu never
   mounted a heartbeat producer in its render loop.
7. The OS-kernel and OS-host Nx targets failed before Cargo started because their package scripts
   used stale repo-library imports; the kernel script also depended on a removed `createScript`
   export.

## Repair

- Added PresenceHeartbeatProducer with a host-wide default minimum interval of 100 ms.
  ArtifactHost::presence_heartbeat owns one producer per open document, publishes the first
  complete peer snapshot immediately, and coalesces faster cursor/viewport/app-presence offers to
  the newest snapshot.
- Added OsTransient, the explicit no-persistence/no-history/no-sync owner for boxes, maps, sets,
  and weak maps. Existing module helpers now delegate to the default authority; isolated runtimes
  can create and reset their own authority.
- Added the versioned OsShellConfigSnapshot and OsShellConfig config-lane adapter. The four
  existing store projections now update one semio.os.config document and preserve sibling
  projections on every write.
- Added the preferences projection to the same config document. Native/browser wgpu PREFS_STORE
  now reads and updates only semio.os.config, preserving layout/dock/window siblings.
- Advanced both app-channel codecs to v7. The missing v5 PureCommand/Emit/Draft and v6 child
  frames are now implemented by the TS twin, and v7 Ephemeral carries the typed ArtifactPack
  presence plus presence/transient generations. VcsArtifactApp exposes that snapshot through its
  object-safe PluginApp contract and plugin_exchange emits it on every drain.
- Mounted React heartbeats over the backbone worker for every open document, carrying the latest
  pointer/viewport and guest presence pack. Mounted the native wgpu render loop onto the same
  ArtifactHost producer through ProgramBridge's channel drain.
- Removed the stale app-level `AppDefinition.actions` initializer from the OS host's generated app
  definition path. Synthetic host windows retain the window-owned action collection; existing host
  fixtures now use the current window/app interaction fields as well.
- Repaired the existing OS-kernel and OS-host package scripts to use the repository's current
  `BundleScript`/`ScriptRouter` API and correct repo-library paths.
- Reconciled the native renderer with the current shared architecture: enabled the kernel sync
  feature only for native targets (keeping Tokio net/mio out of wasm), imported graph/map hosts
  from their owned modules, migrated operation fields to
  mutations, moved actions/interactions to their current manifest owners, propagated draw clips,
  handled interaction actions, used the manifest platform type, mounted current sync-host types,
  and replaced deleted map push-setters with render-time interaction synchronization.
- Reconciled the shared UI WGPU raster pass lifetimes so the silhouette mask borrows its renderer
  and frame buffers for the render-pass lifetime without copying or rebuilding resources.
- Extended existing Rust and TypeScript test modules; no new production/test source files were
  introduced.

## Files

- `🧰️framework/🔨️modules/🎠️kernel/🟦️component.ts`
- `🧰️framework/🔨️modules/🖥️platform/🟦️component.ts`
- `🧰️framework/📦️packages/🟦️typescript/🟦️glue.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🟦️component.ts`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/PluginRuntime/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ShellHost/🟦️component.tsx`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/ProgramBridge/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Shell/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/EngineCanvas/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Scenes/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Dock/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/Interpreter/🧊️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/Cargo.toml`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs`
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts`
- `🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts`

## Verification

- bun nx run @semio-tech/framework:test --skip-nx-cache: **150/150 passed**.
- git diff --check: **passed**.
- Focused sync-feature Rust heartbeat tests: **2/2 passed**.
- bun nx run @semio-tech/framework-os-kernel:check --skip-nx-cache with the ticket-local
  CARGO_TARGET_DIR: **passed** in 1m 08s.
- bun nx run @semio-tech/framework-os-host-rs:check --skip-nx-cache with the ticket-local
  CARGO_TARGET_DIR: **passed** in 7.07s.
- Direct ticket-local framework OS host cargo check --lib: **passed** in 5m 53s.
- The app-conformance lane subsequently confirmed Puzzle compiled through the repaired shared host
  and completed its isolated matrix run; Space also passed the repaired host compile stage.
- Framework OS TypeScript suite: **214/218 passed**. The four failures are unrelated missing
  generated inputs (two absent binary wire fixtures, duplicated by Vitest discovery, and two
  absent generated workflow wasm imports); every app-channel codec test passed.
- React renderer long-suite budget result and direct WGPU validation are recorded in
  scratch-a2-a4-verification.txt.
- React renderer lint: **passed**. Its focused `framework plugin runtime` Vitest filter also passed:
  the final rerun was **7/7 passed**, 307 skipped, in 19.47s. No heartbeat-specific React test
  exists in the current test module, so the 300-second full long target was not repeated.
- Focused ticket-local sync heartbeat rerun: **2/2 passed**, 885 filtered out, Cargo exit 0.
- Ticket-isolated `cargo check --manifest-path <wgpu>/Cargo.toml --tests --message-format=short`:
  **passed with exit 0 and zero errors**. After the manifest's final target-specific sync refinement,
  the native confirmation finished in 10.17s. Repair iterations reduced the architecture-drift set
  from 54 to 39 to 14 to 2 to 0 errors. All Cargo validation used only the ticket-local
  `🎯️target-a2-a4` and did not contend with other lanes.

## Remaining

- Generate the missing framework-OS binary wire fixtures and workflow wasm bundle so its complete
  Vitest suite can be green in a clean checkout.
- Presence interaction-domain assembly remains host-side Rust functionality; the React mount
  currently sends typed app presence plus cursor/viewport and leaves optional interaction absent.
- Full native window launch and visual interaction remain part of the runtime/app matrix; this lane
  establishes that the shared native renderer, heartbeat/config mounts, and tests compile together.
