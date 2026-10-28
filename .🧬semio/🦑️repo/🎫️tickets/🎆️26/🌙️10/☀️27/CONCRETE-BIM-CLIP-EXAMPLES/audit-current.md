# Audit: Current BIM Clip Code, Tests And Registrations

Read-only Haiku audit (agent a19d868dd4475cf3d). `C` = `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip`, `EA` = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring`, `EAC` = `EA/🎬️clip`.

## Tests

`C/🧪️tests/🐍️.py`: 20 contracts in 5 classes.

- **BimModelContract** (7, all WALL-01 specific; rewrite): identity/composition/inheritance, `information` language handling (ValueError unknown property / `fr` / empty; TypeError when language omitted), views from one object (NumPy determinant 18 / 5.4), partial models subsets, FEM translation mapping/loss/area sum, event replay + stale detection, invalid events.
- **BimClipContract** (keep): AST finds each scene class and a literal `NARRATION` (≥ 3 clauses, unique keys, 3 non-empty strings, en ≠ de), Manim discovery returns exactly the scene; every clause × language × audience fits `caption_bar` (speaker ≤ 2 lines, width ≤ frame, needs env `SEMIO_TEACHING_OUTPUT_ROOT`); curriculum registers six standalone videos (ids, entries, outputs, presentation titles = cases `chapters[].title`); standalone entries route only their scene; playlist routes all six.
- **BimLanguageContract** (keep): speech roots per language (reads `Beat1` NARRATION length), render children env, composition partitioned by language.
- **BimSelectedSourceContract** (keep; synthetic fixtures `source_validation` in cases).
- **BimBrowserMetadataContract** (keep; runs `🟨️metadata.mjs` via `repo tools exec --light node`, needs `repo` on PATH).

`C/🧪️tests/🟨️.cjs`: AJV 2020 strict validates model JSON against schema; 5 `invalidModels` mutations (paths `building.name.de`, `revision`, `links[0].thickness`, `representations[0].kind`, `languages`) must be rejected; energy manifest schema accepts `source_validation.manifest` and rejects 4 `invalid_structure` cases.

`C/🧪️tests/🟨️metadata.mjs`: browser renderer `--sources` mode, no browser launched. Unrelated to content.

Cases `C/🧪️cases/🔣️.json`: `wall`, `hierarchy`, `representations`, `models`, `event`, `updated`, `losses`, `invalidEvents`, `invalidModels` are WALL-01 specific; `chapters` (folder, scene, video_id, entry, title, output), `audiences`, `source_validation`, `browser_metadata` must stay.

Tests outside BIM read only ids, segment counts (1 chapter + 1 segment per video, global ordinals 114–119) and presentation titles — unaffected unless ids or segment counts change.

## Caption And Timing Policy

- `caption_bar`: wrap at 11.5 units, > 2 lines raises ValueError (EA/🎨️visuals:2027–2068). No character or word cap is enforced; the prior ticket used ≤ 115 characters as policy.
- `hold_for` budget = max(measured TTS, words / 2.5 en or / 2.2 de, words / 1.6 reading floor ≥ 1.5 s); floor 0.3 s.
- `ClipScene.step(key, …)` looks the key up in `NARRATION` (StopIteration on a bad key), swaps the caption, plays animations with `run_time=seconds`, then holds for the clause.
- Recorded clause audio must fit the window + 0.02 s.

## Registrations

- Manifest `⚡️energy/🎬️clip/🔣️.json` (six BIM videos, chapters `BIM_01_Information` … `BIM_06_Consistency`, segment scenes `Beat1…Beat6`), presentation `EAC/🪟️presentation/🔣️.json` (topic `bim`), and their test cases — unchanged as long as ids, scene names and one segment per video stay.
- `🏗️bim/❓️quiz/🔣️.json` does not use WALL-01 values: no quiz change needed.
- `C/README.md` lines 39 and 61 carry 6 × 3 × 0.30 m, 18 m², 5.4 m³, 0.40 m, 7.2 m³ — rewrite.

## Render

`bun EA/📜️script.ts render --video <id> --language en|de --audience speaker|spectator [-q l|m|h] [--resolution W,H] [--fps N] [--jobs N]`; outputs under `.🧬semio/🎓️teaching/⚡️cache/videos/<lang>/<audience>/rendered/<Output>_<audience>_<lang>_<res>p<fps>.mp4`. `bun EA/📜️script.ts test` runs the suites (sets `SEMIO_TEACHING_OUTPUT_ROOT`). Editing shared presentation JSON makes every lesson's certificate stale — avoid.

## Prior Decisions To Keep

- Methods are application behaviour; IFC carries declarative data, relations and constraints.
- A product is distinct from its representations; several representations describe one product.
- A shared schema does not imply shared instances, identity, revisions or consistent values.
- Translations to analysis can map one-to-many; lossless inverse translation is not guaranteed; IFC has its own structural analysis domain, so the chosen analysis schema is an example.
- A mesh carries geometry, not building semantics.
- `GlobalId` gives stable references; it does not update metadata, resolve conflicts or propagate revisions. BCF topic GUIDs differ from IFC GlobalIds.
- Do not imply unlabelled data is meaningless; data gains meaning through context.
