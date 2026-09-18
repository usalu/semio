# 📓️ P0 — wgpu renderer compile restore

Packet P0 of ticket 26/09/17/WGPU-RENDERER-REACT-PARITY. Brief: restore `cargo check -p
semio-framework-os-renderer-wgpu --lib` (native + `wasm32-unknown-unknown`), which was red with 11
errors in two families.

---

## 1. Result

| Check | Before | After |
|---|---|---|
| `cargo check -p semio-framework-os-renderer-wgpu --lib --keep-going` | **11 errors**, 22 crate warnings (`🗑️generated/native-check.txt`) | **green — 0 errors**, 50 crate warnings, `Finished dev in 31.58s` (`🗑️generated/p0-native-check.txt`) |
| `… --target wasm32-unknown-unknown --keep-going` | 0 errors, 52 crate warnings (`🗑️generated/wasm-check.txt`, **stale** — see §4.3) | **green — 0 errors**, 52 crate warnings, `Finished dev in 2m 06s` (`🗑️generated/p0-wasm-check.txt`) |

Both "green" verdicts were observed in this packet's own runs, exit status 0. Total diff: **25 insertions,
4 deletions in 2 files** — no behaviour change beyond the two contract ports.

---

## 2. Family A — `ViewModel.tree_windows` / `tree_viewport_rows` (6 errors)

`semio_framework::ViewModel` (`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4499`) gained two fields on
2026-09-17 from peer ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING:

- `tree_windows: Vec<TreeWindowRequest>` (L4581) — every tree container the host holds state for,
  flattened over all panel bodies.
- `tree_viewport_rows: Option<u32>` (L4587) — rows the tallest visible panel body fits; `None` means
  "the host has not measured a viewport yet".

Six exhaustive struct literals in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`
did not name them. Each now carries `tree_windows: Vec::new(), tree_viewport_rows: None,` appended
after `tool_run_trace_cursor_by_window_id` (post-edit line numbers):

| Site (post-edit) | What it builds |
|---|---|
| `:3995` | boot view state for the configured host program |
| `:4039` | boot view state for the selected-program branch (`ActiveSession { … }` inline) |
| `:4599` | per-refresh view state for a **spawned** panel app |
| `:6861` | `default_view_state` in `switch_to_managed_app` |
| `:6924` | view state minted by the session-role switch |
| `:6998` | view state minted by `switch_to_plugin_app` |

### 2.1 Why empty/`None` and not a tracked value

This is the value React sends **before its first observer report**, not a placeholder. React's
`TreeWindowHostV1::viewStateFields`
(`🧱️elements/🛠️ShellHelpers/🟦️.tsx:2201-2221`) emits
`{ ...(flattened.length > 0 ? { treeWindows } : {}), ...(viewportRows === undefined ? {} : { treeViewportRows }) }`
— i.e. both keys are **absent** until a body has reported, which decodes to exactly `Vec::new()` /
`None`. The requests themselves come from the React `🗣️Interpreter`'s viewport observer via
`reportWindows(bodyKey, requests, viewportRows)` (`🟦️.tsx:2317`).

The wgpu shell has **no such observer and no per-panel tree window state**: greps for
`TreeWindow`/`tree_window` over the wgpu Shell target and `🖱️ui/🎯️targets/🧊️wgpu/` return only two
docstrings, both of which declare the gap on purpose
(`🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:2365` and
`🖱️ui/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs:94`). The peer's own
`📓️p5-wgpu-and-reconcile-law.md` §2.4/§5 and `📓️design-virtualised-tree.md` §6.4 legislate the same:
"No host-side request wiring for wgpu this ticket (documented gap; no `+N` fallback)". So there is
nothing to wire in yet, and stuffing a synthetic default would have been worse than the honest empty:
a non-empty `tree_windows` naming containers no wgpu observer measured would make guests materialise
rows against a viewport that does not exist.

### 2.2 The seam, documented in code

A docstring now sits on `live_view_state`
(`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4368`), which is the one function that restamps the
host-owned per-refresh fields (`locale`, `terminology`, `window_instances`,
`active_utility_by_window_id`, `tool_run_trace_cursor_by_window_id`, `focused_window_id`,
`session_identity`) onto the session's view state. **That is where a wgpu tree-window observer must
feed `tree_windows`/`tree_viewport_rows`**, exactly as React feeds them per refresh — not at the six
minting sites, which only seed a fresh session. The docstring names the React counterpart and the
observable consequence (a guest paints its own first-paint window; scrolling into a spacer band
reveals pitch, not streamed rows).

---

## 3. Family B — `JobReplayRoute.document` → `.artifact` (5 errors)

`semio_framework_actor::JobReplayRoute` (`🧰️framework/🔨️modules/🎭️actor/🦀️.rs:899`) carries
`artifact: [u8; 32]` where it once carried `document`. The rename is **committed and complete
upstream** — `git log -S'pub artifact: [u8; 32]'` on the actor module points at `8773331d23`
(2026-09-15 15:19:47 +0200); the TS mirror (`🎭️actor/🦀️.rs:126`), the digest fold
(`:1643`), the actor unit tests (`🎭️actor/🧪️tests/🔬️unit/🦀️.rs:297,420,456,500`) and the
renderer's own kernel-runtime tests
(`🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs:288,350,816,852`) all
already say `artifact`. Only the renderer's production sites were left behind. There is no shim and
none was added — the route identity simply **is** `artifact` now.

Changes, all in
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`:

