# GIS TypeScript projects

Status: BLOCKED, no target of gis-js, gis-gismap-js or gis-gisterrain could run.

- `repo show project` fails for all three: whole-workspace graph has no sealed generation (needs `repo graph --graph-policy fresh`).
- `@semio-tech/gis-js:test` fails in plan phase: the graph slice worker exits 75 three times with
  `Cannot find module .../🌊️flow/🕸️wasm/🌐️browser/📝️declaration/🤖️generated/🟦️.d.ts`.
- That file is gitignored and written only by `test-browser-ownership` of `semio-framework-os-flow-browser-package`
  (via `📤️projection/🟦️.ts`, moved on 2026-11-21). Running that target fails before writing it:
  `host-runtime exports` assertion in `🏷️ownership/🧪️tests/🏷️browser-ownership/🟦️.ts:59`; the runtime exports
  `FLOW_RETAINED_GRANT` but fixture `🏷️ownership/🧪️tests/🧫️fixtures/🔣️.json` does not list it.
- Owner is a concurrent framework Flow change, outside GIS scope. No files changed.
