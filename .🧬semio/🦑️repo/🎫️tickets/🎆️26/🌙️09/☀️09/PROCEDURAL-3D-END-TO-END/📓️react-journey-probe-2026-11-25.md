# React journey probe (2026-11-25)

File: `react-journey-probe.mjs` (this folder). Run: `cd <ticket> && repo tools exec --light bun -- react-journey-probe.mjs`.
Output: `🗑️generated/journey/react-journey/{results.json,console.txt,<row>.png}`. Rows: `{id, ok, status, ms, detail, screenshot}`; status is `pass`, `fail`, `missing` (with source evidence) or `skipped`.
One chromium (`--js-flags=--max-old-space-size=1024 --renderer-process-limit=1`), a hard 8 minute watchdog, all waits are polls (`until`). Env: `SEMIO_PROBE_URL`, `SEMIO_PROBE_OUT`, `SEMIO_PROBE_ONLY=1,4,...`.

## Rows, selectors and sources

| Row | What | Selectors / lanes | Source |
|---|---|---|---|
| 1-boot | no pageerror/fault, flow nodes painted, preview and flow pixel variance (own PNG decoder), every `[data-slot=window]` has canvas or text; generate mode (`mod+alt+ArrowRight`) and viewer role (`mod+alt+v`) windows | `window.__semioFlowGraphProbe[surface].nodeIds()`, `data-status-json`, `data-fault-code`, surfaces `window:procedural-main/-preview`, `window:generation3d-generate-preview`, `window:procedural-view-preview`, `#playground.navbar.roles.editor` | `🐍️journey-probe.mjs`, `🐍️i18n-a11y-customization-probe.mjs` |
| 2-examples | every option the picker lists (not capped at 8; includes demo, mesh-workbench if listed); mesh digest changed, converged, all node statuses `ok`, fixture oracle where one exists | `[role=combobox]`/`[role=option]`, `data-meshes-json`, `data-status-json`, `🐍️example-oracle.mjs` | `🐍️journey-probe.mjs`, `🐍️interaction-matrix-probe.mjs` |
| 3-hover-select-camera | graph handle hover -> `data-hover-paint-id` on preview; preview hover -> flow `data-selection-json.hoverTarget`; selection both ways; Alt+right drag orbit, wheel zoom change pixels; flow camera and node point unchanged | `__semioFlowGraphProbe[..].entity("handle"/"node", id)` | `🔍️browser-probe.ts verifyShellHover`, `🐍️interaction-matrix-probe.mjs` |
| 4-edit-undo-redo | Artifact panel row, inspector number field, mesh digest, fixture value; `mod+z`, `mod+shift+z` | `[id="panel:procedural-play-graph/height"]`, `[id$="procedural-play-inspector.value.input"]` | `🐍️editor-verbs-keyboard-probe.mjs`, keybindings `generation3d/🦀️.rs:459-460` |
| 5-graph-edit | catalogue add, drag sink to empty (disconnect), drag source to sink (connect), `Delete`, undo until baseline | `#framework.panel.catalogue` rows `catalogue.*`, handle rects, `delete,backspace` (`🦀️.rs:464`) | `🐍️react-gap-probe.mjs`, `🐍️wire-type-guard-probe.mjs` |
| 6-lazy-extension | `No example` -> `Sphere Box Fuse`; MutationObserver (installed before app code) captures the extension notice, then converges | notice regex from `react-lazy-activation-probe.mjs`, `window.semioReactActivateOnDemand` | `react-lazy-activation-probe.mjs` |
| 7-cancel-progress-latency | slowest example, `[data-slot=world-compute-cancel]` click, phase `cancelled`; progress ratios/text; pointermove input delay p95 under 200 ms and rAF after cancel | `world-compute-status`, `data-compute-*`, `data-status-json.cancellable` | `🐍️cancel-preview-probe.mjs`, chord `mod+.` |
| 10-add-generation-chord | `mod+shift+g` in generate mode adds one roster row, generate preview settles | `[id*="procedural3d-play-generate.generation."]` | gap 2; `🦀️.rs:465` |
| 11/12 | select in Rectangle Wire Preview and Hexagonal Mushroom Column by real hover/click; inspector `…procedural-play-inspector.id` names the widget | `data-selection-json.hoverTarget/selectedIds`, `#framework.panel.inspection` | gap 3; `🐍️interaction-matrix-probe.mjs` |
| 13-gumball-drag | select, require `gumballActive`, drag handle candidates, instances move; `missing` if `gumballActive` never published | `data-selection-json.gumballActive`, `data-instances-json` | gap 10; `translateSelection` `🦀️.rs:501` |
| 14-undo-rearm-after-edit | two edit+undo cycles must each restore the digest | as row 4 | gap 9 |
| 15-status-row-a11y-en-de | status hosts/pane carry aria label and live/status role in en and de, text differs | `[data-status-json]` attrs, `world-compute-status` | gap 11 |
| 16-fit-camera | orbit+zoom off, `[data-slot=world-frame-instances]`, `gradeCameraFrames` | `🐍️example-oracle.mjs` | gap 12 |
| 9-export-import-reload | `#action.exportDocument` + `#format` + execute -> download; `#action.importDocumentRequest` file chooser with the export; reload and compare the flow document | `framework.window.proceduralMain.engagement.toggle` | `🐍️io-surface-probe.mjs`, `mod+shift+e`, `mod+o` |
| 8-locale-keyboard-a11y | en<->de via `#framework.settings.language`; Tab sweep reaches every window; mode chord works; CDP accessibility tree and DOM interactive elements all named | `Accessibility.getFullAXTree` | `🐍️i18n-a11y-customization-probe.mjs` |

Row order in the run: 1,2,3,4,5,6,7,10-16,9,8 (locale last).

## Limits
Row 3/13/16 aim points and the hover/selection value shapes follow the cited probes but were written while the page could not boot; first live run may need calibration. Examples without a committed oracle (demo, mesh-workbench) are checked for convergence, changed digest and node states only. No product code was changed.
