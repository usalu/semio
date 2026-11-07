# 🧭️ Audit 1105b aud-e2emap: browser E2E expectations vs produced code

Read-only audit. No source file was edited. Repo root: `/Users/ueli/Documents/semio`. Demonstrator: `♻️mit-bestand/🧺️demonstrator`.

Abbreviations: `SPEC` = `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts`; `PROBE` = `C/1105b-tools/e2eready/probe.ts`; `EXP` = `C/1105b-tools/e2eready/expect.json`; `CAT` = `♻️mit-bestand/🧺️demonstrator/🔨️modules/🌐️locale/🔣️.json`; `UI` = `🧰️framework/🔨️modules/🖱️ui`; `RND` = `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements`.

Verdict legend: **OK** = asserted and produced, en and de present where text is asserted. **PARTIAL** = produced but not fully verified statically. **UNVERIFIED** = producer not located within this audit's time budget (repo grep over the plugin trees timed out). **MISSING** = no producer found.

## 1. Global shell and landing expectations

| Expectation | Asserted (SPEC / PROBE) | Produced | en | de | Verdict |
|---|---|---|---|---|---|
| `html[lang]` = `en` / `de` | SPEC 144, 487, 525, 545, 566; PROBE `until html lang` | `UI/🌐️i18n/🟦️.ts:207,221` (`document.documentElement.lang`) | yes | yes | OK |
| Shell root `[data-shell-id]` lang | SPEC 145, 567 | `RND/🏛️ShellHost/📐️layout-hotkeys/🔶️scope/🟦️.ts:156` (`scope.rootRef.current.lang`) | n/a | n/a | OK |
| Document title = `demonstrator.landing.title` | SPEC 488, 526, 546; PROBE runLanding/runPicker/runSwitch | `♻️…/🔨️modules/🌐️locale/⚛️.ts:23` (`document.title`) | "Entwerfen mit Bestand · Demonstrator" | same text | OK (note: en cell is not English, identical to de) |
| Shell beacon `data-shell-ready` / `-error` / `-not-found` | SPEC 131-152; PROBE `shellOutcome` | `RND/🏛️ShellHost/🖼️canvas-footer/🟠️footer-items/🟦️.tsx:52-97` | n/a | n/a | OK. Note: SPEC comment says `ShellHost/🟦️.tsx #region 🔖️ReadinessBeacon`, which is stale (`ShellHost/🟦️.tsx` is 488 lines and has no beacon) |
| Shell root `data-shell-id` | SPEC 91, 144, 578; PROBE | `RND/🏛️ShellHost/🟦️.tsx:182` | n/a | n/a | OK |
| `data-remount-probe` | SPEC 561-568 | test-written attribute only | n/a | n/a | OK (not a product attribute) |
| Intro skip control `[id="ui.introduction.skip"]`, Escape keybinding | SPEC 166-175, 489, 538, 555; PROBE `dismissIntroduction` | `UI/🎯️targets/⚛️react/🎓️introduction/🟦️.tsx:258,304` | "Skip" (`UI/🌐️i18n/🎓️guidance/🟦️.ts:204`) | "Überspringen" (`:94`) | OK |
| Intro next button, `[data-slot="introduction-info-box"] button` | SPEC 1113 | `🎓️introduction/🟦️.tsx:285,362-363` (`ui.introduction.next`) | "Next" (`:206`) | "Weiter" (`:96`) | OK |
| Intro title `[data-slot="introduction-info-box-title"]` = fundingNotice | SPEC 1118 | `🎓️introduction/🟦️.tsx:295`; text `♻️…/🪧️brand.ts:89` (general intro entry, `demonstrator.brand.shared.fundingNotice`) | "Funding Notice" | "Förderhinweis" | OK. Step index 2 (two Next clicks) is inferred from list order, not independently checked |
| Funding logos: >=3 visible `img` in intro box, sized and decoded | SPEC 1119-1129 | not located (logos come from footer/brand assets) | n/a | n/a | UNVERIFIED |

## 2. Overview (landing) cards

