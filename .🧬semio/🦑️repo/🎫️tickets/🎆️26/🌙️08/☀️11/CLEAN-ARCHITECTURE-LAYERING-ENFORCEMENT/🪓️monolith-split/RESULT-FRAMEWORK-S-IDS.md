# Result: framework no longer names `✏️s` plugin ids (work package 1)

Scope: remove hardcoded `s.`-namespaced plugin ids from framework production code (`🧰️framework`, including the `💻️os` product). Parent-owned areas (Cargo build scripts `📜️script.ts` under `📦️packages/🦀️rust`, `🏪️store`, `🎭️actor`, Member, Media) were not edited.

## Design decisions

1. **Neutral framework-owned ids use the canonical three-segment kind grammar** (`<domain>.<plugin>.<artifact>`, `ArtifactKindId` and the `artifactKind` JSON-schema pattern in `🧬️schema/🗿️artifact-reference`). The audit suggested `io.binary`, `io.text`, `io.sqlite-snapshot`; those are two-segment and violate the grammar, so the ids are `framework.io.binary`, `framework.io.text`, `framework.io.sqlite-snapshot`.
2. **Carriers cannot be injected by registration.** Plugins are separate component instances with their own registries, so a carrier registered by the stdio plugin would be invisible to every other plugin's `io_route`. A framework-defined constant that plugins import works in every instance, so the carriers moved down into the framework and the plugins now reference them.
3. **Plugin-owned data is supplied by the owning plugin through the existing open registration seam** (`PluginBuilder::contributes_topic` / manifest `topicContributions`), not by a framework constant: the space index is now a topic row `os.space-index`.
4. **Test-only pins are injected by the specific layer.** The MCP end-to-end journey takes its pins as a typed argument; the pins and the command that runs the journey live in `✏️s/🧑‍💻dev`.

## Per id

| Id | Role | Decision | Result |
|---|---|---|---|
| `s.stdio.binary@raw/*` (`CARRIER_BINARY`) | Payload-law carrier for raw bytes, used by identify, host import/export and job sniffing | Framework-owned neutral id | `framework.io.binary@raw/*` |
| `s.stdio.txt@utf-8/*` (`CARRIER_TEXT`) | Payload-law carrier for raw text | Framework-owned neutral id | `framework.io.text@utf-8/*` |
| `s.framework.sqlite-snapshot@1/*` (`SQLITE_SNAPSHOT`) | Framework container for native snapshots; never plugin-owned | Renamed to the framework domain | `framework.io.sqlite-snapshot@1/*` |
| `CARRIER_*_DIALECT` in `🎠️kernel/🟦️.ts` | Dead duplicate of the Rust constants, no importer | Deleted; the TS twins now live in `🚪️io/🧬️schema/🟦️.ts` (`CARRIER_BINARY`, `CARRIER_TEXT`, `SQLITE_SNAPSHOT`) | removed |
| `s.stdio.json` in `🖥️host/🦀️.rs` (2952, 3072) | JSON interchange dialect of the legacy composer protocol | Resolved from the composer's declared reads / the artifact's export dialects by the JSON standard id (`rfc8259`, a standard, not a plugin); the `s.{format_kind}` format lookup became the domain-agnostic `dialect_names_format` (`<plugin>.<artifact>` against `<domain>.<plugin>.<artifact>`) | no plugin literal |
| `s.space` + `s.space.space@1/*` (ShellHost TSX, wgpu Shell) | Schema id and dialect of the plugin-owned space artifact-index document | Plugin supplies them via the `os.space-index` topic contribution; the shell reads it (`spaceIndexOf` / `SpaceIndexContribution::from_topic_contributions`). The literal document id `"index"` stays in the shell | no plugin literal |
| `CLIENT_E2E_PINNED_*` (`🌉️mcp/🟦️.ts`) | Test-only pins of one plugin/kind/verb/inference | Typed `McpClientE2ePinsV1` argument of `runOsMcpClientJourney` / `runMcpClientEndToEnd`; pins in `✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts`; the `client-e2e` command moved to the s-dev project as `mcp-client-e2e` | no plugin literal |

Additional ids found while scanning and fixed because they were self-contained: `s.print.chart` to `framework.print.chart` (`📓️print`), `s.2d.drawing` to `framework.2d.drawing` (flow drawing engine), `owns_artifact_kind` in `🛂️describe` no longer assumes domain `s`.

## Files changed (mine)

