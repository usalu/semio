# Computed Plugin Reader Audit — 2026-09-09

## Scope

Read-only audit of composed filesystem readers in canonical TypeScript and TSX test cases below `✏️s` and `🧰️framework`. The repository library and Hub were excluded. The audit also excluded the Stdio, GIS, Norm, and Process reader work and the remaining noncanonical corpus classification assigned to other agents.

The retained probe is [`📜️script.ts`](./🧑‍💻coordination/🔎️computed-plugin-reader-audit/📜️script.ts). It follows locally evaluable `Bun.file`, `readFile`, `readFileSync`, `readdir`, and `Bun.Glob` expressions through literal paths, templates, URL bases, constants, and `join`/`resolve` calls, then checks the resolved target on disk. Its last pre-repair run inspected 462 test files and 1,022 reader expressions; it identified 23 absent resolved paths. Manual review removed repository-root traversal false positives and assigned the remaining actual reader defects below.

## Findings reported for repair

| Source | Broken expression and resolved absence | Verified canonical target / correction |
| --- | --- | --- |
| `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🕸️graph/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts:10-11` | The schema URL crosses through `🧫️fixtures`, so the window schema read targets an absent path and the mutation schema resolves to the fixture instead. | Use the existing direct window and mutation schema JSON paths. |
| `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪟️window/🎚️config/🧪️tests/🔬️window-config-ownership/🟦️.ts:9,15` | The same schema-through-fixture construction points both reads at absent or wrong targets. | Use the existing direct schema JSON paths. |
| `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🧪️tests/🧩️suite/🟦️.ts:20` | The template `./🧫️fixtures/${name}` passed to `new URL` resolves under the case directory, which has no fixture directory. | The named fixtures exist at the IO owner; use `../../🧫️fixtures/${name}`. |
| `✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📺️renderer/🧪️tests/🧪️repluserfacingsuggestiondetail/🟦️.tsx:471` | `resolve(source.directoryname, ...)` accesses an undefined property and throws before the asset read. | `TestSource` defines `source.directory`; the existing `📚️examples/🖼️assets/🎮️play/🔣️.json` asset is reachable from that property. |
| `✏️s/🔌️plugins/🗒️note/🧪️tests/🧭️action-cohort/🟦️.ts:8-14,57` | `root = resolve(import.meta.dir, "..")` is `.../🧪️tests`, making the three `🗿️artifacts` source readers resolve under nonexistent `🧪️tests/🗿️artifacts`; `fixturePath` points to nonexistent case-local `🔣️.json`. | Set `root` to the Note owner (`../..`) and load the existing `🧫️fixtures/🧪️action-cohort/🔣️.json`. The editor source, retained source, and schema all then resolve in the existing Note artifacts tree. |
| `🧰️framework/🔨️modules/🖼️assets/🥽️mesh/🧪️tests/🧩️suite/🟦️.ts:6` | `resolve(import.meta.dir, "🔣️.json")` targets an absent case-local file. | The canonical Mesh fixture exists at `../../🧫️fixtures/🔣️.json`. |
| `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts:223` | `join(import.meta.dir, "🧫️command-boundaries.json")` targets an absent case-local file. | The fixture exists at `../../🧫️fixtures/🧫️command-boundaries.json`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧹️config/🟦️.ts:29` | `packageDir/🧫️fixtures/⚙️config-graph.json` targets an absent case-local fixture. | The existing Dev owner fixture is `../../🧫️fixtures/⚙️config-graph.json`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🧪️tests/🧩️component/🟦️.ts:46` | Four parent segments before `🌉️mcp` stop at `renderer/engine`, where the bridge fixture does not exist. | The existing MCP bridge fixture needs six parent segments before `🌉️mcp/🧵️bridge/🧫️fixtures/📨️frames.json`. |
| `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🧪️tests/🧪️source-contract/🟦️.ts:128-131` | `mutationLeaf` ends in `🔁️set-state` without `/`; the three relative URLs therefore resolve under the parent `🧬️mutations` directory, where no files exist. | Add a trailing slash to the base URL. The descriptor, schema, and fixture exist under `🔁️set-state/{🔣️.json,🧬️schema/🔣️.json,🧫️fixtures/🔣️.json}`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/📦️native-codec-send/🟦️.ts:11` | `new URL("./🧪️tests/🔣️.json", testSourceUrl.href)` targets the removed codec test-local path. | The codec owner has `./🧫️fixtures/🔣️.json`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔗️backbone-detach/🟦️.ts:11` | The same `./🧪️tests/🔣️.json` reader targets the removed backbone test-local path. | The detach owner has `./🧫️fixtures/🔣️.json`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧪️tests/🪪️runtime-identity/🟦️.ts:11` | The same reader targets the removed runtime test-local path. | The runtime owner has `./🧫️fixtures/🔣️.json`. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔣️codec-caller-source/🟦️.ts:280` | The same reader targets the removed codec test-local path. | The send owner has `./🧫️fixtures/🔣️.json`. |
| Fourteen `✏️s/🔌️plugins/.../📚️examples/.../🧪️tests/🧩️example/🟦️.ts` BESTEST cases | `join(here, "../../🖼️assets/🗣️.dsl.semio")` resolves to the removed example-local asset location. | Each canonical subset asset exists under its owner `🖼️assets/<BESTEST>/🗣️.dsl.semio`; from the case directory it needs four parent segments. |

## Concurrently cleared observation

The Writer window-state ownership schema readers were reread after a concurrent update. All four now point directly to their canonical schema JSONs; this audit does not list Writer as an outstanding finding.

## Verification boundary

This was a static filesystem-resolution audit only; no application or native test suite was run. The reported expressions and every proposed canonical target were inspected on disk. Final runtime verification belongs to the coordinator’s closed repair batch.
