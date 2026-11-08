STATUS: 12 new cases x en/de WRITTEN, parse and type-check GREEN, not run in a browser (no activation yet: UNVERIFIED). Baseline untouched (50).

## Files (D = `♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🧭️behaviours`)
- `D/🎭️.ts` spec, `D/🧰️pane/🟦️.ts` helpers (restated, because importing `../🟦️.ts` would register the baseline's 25 cases in this file), `D/🧫️cases.json` data, `D/🎚️config/🟦️.ts` config.
- The baseline `testMatch` is exactly `🎭️acceptance/🟦️.ts`, so the new file is invisible to `test-e2e`. The config spreads the baseline config and only replaces `testMatch`.
- Tags: every case has `@behaviour` plus `@<pane>` (`@lifecycle` for the 4 lifecycle cases). Titles are at most 24 chars (artifact path budget).
- Run baseline: `repo run @semio-tech/mit-bestand-demonstrator:test-e2e` (50). Run new: `PLAYWRIGHT_BASE_URL=<served> playwright test --config D/🎚️config/🟦️.ts` (24), optionally `--grep @aggregator`.
- Owner `reacthost` told in one line.

## Proof (logs in `C/🗑️generated/1105b/e2ecases-*`)
- tsc over the 3 TS files rc 0 (`e2ecases-tsc1.txt`); a negative control (string to number) fails as TS2322, so the check is real.
- `playwright test --list` with the repo's own playwright 1.64.1 and a scratch config (`C/1105b-tools/e2ecases/list*.config.mjs`): baseline-only 50; `--grep @behaviour` 24; `--grep-invert @behaviour` 50; `--grep @lifecycle` 8 (`e2ecases-list.txt`). The real baseline config cannot be loaded by bare node (repo-lib import chain, rc 1 for the untouched config too), so the scratch config stands in for listing only.
- `e2eready/run.sh --check` GREEN 8/8.
- Not proven: any selector or timing against a served page.

## Cases (source of each selector is in the case's docstring)
| Case | Documented intent | Observable |
|---|---|---|
| aggregator tour | tour description "view, catalogue, transform, connection points and fill" | palette `mod+p` > Play Tutorial > `ui.tutorial.bar`, 12 chapter ticks with localized titles in authored order, play advances `ui.tutorial.time`, seek to chapter opens Catalogue tab, stop |
| aggregator transform | "Transform utility moves and rotates" | `[id=transform]` aria-pressed, options `puzzle3d-transform-move/-rotate` named Move/Verschieben, Rotate/Drehen |
| aggregator vortex show | "Show connection points" = "Always" makes all visible | `puzzle3d-play-vortex-show` `data-published-value`, `data-vortices-json` length grows, returns on Selected |
| aggregator fill | Fill tool fills the design, count at slider | `tool.fill` tab, `puzzle3d-fill-count` stepper, tool run group appears, instance ids grow |
| aussuchen filter | "filter by module and typology" | `sourcing-filter-query/-module-slabs/-typology`, pool rows against the 12-kind demo stock |
| bearbeiten chain | "define and visualise processing chains on the workpiece" | catalogue row `process3d-catalogue.wallSaw.wallCut`, step rows +1, workpiece projection changes, undo/redo |
| verfolgen routes | "where components come from and go", "explore the connections" | addFeature with collection routes: routes-count +1, positions/regions unchanged, undo/redo |
| statik model edit | "you edit the FEM model and check the structural response" | addNode x/y/z via window action pane, "Nodes (N)" section +1, caption `Case: dead` settles, undo/redo |
| lifecycle stagger/offscreen/idle/hidden | landing comment: 1.5 s start, one per 35 s; 30 s offscreen; 5 min idle; 60 s hidden | `[data-layered-pane] [data-shell-id]` presence; thresholds parsed from `🟦️.tsx`; idle and hidden use `page.clock` and a `document.hidden` init-script override |

## Findings that correct the audit
- aussuchen filter DOES exist (audit said "no action exists"): `sourcing …/🏊️pool/🦀️.rs` `filter_bar`.
- bearbeiten step add/remove/reorder exist (`🎮️commands/🪜️step`, catalogue rows bind `addStep`).
- verfolgen routes can be added (`add_feature` collection routes, staged select Routes/Routen).
- statik `addNode`, `addBar`, `addSupport`, `addNodalLoad` etc. exist as actions.

## Not written (and why)
- Suggest ("choose a connection point with left click, right click opens the action menu, hover previews, left click places"): needs picking a vortex marker inside the WebGL canvas; no DOM handle, no screen-projection hook. Needs a production test hook; not invented.
- Transform drag (actual move/rotate of an object) and zoom-to-location in verfolgen: camera and gizmo state are not DOM-observable.
- koordinator "compare and merge several CAD windows": no such action found in the cad plugin (matches for merge are selection merge and snapshot diff).
- Responsive swipe and neighbour hints: not asked.

## Risks to expect on first run (all UNVERIFIED)
- Palette flow for the tour (arg form ids `command.*.arg.tutorialId`, `command-*-execute`) is read from `command-registry`, never seen live.
- Window-options fold toggle `framework.window.<id>.measures.unfold`, statik/verfolgen engagement toggle and action ids follow the verfolgen baseline pattern.
- Typology option `data-value="beams/steel"` assumes `typology_flatten` includes interior nodes.
- Fill: assumes Ctrl+Enter starts the tool run when none runs, and that the count means total instances (it commits distinct instances + 3).
- Clock cases assume fast-forward does not expire an in-flight boot deadline (taken after both panes are ready).
- Lifecycle offscreen needs no click inside the pane (only Escape on body) to keep it pristine.

## Koordinator conflict: copy "connects form, building, energy and structural models in shared views" vs spec:995 (other windows unchanged)
Resolved by reading, no contradiction in behaviour:
- The four windows show four separate models of one document: `CadPaneId` = Shape `spatial.shape`, Building `aec.building`, Energy `aec.building.energy`, StructureClassic `aec.building.structure.classic` (`cad …/🧬️schema/📸️snapshot/🦀️.rs:91-104`); each window renders only its own pane's composed child (`build_world_scene_for_pane`, `…/✏️edit/🦀️.rs:488-490`).
- Catalogue rows carry `modelDefinitionId`, so `cad-play-catalogue.spatial.shape.primitive.box` adds into `spatial.shape` only (`📌️panels/🛍️catalogue/🦀️.rs:38`). The only cross-model data in the scene is `references_by_model_definition_id`, external images per model, not propagation.
- So `spec:995` (other windows unchanged) matches the code. "Connects ... in shared views" is satisfied only in the sense of co-display of the four models in one shell; there is no propagation. "Compare and merge" has no implemented action.
- Verdict: the test is right, the copy is loose. Decision for the owner (not made here): keep the copy and accept co-display, or soften it to "shows form, building, energy and structural models side by side".
