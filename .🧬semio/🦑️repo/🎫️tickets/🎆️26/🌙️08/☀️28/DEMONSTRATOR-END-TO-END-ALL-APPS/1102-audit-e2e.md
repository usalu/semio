STATUS: PARTIAL AUDIT, 2 DRIFT FINDINGS, 4 ITEMS UNVERIFIED (framework-level wording and gis/fem composed ids not fully traced; no cargo/bun/repo run)

# 1102 Independent E2E Cross-Check (aud-e2e)

Scope: `expect.json`, `probe.ts`, and `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts` checked against plugin editor source. Read-only. Grep note: `timeout` does not exist on macOS, and unquoted multi-path variables in zsh are a single argument; both were hit and worked around.

## Fixture and variant routing (all 8 panes)

Routing source: `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🔣️.json` maps variant to runtimeVariant. `🪧️brand.ts` DEMONSTRATOR_PANES maps pane id to variant (`statik` to `fem3d`, `energie` to `energy`). Plugin editor roots are under `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor`.

| pane | acceptance pane id | runtime variant | plugin / editor artifact | windows (kind ids) | fixture / case file |
|---|---|---|---|---|---|
| generator | generator | generation3d | 🌀️procedural / 🧊️generation3d | procedural-main, procedural-preview | edits/🧫️cases.json (generator: height, 6 to 6.5) |
| koordinator | koordinator | koordinator | 📐️cad / 📐️cad | cad-play-shape, cad-play-building, cad-play-energy, cad-play-structure-classic | edits (catalogueId cad-play-catalogue.spatial.shape.primitive.box) |
| aggregator | aggregator | aggregator | 🧩️puzzle / 🧊️3d | puzzle3d-main (instances -top, -perspective) | edits (kind "Hexagonal Cut Concrete Forest Left", seed-left-001) |
| energie | energie | energy | 🔋️energy / 🔋️model | energy.model.3d, framework.window.tree, framework.window.table, energy.simulation | simulation/🧫️cases.json |
| aussuchen | aussuchen | aussuchen | 🪵️sourcing / 🗂️curation | sourcing-pool, sourcing-curated, sourcing-preview, sourcing-grid | curation/🧫️cases.json (beam-glulam-gl24h) |
| bearbeiten | bearbeiten | bearbeiten | 🏭️process / 🧊️process3d | process-workpiece | edits (wall-saw-cut, 7 steps) |
| verfolgen | verfolgen | verfolgen | 🌍️gis / 🗺️gismap | gis2d-main | edits (label "Acceptance Reuse Marker", delta 1) |
| statik | statik | fem3d | 🏗️fem / 🧊️3d | fem3d-model, fem3d-results | simulation/🧫️cases.json (fem: phase 0.1, scale 300/600) |

Window element ids are `framework.window.<camelCase kind>` from `🧰️framework/🔨️modules/🖱️ui/🔨️modules/🆔️element-identity/🟦️.ts:40` (`windowElementId`). The probe and acceptance mirror this exactly, so that part is consistent.

## Asserted ids and labels: found or drift

Found (file under `✏️editor` of the plugin noted, counts from grep):
- generator: `procedural-main` (6), `procedural-preview` (16), `procedural-play-inspector` (45), `"height"` widget (46). Edit-mode input `procedural-play-inspector.value.input` composed from the inspector prefix (UNVERIFIED as literal; prefix found).
- koordinator: `cad-play-shape`, `cad-play-building`, `cad-play-energy`, `cad-play-structure-classic`, `cad-play-catalogue` (3 each), `spatial.shape.primitive.box` (27), `cad-play-document` (18). Path `📌️panels/🗿️artifact/🦀️.rs`.
- aggregator: `puzzle3d-main` (16), `puzzle3d-main-top` (9), `puzzle3d-main-perspective` (13), `puzzle3d.object` (1), kind "Hexagonal Cut Concrete Forest Left" (10 in `🧊️3d`, 1 in cad), `seed-left-001` (35). Note: the kind string "Hexagonal Cut Concrete Forest Left" is used by the acceptance spec as a raw DOM id via `[id="${kindId}"]` (`🧪️tests/🎭️acceptance/🟦️.ts` aggregator test). The catalogue DOM id must be the literal kind name with spaces; NOT VERIFIED that the tree row id is the raw string (the framework tree id may be camelCased). DRIFT RISK, see finding 2.
- energie: `energy.model.3d` (30), `energy.simulation` (8), `No energy simulation run` (2), `Kein Energiesimulationslauf` (1), `Finalized` (2), `No result was accepted` (2), `Kein Ergebnis wurde übernommen` (1), `energySimulation` (6), `tool.id`/`tool.label` (tool tab id `tool.energySimulation` is composed as `tool.` + id; plausible, not a literal).
- aussuchen: `sourcing-pool` (6), `sourcing-curated` (2), `sourcing-preview` (2), `sourcing-grid` (6), `beam-glulam-gl24h` (12), `No selection` (4), `Keine Auswahl` (2). Stepper `data-stepper-control` and `Remove`/`Entfernen` labels in `🪵️sourcing` (2 each).
- bearbeiten: `process-workpiece` (3), `process3d-play-inspector` (37), `process3d-play-inspector.step.{id,label,machine,capability,enabled}` (all present), `"Enabled"` (3), `"Aktiviert"` (3). `wall-saw-cut` appears in the process3d asset DSL `🖼️assets/🌲️concrete-forest/🗣️.dsl.semio`, so the fixture carries it. `Wall Saw Slab Cut`, `wallSaw`, `wallCut` were not found in the process editor source (only in fixture data, not verified in full).
- verfolgen: `gis2d-main` (27), `gis2d-play-inspector` (20), `gis2d-play-inspector.{positions,routes,regions}-count` (in `📌️panels/🔍️inspection`), `addFeature` (16 in gis `🗺️gismap`). NOT FOUND as literal in gis editor: `action.addFeature` compound id and `framework.window.gis2dMain.engagement.toggle`. These are presumably composed by the framework window-action machinery; UNVERIFIED.
- statik: `fem3d-model` (3), `fem3d-results` (3), `fem3d-play-results.phase` (6), `fem3d-play-results.transport.play` (4), `fem3d-play-results.deformation-scale` (2, the literal `.control` suffix not seen, composed), `fem3d.panel.results` (1), `Case: dead` (1), `Lastfall` (23), `Abspielen` (2), `"Play"` (2). Caption regex `^(Case|Lastfall): dead$` matches the fem caption pattern.
- Locale catalog `🔨️modules/🌐️locale/🔣️.json` has keys for `demonstrator.landing.title`, `demonstrator.card.open.label.normal`, `demonstrator.brand.shared.fundingNotice`, and `demonstrator.brand.pane.<8 ids>.tagline`. All present once each (en and de texts in the same entry).

