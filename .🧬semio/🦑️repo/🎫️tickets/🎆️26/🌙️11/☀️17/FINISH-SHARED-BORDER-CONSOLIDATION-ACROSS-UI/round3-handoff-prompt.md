# 🔁 Round 3 Handoff Prompt

Continue the open ticket `26/11/17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI` (do not open a new one; it is already open). Ticket folder T = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/FINISH-SHARED-BORDER-CONSOLIDATION-ACROSS-UI`.

Goal: finish the migration from individual element borders to the consolidated shared-border design system end to end — every seam between bordered things is ONE line; no gap-separated bordered siblings; no bordered element inset in another (radical rule in T/brief.md). Fix unrelated fundamental issues you hit. Coordinate from the main chat (Opus), run medium/high work in parallel Sonnet agents, read-only audits in Haiku agents; never idle, never get stuck — fall back to native tools when `repo`/fleet tooling fails.

Read first: T/brief.md (rules, probe, native vite startup), T/SUMMARY.md (rounds 1–2), T/round3-status.md, then the round-3 reports: round3-framework-fixes.md, round3-elements.md, round3-plugins-fixes.md, round3-os-fixes.md, round3-staffage-fixes.md, round3-course.md, round3-apps-open-defects.md, round3-native-fixes.md, round3-live-verification.md, round3-diagram-force.md, and audits round3-audit-*.md.

## Done in round 3 (code + tests; mostly live-probed 0 at 1440x900 and 375x812)
- Framework: `data-shared-border-edges` drives native border sides (hacks removed); Popover/Select/Toggle dock flush to the frame's outer edge; Engagement/WindowSearch/TableAvatar/Tree/LayeredOverview/intro chips/drop indicators/Input/Textarea/TextEditor/DiffView; `--chip-row-height` token (title/footer chip rows + panel header); scroll bodies hide native scrollbar gutter; SharedBorderRow/ToggleGroup `wrap`; all 126 element stories probe 0.
- Plugins: CAD REPL aside + attribute pane, GIS inference, dev stories. OS: staged-args, Paint2d, ShellHost bands/stall stack, TiledMap/NodeGraph, time-travel chip, staging dialog. Staffage: stage toolbar, notices, collections, tables, playground. Hub admin 375. Demonstrator intro docking. Play 375. Presentation arrows/PDF nav.
- Native: kit Field/Section/stacks gapless, nativeSeams consumed in Rust, TUI ├┤ junctions, CardStack layout/paint (light crate).
- Fixed unrelated: Diagram force tests (harness import order), tui-laws compile, UIIntroduction/aggregator test collection, scroll-area import.

## Still open (do these)
1. Card interlock seams 2px apart (lift ignores the 1px line inset): fix inset-aware lift in shared `bands.cases.json` + `bands.ts` + Rust `bands.rs`/tests (single source of truth). Visible in course `#learner` at 768 and physics quiz page at 1440.
2. Finish course crawl with T/round3-course-crawl.js: all quizzes, easy/hard/expert runs, submit dialogs, results, `de`, at 1440/768/375; OS + staffage at 768.
3. Native: wire `paint_card_stack` into new retained OverviewCard/CardStack nodes (schema-first); fix `semio-ui-draw-contract` over line limit (4247/4000, split by domain), 4 failing `semio-ui-tui-laws` chrome/docking/focus tests, hosting test `real_settings_fields_inherit_labels_and_explicit_control_names_win` ("UI arena has no accepting lifetime registration").
4. Pre-existing staffage `catalog-loading` failure; Playwright Chromium not installed (4 Tree/Layout browser tests) — install only via repo tooling with user approval.
5. Final: one independent re-probe pass of every app (course, staffage, demonstrator, play, presentation, hub admin, OS harness, CAD harness, elements) at 1440/768/375; update T/SUMMARY.md with a Round 3 section; delete T/🗑️generated; close the ticket with `ticket_close` listing all changed files.

## Environment notes
- `repo run` is broken; start vite natively per brief (`SEMIO_BUILD_MODE=prototype`, cache under `.cache/b/p/m/<name>`), run vitest via ticket-local configs (`T/vitest-*.config.ts`; `vitest-ui-all.config.ts` includes the owned-locale aggregator). Rust via `repo cargo ... test` works.
- Browser: use the in-app browser, each agent its own tab (`tabs_create`, pass tabId). The pane must be SHOWN and the tab fronted (`tabs_select`) before each probe — hidden pane stops requestAnimationFrame so the compositor never commits (`pending:true`, bogus pairs). Only trust `pending:false`.
- Headless Chrome (T/headless-probe.mjs) aborts in this environment (FATAL path_service 1001) — don't rely on it.
- Agent reports: T/save-agent-report.py extracts a subagent's final text into a markdown file.
- Others edit the same files concurrently: re-read before editing, never revert others, no modifying git commands.
