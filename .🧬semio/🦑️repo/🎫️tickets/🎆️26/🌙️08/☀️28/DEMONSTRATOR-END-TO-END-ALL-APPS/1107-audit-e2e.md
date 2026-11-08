# 1107 Audit e2e (lane aud-e2e)

STATUS: DRIFT FOUND. No e2e selector, role, data attribute or fixture-bound label was removed by commit 0b2d54d02c4. Two harness/data drifts block a full en+de run as briefed (F1, F2). Static only, nothing was run.

Read-only audit. No source or harness file was edited. Only this file was written.

## 1. Scope and method

- Commit 0b2d54d02c4 touched 536 files. 98 of them are under `🧰️framework/🔨️modules/🖱️ui` (confirmed with `git show --stat`).
- Checked against the working tree at audit time (peers edit concurrently; snapshot about 03:30 on 11/07).
- Sources: `C/1105b-tools/e2eready/{expect.json,probe.ts,run.sh}`, `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts` (SPEC, 1131 lines) and its case files `🧬️edits`, `🧬️simulation`, `🧬️curation`, `🧬️console`.
- Producers: UI `🧰️framework/🔨️modules/🖱️ui`, renderer `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer`, demonstrator `♻️mit-bestand/🧺️demonstrator`, plugins `✏️s/🔌️plugins`.
- Method: one `git grep -F` token scan per tree (scan files in `C/🗑️generated/1107/`), then targeted reads of each producer.
- Verdicts: OK = producer found at the stated line and text exact in en and de. PARTIAL = producer found, runtime condition decides the value. UNVERIFIED = producer not located or format not confirmed. DRIFT = expectation does not match current source.

## 2. Drift list (file:line, current value, correct new value)

### F1 DRIFT (breaks de action for koordinator)

- Expectation: `C/1105b-tools/e2eready/expect.json:95` `"rowText": "Box \\d+"` (single language).
- Consumer: `C/1105b-tools/e2eready/probe.ts:259` uses the value verbatim for both languages.
- Source: `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🗣️terminology/🦀️.rs:24` `typology_box: native_en "Box", native_de "Quader"`.
- Correct new value: `"rowText": {"en": "Box \\d+", "de": "Quader \\d+"}`, and `probe.ts:259` selects `rowText[ctx.language]`. Row text is taken as the typology label plus an index. The index format is not runtime-verified.

### F2 DRIFT (harness does not act in de by default)

- `C/1105b-tools/e2eready/probe.ts:819` `index === 0 || process.env.E2E_ACTIONS_ALL_LANGUAGES === "1"`. `run.sh` does not set the variable.
- Brief requires "act (with undo/redo) in en and de". The default run acts only in en.
- Correct: run with `E2E_ACTIONS_ALL_LANGUAGES=1`, or change `probe.ts:819` to always act in every language (drop the `index === 0` gate). F1 must be fixed first, or the de koordinator action will fail.

### F3 COPY DEFECT (not a selector drift, not a test failure)

- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🌐️locale/🔣️.json:21` `demonstrator.landing.title` has en `"Entwerfen mit Bestand · Demonstrator"`, identical to de.
- Correct en value: `"Designing with Existing Stock · Demonstrator"`. The probe compares against the catalog value, so no test changes.
- Completeness check passes today because both cells are non-empty.

### F4 CONDITIONAL DRIFT (statik paused text)

- Expectation: `expect.json` `panes[statik].action.paused` en `"Play"` / de `"Abspielen"`. Probe `probe.ts:384,403` matches with an exact regex (`^Play$`).
- Source: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📊️results/🦀️.rs:203-211`. Live mode renders `labels.play` (`Play`/`Abspielen`, terminology `🗣️terminology/🦀️.rs:98`). Non-live mode renders `"Play / Pause"` (`…/🦀️.rs:211`), which fails the exact regex.
- Correct: keep `Play`/`Abspielen` only if the served results window is live when probed (not runtime-verified). Otherwise the probe must accept `^(Play|Play / Pause)$` and `^(Abspielen|Abspielen / Pause)$`.

### F5 UNVERIFIED (energie run status separator)