Framework DOM tokens (checked in `🧰️framework/🔨️modules/🖱️ui` and `🧰️framework/🛍️products/💻️os/.../📺️renderer` element sources):
- Found: `world-compute-status`, `data-compute-phase`, `window-action-pane`, `demonstrator-locale-picker`, `demonstrator-locale-switcher`, `introduction-body-paragraph`, `introduction-info-box-title`, `data-layered-overview-button`, `data-stepper-control`, `semio-table-host`, `data-layered-card` (LayeredOverview pane), `semio-node-graph-host` (NodeGraph), `semio-tiled-map-host` (TiledMapHost), World3dHost (`semio-world-3d-host`), `data-shell-ready` (ShellHost), `data-element-alias`, `data-row-id` (Table), `framework.toolRun`, `toolRunStart`, `toolRunAbort`, `ui.introduction.skip`.
- NOT FOUND in the scoped element/ui set: `demonstrator-pane-card-open-chip`, `demonstrator-pane-card-tagline`, `data-window-silhouette`, `framework.category.tool`, `framework.category.display`, `tree-description` (partial hits only in the tree item file, not counted).

## Drift and risk

1. Wording not in source (asserted in acceptance and probe): `Aborted, nothing was changed` / `Abgebrochen, nichts wurde geändert` and `Abgeschlossen` (German finalized). Not found in the energy editor. The framework-wide search timed out, so these are NOT VERIFIED anywhere. Both the probe (`expect.json` energie aborted and finalized) and the acceptance spec rely on them. Per-language keys are not in `expect.json`; the German texts are hard-coded regex alternatives. DRIFT (unconfirmed location) plus language-specific wording without a catalog key.
2. Raw kind id as DOM id: `Hexagonal Cut Concrete Forest Left` (aggregator catalogue row, `[id="${kindId}"]`) contains spaces. Source uses it as a string but the tree item id derivation was not checked against `🌳️Tree/🍃️item`. UNVERIFIED, may be a drift.
3. Composed ids not literal in source: `framework.window.gis2dMain.engagement.toggle`, `framework.window.gis2dMain.action.addFeature.execute`, `fem3d-play-results.deformation-scale.control`, `fem3d-play-results.phase.control`, `procedural-play-inspector.value.input`, `tool.energySimulation`. Each is plausible via framework id composition but none were confirmed by reading the composing code. Unverified.
4. Missing DOM tokens asserted by acceptance: `demonstrator-pane-card-open-chip`, `demonstrator-pane-card-tagline`, `data-window-silhouette`, `framework.category.tool`, `framework.category.display`. Not found in the scoped `🖱️ui` and renderer sets. The acceptance overview test (lines ~476-500) would fail if these are absent. Needs a wider check (not run; repo-wide grep timed out).
5. Expected-but-not-checked: `Wall Saw Slab Cut`, `wallSaw`, `wallCut` (process fixture labels), the fem `Case: dead` caption formatting, and koordinator card-level windows. Not traced to a file.

Also noted: `expect.json` energie `idle` and `finalized` regexes are language-specific pairs, which is acceptable only because they are paired per language in the same key. The `aborted` and `noResult` strings are in `simulation/🧫️cases.json`, not `expect.json`, so the drift would not show in the expect file.

## Unverified / not run
- No browser run, no server, no cargo, no bun, no repo command. Only static grep.
- The framework-wide greps timed out and were stopped; findings 1, 3 and 4 need one narrowed grep against `🧰️framework/🔨️modules/🖱️ui/🎯️targets` and `🧰️framework/🛍️products/💻️os/.../📺️renderer` element files.