| Expectation | Asserted | Produced | Verdict |
|---|---|---|---|
| Landing grid `[data-layered-card=<paneId>]`, count = brand panes (8) | SPEC 491-492, 527; PROBE runLanding `overview card count` | `UI/🧱️elements/🥞️LayeredOverview/🧵️pane/🟦️.tsx:228` | OK. Pane ids from `♻️…/🪧️brand.ts` `DEMONSTRATOR_PANES` (8 ids; SPEC pane roster test 473-475 passes by construction) |
| `[data-overview-card]` visible | SPEC 495-496 | `UI/🧱️elements/🃏️OverviewCard/🟦️.tsx:99,108` | OK |
| `[data-window-silhouette]` count 1 | SPEC 497 | `UI/🧱️elements/🗂️WindowChrome/🟦️.tsx:485,506` | OK |
| `[data-window-silhouette-border][data-kind="normal"]` count 1 | SPEC 498 | `🗂️WindowChrome/🟦️.tsx:349,361,387` (`data-kind={resolvedKind}`) | PARTIAL (value `normal` depends on runtime resolution) |
| `[data-slot="window-chrome-chip-cap"]` has class `ui-glass` | SPEC 499 | `🗂️WindowChrome/🟦️.tsx:442` (chipSlot default); class from `chipSurfaceClass` line 460 | PARTIAL (class value needs runtime check) |
| `[data-slot="window-chrome-body-surface"]` has `ui-glass` | SPEC 500 | `🗂️WindowChrome/🟦️.tsx:561`, `bodySurfaceClass` line 461 | PARTIAL |
| `[data-slot="window-chrome-body"]` NOT `ui-surface` | SPEC 501 | `🗂️WindowChrome/🟦️.tsx:421` (bodySlot default) | PARTIAL |
| `[data-slot="demonstrator-pane-card-title-chip"] svg` count 1 | SPEC 502 | `UI/🧱️elements/🃏️OverviewCard/🟦️.tsx:74` (`${slot}-title-chip`) plus icon `⚛️demonstrator-card.tsx:32` | OK |
| `[data-slot="demonstrator-pane-card-tagline"]` text = `demonstrator.brand.pane.<id>.tagline` | SPEC 503; PROBE runLanding | `♻️…/⚛️demonstrator-card.tsx:37`; text from `🪧️brand.ts` `text(locale)` tagline | OK. All 8 keys present in en and de (checked against CAT) |
| `[data-slot="introduction-body-paragraph"]` count 2 | SPEC 504 | `⚛️demonstrator-card.tsx:43`; paragraphs from `demonstratorPaneDescriptionParagraphs` (split on blank lines) | OK. All 8 descriptions split into exactly 2 paragraphs in en and de (checked) |
| Open chip text `demonstrator.card.open.label.normal` | SPEC 505 | `UI/🃏️OverviewCard/🟦️.tsx:134` (`${slot}-open-chip`); label `⚛️demonstrator-card.tsx:21,35` | OK ("Open demonstrator" / "Demonstrator öffnen") |
| No drag handle `[data-slot*="drag"], [data-drag-handle]` count 0 | SPEC 506 | not checked | UNVERIFIED (intro has a `introduction-info-box-drag` slot, which is only in the intro, not on cards) |
| Overview button `[data-layered-overview-button]` | SPEC 562, 726, 734 | `UI/🧱️elements/🥞️LayeredOverview/🟢️overview-button/🟦️.tsx:12` | OK |
| Overview: `[data-layered-overlay]` / strip (not asserted by SPEC) | none | `🥞️LayeredOverview/🟦️.tsx:49,55` | n/a |

## 3. Language picker, switcher, and live switch

| Expectation | Asserted | Produced | en | de | Verdict |
|---|---|---|---|---|---|
| Picker `[data-slot="demonstrator-locale-picker"]` visible, 2 buttons | SPEC 518-520; PROBE runPicker | `♻️…/⚛️locale-picker.tsx:12,24` | endonym "English" (`🔨️modules/🌐️locale/🟦️.ts:20`) | "Deutsch" | OK |
| Picker heading `demonstrator.locale.choose` | not asserted directly | `⚛️locale-picker.tsx:18` | "Choose your language" | "Sprache wählen" | OK |
| No preselection: no `[aria-pressed]`/`[aria-checked]` on picker | SPEC 521; PROBE runPicker | `⚛️locale-picker.tsx:24` (no aria-pressed on picker buttons) | n/a | n/a | OK |
| No cards before choice | SPEC 522; PROBE | landing renders picker instead of cards (`🟦️.tsx:121`) | n/a | n/a | OK |
| `button[lang="en"|"de"]` visible | SPEC 523; PROBE | `⚛️locale-picker.tsx:24` (`lang={locale}`) | n/a | n/a | OK |
| Choose de: html lang, title, cards, reload keeps choice | SPEC 524-530; PROBE runPicker action | `🟦️.tsx:117-119` (commit + setLocale), `🔨️modules/🌐️locale/⚛️.ts:21-24` (persist via `initUiLocaleSync`) | n/a | n/a | OK |
| Switcher `[data-slot="demonstrator-locale-switcher"]`, `button[data-locale]` | SPEC 539-548, 563-566; PROBE runSwitch | `⚛️locale-picker.tsx:38,45` | endonym "English" | "Deutsch" | OK |
| `aria-pressed="true"` on chosen switcher button | SPEC 544; PROBE runSwitch | `⚛️locale-picker.tsx:45` (`aria-pressed={candidate === locale}`) | n/a | n/a | OK |
| Tagline follows switch | SPEC 547; PROBE runSwitch | `⚛️demonstrator-card.tsx:37` | both | both | OK |
| Opened pane survives language switch (remount probe) | SPEC 552-570 | shell root lang (`layout-hotkeys/🔶️scope/🟦️.ts:156`) | n/a | n/a | PARTIAL (remount behaviour is runtime) |
| Language choice fr-FR fallback: picker shown | SPEC 514-515; PROBE `unsupportedBrowserLocale` | `🔨️modules/🌐️locale/🟦️.ts:23` `resolveDemonstratorLocale` returns undefined | n/a | n/a | OK |