Framework:
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs`, `…/🟦️.ts` (constants, TS carriers added)
- `🧰️framework/🔨️modules/🎠️kernel/🟦️.ts` (dead constants removed)
- `🧰️framework/🔨️modules/🛂️manifest/📇️space-index/🦀️.rs` (new child: `SPACE_INDEX_TOPIC`, `SpaceIndexContribution`), mounted in `🛂️manifest/🦀️.rs`; TS twin in `🛂️manifest/🟦️.ts` (`SPACE_INDEX_TOPIC`, `SpaceIndexContribution`, `spaceIndexOf`)
- `🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs` (legacy media bridge)
- `…/🏛️ShellHost/🟦️.tsx` (helpers `findSpaceIndexEditor`, `sessionIsSpaceIndex`; constant `S_SPACE_INDEX_DOCUMENT_ID` renamed `SPACE_INDEX_DOCUMENT_ID`)
- `…/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (minimal exact replacements: constants plus helpers `program_space_index`, `find_space_index_app`, `session_is_space_index`, 7 call sites; `rustfmt` parses the file, the only parse failure is an unrelated peer test file mounted by it)
- `…/🌉️mcp/🟦️.ts`, `…/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts`, `…/📋️project.json` (pins injected, `client-e2e` target removed)
- `…/🔌️plugin/🛂️describe/🦀️.rs`, `🧰️framework/🛍️products/📓️print/🦀️.rs`, `…/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs`, `…/🌊️flow/🖍️drawing/🦀️.rs`
- Tests/fixtures: `🔌️plugin/🖥️host/🧪️tests/🔬️unit/🦀️.rs`, `🔌️plugin/⚛️reactor/💼️jobs/🧪️tests/🔬️unit/🦀️.rs`, `🎠️kernel/🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts` (carrier string), `🚪️io/🧪️tests/🧫️fixtures/🪶️sqlite-snapshot-registration/🔣️.json`, `🚪️io/🧪️tests/🪶️transfer/🛂️route/🧫️fixtures/🔣️.json`, print sqlite fixture `🔣️.json`, drawing-kernel test

s plugins:
- 32 new-mechanism txt leaves (`🔤️txt/🔖️utf-8/✳️any/🦀️.rs`, export and import) of 16 artifacts (writer, mathematical, wfc grid2d and bitmap, vcs, animate, sequence, fem 2d and 3d, reasoning wires, remodel, dag, block 2d/5d/3d, sourcing): `TXT_DIALECT` is now `semio_framework::io_schema::CARRIER_TEXT`; unused `StandardId`/`SubsetId` imports removed.
- 16 TS twins of those io facets (`const TXT`), fem leaf TS doc comments, sequence twin imports `CARRIER_TEXT`.
- `✏️s/🧑‍💻dev/🔱️trinity/🪆️graph/🚪️io/🦀️.rs` (`TXT` = `store::io_schema::CARRIER_TEXT`).
- `✏️s/🔌️plugins/🪐️space/🗿️artifacts/🪐️space/🦀️.rs` (`space_index_topic_contribution`), `🌎️hub/🧩️compositions/🪐️space/🦀️.rs` (`.contributes_topic(...)`).
- stdio doc comments: stdio's `s.stdio.txt`/`s.stdio.binary` are no longer described as the carrier; no behaviour change there.
- `✏️s/🧑‍💻dev/🤝️mcp-client-e2e/🟦️.ts` (new pins), `✏️s/🧑‍💻dev/📜️script.ts` (`mcp-client-e2e` command), `✏️s/🧑‍💻dev/📋️project.json` (target).

Fixtures and generated descriptors hand-fixed (greenfield, no aliases): `sequence/…/🧪️tests/🧫️fixtures/📇️descriptor-parity.json` (4 rows); `fem` and `block` unit tests (sorted foreign list now starts with `framework.io.text`); hub `🌎️hub/🧩️compositions/*/🧩️component/🔣️.json` `ioEntries` of the 12 migrated plugins (32 rows, array re-sorted by the registry key `(from, into)`, verified sorted before and after); hub space component JSON gets the `os.space-index` row in `manifest.topicContributions` and `contributions.topicContributions`.
- `.vscode/🧩️launch.seed.jsonc` and `.vscode/launch.json`: `🛠️dev🌉️os-mcp🤝️client-e2e` now runs `@semio-tech/s-dev-composition:mcp-client-e2e`.

Old-mechanism composer references to the stdio *format* kinds (`DEP_TXT`, `reads()`, capability rows, `Generation3dFormatRow`) were intentionally left: they name stdio as a format owner, which is the plugin-to-plugin edge of work package 4, not a carrier.

