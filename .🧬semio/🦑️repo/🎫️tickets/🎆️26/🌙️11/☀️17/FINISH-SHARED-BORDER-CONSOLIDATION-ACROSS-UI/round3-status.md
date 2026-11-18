# 📍 Round 3 Status (2026-11-18 02:45)

## Done (code + tests, see linked reports)

| Area | Report | State |
|---|---|---|
| Audits: framework, plugins, sites, OS/products, native | [framework](round3-audit-framework.md), [plugins](round3-audit-plugins.md), [sites](round3-audit-sites.md), [os](round3-audit-os-products.md), [native](round3-audit-native.md) | complete |
| Framework root causes: `data-shared-border-edges` drives native sides, Popover/Select/Toggle dock flush (0 offset), Engagement/WindowSearch/TableAvatar/Tree/LayeredOverview/intro chips/drop indicators/Input/Textarea/TextEditor/DiffView | [round3-framework-fixes.md](round3-framework-fixes.md) | code + tests green; not live-probed; 2 suspect test failures open |
| Course: load-fallback, quiz actions, native borders on frames, access chip, presentation actions/reset; `--chip-row-height` token unifies title/footer chip rows (2px near-double seams) | [round3-course.md](round3-course.md) | code + tests green; crawl not re-run |
| Staffage: stage toolbar/progress, notices, collections edges, detail tables, Notice plain actions, ModelFigure frame | [round3-staffage-fixes.md](round3-staffage-fixes.md) | code + tests; toolbar probed 0 at 1440 |
| Staffage playground rows, hub admin 375 (tabs → Select, flush action cells, `ADMIN_ACTION_CELL` hack removed) | [round3-apps-open-defects.md](round3-apps-open-defects.md) | probed 0 |

## In flight (interrupted by session restart, being resumed)

- CAD REPL aside / attribute pane / GIS inference + SharedBorderRow wrap mode.
- OS product: staged-args chips, Paint2d panel, ShellHost bands, stall band, TiledMap/NodeGraph, time-travel chip.
- Native: kit Field/Section gaps, stack default gap, native CardStack, `nativeSeams` in Rust, TUI `├┤` junctions.
- Demonstrator intro docking, play landing 375, presentation selection.
- Navbar/footer vs window 1px pairs, settings panel scrollbar gutter, ButtonGroup 1px overflow, element live sweep (two stalled workers stopped; relaunching).

## Problems

1. **In-app browser unreachable** since ~22:30 (`PreToolUse hook did not respond`): blocks the live probe gate. Fallback `headless-probe.mjs` (playwright-core + system Chrome) is written but Chrome 157 aborts in this process context (`FATAL path_service 1001`), also unsandboxed. Next: retry pane; else run Chrome from the user's terminal context.
2. Session restart stopped 5 workers mid-task; their transcripts persist, resuming.
3. `repo` runner still unusable (round 2 finding); all builds/tests run natively via ticket-local vitest configs.
4. Pre-existing failures seen: staffage `catalog-loading`, `chrome-panel-safe-area` (296 vs 300), Canvas `shared-border-dock`.

## Anticipated

- Code work for all audit findings lands within the next worker cycle (~1–2 h).
- Final gate (probe count 0 at 1440x900 and 375x812 across all apps) depends on browser access returning; until then results stay "code + unit tests, not live-verified".
