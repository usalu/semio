# FEM Interactive Features End to End

Request: make every FEM feature work interactively (selection, utilities such as transform, everything).

## Method
- `page-driver.mjs`: one persistent headless Chromium page per editor, driven by POSTed JS (`d.sh <port>`). `INSTRUMENT=1` wraps the bridge's `reactor.poll`/`stepJob` with timings and per-poll receipts; `DIAG=1` arms `SEMIO_RUNTIME_DIAGNOSTICS` through the WASI environment shim so the guest prints its more-work trace.
- `ui-dump.mjs`: one-shot aria tree, control list and screenshot.
- Inventories: `feature-inventory-2d.md`, `feature-inventory-3d.md` (Haiku). Matrices: `feature-matrix-2d.md`, `feature-matrix-3d.md` (Sonnet agents, on the old build).

## Findings and Fixes

### 1. Every interaction took 14–42 s (framework, reactor)
Measured on wasm 849acedc, FEM 2D:
- One artifact-tree selection: 10 048 reactor polls, 8.7 s inside `reactor.poll`, 15.8 s wall, about 22 s CPU. Idle CPU is 0, and the main thread is blocked only about 3 s of it.
- Each poll is granted 50 M fuel, spends 11–65 units and copies 1–2 items, with no effects and no patches.
- The guest more-work trace for one selection: `typed_operation` 7 438 turns, `command_ingress` 3 220 turns. Boot: `typed_operation` 9 674, `document_load` 2 102, `command_ingress` 472. Streaks of exactly 244 quiet turns per operation.

Causes:
- `plugin_continue_typed_operations` allows 256 units per turn but stops at the first unit carrying any original receipt (`TypedOperationGrant::spent`). Those receipts are settled in-guest (`collect_original_publication_receipts`), so every unit cost a full host round trip.
- `exchange_generic` (command ingress) ran exactly one `plugin_exchange_turn` per turn until the command applied.

Fix:
- `⚛️reactor/🔄️turn/🌀️poll-kernel-turn/🦀️.rs`: keeps driving the continuation within one turn while the previous output advanced (receipt, frames, events or pages), added no host-bound effect, the scan is still runnable, the context does not yield, and the turn has executed less than `TYPED_OPERATION_TURN_SLICE_US` (8 ms).
- `⚛️reactor/🔄️turn/⌨️command-ingress-turn/🦀️.rs`: `exchange_generic` loops the exchange under the same rules (`COMMAND_EXCHANGE_TURN_SLICE_US`, 8 ms). It still stops on a quiet step, a stalled command, a host-bound effect or a refusal.
- `semio-plugin-reactor-turn` checks cleanly (`check-turn-slice.sh`). The laws are being written by a Sonnet agent (`turn-slice-laws.md`).

### 2. No utility armed at startup (FEM 2D)
Canvas clicks select nothing until the user picks "Direct Select" under Utilities. Neither window kind declared `initial_utility_id`.

Fix: `✏️editor/🚀️ui-helpers-and-app-assembly/🦀️.rs` arms `selectDirect` on the Model and Results windows (`window_kind_initial_utility`). Law `canvas_windows_arm_direct_select_initially` (`✏️editor/🧪️tests/🔬️unit/🦀️.rs`).

### 3. Unreadable section headings ("Display", "Playback", "Analysis") (framework UI)
The heading class `text-base` resolved to the theme colour `--color-base` (`var(--base)`, the background), not the font size `--text-base`. Every `text-base` heading in the renderer was affected: Interpreter sections, HubSignIn, SpaceBrowser and AgentDelegations.

Fix: removed the duplicate theme alias `--color-base` from `🖱️ui/🎨️styling/🖌️ui/🎨️.css` (`--color-background` is the same value). The seven `bg-base` usages in the ShellHost components (`🟦️.tsx`, session-notice, local-catalog files, close-guard, notice-bands and its test) now use `bg-background`.

### 4. A top panel covers the bottom panel chips (framework UI)
With the Artifact panel open, its body spans to the canvas bottom and covers the folded bottom-left chips (Display, Results, Remote). They cannot be clicked.

Fix: `🖱️ui/🔨️modules/⚓️panel-anchor/🟦️.ts` `chromeHostedOpenPanelPositionStyle` gives top panels `max-height: calc(100% + var(--navbar-band-height) - var(--footer-band-height))`. Test expectation updated in `🧪️tests/🧪️owned-locale-detector-retirement/🎨️shell-components-1/🟦️.tsx`.

