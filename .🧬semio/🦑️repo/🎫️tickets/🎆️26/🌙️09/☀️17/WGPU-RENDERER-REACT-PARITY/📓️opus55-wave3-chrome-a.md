# Wave 3 chrome A: titlebar, close guard, session-authority notice, OS file drop

Source: `📓️opus55-reaudit-chrome.md` open items 27 and the "New React chrome features" table. Each item verified against React first.

## 1. Desktop titlebar (item 27): VERIFIED NOT APPLICABLE

- `DesktopTitlebar` (`🖱️ui/🎯️targets/⚛️react/🖥️fullscreen/🟦️.tsx`) is exported and used only by `📖️stories/🎭️desktop-titlebar/🧪️.story.tsx`. No React OS shell mounts it (`ShellHost`, `Shell`, and `FrameworkOsShell` never reference it). Its own header says consumers omit `controls` on non-Electron hosts.
- The React OS shell therefore has NO titlebar; the auditor compared against a story-only component. Adding one to wgpu would break parity, so nothing was added.
- Fullscreen behaviour already matches React on both hosts: shell command `os.toggleFullscreen` (+ navbar toggle `ui.fullscreen.toggle`) sets `fullscreen_toggle_requested`; the present step (`🖥️frame-presentation/🎬️present-step-inner`) calls winit `set_fullscreen(Borderless)` natively and the canvas Fullscreen API on wasm; the browser host (`🌐️browser-host`, `wireBrowserFullscreen`) applies the directive.
- Native window drag: the OS shell uses OS decorations (no custom titlebar), so winit drag APIs are not needed.

## 2. Close guard and unsaved marker: IMPLEMENTED, mirrors React

- Saved state = React `deriveSavedStateV1` exactly (`SyncState::saved_state`: none = unknown; saved iff `persisted && pending == 0 && no write failure`). The earlier divergence is gone; the attach seed `Some(ArtifactSyncStatus::default())` became `None` (React's "no status yet").
- Unload predicate (host, native close and browser `beforeunload`) = React `unload`: document with an artifact schema whose store attachment is missing or not acknowledged saved (`publish_host_unsaved`).
- Native close flow = React `requestCloseDocument`: held `CloseRequested` -> "saving" dialog (only Cancel, “Saving “name” before closing…”) for up to 10 s (`SHELL_CLOSE_FLUSH_TIMEOUT_MS`) -> closes by itself once saved, else confirm dialog (body names the document id and appends the typed save failure, en/de, `shell_save_failure_text`) with Keep editing (focused, Escape, veil) and Close and discard (`os.closeDiscard`). `ShellCloseGuard` lives in `SyncState.close_guard`; the dialog is `ChromeDialogRequest::confirmation` (empty confirm action hides the Confirm stop); dialog bodies now wrap.
- Per-window `UnsavedMarker`: `paint_window_unsaved_marker_step` (status role, dot plus failure text, "Unsaved changes" name), painted after the pane chips and time-travel indicator at the bottom-middle anchor (one row above the indicator). React passes no `onRetry`, so no Retry button exists there either; none was added.
- Browser: `unsaved` directive lane (Rust output, JSON schema, fixture, contract, transport, frame-worker, host `beforeunload`).

## 3. Session-authority notice: IMPLEMENTED

React `ShellHost` :216 mounts `SessionAuthorityNotice` while hub configured, session held, authority unverified.

- New fifth status stack `StatusStackKind::SessionAuthority` (top-right, `top-workbench right-double max-w-[28rem]`) fed from `shell_identity_resolution`: `Pending` shows "Verifying shared access…" plus a Cancel action; `HubUnavailable` shows the unavailable text without cancel. Texts are the React strings, en+de (`status.sessionAuthority.*`).
- Cancel (`STATUS_STACK_SESSION_AUTHORITY_CANCEL_CONTROL_ID`) runs `DirectoryState::clear_hub_session_owner` (cancels hub tasks, drops the client, identity and verified authority) and marks identity offline: the wgpu twin of `cancelSessionAuthorityBootstrap`.
- Role=status/alert live-region semantics are not separately mirrored (status stacks have one shared accessibility path).

## 4. OS file drop

- Native IMPLEMENTED: winit `DroppedFile` -> `note_host_file_drop` -> `OpeningState.dropped_sources` + one deferred `openArtifactFile` action; `start_native_open` takes the dropped path instead of the picker, extension-checked against the editor's declared extension; busy answers `document.open-busy`.
- Browser natural-file open on drop: NOT DONE. React's `openNaturalFile` routes the `File` through the format catalog, artifact-opening activation, program creation, bounded byte import and `keepProgramDocument`. The wgpu shell has no browser equivalent of any of those (Open File is native-only), so a drop transport would feed nothing. Needs its own lane (browser natural-file open in `🐚️Shell` + program bridge) first. The browser host currently refuses OS file drags (`dropEffect: none`) so a drop cannot navigate the page away.
- Drag-hover feedback: React paints none (its handler only sets `dropEffect = "copy"`, the OS draws the cursor), so no HoveredFile overlay is painted natively; the OS owns that feedback. The browser shows the no-drop cursor until the lane above exists.

## Tests changed

- `🧪️tests/🧫️fixtures/🧊️wgpu/🌐️browser-worker/📤️output/🔣️.json` and `…/📤️output/🧪️tests/🦀️.rs`: `unsaved` added.
- `🐚️Shell/🧪️tests/🔬️wgpu-chrome-parity-constants`: session-authority stack layout and `ALL.len() == 5`; new host-lifecycle law.

## Verification: NOT COMPILED (queue cancelled per coordinator, lane E compiles)

Run, combined:
- `check -p semio-renderer-wgpu-shell-vocab -p semio-renderer-wgpu-shell-chrome-state -p semio-renderer-wgpu-shell-doc-open -p semio-renderer-wgpu-shell-sync -p semio-renderer-wgpu-shell-state -p semio-renderer-wgpu-shell-window-actions -p semio-renderer-wgpu-shell-view-chrome -p semio-renderer-wgpu-shell-view-chrome-overlays -p semio-renderer-wgpu-shell-actions -p semio-renderer-wgpu-shell-pointer` plus the winit-app crate (`🪟️winit-app`) and browser-worker crate, native and wasm32.
- `test -p` the crate hosting `🔬️wgpu-chrome-parity-constants` (check it depends on shell-window-actions), shell-vocab tests (key table), browser-worker output test; `bun test` the output TS test and a TS typecheck of browser-host, browser-frame-transport, frame-worker.
- Likely compile risks: `ShellSyncOwner.document_id` field name in `open_close_dialog`; unused `ArtifactSyncStatus` import in `🔀️actions/…/📎️sync-attach-and-open`; vocab test that pins the row count.
