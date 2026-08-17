# Diagnosis: Demonstrator Boot Hang

## Symptoms
- All demonstrator pane apps stick in loading
- Console: `resolvePlaygroundBoot(...): Plugin "X" needs "Y", which is not installed`
- `parseBackboneWorkerWire` TypeError from React DevTools `postMessage` into the worker
- `:6029/extensions/watch` and `/plugin-modules/watch` 404s (secondary)

## Root cause (hang)
`expandPluginRegistry` in non-host mode only kept the primary plugin plus `consumes`/`contributes` matches. It did **not** pull the transitive `dependsOn`/`dependencies` closure.

Demonstrator panes map to plugin `demonstrator`, which is **not** a host (`PLUGIN_HOST_CONFIGS` only lists `s`). Boot therefore expands:
- `demonstrator`
- flow/process extensions matching `consumes` tags

…but drops `cad`, `gis`, `procedural`, `process`, `puzzle`, `sourcing`, `stdio`, `flow`, etc. Ordering then reports every extension as missing those deps and drops them — apps never leave loading.

## Secondary
1. Backbone worker decodes every `onmessage` as wire; React DevTools posts unrelated messages → noise/crashes (should ignore non-wire).
2. Puzzle 2d wasm file missing `use crate::editor::puzzle2d::engine::…` imports → `FORCE_PLUGIN_BUILD` / puzzle rebuild fails (staged demonstrator wasm still present).

## Fix plan
1. Expand registry by transitive dependency closure of primary + contributors.
2. Ignore non-wire worker messages.
3. Restore puzzle wasm imports so plugin rebuilds succeed.
