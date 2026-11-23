# English closing card shows a German frame

## Root cause

The frame inside the app window is a stored chapter poster. Poster assets are keyed by chapter scene only, not by language, and the stored posters were captured with `language=de`.

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖼️posters/🐍️.py:111-114` (`asset`) stores a poster as `<lesson clip folder>/🖼️posters/<scene>.png|json`. There is no language component.
- Same file, `:127-137` (`stale`): a poster is stale only if its moment or authored-source digests changed. The language is not compared. Line 171 writes `captured.language` into the sidecar, but nothing reads it.
- Same file, `:265-284` (`resolve`) and `:287-296` (`prepare`): `prepare` recaptures a poster from a freshly rendered clip only when `stale(scene)` is true. After the German capture, an English render never refreshes it, so `resolve` returns the German PNG.
- `⚙️runtime/🐍️.py:868-869` (end of the chapter-assembly step) calls `posters.prepare(...)` and hands the result to `_presentation_hierarchy.capture_posters`. From there the poster goes into the browser batch contract: `🌐️browser/🟨️render.mjs:80-82` maps scene to file, and `🌐️browser/🟦️.tsx:26-29` draws it in `<img data-presentation-poster>`.
- The sidecar of the closing poster, `🧲️physics/🎬️clip/🖼️posters/PhysicalFundamentals_FullSection.json`, says `language=de, profile=final`. Its moment is segment 8, the summary slide, which is the slide in the defect.
- The window chrome (Teaching, breadcrumbs) comes from the UI locale in `batch.language`, so it is English. The embedded image is a baked PNG and is not localized.

## Scope across the stored posters

Language recorded in the 26 sidecars under `<family>/🎬️clip/🖼️posters/`:

| Family | `de` | `en` |
| --- | --- | --- |
| physics (1) | 1 | 0 |
| ventilation (6) | 6 | 0 |
| heating (6) | 6 | 0 |
| energy_balance / demand (7) | 7 | 0 |
| cooling (6) | 4 | 2 (`Cooling_04_SolarRadiation`, `Cooling_05_Systemauslegung`) |

- All overview cards use these same assets. `required(branch)` at `:79-93` pulls the posters of the whole course, topic, lesson and chapter grid. So in an English render, every intro or overview card and every closing window shows a German poster. The two cooling exceptions show English posters inside German renders.
- All five families are affected, both the intro/overview cards and the closing card.
- Not affected: the plain non-windowed English slides. Those come from the Manim segment clips, which are rendered per language, and `sources`/`segment_state` there already include the language.

## Reference titles

German book and norm titles on the English closing slide ("Formeln und Tabellen Bauphysik", "Normen und Richtlinien") are German originals and can stay. The defect is only the headline ("Zusammenfassung ...") and the keywords ("Strahlung · Thermische Masse · Feuchte").

## Minimal clean fix

Key the poster asset by language. Do not add fallbacks, migration code or compatibility layers.

In `🖼️posters/🐍️.py`:

1. `asset(scene, language)` returns `folder / language / f"{scene}.png"` and `.json`, i.e. `<clip folder>/🖼️posters/<en|de>/<scene>.png|json`. This path fits `runtime:470`, which excludes any path containing a `🖼️posters` part.
2. Thread `language` (taken from `settings["language"]`) through `stored`, `stale`, `store` and `resolve`. `stale` then covers "not captured for this language" because the file is missing. `store` uses the same language. `prepare` and `capture` already have `settings`. `resolve(scenes)` gains a `language` argument, and `prepare` passes `settings["language"]`.
3. Drop the now redundant `language` entry from `captured`. Keep profile and audience.
4. Update the module docstring (lines 1-9) to say the path is `🖼️posters/<language>/<chapter>.png|json`.

In `🖌️capture/🐍️.py`: nothing to change. `--language` is already required, and `settings` carries it to `capture`.

Tests (`🖼️posters/🧪️tests/🐍️.py` plus `🔣️.cases.json`, and the synthetic helper `🧰️synthetic/🐍️.py`):

- Add a case: an `en` render with only `de` posters stored fails loudly in `resolve` ("Missing stored chapter posters ... --language en"), and `de` is unaffected.
- Move existing fixtures into the `de/` subfolder.
- Update `asset` call sites to the new signature.

## Stored assets to regenerate

1. Move the 26 existing `de` posters (png and json) into `🖼️posters/de/` in each family, by hand. Their fingerprints stay valid because the sidecar content is unchanged. Alternatively, just recapture them (step 2 with `de`).
2. Capture the English posters (from an up-to-date clip of the output root, else one Manim frame each):
   `bun 🏗️authoring/📜️script.ts posters --language en --profile final`
   Optionally narrow with `--video <id>` and `--output-root <root>`. Add `--force` to ignore staleness. For German use the same command with `--language de`.
3. Expected result: 26 `en` posters, including the replacement for the two existing cooling `en` posters (they currently sit at the language-less path and are not valid German ones).
4. Check in the new `en/` folders. The poster hash flows into the browser batch cache key (`🌐️browser/🐍️.py:108`), so the intro, outro and overview recordings invalidate and re-record on their own.

## Finals that need re-rendering

- English, all five videos: physics (Full_Physical_Fundamentals), ventilation, heating, cooling, energy_balance. Intro, overview and closing windows all embed posters. Both audiences (speaker and spectator) include them.
- German finals: the `de` poster images do not change, so German finals only need re-rendering if a `de` poster is recaptured. They do not need it if the old PNGs are moved as-is. The exception is the cooling German videos: they currently embed the two English cooling posters (04 and 05) and show those wrongly, so cooling `de` must re-render once those two `de` posters exist.
- Segment clips (Manim output) are language-keyed already and do not need re-rendering. Only the presentation (browser) segments and the final joins do.
