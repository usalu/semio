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

### Active E2E Ownership Handoff

Parent assigned acceptance/asset/browser triage for existing E2E session 98132. A single attempted cross-agent `write_stdin` returned `Unknown process id`; no output was consumed and no process was launched, restarted or stopped. Runtime retains the agent-scoped native service handle, complete stream append to `🗑️generated/e2e-contract.log`, and admission/browser-start/terminal notifications. This agent owns the shared-log and acceptance artifact triage. Unit execution remains Runtime-owned; infrastructure batching remains parent-owned.

Runtime confirmed session 98132 admitted one job / 3,584 MB at 14,029 MB available with the unchanged 6,553 MB floor. It is in selected native preparation / command-source imports. No Playwright start, browser assertion or pass is claimed. Current source still declares all 20 acceptance cases and captures a short PNG for each successful pane boot/content case and document-history case. The actual ticket artifact root is 153 characters; adding the declared 102-character reserved suffix totals 255, within the 256-character policy. The runner uses one Chromium worker and the authored real GPU adapter configuration. No assertions, fixture expectations or GPU settings were weakened.

## Retained E2E Diagnostic Readiness

E2E 98132 remains owned by runtime_execution for native polling, because native handles are scoped per agent. This agent owns bounded reads of generated/e2e-contract.log and browser failure triage. No duplicate browser command was launched. Latest observed stream has source-closure/import preparation only, without a browser runner header or case output.

The current twenty-case source and its automatic health fixture were reread without edits. First live evidence should be classified in this order:

1. Authored prepare/serve generation and `Testing Demonstrator at` URL, followed by the actual Playwright test header. Earlier graph/compiler failures are pretest infrastructure failures.
2. Artifact path/title assertions execute before each browser case body; an error here belongs to the existing bounded artifact contract.
3. Per-pane readiness outcome and each declared window’s real surface content. Preserve all content expectations, including independent Koordinator windows and Generator preview. An empty scene after ready needs actual guest/runtime diagnosis; it does not justify lowering the assertion.
4. Visible control mutation, actual result projection, undo and redo. Statik requires settled zero geometry before PageUp and settled changed geometry thereafter; its current source retains that synchronization. Energie status must come from real transport accessibility output.
5. Automatic page/console error teardown applies to every case, including selection and funding. A PNG attachment alone does not imply case success when health teardown fails.

Short `p.png` screenshots are captured and attached for each successful boot-content body and history workflow. Parent can visually inspect retained pixels after actual execution. No screenshot, console result, completed workflow or twenty-case pass is claimed before the browser starts.

Runtime execution later reported one E2E progress burst of 61069 tokens exceeded its 60000-token capture limit before any browser start; its received middle was truncated and the log has an explicit gap. Retained E2E logs cannot now be described as a complete process stream. Our separately owned Source-demand uses direct stdout/stderr file redirection, so that stream does not depend on tool chunk size. No browser pass or failure is inferred from a missing progress region.

## E2E 98132 Terminal Evidence

Runtime_execution confirmed native handle 98132 terminated with exit 1 after parent-directed SIGINT of this own obsolete pretest job, ending with `Original Script process interrupted`. The parent decision used known module initialization before subsequent source saves; no process-specific emitted hash mismatch is claimed. The received final chunk is retained in generated/e2e-contract.log, whose earlier capture-gap note remains. No browser runner, served acceptance assertion, screenshot completion or twenty-case result occurred in this invocation. Runtime unit 45442 and our Source-demand 46731 remain retained; no replacement E2E was launched by this agent.

## Finite Current Acceptance Review During34653 Queue

Read-only review of the current20 expanded browser cases found no confirmed selector/fixture mismatch requiring an edit. The20 titles exactly equal the artifact fixture roster, are ASCII and at most24characters. Automatic browser health remains installed for all cases; exact404 resource-only suppression remains strict. Eight boot cases retain every window/content check, and nine PNG call sites include the eight-pane boot loop plus interaction completion captures. FEM still waits for settled zero geometry before PageUp and then a changed nonzero scene before the scale baseline.