## 4. Pane boot (all 8 panes) and window content

Pane list: `🪧️brand.ts` `DEMONSTRATOR_PANES` (lines 911-968): generator, koordinator, aggregator, energie, aussuchen, bearbeiten, verfolgen, statik. SPEC `PANE_CASES` (216-314) and EXP `panes` (both 8, same order).

Shared surface attributes (all produced):

| Surface | Assertion | Produced | Verdict |
|---|---|---|---|
| world3d host `.semio-world-3d-host`, empty `.semio-world-3d-empty` | SPEC 348-363, 598-600; PROBE worldCount | `RND/🌐️World3dHost/🟦️.tsx:98,103` | OK |
| `data-meshes-json`, `data-instances-json`, `data-vortices-json` | SPEC 354, 1025-1029; PROBE | `🌐️World3dHost/🟦️.tsx:110,112,117` | OK |
| `data-compute-phase`, `data-slot="world-compute-status"` | SPEC 909-910; PROBE settled() | `🌐️World3dHost/🎯️marquee-and-bridges/🟦️.tsx:146-159` | OK |
| `aria-busy="true"` on compute status | SPEC 909 | `marquee-and-bridges/🟦️.tsx:126,146,153` sets `aria-busy` only on the `world-surface-status` node or `undefined`; `world-compute-status` with `aria-busy="true"` not seen | PARTIAL (value may be set in a branch not inspected) |
| node-graph host `.semio-node-graph-host`, empty | SPEC 374-378; PROBE graphWidgetCount | `RND/🕸️NodeGraph/🟦️.tsx:206,279` | OK |
| `data-host-snapshot-json` (widgets[]) | SPEC 380-383, 939-949; PROBE | `🕸️NodeGraph/🟦️.tsx:288` | OK |
| table host `.semio-table-host`, empty | SPEC 395-397; PROBE tableRowCount | `RND/📊️Table/🟦️.tsx:310,317` | OK |
| `[data-row-id]` on rows | SPEC 400, 646, 691; PROBE | `📊️Table/🟦️.tsx:218,280` | OK |
| `data-stepper-control="increment"` | SPEC 716 | `📊️Table/🟦️.tsx:200` | OK |
| tree body `[role="tree"]`, rows `[role="treeitem"]` | SPEC 411-413; PROBE treeRowCount | `🌳️Tree/🟦️.tsx:518` (role=tree); `🌳️Tree/🍃️item/🟦️.tsx:208,346,487` (treeitem) | OK |
| `data-slot="tree-label"` / `tree-description` / `tree-item-row` | SPEC 1049, 1084; PROBE | `🍃️item/🟦️.tsx:274,282,336,406,478,512` | OK |
| tiled map host `.semio-tiled-map-host`, empty | SPEC 432-438; PROBE | `RND/🧭️TiledMapHost/🟦️.tsx:26,29` | OK |
| pane-scoped alias `data-element-alias` for window ids | SPEC 124, 593; PROBE `paneSelector` | `RND/🏛️ShellHost/📋️panel-hosting/🥞️mode-windows/🟦️.tsx:160` (childElementId) | OK |
| placeholder text (`No selection`) on aussuchen preview | SPEC 614, 644; PROBE `placeholder` | `♻️…/🧪️tests/🎭️acceptance/🧬️…` no; producer = plugin `preview/🦀️.rs` `labels.no_selection` (not inspected) | en "No selection" / de "Keine Auswahl" both present (`🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json`, EXP) | PARTIAL (producer in plugin Rust not located) |