## Verification

- `bun repo cargo --store=shared --configuration=prototype -- check -p semio-framework-io-schema`: exit 0, `Finished` (baseline before editing also exit 0). `--tests` for that crate fails with 37 errors, all `cannot find crate serde_json` in `⚠️refusal/🧪️tests` and `🏛️ownership/🧪️tests`: pre-existing, unrelated to this change.
- `-p semio-framework-os-kernel` (and with it os host, plugin, plugin-host): **blocked by source-boundary gate** (`Rust crate Source refused before compiler: semio-framework-job`, baseline and after, identical). Not looped. Static substitute: `rustfmt --edition 2021 --emit stdout` parses every edited Rust file (the 32 leaves, trinity graph io, host, manifest space-index, space artifact, hub space, describe, print, sqlite snapshot, drawing, io-schema, host unit test, wgpu Shell); plus exhaustive grep of every use of each changed constant/id (`CARRIER_*`, `SQLITE_SNAPSHOT`, `S_SPACE_INDEX*`, `space_index_dialect`, `SPACE_INDEX_DIALECT` in framework, `CLIENT_E2E_PINNED*`, old coordinates) showing no remaining consumer. Plugin crates and wasm targets were therefore not compiled.
- TypeScript: `tsc --noEmit` over the touched TS files plus their import closure: 65 errors, none in a file I touched after fixing two mine in ShellHost (type guard `sessionIsSpaceIndex`); the rest are pre-existing in peer files (`📃️UiDocumentStore/🧪️tests/🧪️typedwire`, sourcing sqlite snapshot test, `🔌️PluginRuntime`, `🪪️WasmSessionLoader`, service-composition ownership test).
- Test run: `bun -e` of `testArtifactIoDescriptorParity` for sequence: `checks=45, entries=8` passed (Rust-vs-TS fixture parity with `framework.io.text`). Kernel vitest via `bun repo run @semio-tech/framework-kernel:test` failed before running with `Original selected owner registry changed` (repo build-graph owner registry moved under concurrent edits), not run.
- Not run: any Rust unit tests (fem/block foreign-list tests, host unit, hub, wgpu), the wasm component rebuilds, the hub component descriptor freshness check.

## Must-do follow-ups

1. **Parent-owned file, needs the same rename:** `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📜️script.ts` line 2173 contains an oracle literal `'s.framework.sqlite-snapshot'` (function `sqlite`); it must become `'framework.io.sqlite-snapshot'`, otherwise that oracle no longer matches the updated route fixture `🚪️io/🧪️tests/🪶️transfer/🛂️route/🧫️fixtures/🔣️.json`.
2. Rebuild the space plugin component and regenerate its descriptor: the `os.space-index` row exists in source and in the hand-edited hub JSON, but the shipped wasm only contains it after a rebuild. Same for the 12 plugins whose `ioEntries` order changed (JSON already matches the new registry order).
3. Compile `os-kernel`, `os`, `plugin`, `plugin-host`, the wgpu renderer and the 16 plugin crates once the source-boundary gate admits them.

## Residual `"s.` literals in framework production code (non-comment)

Was 25 lines in the audit; the listed work-package items are 0. Remaining:

- `🛍️products/💻️os/🖥️host/🦀️.rs` `dialect_from_component_kind`: `format!("s.{component_kind}")`. The legacy `ArtifactKindSpec.component_kind` slug carries no domain; fix needs the spec to carry its qualified kind (separate work package).
- `🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🧬️surface-schema/🟦️.ts:110`: `s.${pluginId}[.artifact]` kind derivation (same cause).
- `🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🗄️durable-group/🦀️.rs:3421`: `"s.gis.gismap@1/*"` parent check (`🏪️store`, parent-owned, not touched).
- `🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts:3326` (`s.stdio.`) and `…/🧾️provenance/🏗️authoring/🟦️.ts:143` (`s.stdio.step`): repo product (work package 3).
- Parent-owned build scripts: `🖥️host/📦️packages/🦀️rust/📜️script.ts` 1072-1073 (invalid-dialect test probes) and 2173 (see follow-up 1).
- Fixture-only literals inside production files: `🔌️plugin/👷️job/🦀️.rs:290`, `🔌️plugin/🧬️schema/🪶️sqlite/🦀️.rs:189` (test code).

Framework tests still referencing `s.stdio.*` as plain artifact kinds (store addressing fixtures, hub-source resolution, natural-file lifecycle, dsl originals) are work package 2 and were left.
