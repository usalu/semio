# Poster language fix (Task A) and three scene fixes (Task B)

## Design

Stored posters are keyed by language: `<family>/🎬️clip/🖼️posters/<language>/<scene>.png|json`. `asset(scene, language)`, `stored(scene, language)`, `stale(scene, language)` and `resolve(scenes, language)` take the language. `store` takes it from `settings["language"]`. `prepare` and `capture` pass `settings["language"]`. A poster missing for the render language is treated exactly like a stale one: `prepare()` and `capture()` recapture it, and `resolve()` fails loudly with `Missing stored chapter posters (capture them with script.ts posters --language <l> --profile final)`. The redundant `captured.language` is gone from the sidecars (profile and audience stay).

No other reader exists. The browser `render.mjs` and `🟦️.tsx` receive an explicit `{scene: {file, sha256}}` map built by `prepare()` for the render language (the UI language of the app window), so they needed no change. `runtime/🐍️.py` calls `posters.prepare(..., settings)`, unchanged. `📜️script.ts posters` forwards `--language`, unchanged. No BIM, Grasshopper, Excel, Optimization or parametric lane and no site code reads `🖼️posters/`.

## Files changed (relative to repo root)

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖼️posters/🐍️.py` (language-keyed API, docstrings, error text)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖼️posters/🧪️tests/🐍️.py` (updated call sites, new `PosterLanguageTest`, per-language content check)
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/README.md` (poster storage paragraph)
- `🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py` (`tr("On", "An")`; Venetian-blind label at `win_lo - 0.7`, right edge aligned to `gx - 0.15`)
- `🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🧪️practice/🐍️.py` (`meter(tr("SPF", "JAZ"), ...)`)
- All 26 stored posters (52 files) moved, see below.

Blind label geometry (room centre y 0.25, h 2.6, slab 0.26): `win_lo = -0.166`, label centre y `-0.866`, floor top `y_f = -1.05`, outer bottom edge `-1.31`. The label stays above the floor and the formula box, its right edge (about -1.55) is left of the wall (outer face about -1.5). "shortwave" sits at (-3.6, -0.4), about 0.47 higher and further left, so the two cannot overlap.

## Posters moved (by hand, via a scratch script outside the repo; sidecar `captured.language` read first, then removed)

- `de` (24): cooling 01, 02, 03, 06; ventilation 01-06; demand (EnergyBalance) 01-07; heating 01-06; physics PhysicalFundamentals_FullSection.
- `en` (2): Cooling_04_SolarRadiation, Cooling_05_Systemauslegung.

Sidecar content other than `captured.language` is unchanged, so moment and source fingerprints and the image sha256 stay valid.

## Missing posters now

- `de`: Cooling_04_SolarRadiation, Cooling_05_Systemauslegung (2).
- `en`: all except those two: 24 (cooling 01, 02, 03, 06; ventilation 6; demand 7; heating 6; physics 1).
- Posters whose poster segment sources changed (for example physics Fundamentals and demand Practice through Task B, plus anything edited earlier in this ticket) are stale and are reported by `resolve()` or refreshed by `prepare()`/`capture`.

## Commands to capture

```
bun 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts posters --language en --profile final
bun 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts posters --language de --profile final
```

Optional: `--video <id>`, `--chapter <scene>`, `--output-root <root>`, `--audience speaker|...`, `--force` (recapture non-stale ones). Only missing or stale posters are recaptured without `--force`. A final render of a chapter also refreshes them through `prepare()`. Because `de` has stale-looking sources after this ticket's edits, run `--language de` too (or `--force`).

## Tests (launcher `run-one-path.sh`, `SEMIO_TEACHING_PROFILE=final SEMIO_TEACHING_LANGUAGE=en`)

- `...🖼️posters.🧪️tests.🐍️`: 25 tests, OK (first run had 1 failure: my regex in the new test expected scene before language; fixed). New: `test_a_german_poster_is_never_resolved_for_an_english_render`, `test_assets_live_in_one_folder_per_language_beside_the_lesson_sources`, `test_preparing_an_english_render_recaptures_a_poster_that_exists_only_in_german`.
- `...🎬️clip.🧪️tests.🐍️`: 32 tests, OK.
- `...👥️audience.🧪️tests.🐍️`: 7 tests, OK.
- `physics...🧲️fundamentals.🧪️tests.🐍️`: 13 tests, OK.
- `demand...🧪️tests.🛡️guard.🐍️`: 8 tests, OK. `demand...🧪️tests.🔗️chain.🐍️`: 8 tests, OK.
- `...🪟️presentation.🧪️tests.🐍️` and `...🪟️presentation.🥞️hierarchy.🧪️tests.🧩️chapters.🐍️`: exit 0 (371 s and 376 s).

## Not done

No poster was captured and no render started. No Manim render of the two edited scenes was run (only their unit tests).
