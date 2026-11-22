# Activation build blockers (2026-11-22)

Goal: Rust/wasm build closure of the procedural 3d playground, plus `fit_graph_action_persists_the_fitted_camera_like_a_wheel_zoom`. All builds through `repo cargo` (shared store, prototype).

## Blockers and state

1. `semio-ui-theme` (11 x E0425, `chrome_metrics::TRANSITION_DURATION_MS` etc.)
   - Cause: peer edit in flight. Constants `TRANSITION_*`, `DIALOG_ENTER_*` live in `semio_ui_styling::metrics::chrome` (`🎨️styling/🔤️tokens/🦀️.rs`); the peer finished adding them. No edit by me.
   - Result: `repo cargo check -p semio-ui-theme --lib` passes.

2. Source-size overruns (limit 262144 bytes / 4000 active lines per crate closure)
   - `semio-ui-wgpu-paint`, `semio-plugin-runtime-ops`: `check --lib` of both passes admission (stale at the time of the report; a peer reduced them). No edit by me.
   - `semio-renderer-wgpu-shell-view-chrome`: `check --lib` passes, but the TEST closure (`cfg(test)`, built by the wgpu test) is refused: 4159/4000 active lines.
   - `semio-renderer-wgpu-shell-actions`: refused earlier at 4026/4000 (peer adding icon-import, `🖼️IconImport` region, plus theme-actions edits; files modified minutes earlier).
   - NOT fixed. Cause is real source growth, not embedded fixtures, in monolithic `impl ChromeViewCtx` / `impl ActionsCtx` staging modules (largest: `📥️staging/🍱️view-chrome/🧭️navbar-footer-and-overlays` 1166 lines, `📥️staging/🔀️actions/📨️command-dispatch/📇️directory-and-hub` 688 lines incl. the 270-line `handle_hub_workspace_action`). A cross-crate split is blocked by the orphan rule (inherent impls on a context type owned by the crate itself), so it needs a design step: either move the hub-workspace and tutorial-player members to free functions over `&mut DirectoryState`/`&mut ChromeState` in a new crate, or lower the ctx type into its own crate. Both crates are being edited by peers right now (mtimes 05:04 to 05:38 on staging files), so a blind split would be overwritten or collide.

3. Closure errors found in `generation3d --workspace` check (not the wasm lib closure)
   - `semio-renderer-wgpu-kernel-pool`: `Payload::Cancel(0)` where `Payload::Cancel` is now a struct variant. FIXED: `🏊️kernel-pool/🎛️runtime-observation/🦀️.rs` now `Payload::Cancel { seq: 0 }` (the doc comment still mentions the old shape; harmless).
   - `semio-s-artifact-procedural-3d-retained-commands` (150 errors) and `semio-s-artifact-procedural-3d-testkit` (3 errors): downstream of the framework retained-allocation refactor (`RetainedAllocationEffect` replacing `RetainedCloneGrant`, u64 -> u128 work counters, `RetainedWorkPoll`, removed `*_borrowed_original` methods, `OriginalJobFixtureCaller` no longer in the `semio_framework_job` root, `semio_plugin::artifact_app_laws` gated by `artifact-app-testing`). The crates were untouched for 10+ hours; framework job/actor files changed in the last 24 h. Only `retained-source-laws` (a law crate) depends on retained-commands; testkit is a dev-dependency. Neither is in the playground wasm lib closure. Needs a dedicated port lane after the framework API settles.

## Checks and tests

| Command | Result |
|---|---|
| `check -p semio-ui-theme --lib` | pass |
| `check -p semio-ui-wgpu-paint -p semio-renderer-wgpu-shell-view-chrome -p semio-plugin-runtime-ops --lib` | pass after the kernel-pool fix (first run failed on kernel-pool E0533) |
| `check --manifest-path generation3d/Cargo.toml --workspace` | FAIL: only retained-commands and testkit (above); no errors in the generation3d product crates |
| `test -p semio-renderer-wgpu-engine-attach-tests-2 -- fit_graph_action_persists` | NOT RUN: refused at source admission, `shell-view-chrome` 4159/4000 (earlier `shell-actions` 4026/4000) |

The fit test is therefore still unverified. Next step once a peer or a design-led split brings both crates under 4000: rerun the test command above.

## Source-budget split of the two wgpu shell crates (2026-11-22)

Both crates sat above the 4000 active-line / 262144 byte Rust source budget. The `impl ChromeViewCtx` / `impl ActionsCtx` members were split by call-graph seam into two new crates that sit ABOVE the crate they were taken from (an inherent impl cannot live in a second crate, so each new crate owns its own borrowed-split ctx, the established `bundle` pattern with an `as_*` bridge back). Nothing was joined, nothing raised.

| Crate | Before (lib / test lines) | After (lib / test lines) | Bytes after (test) |
|---|---|---|---|
| `semio-renderer-wgpu-shell-view-chrome` | 4049 / 4283 (4159 admission) | 3286 / 3520 | 231410 |
| `semio-renderer-wgpu-shell-actions` | 3981 / 3984 (4026 admission) | 3633 / 3636 | 237627 |
| new `semio-renderer-wgpu-shell-view-chrome-overlays` | | 900 / 900 | 55345 |
| new `semio-renderer-wgpu-shell-frame-pumps` | | 519 / 519 | 29649 |

