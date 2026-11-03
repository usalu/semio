# Cleanup C1: practice clip, icons, deletion of ad-hoc staffage

V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`.

## 1. Practice clip migrated
`📊️demand/🎬️clip/🧪️practice/🐍️.py` now uses staffage only.
- `_stand`: thermal `standing-adult` idle-breathing (`staffage_motion`, cast `demand-practice-person`).
- `_heater`: thermal `panel-radiator` rated (`demand-practice-radiator`), at physical size (`room_h / 2.5`).
- `_talker` (landlord and tenant, Beat 3 ending): `dark-line` standing-adult recoloured orange/cyan (`demand-practice-talker`), no heat meaning.
- Lamp: new shared helper `pendant_lamp(top, height, on_cast=, off_cast=)` in V (no shared lamp helper existed; M6/M7 used local ones). Returns `group`, `off`, `on` (transparent) and `bulb` point; the Beat 3 switch-on is now an off-to-on crossfade (`demand-practice-lamp-off/on`, thermal). Glow ripples still start at the bulb.
- All five casts are in `V/🧍️staffage/🔣️.json`, `theme: dark`, view thermal (talker dark-line).

## 2. Diagram icons
`object_silhouette` -> `icon_glyph` for microbe (summer-strategy), flame (system-losses), factory and flame (primary-energy). The icon module already held all five outlines (sun, moon, flame, microbe, factory).

## 3. Deleted
- From V `🐍️.py`: `person_glyph`, `seated_person_glyph`, `object_silhouette`, `lamp_glyph`, `_SILHOUETTES`. `_filled_outline` stays (used by `icon_glyph`).
- `V/🫟️silhouettes` (outlines, schema, README); the standing/seated outlines and the four unused dimensions in `V/🔣️.json` and its schema (radiator size defaults remain for `radiator()`).
- Tests: `🧪️tests/🐍️silhouettes.py` + `🔣️silhouettes.json`; in `🧪️tests/🐍️.py` the standing, seated, radiator-vs-person, lamp and raster tests, and the matching keys in `🔣️.cases.json`.
- Source lists: `🪟️presentation/🐍️.py` and optimization `🎨️visuals/🐍️.py` no longer name the silhouette JSON. `📜️script.ts` test list: `🐍️silhouettes` replaced by `🐍️icons` and `🐍️staffage_only`.
- Whole-repo grep (excluding .venv, node_modules, old tickets) finds the removed names only in the guard's case file and the internal-gains test's forbidden-pattern list.

## New tests
- `V/🧪️tests/🐍️icons.py` + `🔣️icons.json`: icon contours (raster oracle with scipy), sun/moon, annotation frames, watt anchors and radiator, backdrop fade (surviving parts of the old silhouette tests).
- `V/🧪️tests/🐍️staffage_only.py` + `🔣️staffage_only.json` (language-agnostic case file: root, forbidden names and paths): scans every clip `🐍️.py` outside tests with a regex and, independently, with `ast`; checks V no longer defines the names; checks the silhouette and activity folders are gone.
- `pendant_lamp` has no unit test of its own (needs the bun exporter); it is exercised by the Beat 3 render.

## 4. Test results
- Pass: icons, staffage_only, V `🐍️` (17 tests); integration, scenes, sections, section_consumers, transforms (40 run); cooling clip tests, internal-gains tests, regulations sun test, semio-style (after setting `SEMIO_TEACHING_OUTPUT_ROOT`).
- Failing, unrelated to this change: `🎬️clip/🧪️tests` `test_whole_chapters_reuse_without_retaining_segment_media` (temp dir not under repo root, `ValueError` in the runtime) and semio-style `test_render_cache_fingerprints_the_complete_semio_style_closure` (`KeyError: 'language'` in `runtime.segment_state`; test passes no language). Both are in the runtime, not in staffage code.
- Not run: the presentation browser tests and `🧪️tests` of other series beyond those listed (the cast cache was invalidated by concurrent source changes, so runs re-exported casts and were slow).

## Renders (l, 15 fps, frames viewed)
Beat 1 (thermal occupant and radiator in the house section; small but legible, the end-section radiator is a thin sliver), Beat 3 (lamp lit with crossfade, orange/cyan talkers standing on the ground line, house). Beat 2: see the end of this file.

Scripts (inputs): `c1_edit_v.py`, `c1_edit_json.py`, `c1_edit_practice.py`, `c1_edit_tests.py`, `c1_render.sh`, `c1_crop.py`.

## Beat 2
Hydraulischer Abgleich rendered and viewed: four thermal radiator end sections sit on the room floors with the supply and return pipes meeting them and the heat ripples above; they are thin slivers (authored end section) but read clearly.

## Follow-up: the two runtime failures (root cause)
Both are pre-existing and not caused by this ticket.
- `test_whole_chapters_reuse_without_retaining_segment_media`: it creates its fixture in `SEMIO_TEACHING_OUTPUT_ROOT` (falls back to the system temp dir) and the runtime then takes a path relative to the repo root. Unset env var -> temp dir outside the repo -> `ValueError`. Passes with the variable set under the repo (and `PYTHONUTF8=1` for the console print of the emoji path). `git status` shows the runtime and the test unchanged against HEAD.
- `test_render_cache_fingerprints_the_complete_semio_style_closure`: `runtime.segment_state` reads `settings["language"]` at HEAD too (5 uses in `git show HEAD:...runtime`); the test passed settings without `language`. Neither file is modified in the working tree. Fixed the test by passing `"language": "en"`.
- Closure check (our concern): `segment_state` hashes `V/🐍️.py`, follows its `import_module` calls (so `🧍️staffage/🐍️.py`) and all non-test `.json` assets under the source folders (so `🧍️staffage/🔣️.json` and `🔣️icons/🔣️.json`). The test now also asserts those three files are fingerprinted with their sha256; it passes. The removed silhouette entries cannot leave stale hashes (the files are gone). The staffage cache under `.🧬semio/🎓️teaching/⚡️cache/staffage` is outside the fingerprint by design (its key hashes the exporter sources and the cast entry).

## Follow-up: tests
- Added `test_pendant_lamp_hangs_from_top_and_switches_on_by_crossfade` to `V/🧪️tests/🐍️icons.py` (fake `staffage` so no bun needed): top at the requested point, both casts equal height and centred, `on` transparent, bulb between top and bottom, crossfade by `set_opacity`. icons test file: 7 tests OK.
- Presentation browser tests (`🌐️browser/🧪️tests`: closure, persisted, main): 14 tests OK headless here (about 400 s).