Per pane (window kind -> surface, SPEC line, EXP match, brand presence, verdict):

| Pane | Window kind (element id) | Surface | content | Brand/manifest kind found in `🪧️brand.ts` | Verdict |
|---|---|---|---|---|---|
| generator | `procedural-main` | nodeGraph | yes | yes (1) | OK for host; plugin kind producer UNVERIFIED |
| generator | `procedural-preview` | world3d | yes (SPEC note: previewEval) | no | UNVERIFIED (plugin manifest) |
| koordinator | `cad-play-shape` | world3d | yes (SPEC note: KNOWN GAP fixed f394df99d4, not compile-verified) | yes (1) | UNVERIFIED (producer in semio-hub-cad, compile blocked per SPEC note) |
| koordinator | `cad-play-building`, `cad-play-energy`, `cad-play-structure-classic` | world3d | yes | no | UNVERIFIED |
| aggregator | `puzzle3d-main` -> `puzzle3d-main-top`, `puzzle3d-main-perspective` | world3d | yes | yes (17) | PARTIAL (instance ids via `framework.window.puzzle3dMainTop`) |
| energie | `energy.model.3d` | world3d | yes | yes (2) | PARTIAL |
| energie | `framework.window.tree` (element `framework.window.frameworkWindowTree`) | tree | yes | generic TreeWindowKit (not located) | UNVERIFIED |
| energie | `framework.window.table` (element `framework.window.frameworkWindowTable`) | table | yes | generic TableWindowKit (not located) | UNVERIFIED |
| energie | `energy.simulation` | tree | yes | yes (3) | PARTIAL |
| aussuchen | `sourcing-pool` | table | yes | yes (1) | PARTIAL |
| aussuchen | `sourcing-curated` | table | no | no | UNVERIFIED (empty branch `.semio-table-empty` produced at Table:310) |
| aussuchen | `sourcing-preview` | placeholder | no (text "No selection"/"Keine Auswahl") | no | UNVERIFIED producer (text in plugin) |
| aussuchen | `sourcing-grid` | world3d | no | no | UNVERIFIED |
| bearbeiten | `process-workpiece` | world3d | yes | yes (1) | PARTIAL |
| verfolgen | `gis2d-main` | tiledMap | yes (canvas paints >1 colour) | yes (1) | PARTIAL (paint check depends on wgpu map; EXP/PROBE note software GPU may fail) |
| statik | `fem3d-model` | world3d | yes (SPEC note: now 2/234 with concrete-forest) | yes (2) | PARTIAL |
| statik | `fem3d-results` | world3d | yes | no | UNVERIFIED |

All 8 panes' ids in SPEC, EXP, and brand match, and EXP's window kind list matches SPEC `kindId` list per pane (PROBE `check` enforces this statically).

## 5. Actions (SPEC describe-blocks vs PROBE action types)

Labels and texts come from `♻️…/🔨️modules/🧪️e2e/📁️artifacts/🧫️cases.json` (`projects[].texts`) and `🧬️simulation/🧫️cases.json`, `🧬️edits/🧫️cases.json`, `🧬️curation/🧫️cases.json`. Each has en and de; all checked.

| Action / assertion | Text or id expected (en / de) | Producer | Verdict |
|---|---|---|---|
| Remove button (curated row) | "Remove" / "Entfernen" | row action button, plugin/framework table | en/de OK; producer UNVERIFIED |
| Enabled toggle on workpiece row | "Enabled" / "Aktiviert" (SPEC 1059; EXP `buttonName`) | plugin `process3d` row button | en/de OK; producer UNVERIFIED |
| Play transport text | "Play" / "Abspielen" (SPEC 846, 879) | plugin `fem3d-play-results.transport.play` | en/de OK; producer UNVERIFIED |
| Caption `Case: dead` / `Lastfall: dead` | regex `^(Case\|Lastfall): dead$` (sim cases) | plugin `fem3d` results caption | en/de OK; producer UNVERIFIED |
| No run text | "No energy simulation run" / "Kein Energiesimulationslauf" (SPEC 795; EXP `idle`) | energy simulation tree | en/de OK; producer UNVERIFIED |
| tool run status | "Finalized ·" / "Abgeschlossen ·", "Aborted, nothing was changed ·" / "Abgebrochen, nichts wurde geändert ·" | `framework.toolRun.N.status` | en/de OK; producer UNVERIFIED (see section 6) |
| final step line | "Final: .* kWh after N of M timesteps" / "Endgültig: .* kWh nach N von M Zeitschritten" | `framework.toolRun.N.steps` | en/de OK; UNVERIFIED |
| accepted result / no result | "The final result is accepted; …" / "Das Endergebnis ist übernommen; …"; "No result was accepted" / "Kein Ergebnis wurde übernommen" | energy simulation tree | en/de OK; UNVERIFIED |
| Action test ids | `framework.toolRun.N.toolRunStart`, `.toolRunAbort`, progressbar, `aria-live` on status | framework tool-run panel | UNVERIFIED (grep of `🧱️elements` found no `toolRun.` id producer; the tool-run panel lives under `🛠️ShellHelpers/⏯️tool-run-panel`) |

