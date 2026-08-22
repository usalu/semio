# Phase 5 Shell Thread-Local Extraction

## Scope

This packet owns only `renderer/engine/elements/Shell/🧊️component.rs`. It does not change renderer host glue, UI-WGPU, OS Infinite, async/services, pack/schema, or plugins.

## Inventory

### Worker-build state extracted

| Former thread-local | Classification | Owned replacement |
| --- | --- | --- |
| `FIND_ITEM_SINK` | Worker-build output | `ShellChromeBuildState.find_items`, with a scoped callback binding for the separately owned Scenes callback |
| `CONTENT_FOCUS` | Worker-build input/output | `ShellChromeBuildState.content_focus` |
| `CHROME_TOOLTIP_TITLES` | Worker-build scratch/output | `ShellChromeBuildState.tooltip_titles` |
| `CHROME_TOOLTIP_HOVER` | Worker-build interaction state | `ShellChromeBuildState.tooltip_hover` |
| `CHROME_DIALOG_STACK` | Worker-build interaction state | `ShellChromeBuildState.dialog_stack` |
| `CHROME_TOUR_STATE` | Worker-build interaction state | `ShellChromeBuildState.tour_state` |
| `CHROME_TOUR_AUTO_CONSIDERED` | Worker-build session state | `ShellChromeBuildState.tour_auto_considered` |
| `CHROME_PREV_POINTER_DOWN` | Worker-build input history | `ShellChromeBuildState.previous_pointer_down` |
| `CHROME_CLICK_EDGE` | Worker-build derived input | `ShellChromeBuildState.clicked_this_frame` |
| `CHROME_TOUR_REVEAL_LATCH` | Worker-build session state | `ShellChromeBuildState.tour_reveal_latch` |
| `CHROME_ELEMENT_RECTS` | Worker-build scratch/output | `ShellChromeBuildState.element_rects` |
| `TUTORIAL_DISPATCH_GUARD` | Worker-build control state | `ShellChromeBuildState.tutorial_dispatch_internal` |

`ChromePrefsState` is also copied into `ShellChromeBuildState.preferences` before construction so settings/theme command trees do not read ambient thread state. Introduction-seen reads and preference/panel writes are performed by the present wrapper; construction consumes explicit snapshots and emits explicit pending introduction writes.

### UI-present-only state extracted

| Former thread-local | Replacement |
| --- | --- |
| `LAST_PERSISTED_PANEL_LAYOUT` | `ShellChromePresentState.last_persisted_panel_layout` |
| `UI_PREFS_LOADED` | `ShellChromePresentState.preferences_loaded` |
| `UI_PREFS_LAST_SYNCED` | `ShellChromePresentState.last_synced_preferences` |

### Deliberate remaining thread-local sites

| Site | Classification | Reason |
| --- | --- | --- |
| `ACTIVE_FIND_ITEM_SINKS` | Scoped callback capability, no authoritative state | Scenes exposes a synchronous callback outside this packet's ownership. The binding points at an owned `Arc<Mutex<Vec<_>>>`, is scoped to one construction invocation, cannot move across threads, and an unbound callback panics instead of silently writing to an empty per-thread vector. |
| `BOOT_HUB_ENV` | wasm boot/present input | Written by the browser bootstrap before Shell construction; not read by chrome construction. |
| `PREFS_STORE` | present-side platform storage | Platform-local I/O is confined to the wrapper surrounding construction. |
| `CHROME_PREFS` | present/glue bridge | Renderer glue outside this packet still reads active theme/layout/worker preferences. Shell construction uses its owned snapshot and action handling mirrors mutations to the bridge. |

The source now contains four `thread_local!` declarations, at the four sites above. No former chrome render-state symbol remains.

## Construction boundary

`ShellState::render_chrome` is the present-side wrapper. It loads platform preferences, hydrates introduction-seen state, publishes native presence, persists panel layout, calls `render_chrome_build`, flushes introduction-seen writes, and persists changed preferences.

`render_chrome_build` consumes caller-provided renderer resources plus `ShellChromeBuildState`. Per-frame scratch maps and click-edge state are reset explicitly. Find-item collection is bound only for the scene callback and collected back into the Shell result.

## Focused coverage