- Expectation: `♻️…/🧬️simulation/🧫️cases.json` `"finalized": "^(Finalized|Abgeschlossen) ·"` and `"aborted": "^(Aborted, nothing was changed|Abgebrochen, nichts wurde geändert) ·"`. Probe `probe.ts:350,358` tests `text(status)` against them.
- Labels exist: `🧰️framework/🔨️modules/⏯️tool-run/🏷️actions-labels/🟦️.ts:84` (`Finalized`/`Abgeschlossen`), `:86` (`Aborted, nothing was changed`/`Abgebrochen, nichts wurde geändert`), and `🦀️.rs:186`.
- No producer of the ` · ` join was found in `⏯️tool-run` or `ShellHelpers/⏯️tool-run-panel` (`toolRunPanelNode(group, \`${group.key}.status\`)`, `…/⏯️tool-run-panel/🟦️.ts:68`).
- If the status text is the bare label, the regex must become `^(Finalized|Abgeschlossen)\b` and `^(Aborted, nothing was changed|Abgebrochen, nichts wurde geändert)\b`. Confirm on one live run before changing.

### F6 AUDIT DOC DRIFT (1105b-audit-e2emap section 4)

- The audit places `[data-row-id]` at `RND/📊️Table/🟦️.tsx:218,280`. The producer is `🧰️framework/🔨️modules/🖱️ui/🧱️elements/📊️Table/🟦️.tsx:218,280` (UI element, not renderer).
- The renderer `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🟦️.tsx:310,317` provides `.semio-table-empty` / `.semio-table-host`. Both are correct as cited there.
- Correct the audit path. No e2e change.

### F7 COMMENT ONLY (SPEC)

- `♻️…/🧪️tests/🎭️acceptance/🟦️.ts:362-367` documents a rename `data-fixture-json` to `data-host-snapshot-json`. Code at `:366` uses `data-host-snapshot-json`. No drift in code.
- Stale line references carried from the 1105b audit (ShellHost beacon region, `Tree/🟦️.tsx ~1936/2013/2088`, `World3dHost ~5063`) are comment-only. Not re-verified here.

## 3. Selector, role and data-attribute verification

Token scan presence (non-test sources) was checked per tree. Verdicts below.