| Line | Before | After |
|---|---|---|
| `:5805` | *(already `artifact: [u8; 32]` — the local `MountedReplayRouteSeed` declaration had been renamed, its uses had not)* | unchanged |
| `:5816` | `document: self.document,` in `MountedReplayRouteSeed::compose` | `artifact: self.artifact,` |
| `:6910` | `let document_digest = JobReplayRequest::from_spawn(&app_id, &[]).tool;` | `let artifact_digest = …` |
| `:6935` | `MountedReplayRouteSeed { …, document: document_digest }` (app activation) | `…, artifact: artifact_digest` |
| `:7041` | `MountedReplayRouteSeed { …, document: parent_route.document }` (extension activation inherits the parent's identity) | `…, artifact: parent_route.artifact` |

The value is unchanged in kind: the digest is still `JobReplayRequest::from_spawn(&app_id, &[]).tool`
— the app/artifact id's tool digest — and the extension seed still inherits its parent app's identity
verbatim. Only the field name and the local binding's name moved with the contract. The `📜️script.ts`
gate at `:4670` asserts the `replay_routes: [Option<MountedReplayRouteSeed>; JOB_PROGRESS_ACTIVE_CAPACITY]`
declaration, which this packet did not touch.

---

## 4. Warnings — counted, not swept

### 4.1 Native, 50 crate-local warnings (was 22)

The count **rose** because the crate now type-checks: `dead_code` analysis only runs after a successful
check, so the 28 extra warnings are the ones the 11 errors were previously hiding. None is new
breakage.

| Count | Warning | Disposition |
|---|---|---|
| **21** | `unnecessary qualification` | **Left, per brief** — 21 sites is over the 20-site threshold, and they span four files two other agents are editing concurrently (`⚙️EngineCanvas`, `🗣️Interpreter`, `🐚️Shell`, `🧊️renderer`), 11 of them clustered in one block at `🧊️renderer/🦀️.rs:14266-14309`. |
| **1** | `unused extern crate` — `extern crate semio_framework_os_kernel as dsl_core;` (`🧊️renderer/🦀️.rs:25`, `cfg(not(wasm32))`) | **Left deliberately.** This is a crate-root **name binding for derive expansion** (`::dsl_core::…`), the same idiom `🔌️plugin/🦀️.rs:8` carries with an explicit docstring saying so; `-W unused-extern-crates` cannot see a path a macro will expand to. Removing it is a live risk for one lint. |
| **28** | `dead_code` (`never used` / `never read` / `never constructed`) | **Left.** These are parity scaffolding, not lint debt: `BrowserWireEvent`, `BrowserBatch`, `TextTarget`, `pointer`, `stateless_dispatch` (browser wire), `dock_tabs_from_ids`, `mode_layout_stacks_v1`, `WindowScopeStackV1`, `dock_seed_active_window_id_v1` (dock/layout), `reserved_shell_chords_v1`, `chord_carries_accelerator_v1`, `KEYBINDING_UNOWNED_CODE` (keybindings), plus retirement helpers (`close_step`, `take_page`, `acknowledge`, `terminal_is_empty`). Deleting them would delete the surfaces this very ticket is about to wire; they are a **parity inventory** and are listed here for the audit waves rather than removed. |

No warning in this crate is an unused import or an unused variable on the native target, so the
"fix if trivial" clause found nothing to apply.

### 4.2 wasm32-unknown-unknown, 52 crate-local warnings — unchanged by this packet

29 `unnecessary qualification` + 9 `unused import` + dead code. The nine unused imports
(`js_sys` in `⚙️EngineCanvas`, `IdentityEnv` in `🐚️Shell`, `std::sync::Arc` in `🧵️frame-job`,
`Deserialize` / `EventModifiers,PointerButton,PointerId,PointerInfo,PointerKind` /
`BrowserPointerKind,pointer` in `🌐️browser-worker`, and `program_bridge::filter_plugins`,
`program_bridge::parse_plugin_entries`, `fetch_font_bytes` in `🧊️renderer`) are **wasm-only** — they
do not appear in the native log, so each is used under `cfg(not(wasm32))` and removing it would break
the native build; the honest fix is a `cfg`-gated `use`, which is a separate, churn-heavy sweep across
files other agents hold. The prior log shows the same 52, so this packet introduced none of them.

### 4.3 The pre-existing `🗑️generated/wasm-check.txt` was stale

It reports 0 errors although Family A's six sites compile on the wasm target too (the same file's
`IdentityEnv` import is warned about in both logs). It was a **queued check that compiled the tree as
of its start** — before the peer's `ViewModel` fields landed. `p0-wasm-check.txt` is the post-fix run
and is the one to trust.

