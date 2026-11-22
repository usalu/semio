# Finisher report: thermodynamic flow view instead of airflow cones

Ticket `K` = `STAFFAGE-THERMODYNAMIC-FLOW-VIEW-INSTEAD-OF-AIRFLOW-CONES` (shared code, no asset wave). `S` = `🎓️teaching/🏛️architecture/🧍️staffage`, `R` = `S/🎯️targets/⚛️react`. Finished 2026-11-22 23:06. The session was restarted once at about 22:20 (one running suite was lost and rerun).

## State found

The five work packages of the reopened ticket (`controls`, `physics`, `model-flow`, `exports`, `legacy-layers`) were implemented further than their notes say: the notes end at 15:40 with "not re-run, the dependency store is gone", the source was edited until 16:49 and the last logs in `K/🗑️generated` showed `flow-exports` 34/34 and nine React suites 107/107. What the notes list as open and is in fact done in the code: the root travel of a walking clip in drawings and stage (`shownFlow`, `moveFlow`), the default layers (`DEFAULT_FLOW_LAYERS`, pathlines only on request), the CLI doc string.

| Part of the acceptance | Before (measured 22:11) | After |
|---|---|---|
| Legacy removal (emitter cones, dashed streamlines, condensation discs, `transport*` options) | no name left in `S` and the course visuals (sweep for `emitterGlyph`, `emitterMarkers`, `emitterPaint`, `glyphPaints`, `TRANSPORT_LAYERS`, `transportStrokeMm`, …: only prose and the negative cast vectors); the ticket's own scripts still passed the removed option `streamlines` | scripts fixed |
| Type check | library 0 errors; React package 2 errors, both in `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/⚙️configuration` (`chooseFile`, not staffage) | the same after all edits |
| Physics, field | suites green; finding of the notes confirmed by measurement: the electric fan heater showed 137.6 K on its streams while its parcels carry 41.2 K (two heating stages through one outlet were summed) | fixed: 79.6 K (the Gaussian centreline of one stream), see below |
| Exports | `flow-exports` 34/34; `reference-environment` 4 red: its reference drawing asked for all five layers while the CLI default is four | reference fixed, the three byte comparisons green |
| Model flow | `flow-model` 12/12, `model-digests` not run since the fixture was regenerated | 43/43 |
| Viewer controls, element, downloads | never run after the last edits | green (below) |
| `[DEBUG] ` logs | none in `S`; 14 in the kept ticket scripts | none |

## What I changed

- `S/🔨️modules/💨️transport/🌊️flow/🟦️.ts`: overlapping jets and plumes mix instead of summing their scalars. Each stream has a profile share `c_g` (2 on its centreline); where the shares of several streams sum beyond `CENTRELINE_SHARE` = 2 no ambient air is left in the mixture and temperature excess, vapour excess and condensate are scaled by `2/Σc_g` (the share-weighted mean). A single stream is bit-identical (the rule needs two streams), velocities still add. Module text and `S/README.md` ("Air flow", Field) say so.
- Test of it, language-agnostic with a third-party oracle: `S/🧫️fixtures/🌊️flow-field/🔣️.json` has a third case `twin` (two equal top-hat jets through one outlet, emitters `twin-a`, `twin-b`), `S/🧪️tests/🌊️flow-field/🐍️.py` carries the rule in its numpy field and regenerated the fixture (the expected values of `plume` and `jet` came out identical to before, compared key by key), `🟦️.ts` has the new case "gives two jets through one outlet the temperature of one and twice its velocity" and the two `it.each` cases for `twin` (numpy field, scipy `solve_ivp` streams), `🥒️.feature` the scenario.
- `S/🧪️tests/🌤️reference-environment/🟦️.ts`: the reference drawing uses `DEFAULT_FLOW_LAYERS` like the command line it is compared with.
- One bin rule instead of two: `excessBinOf` / `excessOfBin` live in `S/🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts` (used by `flowView`), `S/🔨️modules/🧊️volume/🌊️flow/🟦️.ts` imports them, `S/🔨️modules/🧊️volume/🟦️.ts` and `S/📦️packages/🟦️typescript/🟦️.ts` export them from there, `S/🧪️tests/🌊️flow-model/🟦️.ts` imports from there.
- One loop rule: `flowPeriodS` of `R/🔨️modules/🌊️flow/🟦️.ts` is the library's `flowLoopS`.
- `S/README.md`: the mixture rule; the comment of the first CLI example ("the default flow layers").
- Ticket scripts: `flow-smoke.ts`, `flow-profile.ts`, `flow-session-probe.ts`, `physics/parcel-digest.ts` no longer name `streamlines`; `[DEBUG] ` prefixes removed from `flow-session-probe.ts`, `model-flow/{smoke,clip-probe,fbx-probe,glb-probe,regenerate-digests}.ts`; new `physics/excess-probe.ts` (parcel excess against stream excess per asset).
- Removed: `K/🗑️generated`, `K/exports/🗑️generated` (and the smoke output of this session).
- `U/requests.md`: sections `## flow view — repo run hangs for @teaching/staffage` and `## flow view — failures of neighbouring suites that are not flow code`.

Files (repo-relative). Updated:

