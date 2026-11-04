# Generated Manifest Prerequisites

## Result

Restored both missing canonical TypeScript projections. Focused strict TypeScript checking of the actual Canvas source and its import closure now exits **0**, with no diagnostics. The previous Tree/virtual-window ending-return diagnostic is absent. No Tree or other manual product source was edited; no locks, dependencies, stubs, modifying Git commands, or whole-workspace compilation were used.

## Canonical Generators

- Manifest: `🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`, `validate()` and `render_typescript()`. The canonical exporter test calls precisely these functions (`📽️projection/🧪️tests/🦀️.rs`).
- UI contract: `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🟦️projection/🦀️.rs`, the same functions. Its canonical exporter additionally asserts 92 metadata types, which the runner also checks.

Both renderers depend only on the Rust standard library and their owned adjacent metadata modules. To avoid the framework's large Cargo graph, `manifest-generator.rs` in this ticket imports these existing modules directly without copying or rewriting their implementation. A single small rustc compilation executes the exact canonical renderers. No generated TypeScript was handcrafted.

## Outputs

Relative to `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/`:

| Output | UTF-8 Bytes |
| --- | ---: |
| `🪪️manifest/🟦️.ts` | 97420 |
| `📜️ui-contract/🟦️.ts` | 49345 |

Both outputs were read back and byte-compared with the canonical rendered strings. Metadata validation passed for both and the UI contract metadata count was 92. The compiled ticket runner was removed immediately after execution; existing generated ticket output owned by other validation was preserved.

## Reproduction

Generation was run through `/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light node -- -e` using Node's `spawnSync` to invoke the absolute repo launcher with:

```text
tools exec --light rustc -- --edition=2021 --crate-name manifest_prerequisite .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️17/SHARED-UI-BORDERS/manifest-generator.rs -o <absolute ticket>/🗑️generated/manifest-generator
```

The same Node invocation ran the resulting binary from the repository root, then deleted only that binary. Rustc used no Cargo dependency graph.

The subsequent exact source check (exit 0):

```sh
/home/clstb/.local/share/semio/launcher/bin/repo tools exec --light node -- node_modules/typescript/bin/tsc --noEmit --target ESNext --module ESNext --lib DOM,DOM.AsyncIterable,DOM.Iterable,ESNext --moduleResolution bundler --jsx react-jsx --strict --esModuleInterop --isolatedModules --resolveJsonModule --allowImportingTsExtensions --skipLibCheck '🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx'
```

No broad UI test suite was rerun: the supplied 713 passing tests were existing evidence. Shared-ticket MCP lifecycle remains with the lead; this assignment restored only canonical generated prerequisites and validated the focused source check.