| Expectation (SPEC / probe) | Producer (file:line) | Verdict |
|---|---|---|
| `[data-shell-id]`, `data-shell-ready/-error/-not-found` | renderer `🏛️ShellHost/🟦️.tsx` (shell root), `🏛️ShellHost/🖼️canvas-footer/🟦️.tsx:52-97` (beacon) | OK |
| `data-element-alias` | renderer `🏛️ShellHost/📋️panel-hosting/🟠️mode-windows/🟦️.tsx:165` | OK |
| `framework.window.<segment>` id derivation | `🖱️ui/🔨️modules/🆔️element-identity/🟦️.ts:18` and `🧱️elements/🆔️ElementId/🟦️.tsx:27` are byte-identical to probe `elementIdSegment` (`probe.ts:47`) | OK |
| `[id="ui.introduction.skip"]` | UI `🎯️targets/⚛️react/🎓️introduction/🟦️.tsx:304` (keybinding `:258`) | OK |
| `data-slot="introduction-info-box"` (stack), `-title`, `-chip`, `-drag`, `introduction-body-paragraph` | UI intro `:285`, `:295`, `:294`, `:298`; body paragraph in `⚛️demonstrator-card.tsx` | OK |
| Intro Next/Skip text | UI `🌐️i18n/🎓️guidance/🟦️.ts:204,206` (en Skip/Next), `:94,96` (de Überspringen/Weiter) | OK |
| Funding logos `img` in intro box | UI `introduction/🟦️.tsx:74-75`; brand `🪧️brand.ts:96-98`; asset files exist under `🖼️asset/🪧️logos/` | OK |
| `[data-layered-card=<id>]`, `[data-layered-overview-button]` | UI `🥞️LayeredOverview/🧵️pane/🟦️.tsx:228`, `🟢️overview-button/🟦️.tsx:12` | OK |
| `[data-overview-card]`, `[data-window-silhouette]`, `[data-window-silhouette-border][data-kind]` | UI `🃏️OverviewCard/🟦️.tsx:99,108`; `🗂️WindowChrome/🟦️.tsx:349,361,387,485` | OK (kind value = PARTIAL, runtime) |
| `window-chrome-chip-cap` (default slot), `window-chrome-body` | `🗂️WindowChrome/🟦️.tsx:421,442` | OK |
| `window-chrome-body-surface` | `🗂️WindowChrome/🟦️.tsx:561` (rendered only when `geometry.bodyRegion`) | PARTIAL |
| `ui-glass` class on chip and body-surface | `🔨️modules/🌈️surface-presentation/🟦️.ts:12`, used at `🗂️WindowChrome/🟦️.tsx:460-461` | OK (runtime class value PARTIAL) |
| `demonstrator-pane-card-title-chip` (+ `svg`), `-open-chip`, `-tagline` | `🃏️OverviewCard/🟦️.tsx:74,134`; `⚛️demonstrator-card.tsx` (`slot="demonstrator-pane-card"`, tagline span) | OK |
| `demonstrator-locale-picker`, `button[lang]`, no `aria-pressed` on picker | `♻️…/⚛️locale-picker.tsx:3,15` | OK |
| `demonstrator-locale-switcher`, `button[data-locale]`, `aria-pressed` | `⚛️locale-picker.tsx:38,44-45` | OK |
| `world-compute-status`, `data-compute-phase`, `aria-busy` | renderer `🌐️World3dHost/🎯️marquee-and-bridges/🟦️.tsx:126-159` | OK (`aria-busy="true"` only while computing with labels) |
| `.semio-world-3d-host` / `-empty`, `data-meshes-json`, `data-instances-json`, `data-vortices-json` | renderer `🌐️World3dHost/🟦️.tsx:98,103,110,112,117` | OK |
| `.semio-node-graph-host` / `-empty`, `data-host-snapshot-json` | renderer `🕸️NodeGraph/🟦️.tsx:206,279,288` | OK |
| `.semio-table-host` / `-empty`, `[data-row-id]`, `data-stepper-control="increment"`, `role=spinbutton`, `aria-valuemax` | UI `📊️Table/🟦️.tsx:218,280` (row ids); renderer `📊️Table/🟦️.tsx:180,194,200,310,317` | OK (path note F6) |
| `[role=tree]`, `[role=treeitem]`, `tree-label`, `tree-description`, `tree-item-row` | UI `🌳️Tree/🟦️.tsx:518`; `🌳️Tree/🍃️item/🟦️.tsx:208,274,282,346,406,487` | OK |
| `.semio-tiled-map-host` / `-empty`, canvas | renderer `🧭️TiledMapHost/🟦️.tsx:26,29` | OK |
| `[data-tab-id="framework.panel.<tab>"]`, `framework.panelTab.*` | UI `🧭️PanelTabBar/🟦️.tsx:428,431`; renderer `🏗️dock-assembly/…/🟦️.ts` | OK |
| `[data-tab-id="tool.energySimulation"]`, `framework.category.tool` | renderer `🛠️ShellHelpers/🎛️command-registry/🟦️.tsx:585` (`tool.${tool.id}`); `🛠️ShellHelpers/🔩️plumbing/🟦️.ts:22` | OK |
| `ui.mobilePanel.toggle` | renderer `🖐️pointer-input/🎯️shell-hit/🦀️.rs`, `🧭️navbar-footer-and-overlays/🦀️.rs` | OK |
| `framework.toolRun.<n>`, `.status`, `.steps`, `.toolRunStart`, `.toolRunAbort`, `role=progressbar`, `aria-live` | renderer `🛠️ShellHelpers/⏯️tool-run-panel/🟦️.ts:68`; `toolRunStart`/`Abort` in `🪝️window-actions` and `🔌️PluginRuntime`; progressbar in `🗣️Interpreter` and `🧵️TaskManager` | OK for ids; status text separator F5 |
| `window-action-pane` | renderer `🛠️ShellHelpers/🪟️window-panes/🟦️.tsx:153` | OK |
| `framework.window.gis2dMain.engagement.toggle` | renderer `🖐️pointer-input/🎯️shell-hit/🦀️.rs:156` | OK |
| `framework.window.gis2dMain.action.addFeature.execute`, `[id="action.addFeature"]`, `action.addFeature.arg.{label,lon,lat}` | action-id derivation documented in renderer `🧱️elements/🐚️Shell/…/🔲️windows/🧾️panel-records/🦀️.rs:422` (`action.<id>.arg.<…>`); no verbatim producer line | PARTIAL |
| `gis2d-play-inspector.{positions,routes,regions}-count` | plugin gis (`positions-count` 2, `routes-count` 2, `regions-count` 2 hits) | OK |
| `process3d-play-inspector.step.enabled`, `procedural-play-inspector.value.input` | plugins process3d / procedural (`step.enabled` 22 hits, `value.input` 39 hits) | OK |
| `fem3d-play-results.transport.play` | `🏗️fem/…/📊️results/🦀️.rs:203,211` | OK |
| `fem3d-play-results.phase.control` | slider row `…/🦀️.rs:222` (`slider_row` appends `.control`, `:96`) | OK |
| `fem3d-play-results.deformation-scale.control` | number row `…/🦀️.rs:247` (`number_row` appends `.control`, `:108`) | OK |
| `cad-play-catalogue.spatial.shape.primitive.box` | `🛍️catalogue/🦀️.rs:39` (`cad-play-catalogue.{typology}`); typology `🧰️kit/🦀️.rs:46` | OK |
| `fem3d.panel.results` (tab id) | `…/📊️results/🦀️.rs:39` (`PANEL_TAB_ID`) | OK |
| `fem` caption `^(Case\|Lastfall): dead$` | `…/🪟️windows/📊️results/🦀️.rs:327` (`"{}: {}{}"`), labels `🗣️terminology/🦀️.rs:112` (`Case`/`Lastfall`) | OK |
| Window kinds (all 8 panes, 19 kinds; 21 window ids incl. 2 puzzle3d instances) | `procedural-main/preview`, `cad-play-{shape,building,energy,structure-classic}`, `puzzle3d-main`, `energy.model.3d`, `energy.simulation`, `sourcing-{pool,curated,preview,grid}`, `process-workpiece`, `gis2d-main`, `fem3d-{model,results}`: all found in plugin sources | OK |
| `framework.window.tree` / `.table` (generic kits) | `framework.window.frameworkWindowTree/Table` per SPEC note | OK |

