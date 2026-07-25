---
name: S OS Studio Navigation and Create
overview: Fix double-click studio navigation in the wgpu renderer (currently a no-operation because plugin "navigate" operations are dropped) and add a footer "Create" tool group to S-Home with Temporary / File / Folder studio-persistence kinds.
todos: []
isProject: false
---

# S OS: Working Studio Navigation + Create (Temporary / File / Folder)

Scope: **wgpu renderer only** (not the React reference shell). Two example bugs from the request map to concrete gaps found in the code:

1. Double-clicking the Demo Studio does nothing → the S plugin emits `{ "operation": "navigate", "uri": "/studios/{id}" }` but wgpu's `apply_operations` never handles `"navigate"`.
2. No footer "Create" group → S-Home's `mode_tools` are never shown because wgpu's `refresh_ui` only uses dynamic `plugin.tools()` (always empty; `SHomeApp`/`SStudioApp` don't override it), with no static fallback to `AppDefinition.modes[].tools` like the React shell has.

Confirmed with the user:

- **Folder** persistence (`.semio/studio.db` SQLite) is **native-wgpu-only** for now (SQLite crate `rusqlite` is native-only; no browser OPFS/sqlite-wasm infra exists). In the browser/wasm build, the Folder button is disabled/hidden.
- **Temporary** = pure in-memory, lost on reload. **File** = user picks/saves one standalone `.json` file (native: `rfd` dialog; wasm: existing download/upload operation flow).

Plugins (`s/plugin`) are compiled **natively** for native wgpu (`cargo build -p semio-framework-renderer-wgpu --bin semio-wgpu-native --features native-bin`, plugin built as a native cdylib loaded via `libloading`, see [framework/renderer/wgpu/script.ts](framework/renderer/wgpu/script.ts) `NativeBuildScript`) and to **wasm32** for the browser build. So `#[cfg(not(target_arch = "wasm32"))]` inside `s/plugin/rs/lib.rs` correctly gates native-only code (e.g. `rusqlite`).

---

## Phase 1 — Make navigation actually work

### 1a. Handle `"navigate"` operations in `apply_operations`

File: [framework/renderer/wgpu/rs/lib.rs](framework/renderer/wgpu/rs/lib.rs) — `apply_operations` (around line 7584), which currently branches on `setDocument` / `setPanel` / `downloadMediaExport` / `requestFileOpen` / `spawnProgram` but not `navigate`.

Add a `"navigate"` branch that records the target `uri` and, after the operations batch, calls a new `apply_shell_uri(&mut self, uri: &str)` (mirrors React's `applyShellUri`, `framework/renderer/react/os-shell.tsx:861-890`):

```rust
// inside apply_operations, alongside the other operation branches
if let Some(uri) = operation.get("operation").and_then(|v| v.as_str()).filter(|v| *v == "navigate")
    .and_then(|_| operation.get("uri")).and_then(|v| v.as_str()) {
    navigate_uri = Some(uri.to_string());
}
```

`apply_shell_uri` logic (new method on the shell state impl, next to `boot`/`refresh_ui`):

- path = `uri` before `?`; regex-free match on `^/studios/([^/]+)$`.
- No match → if current `session.app.id != S_HOME_APP_ID`, switch to Home via a new `switch_to_s_app(S_HOME_APP_ID)` helper (mirrors `switchToSApp`, `os-shell.tsx:831-858`): find app in the `s` plugin's manifest, `create_app`, build `ActiveSession`/`ViewState`/`active_window_id` from the app definition, `refresh_ui`.
- Match → `switch_to_s_app(S_PLAY_APP_ID)` if not already there, then (unless `self.open_studio_id == Some(studio_id)`) directly dispatch an `openStudio` command to the now-current studio session's instance (same pattern as `dispatch_command`, `lib.rs:7524-7581`, but against the just-created session) and `apply_operations` any resulting operations (e.g. `setDocument`). Track `open_studio_id` on `ShellState` (new field next to `uri_history`/`uri_index`, `lib.rs:6958-6959`) to avoid re-dispatch loops, mirroring React's `openStudioIdRef`.

Also call `self.push_uri(uri)` (existing helper, `lib.rs:9262-9266`) when navigating, and guard the tail of `apply_operations` (the `view_state`/`document_changed` re-attach logic at `lib.rs:7651-7659`) so it doesn't clobber the freshly-switched session with the stale pre-navigation `view_state`.

### 1b. Wire back/forward/up to actually navigate

`handle_shell_hit`'s `"ui.nav.back"` / `"ui.nav.forward"` / `"ui.nav.up"` (`lib.rs:8280-8299`) currently only mutate `uri_index`/`uri_history` without re-applying the URI. After mutating, call `self.apply_shell_uri(&self.shell_uri()).await` so Back/Forward/Up actually switch sessions (parity with the URI-driven `useEffect` in React, `os-shell.tsx:892-897`).

### 1c. Static tools fallback so footer tools render at all

File: same file, `refresh_ui` (`lib.rs:7335-7338`). After the dynamic `plugin.tools(...)` call, add the React-equivalent fallback (`os-shell.tsx:772-774`):

```rust
if self.active_tools.is_empty() {
    let active_mode_id = session.view_state.active_mode_id.clone()
        .or_else(|| session.app.default_mode_id.clone())
        .or_else(|| session.app.modes.first().map(|m| m.id.clone()));
    self.active_tools = session.app.modes.iter()
        .find(|m| Some(&m.id) == active_mode_id.as_ref())
        .map(|m| m.tools.clone())
        .unwrap_or_default();
}
```

Without this, the `mode_tools("explore", …)` declared on `create_home_app()` ([s/plugin/rs/lib.rs:2277-2283](s/plugin/rs/lib.rs)) never reaches the footer, regardless of what's added in Phase 2.

---

## Phase 2 — Footer "Create" tool group

File: [s/plugin/rs/lib.rs](s/plugin/rs/lib.rs), `create_home_app()` (line ~2271-2294). Replace the standalone `"New Studio"` button with a `tool_collection` (same pattern as `s-play.history`, `lib.rs:2338-2354`, and CAD's `tool_collection("view", …)`, `cad/plugin/rs/lib.rs:1969-1979`):

```rust
.mode_tools(
    "explore",
    vec![
        tool_collection(
            "s-home.create",
            "plus",
            "Create",
            vec![
                tool_button("s-home.create.temporary", "zap", "Temporary", s_home_cmd("createStudio", Some(json!({"kind": "temporary"})))),
                tool_button("s-home.create.file", "file-json", "File", s_home_cmd("createStudio", Some(json!({"kind": "file"})))),
                tool_button("s-home.create.folder", "folder", "Folder", s_home_cmd("createStudio", Some(json!({"kind": "folder"})))),
            ],
        ),
        tool_button("s-home.import", "upload", "Import Studio", s_home_cmd("importStudio", None)),
    ],
)
```

Collection expand/collapse and click dispatch already work generically (`render_footer_tool_nodes`, `lib.rs:9493-9640`; `handle_shell_hit`'s `framework.tool.collection.*` toggle, `lib.rs:8313-8322`) — no wgpu shell changes needed beyond Phase 1c.

---

## Phase 3 — Three persistence kinds behind `createStudio`

File: [s/plugin/rs/lib.rs](s/plugin/rs/lib.rs) `createStudio` handler (`lib.rs:1356-1372`) and [framework/product/os/core/rs/lib.rs](framework/product/os/core/rs/lib.rs).

Read `args.kind` (`"temporary" | "file" | "folder"`, default `"file"` for the existing `mod+n` keybinding). Introduce a small `OsStudioPersistenceKind` distinction and route to the right backbone `Arc<dyn OsBackbonePort>`:

### Temporary

- Add a process-lifetime `TEMP_CATALOG_PORT: LazyLock<Arc<dyn OsBackbonePort>>` (a `vcs::MemoryBackbonePort`, `vcs/rs/lib.rs:606-634`) alongside the existing `CATALOG_PORT` (`s/plugin/rs/lib.rs:155-176`) — never touches `LocalStorageBackbonePort`, so nothing survives reload.
- `create_os_studio(name, TEMP_CATALOG_PORT.clone())` (same function, different port — `STUDIO_CATALOG_URIS` tracking in `framework/product/os/core/rs/lib.rs:1233-1247` is already keyed per-port-pointer, so this isolates cleanly).
- `openStudio` (`s/plugin/rs/lib.rs:2030-2046`) currently hardcodes `catalog_port()`; change `load_os_studio_document` lookup to try the persistent port, then the temp port (small `resolve_studio_port(id)` helper), so navigating into a just-created temporary studio still loads.

### File

- Keep today's default behavior (single JSON via `dev://studio/{id}` on the persistent `LocalStorageBackbonePort` — this already is "one embedded JSON blob").
- **Native**: additionally prompt a save location via `rfd::FileDialog::new().set_file_name(...).save_file()` (mirrors `download_media_export`, `lib.rs:11866-11875`) and attach a new native-only `NativeFileBackbonePort` (implements `OsBackbonePort::read/write` via `std::fs`) through the existing `LocalJsonBackbone` (`framework/product/os/core/rs/lib.rs:1098-1141`, already gated on `local://` URIs) so subsequent edits keep writing to that same `.json` file.
- **Wasm**: no filesystem access (matches the agreed Folder scope decision) — after creating, emit `downloadMediaExport` with the full document JSON so the user gets a one-time `.json` download; live editing continues against the in-memory/localStorage dev backbone for the rest of the session.
- The folder-picker round trip for native follows the existing `requestFileOpen` pattern (`lib.rs:7613-7639`, `11877-11888`): S plugin returns `{"operation":"requestFileOpen", ...}`-style operation, wgpu's native `rfd` dialog runs, then re-dispatches `createStudio` with the chosen path appended to args.

### Folder (native-only)

- New operation emitted by S plugin, e.g. `{"operation": "requestFolderPick", "importCommand": "createStudio", "args": {"kind": "folder", "name": ...}}`, handled in `apply_operations` next to `requestFileOpen` (`lib.rs:7613-7639`) using a new native-only `pick_folder() -> Option<String>` (`rfd::FileDialog::new().pick_folder()`, `#[cfg(not(target_arch = "wasm32"))]`; returns `None` on wasm32, so the flow silently no-operations in the browser — pair with hiding/disabling the Folder button when compiled for `wasm32` via `#[cfg(...)]` around that `tool_button` in Phase 2's `create_home_app()`).
- On the resulting `createStudio` dispatch (now carrying `folderPath`), native-only code (`#[cfg(not(target_arch = "wasm32"))]` in `s/plugin/rs/lib.rs`) creates `<folder>/.semio/` and opens/creates `<folder>/.semio/studio.db` via `rusqlite` (add as `[target.'cfg(not(target_arch = "wasm32"))'.dependencies]` in [s/plugin/rs/Cargo.toml](s/plugin/rs/Cargo.toml), matching `compose/client/lib/rs/Cargo.toml:51-52`).
- New `SqliteFolderBackbonePort` implementing `OsBackbonePort::read(uri)/write(uri, payload)` over a simple `documents(uri TEXT PRIMARY KEY, payload TEXT)` table in that `studio.db` (one DB per opened folder; reuses `DevJsonBackbone.sync`/`create_os_studio` unchanged — only the port differs).

---

## Phase 4 — Repo process

- Open a ticket via the repo MCP before implementing (per `AGENTS.md`), associating it with the closest existing goal (`Running Sketchpad` / r26-02, or the generic open `r26-03` cycle — no goal directly named "S"/"Studio" exists today).
- Implement changes directly in the listed existing files using `#region`/`pub mod` grouping as needed; no new files besides what's ticket-scoped.
- Manually verify: native wgpu build double-click into Demo Studio and back; footer Create → Temporary/File/Folder on native; Create → Temporary/File on the wasm/browser build (Folder hidden); confirm no `rusqlite`/native-only code leaks into the wasm32 component build.
- Close the ticket with a summary and the full list of touched files.
  </plan>
  <todos>[{"id": "navigate-operation", "content": "Handle navigate operation in wgpu apply_operations + add apply_shell_uri/switch_to_s_app session switching"}, {"id": "nav-history", "content": "Wire back/forward/up hit handlers to re-apply the shell URI"}, {"id": "static-tools-fallback", "content": "Add static AppDefinition.modes tools fallback in refresh_ui so footer tools render"}, {"id": "create-tool-group", "content": "Replace New Studio button with Create tool_collection (Temporary/File/Folder) in create_home_app"}, {"id": "temporary-kind", "content": "Add temp in-memory catalog port + resolve_studio_port for Temporary studios"}, {"id": "file-kind", "content": "Implement File kind: native save dialog + NativeFileBackbonePort via LocalJsonBackbone; wasm download fallback"}, {"id": "folder-kind", "content": "Implement native-only Folder kind: pick_folder operation, .semio/studio.db via rusqlite, SqliteFolderBackbonePort"}, {"id": "ticket-verify", "content": "Open ticket, implement, manually verify native + wasm builds, close ticket with summary"}]</todos>
