# ⚖️ W5b — Interaction-parity probe (`🐍️parity-interact-probe.mjs`)

One user journey, driven identically on both renderers, with a per-step action/outcome log, a structure
delta and a screenshot per renderer, so behavioural parity is **measured**, not eyeballed.

The blocker this packet removed: `dumpStructure`'s own header states that the shell's navbar, footer, dock,
window caps, pane chips and overlays "are rendered by this crate's own immediate-mode widgets code directly
into the composited canvas frame, never through `UI_ENGINE` at all, so they're structurally unreachable
from here". A React probe reads chrome from the DOM; the wgpu target published nothing about it at all.
`dumpChrome` closes that gap.

---

## 1. Introspection additions

### 1.1 Rust — the chrome ledger

`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs`

| what | where |
| --- | --- |
| `//#region 🎯️ChromeLedger` + bounds (`CHROME_HIT_CAPACITY 2048`, `CHROME_ACTION_CAPACITY 128`, `CHROME_ACTION_ARGS_BYTES 512`) | `:2577` |
| `struct DumpHitTarget { controlId, kind, rect, windowId?, controllerId?, action?, dragAxis? }` | `:2599` |
| `struct DumpDispatchedAction { seq, controllerId, action, windowId?, args?, atMs }` | `:2617` |
| `struct ChromeLedger` + `static CHROME_LEDGER: WorkerCell<ChromeLedger>` | `:2629`, `:2636` |
| `chrome_ledger_now_ms` (`js_sys::Date::now` — the frame Worker owns no `window.performance`) | `:2641` |
| `chrome_action_args` (args as JSON via `dsl::json::from_dsl_value`, truncated on a char boundary) | `:2653` |
| `chrome_hit_row` (projection; `windowId` only for retained body rows) | `:2665` |
| `pub fn note_chrome_hit_registry` / pure `ledger_publish_hits` | `:2682`, `:2690` |
| `pub fn note_dispatched_action` / pure `ledger_push_action` | `:2698`, `:2707` |
| `struct DumpChrome { armed, generation, windowId?, hits, actions }` / `project_chrome_dump` | `:2720`, `:2732` |
| `#[wasm_bindgen(js_name = dumpChrome)] pub fn dump_chrome` | `:2802`–`:2803` |

Diagnostics-gated exactly like W3a's per-frame censuses (`semio_framework_trace::runtime_diagnostics_enabled`);
an unarmed page answers `armed: false` rather than an empty registry, so a probe tells "switch off" from
"nothing registered". Every mutation has a pure half a native law can drive.

### 1.2 Rust — the two Shell taps

`…/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`

- `:7459` — `note_dispatched_action(&action)` at the head of `dispatch_action`, the single funnel every
  chrome press, keybinding, command-palette entry, panel row and retained body action crosses.
- `:9335` — `note_chrome_hit_registry(input.hits(), &self.retained_hit_windows)` in
  `publish_retained_hit_registry`, right after `InputState::publish_hits`. Only a COMPLETE chrome walk
  publishes, so the snapshot and the pointer's own authority can never disagree.

### 1.3 TS bridge

- `…/🎯️targets/🧊️wgpu/🚚️browser-frame-transport/🟦️.ts:221` — `"chrome"` added to `BrowserFrameIntrospectionProbe`.
- `…/🎯️targets/🧊️wgpu/🎞️frame-worker/🟦️.ts:50` (binding type), `:369` (probe → `bindings.dumpChrome`).
- `…/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:95` (type), `:100` — `dumpChrome: probe("chrome")` on
  `window.semioWgpuIntrospection`.

### 1.4 React — the same action-log shape

The DOM host already funnels every input through the Input Causality Ledger, but only exposed a *census*
(counts). It now also exposes the entries, so the probe reads ONE shape from both renderers:

- `…/🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts:163` — `InputLedgerRecordV1 { inputSeq, controllerId, action, origin, windowId, causedBy, outcome }`.
- `:181` / `:270` — `recent(limit?)`, issue order, open entries visible as themselves, bounded by `historySlots`.
- `:216` — `record()`; `InputLedgerEntryV1` now also keeps `controllerId` (it was thrown away at `issue`).
- `…/🧱️elements/🏛️ShellHost/🟦️.tsx:4217` — `globalThis.__semioInputLedger` answers `{ …census, recent }`.

### 1.5 Laws

- `…/🧱️elements/🗣️Interpreter/🧪️tests/🔬️wgpu-introspection/🦀️.rs:214`–`:305`, region `🎯️ChromeLedgerLaws`:
  `chrome_hit_rows_carry_absolute_rect_kind_and_only_body_rows_name_a_window` `:227`,
  `chrome_action_ledger_mints_monotonic_seq_and_stays_bounded` `:248`,
  `chrome_action_args_are_truncated_on_a_character_boundary` `:269`,
  `chrome_dump_window_filter_keeps_chrome_and_reports_the_diagnostics_gate` `:281`.