## 4. DEMONSTRATOR_PANES, PANE_CASES and expect.json

- `♻️…/🪧️brand.ts:911-968` `DEMONSTRATOR_PANES`: 8 panes in order generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik.
- `♻️…/🧪️acceptance/🟦️.ts:216-314` `PANE_CASES`: same 8 panes, same order.
- `expect.json` `panes`: same 8 ids, same order. Window kinds and surfaces match `PANE_CASES` exactly (checked by hand and by the `check` guard `probe.ts:718-726`).
- Panes in brand (`variant`): generator, koordinator, aggregator, energy, aussuchen, bearbeiten, verfolgen, fem3d. Not used by e2e.
- Brand description and tagline keys: all 8 panes x {label, tagline, description, windowTitle, shortWindowTitle} present in `🔣️.json` with non-empty en and de. Every description splits into exactly 2 paragraphs in en and de (checked with python, no repo runs).

## 5. User-visible strings, en and de

Verified exact en and de wording in producers:

| Use | en | de | Producer |
|---|---|---|---|
| Intro skip / next | Skip / Next | Überspringen / Weiter | UI `🌐️i18n/🎓️guidance/🟦️.ts:94,96,204,206` |
| Funding notice title | Funding Notice | Förderhinweis | catalog `🔣️.json:42` |
| Locale heading | Choose your language | Sprache wählen | catalog `:24` |
| Card open chip | Open demonstrator | Demonstrator öffnen | catalog `:25` |
| Pane tagline / description / label | per pane, all keys present | per pane, all keys present | catalog |
| Enabled (process row toggle) | Enabled | Aktiviert | `🏭️process/…/🗣️terminology/🦀️.rs:16` |
| Remove (curated row) | Remove | Entfernen | `🪵️sourcing/…/🗣️terminology/🦀️.rs:30` |
| No selection (aussuchen preview) | No selection | Keine Auswahl | `🪵️sourcing/…/🗣️terminology/🦀️.rs:31` |
| Play (fem transport) | Play | Abspielen | `🏗️fem/…/🗣️terminology/🦀️.rs:98` (PARTIAL, F4) |
| Case caption | Case | Lastfall | `🏗️fem/…/🗣️terminology/🦀️.rs:112` |
| Energy idle | No energy simulation run | Kein Energiesimulationslauf | `🔋️energy/…/⚡️simulation/🦀️.rs:17` |
| Energy result | The final result is accepted; the simulation never writes the document | Das Endergebnis ist übernommen; die Simulation schreibt nie ins Dokument | `🔋️energy/…/⚡️simulation/🦀️.rs:12` |
| Energy no result | No result was accepted | Kein Ergebnis wurde übernommen | same, `:12` |
| Energy final step | Final: | Endgültig: | `🔋️energy/…/🧵️simulation-session/🔣️.json:157` |
| Tool run finalized / aborted | Finalized / Aborted, nothing was changed | Abgeschlossen / Abgebrochen, nichts wurde geändert | framework `⏯️tool-run/🏷️actions-labels/🟦️.ts:84,86` (F5) |
| CAD box row | Box | Quader | `📐️cad/…/🗣️terminology/🦀️.rs:24` (F1) |
| Aggregator row | Hexagonal Cut Concrete Forest Left | (name, not localized) | `🎥️shooting` example and manifest |
| Bearbeiten step | Wall Saw Slab Cut | (name) | `🏭️process` `.dsl.semio` fixture |
| Document title | Entwerfen mit Bestand · Demonstrator | identical (F3) | catalog `:21` |