- Compile-time `Send` assertion for `ShellChromeBuildState`.
- Native cross-thread move test preserving focus, tooltip, and preference state.
- Native cross-thread find-item callback test proving output is routed into the explicitly owned sink.
- Unbound find-item callback test proving there is no silent empty thread-local fallback.
- Existing focus, tooltip, dialog, tour, element-rect, panel-layout, and preference-sync tests now address owned state directly.

## Production threads

The two existing native Shell threads were not folded into this packet:

- The directory stream deliberately drives a `?Send` future using the shared directory runtime's local `block_on`; a Send worker pool cannot host it without changing directory transport/service contracts outside Shell ownership.
- Identity bootstrap isolates blocking native HTTP. Moving it to deadline-aware runtime scheduling would require the async/services ownership work already called out by its existing contract.

They are not chrome-render state and changing either would collide with excluded runtime/service surfaces.

## Verification

### Static inventory

`rg -n 'thread_local!' …/Shell/🧊️component.rs`

Result: four declarations at `ACTIVE_FIND_ITEM_SINKS`, `BOOT_HUB_ENV`, `PREFS_STORE`, and `CHROME_PREFS`.

`rg` for all removed thread-local identifiers

Result: no matches.

`git diff --check HEAD -- …/Shell/🧊️component.rs …/📓️p5-shell-thread-local-extraction.md`

Result: passed with no whitespace errors.

`rustfmt --edition 2021 --check …/Shell/🧊️component.rs`

Result: the file parsed successfully; the command reported formatting differences in this pre-existing large source file. No formatting rewrite was applied because it would produce an unrelated whole-file diff. This check was used only as a parser check.

### Native Nx test

`bun nx run @semio-tech/framework-renderer-wgpu:test-quick`

Result: failed before compiling the renderer. Concurrent `semio-framework-os-kernel` edits produced 80 async/sync mismatch errors in `os/store/🦀️component.rs`, including synchronous `decode_op` calling future-returning `ByteReader::new`, `read_command_str`, and `read_command_ops`. No Shell test result can be claimed from this run.

Retries after concurrent repairs reduced the upstream failure first to 43 errors and then to 13, but still did not reach the renderer. Remaining failures included stale `.await` calls on synchronous pack helpers in `os/store/🔄️sync/🦀️component.rs`, an actor supervisor/pool signature mismatch, and a missing `ArtifactActor::run` during the concurrent actor refactor.

### Wasm Nx build

`bun nx run @semio-tech/framework-renderer-wgpu:wasm`

Result: failed before compiling the renderer. The wasm build reached the same concurrent OS-kernel/pack transition with 10 OS-kernel errors plus a pack value encoder mismatch, then Trunk exited with status 101. No Shell wasm result can be claimed.

## 2026-08-22 Shell Compile Repair

The final Shell compile sweep found two build-phase helpers whose receivers no longer matched their owned state after the thread-local extraction:

- `render_footer` registers utility tooltips and advances footer collection expansion through `ShellChromeBuildState`.
- `render_overlay` advances tooltip/dialog/tour state through `ShellChromeBuildState`.

Both receivers are now `&mut self`. This is a build-state ownership correction only: the methods still receive draw lists, font/icon atlases, input, and theme explicitly, and no GPU, window, surface, or presentation state moves to a worker. The `ShellChromeBuildState: Send` assertion and present-side `render_chrome` wrapper remain intact.

Verification command:

`cargo check -p semio-framework-os-renderer-wgpu --message-format=short`

Result after the receiver repair: all six prior Shell mutability diagnostics at the former lines 9553, 9566, 9974, and 10030-10032 are gone. The only two remaining Shell diagnostics, at `render_ui_node` call sites 9650 and 9880, are caused by a directly upstream Interpreter contract mismatch: `render_ui_node` accepts `HashMap<String, infinite_world::World3dState>`, while Shell and the current Infinite world handlers consistently own `HashMap<String, infinite_world::world::World3dState>`. The calls already have the correct arity and argument order; the Interpreter signature must converge on the world-module state type before a green mounted renderer result can be claimed.

### Renderer lint

`bun nx run @semio-tech/framework-renderer-wgpu:lint`

Result: passed. The renderer color-literal lint reported `framework-renderer-wgpu: color-literal lint passed`, and Nx completed successfully.

## Remaining validation

- Retry `bun nx run @semio-tech/framework-renderer-wgpu:test-quick` after the OS-kernel compile gate clears.
- Retry `bun nx run @semio-tech/framework-renderer-wgpu:wasm` after the shared OS-kernel/pack gate clears.
