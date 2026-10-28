# Activity Pose Consistency

## Scope

The final Cooling review found hollow line figures for sleeping, walking and sprinting beside the filled seated figure. The chapter-local activity profile now loads three handcrafted closed SVG silhouettes through Manim. The existing seated geometry, four activity positions and heat values (80, 100, 200 and 800 W) remain unchanged. No shared profile, renderer or dependency was changed; only five sources in Cooling_02_InternalGains require refresh.

## Schema and Tests

Portable cases and the JSON schema preceded implementation. The original helper failed 12 geometry/raster subcases. The first SVG implementation exposed a 462-pixel enclosed hole in the sprint wrist contour; that contour was corrected. The final three tests pass (0.299 s): two-scale geometry and attachment checks, independent Manim Rectangle size oracles, actual Manim Camera raster masks checked with SciPy connected components/fill-holes, and seated/scientific-data preservation. The canonical authoring test command explicitly registers the local test module.

The 1600×900 comparison uses the actual production helper and unchanged scientific labels. Final visual acceptance and refreshed media evidence are recorded below after canonical publication.

## Reproduction

All commands run through the repo binary with UV_PROJECT_ENVIRONMENT and SEMIO_TEACHING_ENV set to C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python, PYTHONUTF8=1 and PYTHONDONTWRITEBYTECODE=1. Output/cache roots are ticket-local.

- Managed uv run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 --group test python -B -m unittest 🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🏃️activity.🧪️tests.🐍️
- Canonical authoring 📜️script.ts render --video cooling --chapter Cooling_02_InternalGains --quality l --resolution 640,360 --fps 6 --jobs 4 --no-play --log-output --output-root <ticket>/🗑️generated/full
- Canonical authoring 📜️script.ts render --video cooling --quality l --resolution 640,360 --fps 6 --jobs 4 --no-play --log-output --output-root <ticket>/🗑️generated/full
- Managed python -B <ticket>/verify-full.py and <ticket>/verify-sources.py

## Published Media and Acceptance

All five affected canonical sources (73–77) refreshed and Cooling reassembled through the canonical runtime; both commands exited 0. Current source/provenance checks pass for all 110 canonical sources. Independent audit accepted the standalone 1600×900 comparison, 12 actual source-74 frames (including targeted 30–40 seconds), and all eight frames of the refreshed full Cooling sheet. The filled comparison is visible at 209.5 seconds in the final movie. No actionable visual finding remains; see full-render-audit.md for exact sampled evidence.

Source 74 SHA-256: 4c45a259a06912d02c2fdbff45cd3e7497938ca76de851caf9e6ffc360003441.

Final Cooling SHA-256: 22b50ab73f2e75bf4c5f58ef54c9f3df09182ef4c649e65d48a2d9a9d1ab3f95. All 6,284 frames decoded at 640×360, nominal 6 fps, duration 1047.323 seconds. The four other full-movie SHA-256 values remain unchanged; all hashes are retained in full-render.md. This chapter-local repair introduced no dependency or shared-profile change.

## Canonical Aggregate Result

The canonical authoring test entry completed with exit code 0: 90 tests in 529.889 seconds, OK (one existing Windows symlink skip). The three activity tests run through explicit module registration in this command. No source edits followed final media/provenance verification.
