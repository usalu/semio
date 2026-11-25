# Framework UI fixes (F4, F6, F8, F12, F14)

| Id | Root cause | Fix | Status |
|---|---|---|---|
| F4 | `useChromePanelSafeArea` graded the whole open pane (960 px tall) against every shell panel. Its foot overlaps the full-width bottom chrome band (y 971-1000), which no inline shift can clear, so `chromePanelSafeArea` (moved box must clear ALL panels) gave up and left the pane at x=0 UNDER the docked Artifact panel, while the window content (dead-line padding) still reserved 300 + 300 px: empty gap, 200 px canvas. | New pure `chromePanelHeadBox` (anchor-corner strip); `Pane` passes `headPx: 32` so a pane yields to docks covering its header, not to chip rows/bands at its tail. Shared Three oracle/fixture/Rust twin unchanged (pure rule untouched). | Verified live: with Artifact open pane x=300 (300 wide), canvas 600/200 with grid; without Artifact pane x=0. |
| F6 | Not reproducible on the current vite-served TS: arg select is 160x28 inside the Set Locale section. Earlier 0x0 came from a collapsed/clipped section state. No change. | none | not reproduced |
| F8 | Already fixed by `roleButtonDisabledV1` (busy gate only, 14:47). Live: Viewer then Editor round trip works, Editor enabled while Viewer is active, disabled only mid-switch. | none | verified |
| F12 icons | 3D manifest action `resultAnimationTick` used unregistered icon `timer` (same class as 2D row 6). Section row already uses `beam` in source (served wasm is older and still paints `ruler`). | `timer` -> `clock`; new law scans all 3D icon literals against the generated registry. | law red with `timer`, green now |
| F12 Poisson | Slider readout `w-large text-end` sat flush on the panel edge. | `pe-tiny` on `sliderValueClassName` (not re-verified live: Materials rows were virtualized away in the final run). | applied |
| F14 | Not reproducible live (children stay rendered while toggling Materials/Sections; `treeWindowKeepOffscreenRowsV1` already keeps them). Remaining hazard: a freshly opened container asked for a full viewport (48 rows) beyond the node ledger (103) and could starve visible windows. | `viewStateFields` caps fresh-open rows to the ledger left by measured windows (`TREE_WINDOW_BODY_NODE_BUDGET`). Existing test updated (3 fresh containers get 33 rows). | law added |

Files: `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🛟️chrome-safe-area/🟦️.tsx`, `…/🧱️elements/📌️WindowPane/🟦️.tsx`, `…/🧱️elements/🎚️Slider/🟦️.tsx`, `…/🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx` (+ its test), `…/🧪️tests/🛟️chrome-panel-safe-area/🟦️.ts`, 3D `📋️manifest/🦀️.rs` and new `🧪️tests/🎨️icon-registry/🟦️.ts`.
Run: `node_modules/.bin/vitest.exe run --configLoader native --config <M>/ui-vitest.config.mjs` (20 tests green). Screenshots in `🗑️generated/ui-shots/`.

## Follow-up: Pane-related suites and Poisson readout

- ui-react config (`⚛️react/🧪️tests/🎚️config`, with `SEMIO_VITEST_POLICY` using forward-slash paths), filters `owned-locale` (shell-components-8/9, control-chrome-2, Window component), `footer-chips`, `window-pane-exclusivity`, `safe-area`: 5 files, 593 tests, all green. Nothing broken by the Pane/safe-area change.
- Own config (`ui-vitest.config.mjs`): chrome-panel-safe-area, 3D icon registry, tree-windows: 3 files, 20 tests, green.
- Poisson: `pe-tiny` produced no CSS (padding stayed 0). Replaced by `px-tiny` (class already used elsewhere). After reload and scrolling the Artifact tree to Materials, the readout `0.3` has 9.6 px padding, right edge inside the panel, fully visible (`poisson4.png` in `🗑️generated/ui-shots/`). A vite reload of the page is needed to see it (no HMR for this class).
