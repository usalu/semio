# Summary: Grasshopper and Excel Nakagin Clips

## What was built

- **Nakagin teaching model** (`🎓️teaching/🏛️architecture/🏢️nakagin`): JSON source of truth with schema, derivations (slots, planes, tree paths, program areas, envelope, heating, climate, variants), Manim drawings, cases, Python and Node tests, README (en, de).
- **Grasshopper engine** (`🦗️grasshopper/🎬️clip/🧱️model`): stdlib data trees, parameters, casting, Path Mapper, 68 components, five canvases, 45 named views, schema, cases.
- **Grasshopper canvas toolkit** (`🎨️visuals`, `🎨️visuals/🦗️canvas`): `ClipScene`, widgets (component, param, slider, panel, wire, group, ribbon, Param Viewer), 51 views rendered and checked.
- **Excel engine** (`📊️excel/🎬️clip/🧱️model`): A1 references, copy translation, tokenizer/parser/evaluator with dependency order, 22 functions, structured table references, 3D references, event-sourced edits, en/de formula text, bilingual labels and sheet titles; 7-sheet Nakagin workbook.
- **Excel workbook toolkit** (`🎨️visuals/📗️workbook`): window, ribbon, name box, formula bar, grid, tabs, reference frames, fill handle, trace arrows, error tags, bar chart, cell pop-up.
- **9 videos** (clauses per language, all with en and de narration):

| Video | Clauses | Shows |
| --- | --- | --- |
| grasshopper_anatomy | 24 | Capsule shell on an empty canvas: script, slider into Box, component anatomy, floating vs input param, panel, states, plugins |
| grasshopper_capsule | 23 | One parametric capsule: sliders move dimension lines, rounded edges, round window cut, four bolt points, types A to D, placement on planes |
| grasshopper_tower | 22 | Core, planes per storey and facade, rotations, capital, podium massing, both towers, counts |
| grasshopper_casting | 21 | Param types and casts (text to number, number to integer, point to plane, ...) and failures |
| grasshopper_trees | 22 | The 140-capsule tree `{BUILDING;TOWER;STORY;FACADE}`, path anatomy, Flatten, Graft, Simplify, Path Mapper, matching |
| excel_anatomy | 23 | Workbook, sheets per type, cell, range, name box, formula bar, formats, first formula (capsule floor area), recalculation |
| excel_references | 22 | Relative, absolute, mixed, cross-sheet, 3D and named references, copy translation, errors |
| excel_program | 24 | Room program, capsule list table, COUNTIF(S), SUMPRODUCT, XLOOKUP, totals, chart, counts lighting capsules |
| excel_energy | 23 | Envelope per type, U values, H_T, H_V, load, annual demand with Tokyo climate, what-if, building sum |

  Totals: Grasshopper 112, Excel 92 clauses.
- **Registration**: entries and playlists for both series, manifest `⚡️energy/🎬️clip/🔣️.json` (nine entries), presentation topics, collection titles, hierarchy and presentation portable cases, registry tests per series, `📜️script.ts` test lists, READMEs and transcripts (en, de).
- **Oracles** (tests only): rhino3dm (planes, transforms, boxes, bounding boxes), openpyxl (A1, `Translator`, .xlsx writing), formulas (recomputes every workbook cell), NumPy (volumes, areas, degree days, envelope), AJV (schemas, invalid mutations, independent JS oracles).

## Key decisions

- 140 capsules = 70 branches x 2 (tower A 27 + tower B 43).
- Capsule 2.5 x 4.0 x 2.5 m, interior 2.3 x 3.8 x 2.1 m, rounded profile, window on the front, four bolts on the back.
- Types A to D count 36 / 35 / 34 / 35 by the pattern `order[(tower + storey + 2 facade + item) mod 4]`.
- Every value carries a casting basis marker (`sourced`, `assumed`, `derived`; rounding `conflicting`).
- JSON is the source of truth: Nakagin JSON, engine catalogues and workbook JSON; engine and Excel values are cross-checked against `🏢️nakagin` in tests.
- The BIM `Panels` mixin was removed from both `ClipScene`s to decouple the renders from BIM.
- Texts are bilingual labels (en, de) throughout, including sheet tab titles and workbook constants.

## Test results (`tests-final.md`)

- Nakagin Python 29 OK, Node OK.
- Grasshopper Python 57 OK, Node OK, registry 5 OK, presentation 12 OK.
- Excel Python 83 OK, Node OK, registry 5 OK.
- Excel presentation suite: last run was terminated externally (exit 143) at `test_native_lifecycle`; result unknown and must be rerun alone.
- Energy authoring registry/curriculum: 31 OK (1 skipped); presentation/hierarchy 35 with 2 environment errors in browser capture of `PhysicalFundamentals_FullSection`, unrelated.

## Known open items

- Casting rules without a source are unverified (Number to Integer tie rule is `conflicting`: Rutten 2010 rounding vs 2012 truncation, Entwine path order, Simplify Tree defaults, Center Box inputs, failed-cast bubble text); verify in Grasshopper.
- German Excel labels (`#ÜBERLAUF!` unconfirmed, ribbon names, function names, table names) must be checked in the app.
- HDD 1585.7 K d comes from the Wikipedia Tokyo table of JMA normals (monthly-mean method); design temperature 0 C and u_window 1.1 are assumed.
- All nine verify reports end NOT READY with remaining by-design layout findings (status-bar sums, wires behind component bodies, opaque overlays on gridlines) plus per-video majors listed in the `verify-*.md` files.
- Series registry taxonomy has no `🎬️clip` kind for the new series (BIM precedent followed).
- Renders in `🗑️generated` were still being written when this summary was prepared; delete `🗑️generated` after delivery.

