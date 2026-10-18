# Result c6ts: monolith census 6 TypeScript slice (P71, P72, snapshot over-10k)

`F` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split`. `C` = this ticket folder. Nothing was built through `bun repo`, `nx`, `cargo` or `bazel`. No git command that modifies anything was run.

## Verdict

| File | Generated? | Before | After | Verdict |
| --- | --- | --- | --- | --- |
| P71 `✏️s/🔌️plugins/🏛️architect/…/🪶️sqlite/📸️snapshot/🟦️.ts` | no | 2,845 | leaf 260, largest child 976 (8 children) | split |
| P72 `✏️s/🔌️plugins/🗄️stdio/…/7️⃣1.7/…/🧱️base/🚪️io/📝️text/📸️snapshot/🪪️native-json/🧬️schema/🟦️.ts` | no | 8,111 | leaf 292, largest child 910 (17 children) | split |
| over-10k `✏️s/🔌️plugins/🗄️stdio/…/7️⃣1.7/…/🧱️base/🧬️schema/📸️snapshot/🟦️.ts` | no | 11,167 | leaf 197, types in 13 domain files (largest 133); the 10,200-line `schema` copy is replaced by a JSON import of `🔣️.json` | split, schema-first |

Generated check (all three are hand-written):
- No generator header in any of the three. No generator in the repo emits them: the library `schema parsers` generator writes `//#region 🚪️Parsers` blocks with `<prefix>Object/Array/String` helpers, while these leaves use `validateAgainst` and `registerSchemaDocument`. `bun F/c5e-tools/check-scope.ts ✏️s/🔌️plugins/🗄️stdio` and `… 🏛️architect` print `fresh: pending=0 dangling=0 helperless=0 drift=0 orphans=0` (`unowned=52` and `unowned=72`), so the generator owns none of their parsers.
- The embedded `schema` of the snapshot leaf deep-equalled `🧬️schema/📸️snapshot/🔣️.json` (checked), so by coordinator decision it is now `import schema from "./🔣️.json" with { type: "json" }` (the convention the PDF facade already uses for `📜️artifact-definition.json`). The native-json schema is the same document without the `x-semio-ui` annotations and with 28 other definitions that differ, so it is its own hand-maintained transport schema.
- Not touched: `c5h`'s parser-projection generator work. `F/RESULT-CENSUS5-H.md` did not exist when I finished. I did not edit the generator.

## What changed

P71 architect (domain children next to the leaf, names follow the model modules and the Rust siblings `📏️cells` and `💰️backing`):
- `📏️cells` (scalar codecs, `Cursor`), `💰️backing` (`ordered`, `Reader`, `header`, `quantity`), `🏷️symbols` (closed vocabularies), `🔗️trace`, `🗃️decision-records` (976), `📋️requirements` (976), `🏗️program-structure` (446), `📜️document-root` (meta, project, governance, child slots).
- The leaf keeps `count`, `forecast` and the two public functions (`programSnapshotToSqliteDatabase`, `programSnapshotFromSqliteDatabase`), so no importer changed.
- `model.X` became explicit named imports. Only names another file imports carry `export`.

P72 native-json schema:
- `export const schema` keeps the root keys (`properties`, `required`) and builds `` by spreading `cosDefinitions`, `documentDefinitions`, `metadataDefinitions`, `navigationDefinitions`, `formDefinitions`, `colorDefinitions`, `contentDefinitions`, `imageDefinitions`, `shadingDefinitions`, `stateDefinitions`, `fontDefinitions`, `annotationDefinitions`, `annotationKindDefinitions`, `streamRoleDefinitions` from `native-json/🧬️schema/<domain>/🟦️.ts`. It cannot be a JSON import: it is a different document (no `x-semio-ui`, 28 definitions differ) and has no `🔣️.json`.
- `PdfOp` (a 71-variant `oneOf`) is split into `🖋️content/🛤️path`, `🔤️text`, `🎨️paint` variant arrays, spread in the original order. `` key order is now grouped by domain instead of alphabetical; validation is by JSON pointer, so order carries no meaning.

Snapshot leaf (schema-first):
- The 83 `export interface/type` declarations moved verbatim into `🧬️schema/📸️snapshot/<domain>/🟦️.ts` (types only, `import type` between domains): `🧩️cos`, `📄️document`, `📇️metadata`, `🎯️navigation`, `📝️form`, `🌈️color`, `🖋️content`, `🖼️resource` (+ `🌈️shading`, `🎚️state`), `🔤️font`, `📌️annotation` (+ `🏷️kind`).
- `PdfSnapshot`, the validator region and the 83 `parse<Export>` functions stay in the leaf, which now reads the schema from `./🔣️.json`. The leaf no longer exports `schema` (no importer used it). The 17 interim definition files were deleted; only `🧬️schema/📸️snapshot/🧬️schema/♻️retirement/` remains there.
- The compatibility re-export `export type { Binary64 }` is gone; importers use `@semio-tech/framework/value/ieee754` directly.

Importers repointed (78 files, no barrel): every file under `7️⃣1.7/🪆️subsets/🧱️base` that imported the snapshot leaf now imports each type from its domain file. That is the sqlite and native-json domain siblings and their tests, `🧬️schema/🪪️stream-roles` (+ test), `🧬️schema/🔺️diff/🟦️.ts`, 37 of the 64 `🧬️schema/🧬️mutations/*/🟦️.ts` payload files, and the facade `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🟦️.ts` (`PdfSnapshot` from the leaf, `PdfPage` from `📄️document`, `Binary64` from ieee754). The 4️⃣1.4 PDF tree has its own leaf and was not touched. Quote style of each file was kept (the mutation and diff files use single quotes).