Per action, PROBE vs SPEC coverage:

| Pane | SPEC test | PROBE action | Gap |
|---|---|---|---|
| generator | `generator undo-redo` 932-975 (widgets, synapses, geometry, undo+redo) | `graphValueEdit` (undo+redo) | PROBE does not check `synapses` ids or `widgetIds` set. Partial |
| koordinator | `koordinator undo-redo` 977-1003 (4 windows) | `catalogueAdd` | PROBE does not check that row text reappears after redo beyond rowText regex; OK otherwise |
| aggregator | `aggregator undo-redo` 1005-1038 (meshUrl, seed mesh id, vortices count) | `catalogueAdd` | GAP: PROBE does not check `meshUrl`, seed `meshId` match, or `vortexCount` (11). SPEC does |
| bearbeiten | `bearbeiten undo-redo` 1040-1076 (enabled field, geometry) | `rowToggle` | PROBE checks projection and enabled field; OK |
| verfolgen | `verfolgen undo-redo` 1078-1106 | `featureAdd` | OK |
| energie | `energie run-cancel` 788-824 | `toolRun` | PROBE does not check `progressbar` visibility or the `aria-live` attribute. Minor GAP |
| statik | `statik undo-redo` 834-882 (change, undo, redo; phase; scale) | `paramScale` | GAP: PROBE does change+undo only; no redo step. SPEC does change, undo, redo |
| aussuchen preview | `aussuchen preview` 637-658 | `rowSelectPreview` | OK |
| aussuchen undo-redo | `aussuchen undo-redo` 684-744 (increment, keyboard increment, remove, undo/redo sequence, offscreen 31 s revisit) | NONE | GAP: PROBE has no curation undo-redo or revisit test. EXP has no `curation` action (curation cases referenced only as `rowSelectPreview.cases`) |
| overview chrome en/de | SPEC 482-509 (card DOM) | `runLanding` tagline only | GAP: PROBE does not check silhouette, glass classes, title chip svg, paragraph count, open chip label |
| language live remount | SPEC 552-570 | none | GAP |
| funding logos | SPEC 1108-1131 | none | GAP |
| simulation `restarts` table | `🧬️simulation/🧫️cases.json` `restarts` (6 cases) | not referenced by SPEC or PROBE (the `isNewToolRunGroup` helper in `🧬️simulation/🟦️.ts` is used by SPEC 815) | Data present, only partly consumed |

## 6. Producers not located (budget exceeded)

These were searched for and not found under the directories searched. Repo-wide greps over `🧰️framework` and `🌎️hub` exceeded the 120 s limit on this filesystem and were stopped. Treat as UNVERIFIED, not MISSING.

- `framework.toolRun.<n>` run-group ids and `.status`, `.steps`, `.toolRunStart`, `.toolRunAbort` (SPEC 779-786, 800-822; PROBE `latestToolRunGroup`).
- `framework.panel.<tab>` and `framework.panelTab.framework.panel.<tab>` ids; `data-tab-id` (SPEC 900-904). Only `ui.mobilePanel`-style toggles matched in `RND/🏛️ShellHost/📋️panel-hosting/🔶️brand/🟦️.tsx` (lines 21-30).
- Plugin element ids: `fem3d-play-results.{transport.play,phase.control,deformation-scale.control}`, `process3d-play-inspector.step.*`, `procedural-play-inspector.value.input`, `gis2d-play-inspector.{positions,routes,regions}-count`, `framework.window.gis2dMain.engagement.toggle`, `framework.window.gis2dMain.action.addFeature.execute`, `action.addFeature.arg.{label,lon,lat}`, `cad-play-catalogue.spatial.shape.primitive.box`, `puzzle3d.object.<hex>`.
- Window kinds not in brand: `procedural-preview`, `cad-play-building`, `cad-play-energy`, `cad-play-structure-classic`, `sourcing-curated`, `sourcing-preview`, `sourcing-grid`, `fem3d-results`, `framework.window.table`, `framework.window.tree`.
- `window-action-pane` slot: not found in `UI/🧱️elements/📌️WindowPane`.
- `ui.mobilePanel.toggle` (PROBE `openCompactPanel`): not confirmed.