---

## 5. Open seams for the rest of the ticket

1. **wgpu tree-window observer (the real P0 follow-up).** `tree_windows` / `tree_viewport_rows` are
   sent empty forever until the wgpu shell mounts the equivalent of React's `🗣️Interpreter` viewport
   observer and feeds `live_view_state`
   (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4368`). Needs: a per-panel-body `TreeWindowHost`
   analogue keyed by **authored node key** (React's `TreeWindowBodyState`, `🟦️.tsx:2075`), open-state
   toggling on chevron clicks, a scroll-driven row measurement, and React's two constants —
   `TREE_WINDOW_REPORT_DEBOUNCE_MS = 40` (scroll reports debounced; an OPEN toggle is **not**
   debounced) and `TREE_WINDOW_DEFAULT_ROWS = 48` (fallback only for a never-measured container).
   Until then wgpu trees paint a spacer band with no rows behind it.
2. **The retained mounted-layout path is still unwindowed** — peer §2.4(2):
   `🖱️ui/🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs`, `📌️mounted_layout` and `🖌️paint` walk
   `UiTreeItemNode`/`UiTreeSectionNode` directly and need a spacer **node**, not just a height.
3. **28 dead-code symbols (§4.1) are the wgpu parity inventory** — browser wire events, dock tab/stack
   helpers, keybinding chord helpers. Audit waves should treat that list as "declared but unreached"
   rather than as cleanup.
4. **21 native + 29 wasm `unnecessary qualification` sites** remain; worth one dedicated sweep once the
   concurrent edits on `⚙️EngineCanvas` / `🗣️Interpreter` / `🐚️Shell` / `🧊️renderer` settle.
