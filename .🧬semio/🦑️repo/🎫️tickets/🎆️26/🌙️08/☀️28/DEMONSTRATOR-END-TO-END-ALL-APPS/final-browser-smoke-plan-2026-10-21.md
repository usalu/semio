# Final Continuous-Service Browser Smoke

Use only the actual URL announced by the retained authored serve session, retaining that session throughout. No guessed port, fixture server, network stubs or substituted static response. Attach pageerror and console-error capture before first navigation. Require zero pageerrors; use the production E2E significantConsoleErrors helper: its ONLY exception is the exact browser `Failed to load resource: the server responded with a status of 404` diagnostic (optional ` (Not Found)`, case-insensitive). Do not ignore arbitrary 404 text, WASM, plugin or network errors.

Open actualURL `/`, dismiss `[id="ui.introduction.skip"]` by click or documented Escape fallback and prove disappearance. Expect eight `[data-layered-card]` cards. Each `[data-layered-card="<pane>"] [data-overview-card]` has nonempty title/tagline, two introduction paragraphs and `[data-slot="demonstrator-pane-card-open-chip"]` text `Demonstrator öffnen`. The authored brand explicitly locks German (`DEMONSTRATOR_LOCALE="de"`); this smoke should verify German, not assume an English default or invent a language switch. Use language-neutral IDs for controls; bilingual Remove/Entfernen is available only where the actual E2E uses it.

For each hash below, wait up to120s for exactly one `[data-shell-id="<pane>"]` and its data-shell-ready beacon; data-shell-error or data-shell-not-found fails. Dismiss pane-scoped introduction. Read actual scene output with up to60s for first content; readiness alone is insufficient. Scope every ID selector to its pane and accept exact id OR data-element-alias token. Window IDs below already apply the production camel-case conversion.

| Hash | Required windows/content |
| --- | --- |
| #generator | framework.window.proceduralMain node graph `.semio-node-graph-host` data-host-snapshot-json widgets nonempty; framework.window.proceduralPreview world scene nonempty |
| #koordinator | framework.window.cadPlayShape, cadPlayBuilding, cadPlayEnergy, cadPlayStructureClassic; each world scene nonempty |
| #aggregator | framework.window.puzzle3dMainTop and framework.window.puzzle3dMainPerspective; two alias matches for framework.window.puzzle3dMain; both world scenes nonempty |
| #energie | framework.window.energyModel3d world scene; framework.window.frameworkWindowTree role=treeitem; framework.window.frameworkWindowTable [data-row-id]; framework.window.energySimulation role=treeitem; all nonempty |
| #aussuchen | framework.window.sourcingPool table nonempty; sourcingCurated initially empty; sourcingPreview initially Keine Auswahl/No selection; sourcingGrid scene host exists but curated geometry initially empty |
| #bearbeiten | framework.window.processWorkpiece world scene nonempty |
| #verfolgen | framework.window.gis2dMain actual tiled map host and canvas paints more than one flat color; inspect production tile output, not a blank mounted canvas |
| #statik | framework.window.fem3dModel and framework.window.fem3dResults world scenes nonempty |

World content is `.semio-world-3d-host` with parsed data-meshes-json plus data-instances-json count>0, not canvas presence. Table is `.semio-table-host [data-row-id]`; tree is actual role=treeitem. Reuse exact authored PANE_CASES and helpers from demonstrator/🧪️tests/🎭️acceptance/🟦️.ts rather than weakened screenshot-only checks.

Minimal real interaction: on Aussuchen click the actual stock row selected by neutral curation fixture name/objectId in sourcingPool. Prove preview changes from no-selection text to a nonempty world host and its instance IDs equal that objectId. Return through `[data-layered-overview-button]`, open another pane through its visible card, then return to confirm shell navigation remains responsive. Existing authored tests additionally verify edits/history, Energy run/cancel and Statik update; this final continuity smoke does not replace those20 acceptance cases. No unverified menu ID is invented: generic command/menu interactions must reuse the current acceptance helper selectors if needed.

Retain short pane screenshots and console outcome in ticket generated output only. Capture final live actualURL and session ownership; do not close the service while reporting it as retained. This plan was source-reviewed, not executed.


## Current Execution Declaration Readback

2026-10-22 finite read-only check: demonstrator `📋️project.json` still declares uncached `test-e2e` → `serve-e2e`, runner `🔨️modules/🧪️e2e/📜️script.ts test`, no argument forwarding, config/spec runtime roots and brand DATA resource. Current `🔨️modules/🧪️e2e/🎚️config/🟦️.ts` mounts only `🧪️tests/🎭️acceptance/🟦️.ts`, one Chromium project and one worker; it requires the owned PLAYWRIGHT_BASE_URL and starts no substituted server. The authored twenty cases remain: roster/overview, eight pane boots, two sourcing interactions, Energy finish+distinct new-group abort, Statik deformation undo/redo, five remaining pane edit/history cases, and funding logos. Overview requires eight visible cards; boots verify visible authored windows, meshes/instances/node/table/tree content or painted map as applicable. Automatic browserHealth still captures pageerror and console errors in every browser case and applies the existing narrow significantConsoleErrors filter. Statik retains zero-phase→nonzero waits, current solved caption and exact projection restoration. Energy retains finalized status/final step/accepted result then distinct restart group, enabled abort and no-result status. No concrete coverage omission found; this is source readiness, not execution evidence. Normal continuous `serve` remains uncached, activate-prototype-dependent and authored runtime script; final smoke must use its actually announced URL as above.
