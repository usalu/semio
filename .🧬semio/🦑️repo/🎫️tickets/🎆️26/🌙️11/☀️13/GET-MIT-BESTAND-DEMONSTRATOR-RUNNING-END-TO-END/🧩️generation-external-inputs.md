# Generation external inputs failure

Failure: `GENERATION_EXTERNAL_INPUTS` ("Bounded external input references required") on `activate-prototype`.

## Transport

No JSON transport is involved. `graph.externalInputs` is set in one place only: `⚡️caching/🚀️bootstrap/📜️script.ts:411`, from `nxBuildEngineExternalInputsV1()` (`library/🟨️.mjs:89`), passed in-process through `loadBuildGraphV1` -> `loadBuildGraphSliceV1` (`🕸️graph/📥️load/🟦️.ts:165`) -> `loadGenerationGraphV1` (`🔌️load/🟨️.mjs:357`). It is always an array (never null); the `=[]` default only covers `undefined`.

## Root cause

The list is built from `nxBuildEngineClosureV1().packages` (`library/🟨️.mjs:56-68`). A module specifier whose package segment is empty (e.g. an absolute or malformed import found while another agent was editing engine sources) produced `""` in that set: reproduced once as `["", "nx", "typescript"]` (1 invalid item), and the second run, after the engine sources settled, gave `["nx","typescript"]`. The schema (min length 1) then rightly refused it, but the message did not say which item.

## Changes

- `🕸️graph/🕰️generations/🟨️.mjs` `readGenerationExternalInputsV1`: the failure now names what was wrong (received type or null, item count vs max, item index and empty/longer-than/non-string).
- `library/🟨️.mjs` closure `admit`: throws `Private Nx engine import names no package: "<specifier>" from <file>` instead of admitting an empty package name, so the culprit file is named at the source.
- Tests in `🕰️generations/🧪️tests/🌊️stream/🔌️external/🟦️.ts`: "rejected external roots say what was wrong", "engine external roots are non-empty bounded package names". 4 tests pass (run from repo root with `SEMIO_TEST_ARTIFACT_DIR` set, `repo tools exec bun -- test`).

## Status

The failing run is likely transient from in-flight edits; rerun `activate-prototype`. If it recurs the message now names the culprit file or item.