(Lines measured with the repo's own `measureRustCrate` closure counter, macOS host cfg; the test target of view-chrome also exceeded the byte limit, 280082 > 262144, now 231410.)

### What moved
- `🪧️shell-view-chrome-overlays` (ctx `ChromeOverlaysCtx`, staging `📥️staging/🪧️view-chrome-overlays/🧭️overlay-steps`): from the 761-line tail of `view-chrome/🧭️navbar-footer-and-overlays`: tutorial bar (+ chapter ticks), context menu, chrome tooltip, agent chat header, transient notice, plugin install, document opening, status stacks (`render_*_step`, `status_stack_cells`, `apply_status_action`). Only external callers existed (`view-panels` `render_*` frame, `view-overlays` `render_overlay_step`, one Shell law test), so the crate sits above `view-chrome` and reaches `anchor_rect` through `as_chrome_view()`. Callers use the new `as_chrome_overlays()` bridge of `OverlaysViewCtx` and `PanelsViewCtx`.
- `🚰️shell-frame-pumps` (ctx `FramePumpsCtx`, staging `📥️staging/🚰️frame-pumps/🧭️pumps`): the frame-driven roots of `actions`: `pump_directory_events` (native and wasm), `pump_media_actions`, `advance_document_opening`, `poll_auto_checkin`, `drain_operation_publications`, `reattach_remembered_local_folder`. These are only called from `shell-pointer` (`pump_sync_events`, now `as_frame_pumps()`) and two Shell law files; they reach the rest of the actions through `as_actions()`. `finish_native_open` became `pub`; the empty `⏪️time-travel` staging module was removed.
- Shell tests updated for the new ctx: `tour-overlay-and-chord-glyphs` (`ChromeOverlaysCtx`), `hub-projection-workspace` and `wgpu-time-travel` (`FramePumpsCtx`); the three owning test crates got the dependency.
- Registrations: root and `🧰️framework` workspace members, `📋️build-owners.json`, both `📋️project.json`, wasm compiler fixture `🔣️.json`, `🧯️test-support/📜️repo-text` `SHELL_SOURCE_ROOTS`, dependents' `Cargo.toml` (pointer, view-overlays, view-panels, tests-input-3, tests-docs-1, tests-docs-2; `testkit` feature forwarding). The new crates have no external dependencies, so `🔒️dependencies.json` needs no row.

### Collateral fixes needed to compile (not mine, minimal)
- `window-actions/📎️local-folders/📥️restore.rs`: missing `use std::future::Future;` (E0405, peer edit of 06:54).
- `view-panels` Cargo: missing `semio-renderer-wgpu-engine-board` dependency for `paint2d_keyboard_announcements` (peer edit 06:36).

### Verification (`repo cargo`, shared store, prototype)
- `check -p actions -p frame-pumps -p pointer -p view-chrome -p view-chrome-overlays -p view-overlays -p view-panels --all-targets --features <view-chrome|overlays|view-overlays|view-panels|pointer>/testkit`: Finished, no errors; source admission passes for all (no boundary refusal). Without `--features ...testkit`, `view-chrome` lib-test alone fails (`shell-async` testkit items); that is the existing feature gating, not the split.
- `test -p actions -p view-chrome -p view-chrome-overlays -p frame-pumps --features ...testkit`: view-chrome 6 passed, 0 failed; actions, overlays, pumps 0 tests, 0 failed.
- `test -p semio-renderer-wgpu-engine-attach-tests-2 -- fit_graph_action_persists`: source admission passes and the whole shell chain compiles, but the test crate itself does not compile: 29 errors in `⚙️EngineCanvas/🧪️tests` (`🕸️wgpu-node-graph`, `🫳️node-graph-gestures`, `🖌️wgpu-paint2d-engine`): `DagHost.host_snapshot`, `DagHost::screen_hit`, `DagScreenHit`, `GraphHost::selected_node_ids_json`, `paint2d_pointer_cancel_into`, `NodeGraphHover.port_ids`, crates `ui_gpu` / `semio_os_infinite_board_dag_graph` unresolved. Test/engine API drift from the concurrent dag/graph host work, unrelated to this split. The fit test is still NOT RUN.
- `check --tests` of `shell-tests-input-3`, `shell-tests-docs-1`, `shell-tests-docs-2`: fail with errors that exist independent of the split (`operation_publications_from_json` missing in program-bridge, `take_media_actions`/`push_media_action`/`pinned_app_for`/`relay_open_plan` unresolved, `Ctx::of(&mut shell)` on an `&mut` binding for untouched `SessionCtx`/`ActionsCtx` calls); the changed call sites use the same pattern as their untouched neighbours and add no new error kind.
- Environment: several attempts died on transient `couldn't find file`/`couldn't read` for registry sources and build outputs (`memmap2`, `errno`, `mio`, `serde_derive_internals`, `wgpu blit.wgsl`, `cranelift isle_numerics.rs`), files that exist afterwards; reruns resumed from the shared store.
- wasm: `check --target wasm32-unknown-unknown -p semio-renderer-wgpu-shell-frame-pumps -p semio-renderer-wgpu-shell-view-chrome-overlays`: Finished (exit 0), so the moved `cfg(target_arch = "wasm32")` pump members and imports resolve.