## 7. Harness and project wiring

- `EXP` sources (brand, catalog, spec, three case files) all exist at the stated paths. Verified.
- `EXP` `unsupportedBrowserLocale` = fr-FR, `landing.pickerChoose` = de, timeouts 120000/60000/60000, all match SPEC constants (120 s shell, 60 s content).
- Console policy: SPEC `🧬️console/🟦️.ts` and PROBE `openSession` both filter only the exact "Failed to load resource ... 404 (Not Found)" message (or without reason). Consistent with `🧬️console/🧫️cases.json`, which marks 404 insignificant and "404 (Not Found); runtime crashed" significant.
- `run.sh` writes output to `C/../../🗑️generated/1105b/e2eready` (ticket root). Matches AGENTS (generated under ticket root).
- `probe.ts` imports `playwright` from its own folder under `1105b-tools/e2eready`. Resolution of that package from the ticket folder was not verified.
- Brand pane regex (`DEMONSTRATOR_PANES` literal ending `\n];`) is shared by SPEC and PROBE; matched at `🪧️brand.ts:911-971`.

### `📋️project.json` (`♻️mit-bestand/🧺️demonstrator/📋️project.json`)

| Target | Command (cwd `♻️mit-bestand/🧺️demonstrator`) | Port / note |
|---|---|---|
| `serve` | `repo ./🔨️modules/🧩️runtime/📜️script.ts serve` | port 6029, env `MIT_BESTAND_DEMONSTRATOR_PORT=6029` |
| `dev` | `repo ./🔨️modules/🧩️runtime/📜️script.ts …` (dev variant) | port 6029 |
| `serve-e2e` | `repo ./🔨️modules/🧩️runtime/📜️script.ts serve-test`, env `SEMIO_BUILD_MODE=prototype`, continuous | dependsOn `prepare-e2e` |
| `prepare-e2e` | `repo ./🔨️modules/🧩️runtime/📜️script.ts prepare-test`, env `SEMIO_BUILD_MODE=prototype` | dependsOn `activate-prototype`, `workspace:deps-browsers` |
| `test-e2e` | `repo ./🔨️modules/🧪️e2e/📜️script.ts test` | dependsOn `serve-e2e`; no cache |
| `test` | `repo ./📜️script.ts test` | unit/acceptance-free |
| `activate-prototype` | `repo ./🔨️modules/🧩️runtime/📜️script.ts activate` | prerequisite |

The probe (`run.sh`) does not start a server. It expects a running demonstrator, for example `serve` on `http://localhost:6029`.

## 8. Key findings

1. Every SPEC assertion on the shell beacon, `data-shell-*`, world/node/table/tree/tiled surface attributes, overview card and picker has a producer, and every asserted UI string exists in both en and de (CAT: 146 keys, 0 incomplete; `demonstrator.brand.pane.<id>.{label,tagline,description}` present for all 8 panes; 2 description paragraphs each).
2. PROBE covers boot, content, one action per pane, landing tagline, picker, switcher, and the static cross-checks. It does not cover curation undo/redo with revisit, statik redo, aggregator mesh/vortex checks, overview card DOM, live remount, or funding logos. Those gaps are listed in section 5.
3. The plugin-owned element ids and tool-run ids (section 6) are the main unverified surface. Their producers live in plugin/framework trees the search could not cover within budget.
4. Stale doc references in SPEC: `ShellHost/🟦️.tsx` beacon region and "~line 6687-6717" (file is 488 lines); `World3dHost/🟦️.tsx ~5063` (attrs are at 98-117); `🌳️Tree/🟦️.tsx ~1936/2013/2088` (treeitem lives in `🍃️item/🟦️.tsx`). Comments only, no behaviour impact.
5. `demonstrator.landing.title` has identical en and de text ("Entwerfen mit Bestand · Demonstrator"). The en cell is not English. Both cells exist, so the completeness check passes.

No source file was modified. Only this report was written.