Items in `🧬️edits`, `🧬️simulation`, `🧬️curation` cases JSON that are fixture names (Glulam GL24h 200x400, Acceptance Reuse Marker, seed-left-001) are data inputs. Presence confirmed in `🪵️sourcing` and the demonstrator curation fixture. The label `Acceptance Reuse Marker` is typed by the probe, not produced.

## 6. Per-pane matrix (en and de)

| Pane | Kinds and surfaces | Selectors and ids | Action texts en / de | Verdict |
|---|---|---|---|---|
| generator | procedural-main (nodeGraph), procedural-preview (world3d): OK | `procedural-play-inspector.value.input`: OK | none (graphValueEdit) | OK; de act needs F2 |
| koordinator | cad-play-{shape,building,energy,structure-classic}: OK | `cad-play-catalogue.spatial.shape.primitive.box`: OK | rowText Box / Quader | DRIFT F1, F2 |
| aggregator | puzzle3d-main, instances top/perspective: OK | seed/kind names: OK; meshUrl `.glb` not checked | row text kindId: en=de | OK; F2 |
| energie | energy.model.3d, framework.window.tree/table, energy.simulation: OK | `framework.toolRun.*`, `tool.energySimulation`: OK | idle, result, final, aborted: texts exact | UNVERIFIED F5; F2 |
| aussuchen | sourcing-pool/curated/preview/grid: OK | `[data-row-id]`, stepper, spinbutton: OK | No selection / Keine Auswahl; Remove / Entfernen: exact | OK; F2 |
| bearbeiten | process-workpiece: OK | `process3d-play-inspector.step.enabled`: OK | Enabled / Aktiviert: exact | OK; F2 |
| verfolgen | gis2d-main (tiledMap): OK | engagement toggle: OK; `action.addFeature` PARTIAL | no text asserted | PARTIAL; F2 |
| statik | fem3d-model, fem3d-results: OK | phase.control, deformation-scale.control, transport.play, `fem3d.panel.results`: OK | Play / Abspielen (live only, F4); Case / Lastfall: exact | PARTIAL F4; F2 |

## 7. Harness notes (not drifts, for the run)

- `run.sh` writes to `C/🗑️generated/1105b/e2eready` (ticket root). Matches AGENTS.
- `probe.ts` imports `playwright`; resolution from the ticket folder was not verified.
- Console policy (`probe.ts:152`) matches `🧬️console/🧫️cases.json` (404 insignificant, other errors significant).
- `expect.json` timeouts (120000/60000/60000) match SPEC constants.

## 8. Not verified

- All PARTIAL and UNVERIFIED rows need one served run to close: class values, live vs non-live fem transport, energy status separator, `action.addFeature` id, `Box N` numbering, funding logo load.
- No browser run, no repo command, no build.
