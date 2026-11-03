# r2-tc report: test coverage audit of the staffage bridge

## Already covered (before this round)
- Cast exporter (`🧪️tests/🎭️cast-export/🟦️.ts`, fixture `🧫️fixtures/🎭️cast-export/🔣️.json`): schema in ajv and twin, theme ink, transport plumes, timeline, 600-frame cap for a clip, mirror, determinism, abort, unknown ids; front/plan casts in `front-projection` and `plan-projection`; load clamp in `clip-sampling`.
- Python (`V/🧍️staffage/🧪️tests/🐍️.py`): placement vs svgelements, Manim SVG import cross-check, bounded frame cache, motion stepping, keys. `pendant_lamp` in `V/🧪️tests/🐍️icons.py`. Guard in `🐍️staffage_only.py`.

## Gaps found and closed
TypeScript (`🧪️tests/🎭️cast-export/🟦️.ts`, `🧫️fixtures/🎭️cast-export/🔣️.json`, new `🥒️.feature`):
- Only 8 invalid-cast vectors: added 16 (duplicate transport, viewpoint, scale 0, fps 0/121/7.5, mirror text, language, unknown property, non-slug/missing asset, environment plus timeline, empty timeline, negative t, ease, timeline outside thermal). Each is rejected by ajv and the twin.
- The course manifest (48 casts) was never validated against the schema: now ajv plus twin plus a catalog check (asset, pose, clip, viewpoint, preset exist).
- The 600-sample cap for a pose timeline (non-looping branch, last sample on the last key time): new case.
- Timeline given out of order equals the sorted one: new case.
- Cast-level front and plan viewpoints, including mirror in plan and front, differing pictures, plan depth, plan limbs stretching over a clip (followTip): new fixture casts and cases.
- Third-party cross-checks stay ajv (schema) and jsdom (SVG).

Python:
- `Schema`: the course manifest and the TS fixture vectors (valid and all 24 invalid) validated by `jsonschema` (Draft7) against the same schema file, so both languages share the vectors.
- `Reference`: lazy regex parser vs `xml.etree.ElementTree` (path count, group matrix, `_slot_count`, first transformed point, stroke styles).
- `Fit`: `fitted_staffage` unit equals the largest fitting scale (cases in the json), cross-checked with the svgelements bounding box; `rate` makes a `_Motion`, `loop` forces looping (4 cases).
- `Rates`: rate times scene time maps to the expected frame (4 cases).
- `staffage_only`: the guard flags the violation vectors (regex and AST), passes the clean ones, finds literal cast names (vectors), and every literal cast named by a clip, the library or `_WATT_CASTS` exists in the manifest (new real check, all pass). Vectors live in `🔣️staffage_only.json`.

## Results
- vitest (ticket config `q2-vitest-alias.config.ts`, filter `cast-export`): 1 file, 44 tests passed (was 22).
- Python: `🧍️staffage.🧪️tests.🐍️` plus `🧪️tests.🐍️staffage_only`: 41 tests OK (17 of them new or in new classes). `pendant_lamp` unchanged, covered by `🐍️icons.py`.

## Open
- No separate test that `_run` kills the exporter process tree on cancel (needs a real subprocess).
- The 600-frame pose-timeline test solves 600 thermal frames (a few seconds to a minute under load).
- Features exist as `.feature` only for cast-export; the Python side has no `.feature` (cases are json).
