# S-GENERATOR-PREVIEW-OWNERS

## Outcome

All 14 taxonomy-v7 `owned` generator projects now register `preview-generated` through their existing `📜️script.ts`. The owner command emits one compact canonical JSON document and performs no live-output writes:

```json
{"contractId":"…","nodes":[{"bytesBase64":"…","mode":420,"nodeKind":"file","path":"…"}],"schemaVersion":1,"staleRemovals":[]}
```

Keys are lexical, paths are repository-relative NFC and UTF-8-byte sorted, directory bytes are empty, generated file bytes are base64, modes are explicit, and stale removals are byte sorted. No current owner emits a symlink; the frozen protocol reserves UTF-8 link-target bytes for that node kind.

Nine owners reuse an in-memory renderer. Actor, async, framework manifest, shell, and UI contract run their exact cargo exporter with both `SEMIO_TYPEGEN_OUT` and `CARGO_TARGET_DIR` under an isolated `mkdtemp` root and remove it in `finally`. Actor/async/shell generation now removes stale siblings from their singly-owned generated directory. Assets, graph, plugin registry, scale fixture, and styling generation remove the same stale paths reported by preview. Print continues to use `renderPrintLatexTokenStylesheet`; its owner router now supplies the schema-owned `🎨️styling/🔣️tokens.json` after the old package-local token path was removed.

## Owner matrix

| Contract | Owner target | Result | Nodes | Stale now |
|---|---|---:|---:|---:|
| `actor-typegen` | `@semio-tech/framework-actor-rs:preview-generated` | pass | 2 | 0 |
| `assets-build` | `@semio-tech/assets:preview-generated` | pass | 289 | 0 |
| `async-typegen` | `@semio-tech/framework-async-rs:preview-generated` | pass | 2 | 0 |
| `framework-manifest` | `@semio-tech/framework-rs:preview-generated` | pass | 1 | 0 |
| `graph-catalog` | `@semio-tech/framework-graph:preview-generated` | pass | 23 | 0 |
| `plugin-registry` | `@semio-tech/plugin-registry:preview-generated` | pass | 10 | 0 |
| `print-latex-tokens` | `@semio-tech/print:preview-generated` | pass | 1 | 0 |
| `scale-fixture` | `@semio-tech/framework-os-dev:preview-generated` | pass | 3 | 0 |
| `schema-entity-catalog` | `@semio-tech/framework-schema:preview-generated` | pass | 3 | 0 |
| `shell-typegen` | `@semio-tech/framework-os-shell-rs:preview-generated` | pass | 2 | 0 |
| `styling-tokens` | `@semio-tech/ui-styling-tokens:preview-generated` | pass | 10 | 0 |
| `ui-axes` | `@semio-tech/ui-rs:preview-generated` | pass | 2 | 0 |
| `ui-contract` | `@semio-tech/ui-contract-rs:preview-generated` | pass | 1 | 0 |
| `wgpu-frame-worker` | `@semio-tech/framework-renderer-wgpu:preview-generated` | pass | 1 | 0 |

All 14 exact owners return 350 complete node records and zero current stale removals.

## Repaired cargo blockers

The first matrix exposed two unrelated Rust test-compilation defects. Both received the compiler-directed minimal source repair before the final matrix:

- `🎭️actor/🦀️component.rs:5515`: compare the borrowed payload through `detail.as_slice()` rather than `&Vec<u8> == [u8; 3]`.
- `🎯️action-bus/🦀️component.rs`: derive `Debug` for `ToolWirePage`, satisfying the four test-only `Result::unwrap` error bounds.

Direct isolated reruns then returned `actor-typegen:2:0` and `framework-manifest:1:0`; the complete clean matrix repeated both successes.

## Acceptance evidence

The language-neutral matrix invoked every owner router directly, parsed stdout with `JSON.parse`, required `JSON.stringify(parsed) + "\n"` byte equality, exact root/node keys, schema version 1, NFC, byte ordering, valid kind/mode/base64 fields, empty directory bytes, and ordered stale removals. The final clean run passed all 14 owners:

```text
assets-build:289:0                 actor-typegen:2:0
graph-catalog:23:0                 async-typegen:2:0
plugin-registry:10:0               framework-manifest:1:0
print-latex-tokens:1:0             shell-typegen:2:0
scale-fixture:3:0                  ui-contract:1:0
schema-entity-catalog:3:0
styling-tokens:10:0
ui-axes:2:0
wgpu-frame-worker:1:0
```

Before and after the complete matrix, a SHA-256 inventory covered every registered owned output root, including ignored files, node kinds, modes, symlink targets, directory membership, file bytes, and absent roots:

```text
before  1a13ad605a852b8f3e9e5996a1c06015f9eaa1d87eee24c36fbd503f563c1364
after   1a13ad605a852b8f3e9e5996a1c06015f9eaa1d87eee24c36fbd503f563c1364
```

The nine in-memory manifests were also scanned independently with existing third-party `fast-glob`; every current physical output path belonged to its preview manifest. One schema Go output is currently absent, correctly remaining an expected generated node rather than being dropped from preview.

Project metadata validation loaded all 14 owner `📋️project.json` files and proved each schema-owned `previewTarget` equals the exact project name plus `:preview-generated`; every target uses `nx:run-commands`, the exact owner cwd, and only `bun ./📜️script.ts preview-generated`. `git diff --check HEAD -- <touched paths>` returned clean.

The requested FE0F regression correction split source and canonical mutation directory identities in the language-neutral golden. Source helpers retain selector-less plant paths while destinations/catalog rows use registry-canonical VS16 paths. Focused evidence:

```text
bun test …/🧪️index.test.ts '--test-name-pattern=(language-agnostic mutation projection golden agrees with fast-glob discovery|projects every registered golden bundle into artifact profile storage)'
2 pass; 215 filtered; 0 fail; 51 expect() calls; 4.05s
```

## Touched paths

Each of these 14 owner directories has its existing `📜️script.ts` and `📋️project.json` updated:

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/🖼️assets/📦️packages/🟦️typescript`
- `🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust`
- `🧰️framework/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry`
- `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript`
- `🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu`

Golden-only correction:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️index.test.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️mutation-path-projection/🔣️.json`

Minimal cargo unblockers:

- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
- `🧰️framework/🔨️modules/🎯️action-bus/🦀️component.rs`

No taxonomy, discovery, normalization engine, root CLI, catalogs, Compose/temp-Compose, Git state, `package.json`, or runtime dependencies were changed.

## Engine handoff

The engine can invoke each schema-owned `previewTarget`, require the frozen stdout shape above, hash decoded file/link bytes into `TaxonomyGeneratorNodeRecord`, and treat `staleRemovals` as absent post-state. It must reject non-JSON stdout, out-of-root paths, duplicate paths, wrong ordering/modes, and any post-generate mismatch.
