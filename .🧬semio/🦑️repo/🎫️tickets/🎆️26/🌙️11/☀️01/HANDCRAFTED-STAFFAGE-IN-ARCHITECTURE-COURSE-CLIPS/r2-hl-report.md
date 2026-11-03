# R2 language audit (agent hl)

Scope: the 41 catalog asset files changed or added in this ticket, the catalog root manifest `🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json`, and the visuals manifest `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json`. Text only, no geometry changed.

## Checks run

- Locale pairs: every `{en, de}` text object in the 41 asset files (3525 pairs) has both languages non-empty. 0 problems. Root manifest: 316 pairs, 0 locale problems. Visuals manifest: no text fields.
- Pose and clip labels: every pose and clip in each changed asset has a bilingual `title` and `description`. Every manifest pose/clip id exists in its asset file. Manifest titles match asset titles in both languages. 0 issues. (The 374 "EXTRA" lines are asset poses/clips that the root manifest does not list. The manifest is a curated subset by design, so these are not defects.)
- Root manifest pose and clip ids are plain id strings, as at HEAD. They are not text and are not counted as gaps.
- Visuals manifest: 144 casts, every `asset`, `clip` and `pose` resolves in the catalog. 0 problems.
- ASCII umlaut substitutes (ae/oe/ue for ä/ö/ü) in German: none in the asset files. Root German hits (`Querholm`, `Mauerwerkswand`, `Trauerweide` and similar) are correct German. `europaeus` is the Latin species epithet in `Erinaceus europaeus` and is correct.
- English leaks in German strings (heuristic on common English function words): 0 after the fix below.
- Title case: asset titles and root manifest titles are Title Case, pose and clip titles are sentence case. Consistent apart from the fix below.
- Catalog check, full run: `repo tools exec bun -- .../🔨️modules/🗂️catalog/🩺️check/🟦️.ts` (no arguments). `✓ manifest`, 145 `✓` lines, 0 `✗`, exit 0. The run was made after my edits, and the finned-heat-exchanger file changed on disk during the run from another agent.

## Fixes made (Edit, text only)

1. `🧍️humans/walking-commuter/🔣️.json`, symbol `walking-direction-marker`: the English text sat in `title.de` and the German text in `description.en`. Now `title.de` = "Gehende Person als Marke" and `description.en` = "Position dot with a direction arrow for crowded overview plans at 1:500 and smaller."
2. `🏗️machinery/finned-heat-exchanger/🔣️.json`: "Register" is neuter in German, and the text used "der/Der Register" and "ein ... hoher Register" in about 15 places. Corrected to das/Das, hohes, kaltes, heißes, durch das, Das 5 kg schwere Register. Pose titles now "Kaltes Register" and "Heißes Register". Notes on these fixes: the Lamellenregister compounds are unchanged.
3. `🔌️appliances/gas-boiler/🔣️.json` (2 places) and `🔌️appliances/split-ac-indoor-unit/🔣️.json` (rig bone title): "Wärmetauscher" changed to "Wärmeübertrager", matching the plate-heat-exchanger term used in the catalog.
4. `🔌️appliances/split-ac-indoor-unit/🔣️.json` clip `phases[1].title`: "Lüfter und Tauscher laufen hoch" changed to "Lüfter und Verdampfer laufen hoch".
5. `🏗️machinery/expansion-valve/🔣️.json` clip `open-up`: "Aufgehen" changed to "Öffnen".
6. Title case consistency: "Wall-mounted Classroom Chalkboard" changed to "Wall-Mounted Classroom Chalkboard" in `🪑️objects/chalkboard/🔣️.json` and in the root manifest.

## Terms checked and correct

Plattenwärmeübertrager (plate-heat-exchanger), Sorptionsrad (sorption-wheel), Verdampfer (split-ac, finned-heat-exchanger), Verflüssiger (finned-heat-exchanger), Scrollverdichter (scroll-compressor), Expansionsventil (expansion-valve), Thermostat-Heizkörperventil (thermostatic-radiator-valve), Umwälzpumpe (circulation-pump), Warmwasser-Standspeicher (hot-water-cylinder), Lüftungsgerät mit Wärmerückgewinnung (mvhr-unit), Pendellüfter (decentralised-hrv-fan). Units use the German forms (l, Grad, Kopf).

## Open (outside this ticket's changed set, not touched)

- Symbol titles and descriptions with the same en/de swap as walking-commuter, in unchanged files:
  - `👤️humans/running-child/🔣️.json:2805`: `title.de` is English ("Position dot with a direction arrow ...").
  - `👤️humans/wheelchair-user/🔣️.json:3689`: `title.de` is English.
  - `👤️humans/seated-reader/🔣️.json:3431`: `title.de` is English.
- Root manifest and the "Register" gender check were done by reading the German text, not by a grammar tool. The German remains a careful reading, not a native review.

## Test output counts

- Bilingual audit: 41 asset files, 3525 pairs, 0 problems.
- Root manifest: 316 pairs, 0 locale problems.
- Pose/clip label check: 0 issues apart from 374 expected manifest-subset lines.
- Visuals casts: 144 casts, 0 problems.
- Catalog check (full): `✓ manifest`, 145 `✓`, 0 `✗`, exit 0.

Helper scripts are in `🧰️hl/` (`bilingual_audit.py`, `pose_clip_labels.py`, `de_review.py`, `cast_refs.py`). Generated output was removed.
