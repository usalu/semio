STATUS: READY, no server run yet (marker serve-up absent at 13:39, act18 still running). Dry-load GREEN 8/8, tsc rc 0, all aud-e2e drift items resolved statically (no production fault). Browser action paths UNVERIFIED.

C = ticket folder. Run: `C/1102-tools/e2eready/run.sh http://127.0.0.1:6029/` (output `C/🗑️generated/1102/e2eready/{e2eready.txt,e2eready.json,run.log,run.rc,<unit>.png}`).

## Drift re-verification against current source (2026-11-02)
- Pane ids: dry-load asserts equality with `🪧️brand.ts` DEMONSTRATOR_PANES (8, same order).
- Window kinds and surface kinds: every kind in expect.json exists in `🌎️hub/🧩️compositions/🎪️demonstrator/🧩️component/🔣️.json` or its owning composition (fem3d-*, energy.* live in the fem/energy compositions). Surface kinds match descriptors (node-graph, world-3d, table, tiled-map); `sourcing-preview` is declared world-3d but is a placeholder text body on a fresh document, as the spec's PANE_CASES already says.
- Spec and probe window kinds identical (dry-load guard). No drift found in the acceptance suite; no D/ edits needed, no patches needed.
- Production selectors still present: `data-shell-id`, `data-tab-id`, `data-layered-card`, `data-layered-overview-button`, `demonstrator-locale-picker|switcher`, `world-compute-status`/`data-compute-phase`, `semio-world-3d-host`, `semio-node-graph-host` + `data-host-snapshot-json`, `data-instances-json`, `ui.introduction.skip`, `ui.mobilePanel.toggle`.
- Element ids derived at run time (inspector/transport/phase/engagement/addFeature) are not statically greppable; they remain identical to the ids the Playwright spec uses, and are proven only by the server run.
- statik German gap from 1101 is closed: caption is `labels.case_caption` (localized) and the case regex is `^(Case|Lastfall): dead$`.

## Probe changes (C/1102-tools/e2eready/probe.ts)
- Compact layouts: `docTab`/`paneTab` open the merged panel via `ui.mobilePanel.toggle` (the navbar "Panel" Toggle) when the tab button is not visible; `E2E_VIEWPORT=WxH` selects a phone/tablet viewport (default 1600x1000).
- GPU: real adapter first (`--use-angle=metal` on darwin, `gl` elsewhere, `--enable-gpu --ignore-gpu-blocklist`). The run prints `GPU adapter: <renderer>` and flags SOFTWARE adapters (SwiftShader never boots the wgpu TiledMapHost); SwiftShader only via `DEMONSTRATOR_E2E_GPU=swiftshader`.
- run.sh default output moved to `🗑️generated/1102/e2eready`.
- Official `test-e2e` config (`🔨️modules/🧪️e2e/🎚️config/🟦️.ts`) already uses metal first.


## aud-e2e drift items (C/1102-audit-e2e.md), verdicts by static source reading 13:10
- energie wording "Aborted, nothing was changed" / "Abgebrochen, nichts wurde geändert", "Finalized" / "Abgeschlossen": CONFIRMED in `🧰️framework/🔨️modules/⏯️tool-run/🏷️actions-labels/🟦️.ts:84-86` (framework tool-run labels, not the energy editor; "Finalized" also in the energy simulation window). No drift.
- `demonstrator-pane-card-open-chip`: CONFIRMED composed: `demonstrator-card.tsx:35` passes slot `demonstrator-pane-card` to `OverviewCardOpenChip`, which renders `data-slot={`${slot}-open-chip`}` (`🃏️OverviewCard/🟦️.tsx:134`). No drift.
- `demonstrator-pane-card-tagline`: CONFIRMED literal in `demonstrator-card.tsx:37`.
- `data-window-silhouette`: CONFIRMED in `🗂️WindowChrome/🟦️.tsx`, LayeredOverview and `🎨️.css`.
- `framework.category.tool`: CONFIRMED `FRAMEWORK_CATEGORY_TOOL_ID` in `🛠️ShellHelpers/🔩️plumbing/🟦️.ts:22` (outside `🔨️modules`, which is why the audit grep missed it); `framework.category.display` in `🛂️manifest` (`PanelTabKind::DisplayCategory`). Tool tab leaf id `tool.<toolId>` from `buildToolTabs` (`command-registry/🟦️.tsx:585`), so `tool.energySimulation` is composed correctly.
- Aggregator catalogue id: the audit misread; `🧬️edits/🧫️cases.json` has kindId `HexagonalCutConcreteForestLeft` (NO spaces), used as `[id=...]` by spec and probe. The string occurs in the block stories fixtures; the tree-row id equality is UNVERIFIED until the server run (identical selector in the spec). Aggregator = puzzle3d (`puzzle3d-main*` windows).
- `gis2dMain.engagement.toggle`, `…action.addFeature.execute`, `fem3d-play-results.deformation-scale.control` / `.phase.control`, `procedural-play-inspector.value.input`: composed by framework; no literal in plugin sources. UNVERIFIED statically; identical to the ids the Playwright spec uses; first proof is the server run.
- No production fault found; no patches.

## Server run
Not yet. Marker `C/🗑️generated/1102/serve-up` absent at 13:10 (act18 running per brief addendum).

## Window batch B11 (ts2d booleans) — SKIPPED on purpose, marker created
- Diffed targets vs `1101-tools/ts2d/patches/` before applying: `booleans.ts` and `booleans.test.ts` are the old UnitRetirement versions (untouched since Oct 17-18); `paths.ts` differs from the patch only in lines 73/91/97 (successor, not stale). So a whole-file copy would have been safe against lost edits.
- Why skipped: the demonstrator closure does not use `framework/2d` TS at all (no import in `🎪️demonstrator` composition or `♻️mit-bestand/🧺️demonstrator`; Rust BooleanJob is already funded). The patch REMOVES `BooleanJob.advanceWork`, `BooleanOptions` and changes `intoRetirement().job` to a ledgered cursor, but the live consumers `✏️s/🔌️plugins/🖍️draw/.../🎬️scene/🎨️paint/🟦️.ts` (lines 37, 84) and `📍️placement/🟦️.ts` (line 30) still call `advanceWork(1)` and type the job as `WorkRetirement`; applying would break the draw plugin. Porting draw's scene jobs to caller-funded ledgers is a separate, larger lane (draw), not window work.
- Current tree check: `bun test ./…/🔀️booleans/🧪️tests/🟦️.ts ./…/🛤️paths/🧪️tests/🟦️.ts` = 58 pass, 0 fail (`🗑️generated/1102/win-B11-before.txt`). No file changed.
- Follow-up for a draw lane: move draw paint/placement to `advance(items, ledger)`, then run `1101-tools/ts2d/patches/apply.py` (diff first).
