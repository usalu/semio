# Acceptance Hardening 2026-10-21

Parent owns ticket lifecycle and browser execution. No product runtime or manifest edits.

## Implemented

- Shared automatic Playwright fixture collects page errors and console errors for every browser test, including selection, funding, and new curation workflow. Teardown asserts both lists even when the body fails.
- Only the exact browser resource-404 diagnostic is exempt. HTTP 400–403/405–409, server failures, TRACE errors, application messages containing 404, and suffix-appended failures remain significant. Fifteen neutral JSON cases validate the language-independent contract with Ajv and compare the custom classifier with a separately expressed Ajv enum oracle.
- Aussuchen selection now requires authored demo beam-glulam-gl24h and its exact preview instance ID, instead of accepting whichever row happens to be first.
- New neutral curation scenario exercises visible increment, keyboard increment, removal, undo and redo. It asserts the pool and curated count, exact row identity, expected grid instance multiplicity, and complete instance projection restoration at each previously observed count. Grid default line-behind and single-box beam geometry were verified in production Rust source.
- Extended curation with real overview/card navigation through Bearbeiten, a 31-second offscreen interval (beyond the pristine-pane 30-second suspension threshold), revisit state assertions, and continued redo/undo. Dirty-pane retention is triggered by actual visible controls.
- Every successful deep-link pane case attaches its rendered shell PNG to Playwright results, plus the curation case after revisit. The E2E owner places those results under the ticket generated output.
- Removed stale suite header claiming obsolete local boot hooks.

## Evidence

Focused test command: `repo run @semio-tech/mit-bestand-demonstrator:test -- short ./🧪️tests/🎭️acceptance/🧬️console/🧪️.ts`. The command remained at host capacity admission (3584 MB required; negative spare above 6553 MB floor; load up to 15.05) and was cancelled using SIGINT after the parent queued the full test owner. It never executed the initial red regression. Parent full unit route covers the new neutral tests. Browser execution remains owned by runtime_execution. No passing unit or browser result is claimed in this report.

## Source Basis

Sourcing demo DSL declares beam-glulam-gl24h, Glulam GL24h 200×400, availability 24, and empty curated rows. Pool production UI emits a keyboard-operable spinbutton with data-stepper-control increment. Curated production UI exposes Remove/Entfernen. Global shell keyboard route handles Control/Meta+Z and shifted redo after a non-input click. Grid configuration defaults to line-behind and kind_instances_json gives one box instance per count for this beam.

## Remaining Ticket Scope

This adds real mutation/history fidelity for Aussuchen. It does not establish every other pane’s edit, simulation, cancellation, persistence, round-trip, or simultaneous eight-pane readiness. Those remain unverified until source-specific workflows and served browser evidence are completed.

## Changed Files

- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎚️config/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️console/🔣️.json`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️console/🧫️cases.json`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️console/🟦️.ts`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️console/🧪️.ts`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️curation/🔣️.json`
- `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️curation/🧫️cases.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/acceptance-hardening-2026-10-21.md`

## Energie And Statik Workflows Added

Execution remains pending under the parent-owned admitted routes. No new command was queued for these tests. The suite now declares 15 browser cases. Neutral simulation JSON and schema are validated by the existing Ajv/Vitest acceptance-contract include.

Energie selects the authored tool tab through actual progressive dock chrome, focuses its simulation tree, and uses the framework mod+enter chord. It discovers real numeric run IDs from production DOM transport identities. It asserts a real progressbar and live announcement attribute, Complete status plus final result tier, finalizes the run, then starts a separate run through its real Start button and aborts through its real generation-bound Abort button. It asserts Aborted status and the plugin’s no-result-accepted projection. It attaches a rendered finalized screenshot. Static tree text mentioning aria-live/status is never treated as real transport semantics.

Statik first requires exact unsuffixed Case: dead. It opens the production Results tab, verifies paused playback, sets phase 0.1 using the native Home/PageUp slider keys, and waits for that phase to reach geometry. It changes the document-owned deformation scale 300→600 using the authored blur-commit number input, then exercises undo and redo. Each step requires changed world geometry followed by the exact current-results caption with no pending/fault suffix; revisiting a known scale must restore the full meshes-plus-instances projection. Reading both geometry lanes avoids assuming deformation lives only in instances. It also confirms phase/playback stay fixed and attaches a final rendered screenshot.

Source basis: closure-audit-2026-10-21.md Minimal Energie And Statik UI Workflows; production ToolRun panel-groups Rust, PanelTabBar data-tab-id and aria-pressed controls, tool-tab activation reconciliation, FEM Results panel and result rendering. Additional verified defaults: FemResultsAnimation defaults to phase=1, playing=false, waveform=Ramp. Slider PageUp without detents advances ten 0.01 steps.

Additional files: `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️simulation/🔣️.json` and `🧫️cases.json`. Existing acceptance spec and acceptance contract test updated.

## Review Correction

The independent review identified optimistic slider aria state could let the FEM phase comparison capture the intermediate zero-phase scene. The test now waits for zero-phase geometry first, captures that projection, then waits for PageUp geometry to differ from that settled zero projection before scale/history comparisons. Execution remains pending.

## Remaining Five Document Workflows Added

The acceptance suite now declares 20 short ASCII browser titles and the artifact path fixture lists all five new titles (each at most24 characters). Every new case uses the shared strict browser health fixture and capturePaneArtifact, preserving bounded persisted PNG paths and attachments. No new command queue or product runtime changes. Actual execution remains pending under the parent-owned routes.

- Generator: authored seven widget/six synapse IDs; connected height6→6.5 via tree selection and blur-committed inspector input; graph+evaluated geometry change; undo/redo restore canonical widget/synapse payloads and geometry.
- Koordinator: visible catalogue Box activation in focused Shape pane; capture actual minted instance ID and matching raw object row/Box label; undo/redo restore the ID/geometry; other three pane projections stay unchanged.
- Aggregator: catalogue click on authored forest-left kind; one actual minted object ID appears in both projections and artifact tree; mesh reference matches the authored seed and 11 copied vortex records identify the new object; undo/redo restore both window projections and row presence.
- Bearbeiten: toggle the authored wall-saw-cut enabled action, retaining all seven original step IDs and proving selected step identity/label/origin fields. Inspector true→false and workpiece geometry changes; undo/redo restore both. No stock-reset operation is used.
- Verfolgen: visible window Actions rail stages a labelled real feature at the authored default coordinates and executes Add Feature. Position count increases by1 and round-trips through undo/redo; route/region counts stay unchanged. Counts are the supported public document projection; no hidden map probe or unsupported fabricated feature selection is used.

Geometry helpers discard only ephemeral selection/hover/highlight paint. They retain document geometry and wait for the production compute status to stop being busy and reject cancellation. All five recipes are grounded in acceptance-audit-2026-10-21.md. New neutral inputs/schema reside under acceptance/🧬️edits; Ajv validation is included by the existing acceptance-contract Vitest file.

Additional changed files: `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧬️edits/🔣️.json`, `🧫️cases.json`, and `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json`. Acceptance spec and neutral contract test updated. Parent’s FEM Home→zero geometry→PageUp synchronization fix is preserved.

Static fixture inspection:20 unique ASCII titles, maximum title length21; newly added source paths are98–103 UTF-16 code units (all below256). This is data inspection only, not a passing unit/browser run.

Independent read-only review completed by closure_exploration; details in closure-audit-2026-10-21.md. No confirmed source blocker found. In particular, new puzzle object is explicitly selected by the retained command, and touches_object visibility publishes its11copied vortices; CAD raw ID/Box label, process fields, generator value prefix, GIS defaults and20-title roster were source-checked. This remains source evidence; runtime execution has not passed.

## Continuation Readiness Review

Read-only inspection of current source after the 20-case implementation. No source mutation was needed and no additional build/test process was queued. The strict health fixture, bounded capturePaneArtifact helper, short title roster, all five new history workflows, and parent FEM Home→zero geometry→PageUp synchronization remain present. Every neutral JSON/schema parses as JSON; this is source inspection, not Ajv or browser execution.

Current Vitest include explicitly discovers acceptance/🧬️console/🧪️.ts at every test level, including short. That file declares 19 examples:15 console cases plus console schema, curation schema, Energy/FEM schema, five-edit schema. Its Ajv oracle independently classifies the console examples; only exact browser-resource404 errors are exempt. Playwright testMatch remains the single acceptance/🟦️.ts file, one Chromium worker, real GPU default. The fixture lists20 unique ASCII titles (max21chars≤24; count≤32).

### Exact Acceptance Source Inventory

All paths below are relative to ♻️mit-bestand/🧺️demonstrator. Parent owns additional artifact helper/schema, runner, and build-owner changes.

- `🧪️tests/🎭️acceptance/🟦️.ts`
- `🧪️tests/🎚️config/🟦️.ts`
- `🧪️tests/🎭️acceptance/🧬️console/🔣️.json`
- `🧪️tests/🎭️acceptance/🧬️console/🧫️cases.json`
- `🧪️tests/🎭️acceptance/🧬️console/🟦️.ts`
- `🧪️tests/🎭️acceptance/🧬️console/🧪️.ts`
- `🧪️tests/🎭️acceptance/🧬️curation/🔣️.json`
- `🧪️tests/🎭️acceptance/🧬️curation/🧫️cases.json`
- `🧪️tests/🎭️acceptance/🧬️simulation/🔣️.json`
- `🧪️tests/🎭️acceptance/🧬️simulation/🧫️cases.json`
- `🧪️tests/🎭️acceptance/🧬️edits/🔣️.json`
- `🧪️tests/🎭️acceptance/🧬️edits/🧫️cases.json`
- `🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json`

Current line landmarks in acceptance/🟦️.ts (line numbers identify this review snapshot):

- `const test =`: line 42
- `async function capturePaneArtifact`: line 65
- `for (const paneCase`: line 485
- `test('aussuchen preview'`: line 549
- `test('aussuchen undo-redo'`: line 596
- `test('energie run-cancel'`: line 699
- `test('statik undo-redo'`: line 741
- `test("generator undo-redo"`: line 839
- `test("koordinator undo-redo"`: line 884
- `test("aggregator undo-redo"`: line 912
- `test("bearbeiten undo-redo"`: line 947
- `test("verfolgen undo-redo"`: line 985

### First Live Diagnostic Routing

1. Capacity wait lines establish no unit/browser execution. Unit97994 and E2E83869 remain parent/runtime_execution-owned; do not infer test progress from elapsed admission time.
2. Before UI actions, browserHealth rejects overlength outputPath/outputDir or a long title. Diagnose artifact root/runner policy if this assertion fails; it is distinct from pane runtime.
3. Browser shell cases first wait data-shell-id and then ready/error/not-found. On a failure, inspect the exact shell error attribute and failing module/descriptor response in the Playwright error context/trace; do not relax content expectations or broaden routes.
4. A window attaches before its first real content publication. Per-pane empty-host or missing JSON/rows after the content budget points to that surface/fixture pipeline. Every successful boot retains p.png for visual fixture review.
5. A health teardown failure reports page exceptions separately from significant console errors. A400–403/405–409 resource failure or TRACE-prefixed error now fails by design. Funding and interaction cases share this check.
6. Generator: first edit gate is exact7widgets/6synapses, then height inspector input and evaluated geometry. CAD: minted Shape object ID/raw Box row plus three unchanged panes. Puzzle: same minted object in both views, matching forest mesh and11selected-object vortices. Process: all7steps, selected wall-saw-cut fields and enabled toggle, then workpiece geometry. GIS: real Actions rail AddFeature stage/execute and position-count delta with route/region invariants. Separate absent-control/focus failures from a committed mutation whose projection fails to update.
7. Energy: tool tab+window focus must yield a real numeric framework run, progressbar and live status. Complete alone is insufficient without the final tier. Abort is driven through its current generation-bound button. A static tree string saying aria-live is not transport proof.
8. FEM: exact unsuffixed Case: dead proves current initial/replacement results. Home geometry must differ from full amplitude before PageUp geometry differs from zero; only then capture phase0.1 baseline. Deformation scale300→600 and history must change/restore full geometry while retaining current captions and paused playback.
9. If source-level selectors fail in a served run, retain exact first failing locator/action and screenshot/trace, compare the authored production DOM contract, and fix the confirmed owner. No speculative retries, fake probes, or weakened tests.

No new evidence changes the next action: continue the already queued owner processes until native admission permits actual results, then diagnose the first concrete failure. No passing test/runtime claim is made.