- `🎓️teaching/🏛️architecture/🧍️staffage/README.md`
- `🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/💨️transport/🌊️flow/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🌊️flow-view/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🌊️flow/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🔨️modules/🌊️flow/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🌊️flow-field/🟦️.ts`, `🐍️.py`, `🥒️.feature`
- `🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🌊️flow-field/🔣️.json`
- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🌊️flow-model/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🌤️reference-environment/🟦️.ts`
- in `K`: `flow-smoke.ts`, `flow-profile.ts`, `flow-session-probe.ts`, `physics/parcel-digest.ts`, `model-flow/smoke.ts`, `model-flow/clip-probe.ts`, `model-flow/fbx-probe.ts`, `model-flow/glb-probe.ts`, `model-flow/regenerate-digests.ts`
- `U/requests.md`

Created: `K/physics/excess-probe.ts`, `U/flow-view.md`. Removed: `K/🗑️generated/`, `K/exports/🗑️generated/`. No catalog asset was edited.

## Proof (all run to the end in this session, direct commands, each bounded)

Commands: `bun node_modules/.bin/tsc --noEmit -p tsconfig.json --incremental false` in both packages; `bun node_modules/vitest/vitest.mjs run --configLoader native --config <library or React config> <filter…>`; the fixture through `uv run … --only-group geometry python -I S/🧪️tests/🌊️flow-field/🐍️.py --write` (2.5 minutes).

| Run (after the last source edit unless noted) | Result |
|---|---|
| `tsc` library package, 23:06 | 0 errors |
| `tsc` React package, 23:06 | 2 errors, both `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/⚙️configuration/🟦️.ts` (279, 543), none in staffage |
| `flow-field`, `flow-environments`, `flow-view`, `flow-obstacles`, `transport-field` | 5 files, 91 tests passed |
| `flow-model`, `flow-exports`, `transport-assets`, `thermal-multiview`, `parcel-jet`, `condensation`, `seven-modes`, `model-glyph-size`, `model-glyph-edges`, `model-looks`, `axonometric-depiction` | 11 files, 183 passed |
| `model-digests` | 43 passed (117 s) |
| `model-exports`, `model-scales`, `model-frame`, `model-scale-files` | 4 files, 89 passed (325 s) |
| `model-roundness` | 15 passed |
| `model-batch` | whole suite killed after 11.5 minutes; by name 4 passed, 7 not run |
| `reference-environment` (22:14, after its fix) | 11 passed, 1 failed: catalog asset `solar-thermal-collector` has 34 validation issues |
| React `🌊️flow`, `air-controls`, `address-roundtrip` | 3 files, 43 passed |
| React `flow-downloads`, `custom-element`, `figure-render`, `model-controls`, `viewpoints`, `compare-state`, `shared-borders` | 7 files, 79 passed |
| React `📤️downloads`, `model-view`, `model-viewer-page`, `model-downloads` | 4 files, 57 passed (144 s) |
| `repo test-file S/🧪️tests/🌊️flow-field/🟦️.ts` (the one trial, 23:05) | 1 s, 10 pass, 0 fail |
| `K/flow-smoke.ts` standing adult, electric kettle, electric fan heater in `winter-clear-night`, `office-neutral`, `summer-heatwave-calm`, German | 9 drawings written; develop 2.2 to 3.7 s in 64 slices, next frame 17 to 41 ms |

Run before the mixture rule and the two refactorings, not repeated after (22:13 to 22:16): `cast-provenance`, `schema-twin`, `vapour-pressure`, `phase-change`, `dxf-reopen`, `pdf-reopen`, `frame-timing`, `zip-reopen`, `axonometric-exports`, `-visibility`, `-projection`, `-lazy` green; red there, none in flow code and all in `U/requests.md`: `schema-conformance` (1), `cast-export` (1), `cast-lod` (2), `multiview-coverage` (2), `multiview-sheet` (2).

Smoke facts: the kettle fogs in cold and in neutral air (13 and 20 fog puffs, dew point −8.0 and 11.1 °C) and not at 38 °C / 30 %; the adult's plume carries 12.5 L/s at −5 °C, 3.8 L/s at 22 °C, 1.0 L/s at 38 °C; the fan heater's streams now peak at 80 K in all three. Drawings opened (rasterised with `qlmanage`): the fan heater in `office-neutral` (red jet from the grille lifting off the floor, grey entrained streams with arrow heads from behind and above, none through the casing) and the kettle in `winter-clear-night` (fog cloud with cyan rim leaning downwind, dotted vapour lines, wind streams bending up into the plume, German legends). No cone anywhere.

## What stays imperfect

- Playgrounds carry no flow, and the comparison panes have no air controls of their own (reasons in `K/controls.md`: one clock and one asset frame per flow session). Not started here.
- A clip in GLB/FBX carries the flow of its first instant as static geometry; tracers and parcel paths are not in 3D files. DXF/DWG circles are 32-chord polylines and CAD layers have three colour classes.
- Sheets: the extension lines of the overall dimensions cross the legend block under the side elevation (older than the flow, longer with its legends). Not changed.
- The mixture rule is a bound, not a jet-merging model: beside the common axis two coincident jets read up to the centreline value where one alone reads less (kettle 47 K against 38 K parcels is one stream's Gaussian centreline plus that). Velocities of coincident streams add.
- With `thermalSpanK` of the course (±25 K) a body plume of 1 to 2 K reads grey in the course casts; a span of its own for the air is a decision for the dev.
- Not seen in a browser in this session: the 3D stage and the viewer (the earlier screenshots are from 14:00 to 15:10); jsdom suites and type check only.
- Not run: `model-catalog`, `export-performance`, `course-coverage`, the Manim tests of the course, the site build, and 7 cases of `model-batch` (for the umbrella coordinator). The `walker-line` pins of `export-performance` are stale because `walking-commuter` was re-authored (`K/legacy-layers.md`).
- The `flow-environments` Python oracle still needs CoolProp and psychrolib through `uv run --no-project --with …`; a `psychrometrics` group in the oracle project would pin them.

Asset data: nothing is missing for the flow view. Seen in the data, for the asset owners: the electric fan heater models its two heating stages as two fans of 16 L/s each through the outlet `outlet` (32 L/s in the pose `heating`).

Never-started assets: none (no asset wave). Open shared-code requests of this ticket: the two sections in `U/requests.md`.