Re-read29 authored Rust sources from the current example/mutation inventory, seven current simulation/tool-run sources, eight current DOM projection/identity owners, the procedural and puzzle DSL assets, curation stock DSL, brand, spec and four neutral fixtures. Current renderer still emits tree-label/tree-description, numeric stepper controls, action-pane/engagement IDs, meshes/instances and node-graph snapshot attributes. Per-shell readiness still requires session plus initialExampleReady. Funding remains the third landing-tour step with three authored logos, matching the twoNext actions.

Current procedural height6/min0/max10/step0.5 permits6.5 and the seven widgets/six edges remain the asserted fixture. CAD catalogue still binds addObject and persisted creation. Puzzle retained publish calls the real create_object mutation helper and selects the minted ID; the absence of a literal CreateObject token in this helper is not a behavior defect. Its seed/kind/mesh asset identities match the fixture. Process row action and inspector fields still expose enabled and origin identifiers, with setStepEnabled as a mutation. GIS addFeature still targets positions with default lon/lat0 and CreatePosition. Curation exact beam ID/name/availability24 remain authored. Energy real tool ID, framework transport action/status/progress/step IDs and no-result wording remain consistent. FEM current_revision captions and deformationScale action/control remain consistent.

Historical audit references to simulation-session/🦀️.rs no longer resolve at that local folder: it currently holds the JSON contract/tests, while editor source explicitly links semio_s_artifact_energy_model_session as session. This changes the research path inventory, not an observed acceptance selector or fixture expectation. Its engine implementation was not exhaustively traversed in this finite review. Actual browser rendering, tool scheduling, mutation result/history and screenshots remain unexecuted; source review is not a passing E2E result. No Source edits, native commands or application actions were made by this review.

## Energy Lifecycle Behavior Correction

The finite behavior review found a concrete error missed by the earlier selector-only audit. Actual framework complete_tool_run_job in OS plugin/tool-run/instance/job-admission-and-finalize-turn emits JobComplete then, for mutating:false, immediately emits Finalize in the same method. The authored Energy run contract is mutating:false. A persistent Complete state plus manual Finalize button is therefore not its workflow. Framework action legality permits Abort in Starting/Running/Paused/Complete/Finalizing, and terminal groups expose Start for a fresh run.

Saved neutral schema/cases and unit oracles before the browser change. Energy completion is now automatic; stable Finalized status, final quality-tier step and accepted read-only result text are all required. The second run waits for a distinct nonempty numeric group and captures that identity from the successful poll before immediately targeting its enabled Abort button. An intermediate empty panel can no longer satisfy a not-equal-to-old-group predicate. It still requires actual Aborted and no-result text; natural completion is not accepted as cancellation. No runtime semantics, numerical setting, action API or product instrumentation changed.

Six language-neutral restart cases cover actual/new, empty, same, nonnumeric, child-ID and initial groups. A pure imported helper is compared with an independent Ajv schema reference. The existing console Vitest entry reads the real non-mutating contract and Rust auto-finalization branch, and TypeScript AST verifies that the Energy browser case asserts finalized/final-tier/accepted-result, uses fresh-group validation and has no manual finalize. The normal unit entry already includes this file; the helper is a derived static import. Runtime owns adding the two external text reads as explicit DATA resources on test/canonical targets to preserve their Source/freshness authority. E2E runtime roots and native prerequisites remain unchanged.

Browser health itself has no new confirmed defect: its automatic fixture installs pageerror/console-error listeners before each case and asserts both lists in finally; the anchored404 resource diagnostic exemption does not suppress other4xx/application/trace failures. All20cases retain it. Actual Energy run timing, host projection and cancellation remain pending served browser execution; no passing claim is made.

Actual auto-finalization method begins at source line81. No native invocation was launched for this correction.
