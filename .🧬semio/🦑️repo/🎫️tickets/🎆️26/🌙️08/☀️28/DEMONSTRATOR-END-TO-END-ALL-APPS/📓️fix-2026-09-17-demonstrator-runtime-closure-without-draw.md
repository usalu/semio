# Demonstrator runtime closure without unused flow-extension-draw

## Problem

`consumes: ["flow.extension"]` on `demonstrator`, `procedural`, and `flow` forced the Nx `materialize-release` graph to build every flow extension, including `flow-extension-draw`, which generation3d does not use and which blocked the mit-bestand distribution build.

Reaching `flow` as the `extends` host of any selected extension also re-expanded `flow`'s own `depends-on` list, pulling draw back in after explicit extension lists.

## Changes

1. **Cargo metadata** — `demonstrator` and `procedural` list eight flow extensions explicitly (no draw) instead of `flow.extension`. `flow` lists all nine extensions on `depends-on` for the standalone flow playground only when `flow` is the preparation root.
2. **`runtimeComponentClosure`** — Host plugins linked via `extends` are added **shallowly** (host wasm only, no host `depends-on` / `consumes` expansion).
3. **`🧫️cases.json`** — Removed `flow-extension-draw` from the expected extension roster (29 components in the union).

## Follow-up (puzzle materialize)

Demonstrator build also needed `@semio-tech/puzzle-plugin:materialize-release`. Example fixtures now ship pre-serialized `📄️document.json` (DSL parse fails at wasm descriptor probe for several puzzle2d fixtures). Puzzle3d/5d editors load JSON via `examples::…::SOURCE.document_json()`.

## Verification

- `demonstratorRuntimeComponentIds()` — 29 ids, `flow-extension-draw` absent.
- `bun nx test @semio-tech/mit-bestand-demonstrator` — 28/28 passed.
- `puzzle-plugin:materialize-release` — succeeds after component rebuild.