### Observed working on the old build (slow)
- Direct Select click → node highlighted on both canvases; the Inspection panel shows the node ID, X, Y, Focus and Delete.
- Inspection X edit → the model updates (`p0_l2 · (−0.50, 5.60)`); re-solved `Converged · 38 / 512`.
- Artifact tree lists all sections and their rows; a row click selects.

### 5. Dev servers went blank (repo cache GC)
`/@vite/client` returned 404 and pages stayed empty because the dependency-generation collection (`cache:dependencies`) deleted `.cache/b/d/m/j/<key>` while the running vite (started from that install) still used it. Fix by agent: `🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📦️dependencies/♻️collection/🟦️.ts` keeps any install named by a live process's command line (report `prune-in-use-deps.md`). `native-fem-dev.sh` now picks the newest complete install.

### 6. 2D wedge after edit/undo/redo (`plugin.original-stock-exhausted`)
Not an accounting leak: a job registered before the guest admitted it, and a drive that failed or closed abandoned its running job, so the copy stock could never renew and the parked request stayed Pending. Fixed in `🌉️component-bridge-source/🟦️.ts` and `🔌️PluginRuntime/🚚️handle-turn-drive/🟦️.ts` with a new law (report `copy-stock-leak.md`).

### 7. Browser descriptor was stale
The served `🔣️.json`/`🛂️.descriptor.semio` come from the committed `🌎️hub/🧩️compositions/🏗️fem/🧩️component/🛂️.descriptor.semio`, last described 11-23. Manifest changes (initial Direct Select, 3D action args) never reached the browser. `describe-fem.sh` builds the current `semio-plugin-describe`, re-describes the fresh component and republishes.

### 8. Results captions overlapped the structure (FEM 2D)
Progress/eigen/modal/buckling/playback captions and the von Mises legend were drawn at fixed scene coordinates inside the structure. They now sit above / right of the model's scene bounds (`📊️results/🦀️.rs` `model_scene_bounds`, `caption_position`); law `results_captions_and_legend_sit_beside_the_model`.

### 9. Turn slicing not yet effective in the browser
Fresh wasm (05:15) still boots with ~14 000 one-unit turns (`typed_operation` 9 855, `document_load` 2 137 + 1 500). Root cause investigation handed to the turn-slice agent (`turn-slice-laws.md`).

### 10. Browser state on wasm 07:55 + re-described descriptor (08:15)
- FEM 2D: Demo no longer opens: `shell.documentTransfer.load-failed … original-turn.native-return-refused: authority (invariant-violated)`. Same guest loaded at 05:20 and failed from 06:20; the committed descriptor served via route fails too, so the trigger is host-side (TypeScript changed by peers after 05:18 around document load). Diagnosis delegated (`demo-load-refusal.md`).
- FEM 3D: boots (≈5 min). All twelve Add actions now open staged forms with declared args (e.g. Add Material: Name, Young's Modulus 2.1e11, Shear Modulus 8.1e10). Execute did not show the new material within 4 min (latency or failure: re-check after turn slicing). The Model viewport went blank while the Actions pane was open (re-check).
- The descriptor tool build broke on a peer's temporary `#[path=…/debug-ledger.rs]` include in `🏪️store/🧩️composition/🚪️open/🫴️record/🦀️.rs` (path exists only on their machine); `describe-fem.sh` reuses the already-built describe binary.
- Turn-slice agent: removed the `!advanced` break and loops restored document loads within a turn (`⏱️quiet-slice-turn` module, laws `quiet_*`). Rebuilding wasm with it now.

### 11. Wasm 09:39 (turn slicing live)
- Demo-load refusal gone (agent: guest bug in the 08:16 build, fixed by current source; law `demo-load-law.mjs`, 5/5 passes; report `demo-load-refusal.md`).
- FEM 2D boot 23 s (was 40–90 s); typed_operation boot turns 9 855 → 40; remaining `document_load` 3 882.
- FEM 3D boot 55 s (was ≈312 s); `document_load` 4 922.
- 2D canvas click with the default Direct Select highlights `ridge` on both canvases (initial utility live). Results caption sits above the model (caption fix live).
- New spin A (2D selection): >6 400 `typed_operation` turns each spending ~10 units with zero copy progress; Inspection never updates.
- New spin B (3D addMaterial): 9 186 `reconcile` + 3 042 `reconcile,ui_retirement` turns in 184 s; material never appears.
- Both spins handed to the turn-slice agent.
