# TypeScript agent report (bim)

## Ran
- `repo typecheck` (mcp) on plugin bim, dev variant bim, hub bim guest tests: 62 files, 0 errors, 0 drift, passed.
- `repo test-file` on the bim TS test files (bun plain suite):
  - sqlite snapshot tests 13 pass, schema parsers 68 pass, render world unit 6 pass.
  - `solids-bim-1-wall-depth-three`, `infer-bim-1-solids-three`, hub `guest/🟦️.ts`: load, 0 tests (they are `defineTestAdapter` oracles, executed by the conformance harness against the Rust subject, not by bun test).
  - `export-bim-1-gltf`: reported FAIL under test-file only because its standalone `import.meta.main` block runs without args. Run as intended: `bun <file> check <tests/🧫️fixtures/🧊️gltf>` -> "oracle agrees" for house and components (three 0.182.0). Not a real failure; no change made (the standalone-vs-bun-test routing is a runner quirk).
- Python oracles (`🐍️.py`, ~40 files): NOT run. `repo tools exec uv` cannot run: no sealed graph, and the python 3.12/3.14 interpreters in `.cache/b/t/m/python` have no shapely/ifcopenshell installed; installing needs a PyPI download (not permitted without user consent). Needs shapely 2.1.2 / ifcopenshell 0.8.4.post1 provisioned by the repo's python oracle host (Rust conformance run).

## Repo tool state
- `repo show projects` / `show project` fail: "no sealed generation of the workspace" (needs `repo graph --graph-policy fresh`, hours). I did not start it. Only project name found: `@semio-tech/bim-model-rs` (model package.json).

## Fixes
None needed in TS scope (no failures at root).

## Dev variant launch
- Variant `bim`: entry `✏️s/🧑‍💻dev/🎭️variants/🏙️bim/🚀️entry/🟦️.ts`, vite `.../🏗️builder/🌐️vite/🟦️.ts`, schema `.../🧬️schema/🔣️.json`. No bim entry exists in `.claude/launch.json`.
- Command (same pattern as `process3d-react` launch entry, port 6022):
  `cd /Users/ueli/Documents/semio && repo ./📜️script.ts dev playground bim react prototype`
  (usage: `dev playground <variant> <react|wgpu|native> <prototype|release>`). Port is chosen by the tool, read it from the output. Prerequisite: the plugin component built (`🌎️hub/🧩️compositions/🏙️bim/🧩️component`, rust agent) and generated catalog projection.
- Not started by me.
