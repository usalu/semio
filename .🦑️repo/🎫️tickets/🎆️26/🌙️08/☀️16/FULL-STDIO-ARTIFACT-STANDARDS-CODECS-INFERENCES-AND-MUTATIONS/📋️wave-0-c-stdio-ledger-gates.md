# Wave 0-C Stdio Ledger And Gates

## Delivered Surface

- `📇️catalog.json` is the schema-owned artifact source. Its 36 roster rows derive the ledger's artifact count, dependency edges, directories, extensions, optional registered MIME values, dialects, and discovered codec links.
- `epw.mime` is `null`: EPW remains extension/sniff-routed through `.epw` and has no MIME support claim. `txt` is the sole `text/plain` registrant.
- `📜️script.ts stdio ledger` emits machine-readable `ArtifactDefinition` data. The current verified ledger contains 36 artifacts, 35 registered MIME values, 88 taxonomy dialects, 88 native codecs, and 162 cross-artifact codec links (250 codec definitions total).
- The ledger rejects duplicate raw catalog IDs before `JSON.parse` can overwrite them, then rejects duplicate registered MIME values, extensions, dialect identifiers, codec identifiers, TypeScript exports, manifest IDs, and duplicate DAG edges. It also proves the checked-in DAG equals the definitions' dependency relation.
- Semantic support comes solely from existing standard/subset/schema/IO taxonomy paths. Mutation and inference facets are nullable when no taxonomy exists; the ledger does not claim unsupported facets.
- The manifest now has 36 descriptor rows. Its test derives id/directory/extension/MIME from the catalog; `null` becomes an empty string only at the legacy `FormatDescriptor` boundary.
- The TypeScript facade exports all 36 catalog artifacts and its package test checks that alignment at runtime.
- Ordered workspace Nx targets and matching `launch.json` entries now cover quick, long, exhaustive, schema-parity, standards-coverage, codec, mutation-law, inference, runtime, fuzz, and cross-platform gates.

## Removed False Artifact Root

The generic `🗿️artifacts/🧬️schema` directory contained only the literal `placeholder` EBNF file. A whole-repository exact-path reference scan found no consumers, so the isolated file and its now-empty parents were removed. The ledger now fails if that uncatalogued root returns; it cannot become artifact 37.

## Verification

| Command | Result |
| --- | --- |
| `bun ./📜️script.ts stdio ledger \| jq '.counts'` | Passed: 36 artifacts, 35 registered MIME values, 88 dialects, 250 codec definitions. |
| `bun nx run workspace:stdio-inference` | Passed the ordered quick → schema-parity → standards-coverage → codec → mutation-law → inference chain. Nx emitted a non-fatal `MaxListenersExceededWarning` after the sixth short-lived command process. |
| `bun nx run @semio-tech/stdio-js:test-quick` | Passed: facade exposes 36 catalog artifacts. |
| `bun nx run @semio-tech/stdio-plugin:test-quick` | Blocked before stdio tests by the externally-owned `🧊️gltf` glue mount at `📦️glue.rs:2249`, which references a missing `🧬️mutations/🧭️planning/🦀️component.rs`. Reproduced after W0-B's registry Result integration. |

## Integration Notes

- W0-B owns the IO registry. Its focused tests pass: `FormatDescriptor.mime` remains a string, `registered_mime()` maps empty/whitespace to `None`, and duplicate identity/extension/non-empty-MIME registration rejects atomically. Those tests prove that txt plus empty-MIME EPW succeeds while a duplicate non-empty MIME fails.
- The manifest now calls `register_format_descriptors(...).expect(...)`, so a registry conflict is never discarded at the legacy boundary.

## Touched Paths

- `✏️s/🔌️plugins/🗄️stdio/📇️registry/📇️catalog.json`
- `✏️s/🔌️plugins/🗄️stdio/🛂️manifest/🦀️component.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧬️schema/📸️snapshot/📝️text/🔤️component.ebnf` (removed)
- `✏️s/🔌️plugins/🗄️stdio/📦️packages/🟦️typescript/{📦️index.ts,📜️script.ts,📋️project.json,package.json}`
- `📜️script.ts`
- `📋️project.json`
- `.vscode/launch.json`
