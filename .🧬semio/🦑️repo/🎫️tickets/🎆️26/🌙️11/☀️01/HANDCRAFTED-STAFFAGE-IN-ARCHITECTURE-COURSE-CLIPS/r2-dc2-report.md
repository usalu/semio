# r2-dc2: documentation of the round-2 cast bridge

S = `🎓️teaching/🏛️architecture/🧍️staffage`, V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`.

## Found
- Docs that exist: `S/README.md`, the module docstring of `V/🧍️staffage/🐍️.py`, the cast schema `S/🔨️modules/📤️export/🎭️cast/🧬️schema/🔣️.schema.json` (+ TS twin) and the exporter header. No German twin of any README exists, so nothing to mirror.
- `S/README.md` had `phaseChange`, `thermalSpanK`, `referenceEnvironment`, exporter command and cancellation (from r2-dc/ed), but lacked the cast row table, the 600-frame cap with `rate`, the Python API, figure bounds, cache key and pruning, the guard rules and the add-a-device steps. Its thermal-span text called the per-Scene rule unchecked; the guard checks it.
- Cast schema covers every key the exporter accepts (`KEYS` in the TS twin, 16 fields); keys used by the 98 manifest casts are all in it. `periodS` is not a cast field (derived from the clip duration by the exporter), `figureBoundsM` and `loop` are index outputs, `rate` and `loop` are Python call arguments, `--environment` and `referenceEnvironment` belong to the general export CLI and the asset schema, not the cast exporter. No field was missing, so only descriptions changed.

## Changed (docs only, no behaviour)
- `S/README.md`: thermal span per Scene class; the exporter paragraph (signals, `[pid]`, unknown flags refused); new subsections Cast row (table of all fields), Textless casts and the frame cap (`rate`), Python API (`staffage`, `staffage_motion`, `fitted_staffage(window=, rate=, loop=)`, `staffage_height`, `staffage_parts`, `ensure_casts`, figure-bounded sizing, cache key and pruning, cancellation), Guard (Rule A, B, C, thermal span), Adding a device.
- Cast schema JSON: descriptions of `transportAgeS` (periodic field of a looping clip, derived period), `fps` (default 15, 600-frame thinning), `thermalSpanK` (per Scene class, guard) and `mirror`.

## Checks
- Schema parses as JSON. Python `Schema`/`manifest` tests (jsonschema validates the manifest against the schema): 3 tests, OK.
- Guard `🐍️staffage_only`: 25 tests, OK (392 s, host under load).
- Catalog check: not completed. `repo tools exec bun` waited on host memory pressure ("waiting 94 s for command") and was stopped; the catalog was not touched by this task.

## Open
- The module docstring of `V/🧍️staffage/🐍️.py` was left unchanged; the README is the single place for the API table.
- No German doc exists for the staffage README; none added (no twin to maintain).
