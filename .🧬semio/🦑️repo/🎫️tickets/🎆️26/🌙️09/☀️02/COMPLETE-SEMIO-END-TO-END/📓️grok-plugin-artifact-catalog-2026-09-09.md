# Stdio-First Plugin/Artifact Catalog

Lane: nonempty verified plugin/artifact catalog at OS boot.
Ticket remains open (`2026/09/02/COMPLETE-SEMIO-END-TO-END`). Repo MCP was not used.

## What the catalog now contains

`trusted-catalog-publish` / `generate` emit a first-party stdio catalog with:

- 26 native codecs from the stdio first-party `native-codec-factories.json`
- each `packSchemaHash` is the Node crypto SHA-256 of the real protocol file (26/26 match)
- exactly one open target: `stdio.json` editor (`stdio.native.json.v1`)
- `publication: committed`
- `hubBundle: withheld` — hub `trusted-catalog/current.json` is not written
- `componentAdmission.status: exceeds-limit` — `semio_s_plugin_stdio_component.core.wasm` is 376957060 bytes against the 67108864 (64 MiB) trusted-component bound

Written to:

- plugin-registry `generated/trusted-stdio-catalog.json`
- `.🧬semio/🌐os/trusted-stdio-catalog/trusted-stdio-catalog.json`
- OS reader: `readTrustedStdioCatalog()` in the registry catalog module

Catalog SHA-256 of the committed bytes: `f907a173c2a2f9ca87cb8fbb83d9536e39126b2893776cb9aab87d901ad442e0`

Kinds: `stdio.avi`, `stdio.bcf`, `stdio.csv`, `stdio.deflate`, `stdio.docx`, `stdio.dwg`, `stdio.dxf`, `s.stdio.gltf`, `stdio.jpg`, `stdio.json`, `stdio.las`, `stdio.md`, `stdio.mp3`, `stdio.mp4`, `stdio.obj`, `stdio.pdf`, `stdio.ply`, `stdio.png`, `stdio.pptx`, `stdio.step`, `stdio.stl`, `stdio.svg`, `stdio.tiff`, `stdio.xlsx`, `stdio.xml`, `stdio.zip`.

## What this does not claim

- `catalog-complete` is still withhold-all. This command is incremental stdio publication, not a fake 59/59 complete.
- `plugins.json` still has no `stdio.hashes`. Hashes are only committed from a real owner descriptor pair. Stdio still has no descriptor pair. Inventing `descriptorSha256` would be a fake. `catalog-root` cannot admit the 377 MiB wasm under the 64 MiB bound.
- Hub `load_current` stays empty. Writing `current.json` without an admitted component would fail-closed at hub startup. GIS/VCS providers remain linked and unpublished.
- 18 other owners still lack hashes: `block`, `flow-extension-bim`, `flow-extension-draw`, `imperative-extension-control`, `imperative-extension-effect`, `imperative-extension-logic`, `imperative-extension-math`, `imperative-extension-text`, `playbook`, `playbook-module-procedural`, `process-extension-concrete`, `process-extension-metal`, `process-extension-robotic`, `process-extension-wood`, `sourcing-module-beams`, `sourcing-module-slabs`, `sourcing-module-windows`, `trinity`.
- `layout` / `draw` / `energy` load gaps are unchanged.

## Test evidence

Ran `tests/trusted-stdio-catalog` via vitest (4 tests, 3.85s):

```
Test Files  1 passed (1)
Tests  4 passed (4)
Duration  3.85s
```

Coverage:

1. AJV draft-07 validates the language-neutral fixture and the published `TrustedStdioCatalogV1`
2. Node crypto SHA-256 of every first-party protocol file matches the factory digest; kinds are unique and sorted
3. Written catalog bytes hash to the publication receipt
4. Hostile empty receipts, digest mismatch, and missing protocol fail closed

The default 15s `test` budget on the full registry suite was not used as pass evidence. Unrelated launch / catalog-complete failures in that suite were not treated as this lane.

## Files changed

- plugin-registry `script.ts` — build/publish/verify, generate emit, `trusted-catalog-publish`
- plugin-registry schema `trusted-stdio-catalog`
- plugin-registry fixture `trusted-stdio-catalog`
- plugin-registry test `trusted-stdio-catalog`
- plugin-registry `project.json` — `trusted-catalog-publish`
- plugin-registry catalog module — `readTrustedStdioCatalog`
- plugin-registry generated `trusted-stdio-catalog.json`
- `.vscode/launch.seed.jsonc` and `.vscode/launch.json` — `trusted-catalog-publish` (order 206.0665)
- `.🧬semio/🌐os/trusted-stdio-catalog/trusted-stdio-catalog.json`
- this report

Launch: `bun nx run @semio-tech/plugin-registry:trusted-catalog-publish` (optional `--out <dir>`).
