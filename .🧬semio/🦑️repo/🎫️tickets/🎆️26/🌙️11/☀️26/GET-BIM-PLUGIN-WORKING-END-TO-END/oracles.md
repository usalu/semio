# Oracles report (bim)

## Routing finding
`repo test-file --dry-run` on a `🐍️.py` file: "Not a TypeScript or JavaScript test file (expected .ts, .tsx, .mts, .js, .mjs)". test-file has no python route, so the python oracles cannot run through it. They are run the way their own docstrings document (standalone `check <fixtures dir>`) with the locked workspace interpreter
`.cache/b/d/m/python/environments/workspace-3b97251b6599d245/bin/python` (3.14; shapely 2.1.2, ifcopenshell 0.8.4.post1, lxml 6.1.3, numpy 2.5.0, jsonschema 4.26, deepdiff, jsonpatch all importable). Runner script: `$T/oracles-run.py` (cwd = the tests dir). No repo code changed: adding a python route to test-file is a feature, not a minimal fix; left open.

## Python oracles (standalone check against committed fixtures), 32 entry points
PASS (oracle agrees): infer-energy, export-gbxml, export-svg, infer-phases, export-ifc (incl. its 7 sub-oracles classification-psets, kernel-helpers, template-library, ramps, wall-depth, element-checks, components, which are modules imported by the parent and have no own main), infer-zones, mutate-model-1-any (1576 quintets), infer-spaces, export-ifc4, infer-psets, export-bcf (issues.bcf, 3 topics), infer-sheets, infer-schedules, infer-annotations, export-sheets-pdf, infer-solids-rest, export-energy, infer-ceilings, infer-wall-solids, export-sheets-svg, infer-views, infer-plan-and-diagnostics, infer-ramps, gestures (60 cases), infer-stair-runs, infer-wall-depth, modify (36 cases), infer-families, infer-quantities, infer-wall-joins, infer-components, infer-levels-and-wall-heights, infer-opening-frames, infer-finishes, export-zoning.

FAIL then fixed: `export-bim-1-ifc-energy`: committed tables `🧫️fixtures/🏗️ifc/🔥️energy/🔬️measure/energy-{2x3,4}.json` were stale against the regenerated `.ifc` (slab `ThermalTransmittance` sl-a 0.169068596 -> 0.169031793, sl-g 0.795094883 -> 0.795110836; ~2e-4 relative, from the construction layer data). The audit itself (standard Pset templates, measure types, snapshot) reported no problem; only the table was stale. Rewrote the tables with the oracle's own `write` (independent ifcopenshell measurement of the committed files); `check` now agrees.

## TypeScript oracles
- export-bim-1-gltf: `check` against `🧫️fixtures/🧊️gltf` agrees (three 0.182.0), per previous agent.
- infer-bim-1-solids-three, solids-bim-1-wall-depth-three: `defineTestAdapter` oracles; they only run inside the conformance host against the Rust subject (bun not on PATH here; `repo` provides it). NOT RUN: `$T/model-rust.md` does not exist yet, Rust subject not confirmed compiled. Open: rerun once the model agent reports a green build.

## Open
- Python oracles ran against committed fixtures only (standalone mode). The subject-vs-fixture half (Rust adapters `🦀️.rs` producing the same tables) is the conformance host run, pending the Rust build.
- test-file python route missing (see above).

## Files changed
- `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🏗️ifc/🔥️energy/🔬️measure/energy-2x3.json`, `energy-4.json`
- `$T/oracles-run.py` (runner), `$T/oracles.md`