## Verification (exact tools in `F/c6ts-tools/`)

| Check | Command | Outcome |
| --- | --- | --- |
| Architect units lossless | `archsplit.py verify` | 194 units original, 194 new, 0 missing, 0 extra (modulo the `model.` prefix and `export`) |
| Schema data lossless | `pdfsplit.py verify` for native (bun evaluates the new root + children and the original, sorted-key JSON) | native: equal (65,389 chars), 84 definitions |
| Snapshot schema = JSON import | `pdfjsoneq.ts` (bun: canonical sorted-key JSON of the `🔣️.json` import vs the original TS copy kept in `F/🗑️generated/c6ts-orig/pdf-snapshot.ts`) | equal (117,994 chars), 86 definitions |
| Types lossless | ad hoc compare of `parse_types` over old and new | 83 of 83 declarations identical |
| Type check | `tscdiag.py` (TypeScript 5.9.3 from the bun cache, `paths` for all `@semio-tech/*` workspace exports) over the 189 PDF and architect files, repeated after the JSON switch (same noise codes only, no new code) | architect: 45 diagnostics before and after, identical multiset; PDF: no TS2304, TS2305, TS2459, TS2724 (they were 315 + 148 before the importer rewrite); the rest is environment noise (`Bun`, `bun:test`, `ajv`, `node:*`, framework `import.meta.vitest`) |
| Unused imports | `unusedimports.py` | architect children and PDF new files clean; pre-existing unused names in the generated diff and mutation files were left alone |
| Parse | `bun build --no-bundle` on every new file | ok |
| Runtime, PDF | `pdfruntime.ts` (bun with a path-mapping tsconfig override): the real module graph incl. the leaf and stream-roles cycle | after the JSON import: 86 definitions resolve, 83 parsers present, cos and function round trip, native numbers refused |
| Runtime, architect | `archruntime-all.ts` on the original leaf (copy with absolute imports) and the split leaf, 338 fixture snapshots, to SQLite and back | see "Architect differential" below |
| Parser freshness | `check-scope.ts` | stdio and architect `fresh`, drift=0 |
| Names | node `\p{RGI_Emoji}` / `Extended_Pictographic+FE0F` on all 25 new folder names | all admitted; no sibling shares a first grapheme |
| Path length | longest new path 148 characters (123 to 130 for the largest) | under 256 |

Architect differential: all 338 snapshot fixtures under the plugin's `🧪️tests/🧫️fixtures` convert to SQLite and back with equal results for both leaves, and the sha256 over the canonical database JSON of all 338 is identical: `97b46eadcd7c48013ab7ee366bf282402a93bce65596d4617b5f5940c0ec796e` (original leaf and split leaf). Scripts: `F/c6ts-tools/archruntime-all.ts`.

## Text readers

None for these three leaves: no `include_str!`, `readFileSync` or sha pin names them. Two readers reach them by path and were checked, not changed:
- `🧬️schema/🦀️.rs` of the PDF 1.7 base subset registers the leaf path `📸️snapshot/🟦️.ts` as a served `SchemaLeafDocument` (`export: "snapshot"`). The path is unchanged. The served body now imports its sibling files instead of embedding types and definitions; it already imported `../🪪️stream-roles/🟦️.ts`.
- `📦️packages/🦀️rust/📜️script.ts` of the PDF lists test files of the 4️⃣1.4 leaf only.

## Open risks

1. The PDF `🔺️diff` leaf and the 37 `🧬️mutations` leaves that import snapshot types carry the header "generated … by 🐍️generate-schema-facets.py (ticket 26/09/18/PDF-ARTIFACT-SPEC-COMPLETE)". That script lives in the ticket folder and is not wired into any repo command, but if someone re-runs it, it re-emits `from '../../📸️snapshot/🟦️.ts'` for types that no longer live there (lines 570 and 634 of the script). I edited the emitted files, not the script.
2. The registry serves the leaf text. External consumers of the served TypeScript body see imports instead of the types. This was already the case for the stream-roles import.
3. Resolved: the snapshot `schema` copy is replaced by the JSON import (coordinator decision). The facade bundler must keep resolving JSON modules for the leaf; it already does for `📜️artifact-definition.json`. `pdfsplit.py` still describes the interim 17-file layout for the snapshot and must not be re-run.
4. No vitest or bun test ran: `node_modules` is a dangling symlink (no `ajv`, no `bun:test` types). The existing test files were only type-checked and re-pointed.
5. The Rust side `📐️model/🦀️.rs` (1,951 lines, P57) and `📸️snapshot/🦀️.rs` are not part of this slice; the new TS folders can share names with whatever the Rust split chooses (`<domain>/🦀️.rs` next to `<domain>/🟦️.ts`).

## Coordinator compile list

Static checks above cover the TypeScript. Nothing here touches Rust, so no `cargo check` is required for this slice. If the coordinator runs the TS tests, these are the affected ones:
- `bun test` in `✏️s/🔌️plugins/🏛️architect/…/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts`
- `bun test` for the PDF `snapshotSqliteSources` list in `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/📦️packages/🦀️rust/📜️script.ts`
- `bun test` in `…/7️⃣1.7/…/🪪️native-json/🧪️tests/🔣️transport/🟦️.ts` (imports the native `schema`)
