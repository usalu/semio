# Acceptance Audit — 2026-11-03

Static read only; no builds, tests, browser, or source edits. Prior reports are evidence of past runs, not current verdicts. Current project declaration: ♻️mit-bestand/🧺️demonstrator/📋️project.json. Official acceptance source: 🧪️tests/🎭️acceptance/🟦️.ts below that project.

## Recommended Declared Targets

1. repo run @semio-tech/mit-bestand-demonstrator:test — source-level assertions.
2. repo run @semio-tech/mit-bestand-demonstrator:test-runtime-catalog — catalog contract (depends on s-dev-composition:generate).
3. repo run @semio-tech/mit-bestand-demonstrator:activate-prototype — required union publication; depends on prepare-prototype and activate-lanes-prototype. The latter orders framework-os-dev:activate-energy-react-prototype, activate-fem3d-react-prototype, activate-generator-react-prototype.
4. repo run @semio-tech/mit-bestand-demonstrator:serve — continuous development service on 6029, depends on activation.
5. repo run @semio-tech/mit-bestand-demonstrator:test-e2e — official browser proof; orders serve-e2e -> prepare-e2e -> activate-prototype + workspace:deps-browsers. Do not launch raw Playwright: its config refuses missing PLAYWRIGHT_BASE_URL, which the declared consumer supplies.

## Current Coverage Facts

The acceptance spec asserts all eight pane ids against brand.ts, eight boots with surface content, sourcing selection/curation, energy run/cancel, FEM deformation/history, and generator/CAD/puzzle/process/GIS edit undo/redo. Automatic health fixture rejects page and significant console errors. English/German cover overview, funding, explicit language selection and switching. Language-switch-keeps-panes specifically verifies the koordinator shell survives switching. App boot and mutation cases inherit en-US; they are not repeated in German. Config declares only Desktop Chrome, one worker. No mobile/tablet project or multi-user/reconnect scenario is declared. The sourcing empty-preview placeholder is intentional and separately tested after selection.

Concrete fixture gap: 🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json enumerates 24 titles but omits the current spec title language switch keeps panes (spec line 546). Add this title to keep artifact path coverage complete. This is not proven to block runtime execution.

Concrete native runner drift: ticket 1102-tools/rtpanes/seq.sh still runs statik via --test editor_app_2 -- statik_pane, despite 1102-result-rtpanes.md saying it switched to dedicated statik_pane_smoke. The current FEM editor package Cargo.toml declares both editor_app_2 and statik_pane_smoke; use the dedicated target. Current package path: ✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/Cargo.toml. Older reports reference ✳️any and former package layouts; re-resolve paths rather than replaying scripts blindly.

## Latest Recorded Actionable Failures (Need Reproduction)

Source: 1102-result-rtpanes.md append-only latest entries. No pane is established green in the inspected latest reports; no current browser verdict is recorded in 1102-result-e2eready.md.

| Pane | Most recent recorded failure | Next focus |
|---|---|---|
| energie | s10 13:25, 0/2: WorkLimit, original action fixture dispatch exceeded authored turn bound while loading bestest-600 | Load-example action/effect-only producer; energy session and fixture caller admission |
| generator | s10 14:14 compile: flow host session node-graph-extras line 34 uses ? in non-Result closure | Reproduce flow closure compile; sequence migration failure before this is older |
| statik | editor_app and editor_app_2 stale test closures | Run declared dedicated statik_pane_smoke; script currently selects stale target |
| aussuchen | s10 15:30, 0/1: original return source exceeds admitting lifetime stock | OriginalReturnBudget::bind, job outcome continuation source; diagnose caller grant/spent stock, not raised limits |
| aggregator | s7 18:15, 0/1: puzzle3d-main-top renders no vortices in En | Puzzle top projection/render contract |
| verfolgen | s7 19:08, 0/1: document-load history-invalid Stale | Older shared hydration fault; energy/sourcing subsequently passed this point, so reproduce before modifying |
| bearbeiten | s9 11:48 compile: testing-laws renew_turn missing | Report itself says subsequently defined; reproduce, do not treat as current source defect |
| koordinator | s9 12:34 compile: flow vcs/schema/diff match-arm E0308 | Reproduce flow closure |

Cross-cutting publication: 1102-result-actinfra.md records act21 path length 259 UTF-16 units, then applied replication package rename reducing worst case to 241. Report says Cargo locks still need refresh. Parent must inspect current locks before activation; historical fault is not proof rename/lock remains unfinished. Earlier emitter-index bound fault is reported fixed. 1102-result-serveprep.md proves static union equality (32 components, three lanes), but expressly did not start a server. 1102-result-e2eready.md dry-load 8/8 is preparation, not served proof.

## Evidence Discipline

Do not use old generated rc files or mutable report STATUS headers as current pass/fail. Reports contain superseded failures followed by newer appendices. Confirm production activation, then run official acceptance to obtain current failures. The 1102 audit-smoke gaps on wrong sourcing target, fixture mismatch and schema publication were subsequently repaired according to rtpanes/testkit reports; they should not be reopened without checking current code. Native by-example redo and German body coverage require current law inspection before claiming missing coverage.