- `…/🧪️tests/🎯️input-ledger/🟦️.ts:173` — `recent()` order / open entries / `controllerId` / history bound.
- `…/🧪️tests/📨️browser-frame-transport/🟦️.ts:384`–`:385` — the worker's probe mapping and the boot's
  global now gate `dumpChrome` like their four neighbours.

### 1.6 Gates run

| gate | result |
| --- | --- |
| `cargo check -p semio-framework-os-renderer-wgpu --tests -j 4` | clean (no new warnings) |
| `cargo test -p … --lib introspection_tests` | **12 passed** (4 new) |
| `bun test 🧪️tests/🎯️input-ledger/🟦️.ts` | **37 passed** (1 new) |
| `bun test 🧪️tests/📨️browser-frame-transport/🟦️.ts` | **33 passed** |
| `nx lint`, `generate-browser-boot`, `generate-frame-worker`, `check-browser-worker`, `check-frame-worker` (`NX_DAEMON=false --skip-nx-cache`) | all ✔ |

Two pre-existing failures elsewhere in the crate are NOT this packet's (both are geometry/contract laws in
areas this packet never touches): `shell::chrome_overlays_tour_tests::window_silhouette_border_emits_notched_outline_segments`,
`shell::shell_chrome_parity_tests::the_overlay_row_steps_clear_of_an_open_floating_panel` (W4a's overlay/tour
lane) and `async_boundary_tests::{presenter_ack_retirement_source_mutations_are_denied, raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete}`
(W3d's asset pump).

---

## 2. The probe

`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🐍️parity-interact-probe.mjs`

```
cd .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY && \
  SEMIO_PROBE_TARGETS=react,wgpu SEMIO_PROBE_OUT=w5b-run-1 \
  SEMIO_PROBE_REACT_URL=http://127.0.0.1:6313/?plugin=puzzle3d \
  SEMIO_PROBE_WGPU_URL=http://127.0.0.1:6213/?plugin=puzzle3d \
  bun 🐍️parity-interact-probe.mjs
```

Env: `SEMIO_PROBE_TARGETS` (`react`, `wgpu`, or both) · `SEMIO_PROBE_OUT` · `SEMIO_PROBE_BOOT` (s, 240) ·
`SEMIO_PROBE_SETTLE` (ms/step, 1600) · `SEMIO_PROBE_ONLY=<step,step>` · `SEMIO_PROBE_HEADED=1`.

Output under `🗑️generated/<out>/`: `steps.json`, `parity.md`, `<renderer>/NN-<step>.png`,
`<renderer>/console.txt`. Both files are rewritten after **every** step, so a run that dies half-way still
leaves its evidence; a renderer that never boots is recorded as skipped, never faked.

### 2.1 How one journey drives two renderers

A step names a **control key**. React publishes the key as an element `id`; wgpu publishes the same key in
`dumpChrome().hits` with an absolute page rect the probe aims a real pointer at. Resolution is a ladder —
exact, `.<key>` suffix, containment, then an explicit alias — and **which rung matched is recorded and
enters the verdict**, because a control only one renderer names exactly is itself a parity finding.

Verdict per step: same SET of dispatched action ids + same surfaces added/removed. Deliberately a set, not
a multiset — a guest lane re-registering brush meshes fired 61 times on one React step and 3 on the next
for the same gesture, so counts measure the frame clock. Controller ids, rects and timings are recorded but
not compared (React names `s.puzzle.puzzle3d@1/*#editor`, the wgpu shell names its own host controller).

### 2.2 Journey (37 steps)

boot · dismiss-tour · 5 navbar panels + close · 6 pane chips · split-gutter drag · window cap focus/close ·
reopen via the dock tab bar · orbit / pan / zoom / pick / right-click on the scene · Escape · `mod+k` ·
Escape · `mod+z` · `mod+shift+z` · `mod+shift+f` ×2 · `mod+alt+1` / `mod+alt+2` panel anchors ·
example picker open / switch / dismiss · role viewer → editor.

Ordering is load-bearing and was learned from a failed run: an open picker overlay swallows the next step's
click, and `viewer` makes the shell read-only — an early role switch turned every later step into
`refused: viewer-read-only`. Picker and role therefore come last.

---

## 3. React run (proof)

`http://127.0.0.1:6313` was **not listening** at the time of writing (no process on that port); the run used
the live puzzle3d React play serve on **6013**, which is the same `os/dev` host with the same `?plugin=`
axis. Latest run: `🗑️generated/w5b-react-8/` — 37/37 steps executed, 38 screenshots, no probe errors.

| step | resolution | dispatched verbs |
| --- | --- | --- |
| `dismiss-tour` | exact `ui.introduction.skip` | (none) |
| `panel-{artifact,catalogue,inspection,toolRun,chat}` (+close) | exact | `noteShellCommand`, panel surface in/out each time |
| `pane-chip-{engagement,search}.toggle` | suffix | `addObjectKind` — opening the Actions pane auto-arms a verb |
| `pane-chip-{windowControls,utilityBar.unfold}` | suffix | (none) |
| `pane-chip-{projection,windowOptions}.toggle` | **absent** — React publishes no such chip for this window kind | — |
| `split-gutter-drag` | contains `_r_1b_` | `noteShellCommand`, `registerBrushMesh` |
| `window-cap-{focus,close}`, `window-reopen` | contains (`mode-dock-tab-focus` / `-close` / `mode-dock-tabbar`) | (none) |
| `orbit-drag` | — | `interactionHover`, `interactionSelect`, `setCamera`, `noteShellCommand` |
| `pan-drag` | — | `interactionHover`, `interactionSelect`, `noteWorldNavigation`, `setCamera` |
| `zoom-wheel` | — | `interactionHover`, `noteWorldNavigation`, `setCamera` |
| `pick-instance` | — | `interactionHover`, **`interactionSelect`**, `setCamera` |
| `context-menu` (right click) | — | `interactionSelect`, `setCamera` |
| `chord-escape`, `context-menu-dismiss` | — | `engagementAbort` |
| `chord-undo` / `chord-redo` | — | `undo` / `redo` (+ `shell.windowActivate` refused `undeclared-action`) |
| `chord-command-palette` (`mod+k`), `chord-fullscreen`, `chord-panel-anchor-{left,right}` | — | **(none)** |
| `example-picker-open` → `example-switch` | exact | `setActiveExample` — "Concrete Forest" → "Nakagin Capsule Tower" |
| `role-viewer` → `role-editor` | exact | role attribute flips both ways |

React-side observations worth a look independently of wgpu: `mod+k`, `mod+shift+f` and the panelAnchor
chords dispatch nothing at all; window cap focus/close dispatch nothing (the surfaces do not move either);
`shell.windowActivate` is refused as `undeclared-action` on every undo/redo.

## 4. wgpu smoke (the chrome dump is live)

`🗑️generated/w5b-wgpu-smoke2/` — 6213 answered `dumpChrome` with `armed: true`, 41 hit rows, and the click-by-id
path worked end to end. Early findings, to be confirmed on the coordinator's clean full run:

1. **Control-id drift, tour chips.** wgpu mints `shell.tour.{skip,next,back}`; React mints
   `ui.introduction.{skip,next,back}`. No ladder rung bridges them, so the probe carries an explicit
   `CONTROL_ALIASES` table and reports `resolved: "alias"` — which makes the step DIFFER in `parity.md`.
2. **Control-id drift, panel tabs.** wgpu mints `shell.panel.tab.<anchor>.<key>` (e.g.
   `shell.panel.tab.top-left.framework.panel.artifact`) where React mints the bare
   `framework.panel.artifact` — resolved by suffix on one side and exactly on the other, so every panel
   step is flagged.
3. **The tour overlay leaks pointer input.** Clicking wgpu's `shell.tour.skip` also dispatched
   `interactionHover` + `interactionSelect` into the scene below. React dispatched nothing on that step.
4. **wgpu panel toggles journal nothing.** The panel surface appears/disappears in `dumpStructure`, but no
   action crossed `dispatch_action`; React dispatches `noteShellCommand` per toggle.
5. **Initial panel state differs**: the wgpu shell boots with `framework.panel.artifact` already open, so
   the first panel step CLOSES it where React OPENS it.

The serve was rebuilt by a peer mid-smoke (one step saw `dumpChrome` vanish and the page report no
structure); the probe recorded it as an error on that step and carried on, which is the intended behaviour
against a live rebuild.

---

## 5. What remains

- The full **wgpu half** (all 37 steps) after the coordinator's rebuild, then a `SEMIO_PROBE_TARGETS=react,wgpu`
  run for a populated `parity.md`. The probe is ready; nothing in it is wgpu-rebuild-dependent beyond the
  `dumpChrome` export, which the 6213 serve already carries.
- The React target should move back to **6313** once that serve is up (`SEMIO_PROBE_REACT_URL` default
  already points there).
- `projection.toggle` / `windowOptions.toggle` resolved absent on React; whether the wgpu shell publishes
  them is an open parity question the full run answers.
- The five findings in §4 are candidates, not verdicts — each needs the clean side-by-side run before it is
  filed as a defect.

## 6. Files

- `🐍️parity-interact-probe.mjs` — the probe.
- `🐍️w5b-chrome-recon.mjs` — the selector census the React driver was written from (boots one renderer,
  dumps every `data-*`/`role`/`id` it publishes plus the wgpu hit registry; clicks nothing).
- `🗑️generated/w5b-react-8/` — the proving React run (`steps.json`, `parity.md`, 38 screenshots).
- `🗑️generated/w5b-wgpu-smoke2/` — the wgpu smoke.
- `🗑️generated/w5b-recon-react/` — the recon census.