## Ticket hygiene

No tool-generated files existed outside `🗑️generated`. Kept: all scripts, configs, markdown, and three `.txt` snapshots inside script subfolders (`excel-energy/present_new.txt`, `excel-energy/present_v2.txt`, `gh-tower/scene_v1.py.txt`). Nothing deleted.

## Repo files created or modified (this ticket)

- `🎓️teaching/🏛️architecture/README.md` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🧪️tests/🔣️.cases.json` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/📚️collection/🔣️.json` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🧪️tests/🔣️.cases.json` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🥞️hierarchy/🧪️tests/🧩️chapters/🔣️.cases.json` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🔣️.cases.json` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧬️schema.json` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/pyproject.toml` (modified)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python/uv.lock` (modified)
- `🎓️teaching/🏛️architecture/🏢️nakagin/README.md` (created)
- `🎓️teaching/🏛️architecture/🏢️nakagin/🎨️visuals/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🏢️nakagin/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🏢️nakagin/🔣️.json` (created)
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🔣️.json` (created)
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🟨️.cjs` (created)
- `🎓️teaching/🏛️architecture/🏢️nakagin/🧬️schema/🔣️.schema.json` (created)
- `🎓️teaching/🏛️architecture/📊️excel/⚡️energy/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/README.md` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🎨️visuals/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🎨️visuals/📗️workbook/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🎨️visuals/📗️workbook/🔣️.json` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🎨️visuals/📗️workbook/🧬️schema/🔣️.schema.json` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📋️program/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📍️references/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🗣️transcript.de.md` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🗣️transcript.en.md` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🦴️anatomy/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️cases/🔣️.json` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️tests/🎭️presentation/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️tests/🎭️presentation/🔣️.cases.json` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️tests/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️tests/📇️registry/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️tests/📇️registry/🔣️.cases.json` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧪️tests/🟨️.cjs` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🔣️.json` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/🧱️model/🧬️schema/🔣️.schema.json` (created)
- `🎓️teaching/🏛️architecture/📊️excel/📋️program/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/📍️references/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/📊️excel/🦴️anatomy/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🌳️trees/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/README.md` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🌳️trees/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🎨️visuals/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🎨️visuals/🦗️canvas/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/💊️capsule/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗣️transcript.de.md` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗣️transcript.en.md` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🗼️tower/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🦴️anatomy/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️cases/🔣️.json` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🎭️presentation/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🎭️presentation/🔣️.cases.json` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/📇️registry/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/📇️registry/🔣️.cases.json` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🟨️.cjs` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🔣️.json` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧱️model/🧬️schema/🔣️.schema.json` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🫗️casting/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/💊️capsule/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🗼️tower/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🦴️anatomy/🎬️clip/🐍️.py` (created)
- `🎓️teaching/🏛️architecture/🦗️grasshopper/🫗️casting/🎬️clip/🐍️.py` (created)

## Final State (2026-10-29)

- **Quality round:** after the owner asked for top quality over resolution, every video got a Sonnet quality-director review (`quality-*.md`), its scene agent implemented the ranked improvements, and a final Haiku sweep (`sweep-*.md`) checked every frame in both languages; all sweep findings were fixed or decided (see below).
- **Videos (480p15 speaker previews, en/de):** Grasshopper anatomy 354/324 s, capsule 305/274 s, tower 296/271 s, casting 301/278 s, trees 308/299 s; Excel anatomy 265/239 s, references 323/297 s, program 315/286 s, energy 321/286 s. Files: `.🧬semio/🎓️teaching/⚡️cache/videos/<lang>/speaker/delivery/`, provenance in `delivery.json` (see `render.md`). Every file is newer than its scene, toolkit and model sources.
- **Tests (final run, all scenes final):** Nakagin 29 + AJV, Grasshopper 57 + AJV + registry 5 + presentation 12, Excel 84 + AJV + registry 5 + presentation 16 — all pass (`tests-final.md`).
- **Coordinator decisions in the last rounds:** German user-chosen labels are localized, built-in Grasshopper/Excel UI names stay English (English GH balloons tagged "Grasshopper-Meldung (englisch)"); German Panel text "2.5" is correct (GH parses invariant); German recap glossaries keep the English UI term in brackets; German table name `Kapseln`; free floor area (not the identical net area) drives the Excel program totals; types A/B/D share one shell, which the energy video turns into a teaching beat; "Geschoss" throughout.
- **Cross-session fixes:** the GH/Excel ClipScenes no longer mix in BIM `Panels` (it pulled the BIM model into the render fingerprint); BIM drafting's `P()` now wraps the refactored `📏️geometry.P` tuple into a numpy array at the drafting boundary (another session's in-progress refactor had broken all drawing).
- **Still open (owner):** confirm the casting rules marked "check in Rhino 8" and the German `#ÜBERLAUF!` and structured-reference special items in the German Excel UI; Tokyo monthly means come from the JMA 1991–2020 normals via Wikipedia's table; full-quality (1080p30) and spectator renders were skipped on the owner's request.
- **Hygiene:** `🗑️generated` deleted (frames, logs, exports, render logs); source snapshots (`*.py.txt`, `present_*.txt`) removed; scripts and all reports kept.
