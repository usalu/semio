# Result gen: generators emit no import cycles and import from owning modules

Executor `gen`, foreground, no cargo, no sub-agents, no modifying git command, no `package.json` / `bun.lock` / storage / revision edit.

## Verdict
Both generators fixed at the source; generated code regenerated through the generators' own functions; freshness audits, bun suites, `tsc` and depcruise `no-circular` are clean for these paths.

## 1. Collection-catalog generator (`📚️library/🏗️authoring/🗂️collection-catalog/🟦️.ts`)
- Cause: it imported all row types from the single `typescript.rows` module (the architect schema root), which re-exports the catalog (`RegisterKey`, `RegisterRow`) = cycle; tscp2 papered over it with a 65-type root re-export.
- Fix: `typescript.rows` is now the entry of the TypeScript module tree. New `resolveCollectionRowOwnersV1(catalog, outputPath, read)` follows the relative imports/re-exports of that entry (the catalog left out), requires every row type to be declared by exactly one module, and refuses (throws) when the declaring module reaches the catalog again (cycle). `renderCollectionCatalogTypeScriptV1(catalog, owners)` emits one `import type` per owning module. No schema change (snapshot JSON, digests, schema-catalog untouched).
- Regenerated `architect/…/🧬️schema/🗄️registers/🗂️catalog/🟦️.ts`: imports from `../../🏗️program-structure`, `../../📋️requirements`, `../../🗃️decision-records` (byte-identical to the HEAD version).
- Removed the 3 `export type {…}` row re-exports (65 types) from `architect/…/🧬️schema/🟦️.ts`; the root only imports what it uses.

## 2. Parser-projection generator (`🎛️commands/🧬️schema/🪞️parser-projection`)
- No leaf re-exports any more: `withReexport` removed; `LeafWriter` no longer touches the leaf for a folder parser (only when the leaf lacks the helper block / `Closed` helper).
- Importers import from the file that holds the parser: new `ParserHomes` (`📍️parser-files`) answers "which file holds `parseX` of leaf L"; `importLines` (sibling documents) names the component `🚪️parsers` folder, not the leaf. A shared leaf (named by a package specifier) keeps all its parsers.
- Cycle avoidance by construction: folders import the leaf's guard helpers, so the leaf must never import a folder. New `pinnedParsers`: a parser stays in the leaf when its type is declared there, when the leaf's own code calls it, when a staying parser calls it, or when it sits in a call cycle between folders. `outward` leaf imports removed.
- Discovery: nothing imports a parser file any more, so `SourceStore.tree` adds the `🚪️parsers/🟦️.ts` next to every module-tree file (otherwise the audit/generator would not see the folders).
- `projectParsers` computes distribution candidates and pins first, then writes.
- `test parser-projection` (library typescript `📜️script.ts`) now runs all four suites (ajv-oracle, split-leaves, rendered-forms, nullable-fields).
- Architect `🔺️diff` leaf has no `export {` lines; `apply --closing --distribute <🔺️diff>` is a no-op (touched 0) and does not re-add them.

## 3. Tests (language-agnostic, with third-party oracles)
- `🧪️tests/🗂️collection-catalog`: +2 tests (7 total). TypeScript compiler API as oracle: every imported name is declared at top level in its module, no imported module reaches the catalog; synthetic tree with ambiguous/missing/cyclic row types is refused.
- `🧪️tests/🧩️split-leaves`: 3 updated + 3 new (6 total): no re-export, importCycles via TypeScript compiler (runtime imports) on every fixture, parsers agree with Ajv per owning file, leaf-called parser stays with its callees, mutual recursion across components stays in the leaf, sibling document imports `parsePoint` from the folder.

## 4. Verification
- `bun test` of collection-catalog, ajv-oracle, split-leaves, rendered-forms, nullable-fields: 273 pass / 0 fail. Architect `document-contract`, `transport`, `native-roles`, library `rust-mount-emitters`: pass.
- `tsc -p gen-tools/tsconfig.json` (strict, noUnusedLocals) over generators, tests, architect root, catalog, diff leaf: exit 0.
- Freshness: `catalog-check.ts check` (= `generate-register-catalog --check`) `[]`; `check-narrow.ts` (architect scope): pending/dangling/helperless/drift empty, unowned 72 (baseline); schema-catalog regenerated with the generator's own renderer: identical to tracked.
- depcruise (repo config) on the whole architect schema folder (80 modules incl. catalog and 4 parser folders): 0 `no-circular`. On `🎛️commands/🧬️schema` + both collection-catalog folders (396 modules): 10 `no-circular`, all in `process/…/native-build/💾️storage` and `🔒️leases` (frozen, not mine); none in these paths.
- Full gate `check-all.ts` (✏️s + 🌎️hub): `drift=1`: `stale SemioMeshDiff` in `stdio/…/🔺️mesh/…/🔺️diff/🟦️.ts`, a peer's hand edit of a generated parser (typed-map casts, 19:06), unrelated; left alone.
- `bun repo run @semio-tech/architect-program-rs:check-register-catalog` was queued behind the busy build lease and did not finish; the direct function call above is the same check.

## 5. Open
- Existing split leaves whose leaf-resident parser already calls a folder parser keep that leaf-to-folder import (cannot be moved); none exists today.
- Peer drift `SemioMeshDiff` (stdio mesh) needs `--restate stale` or an accepted hand edit.

Files: generator sources above, `✏️s/…/✳️any/🧬️schema/🟦️.ts`, `…/🗄️registers/🗂️catalog/🟦️.ts` (regenerated, unchanged vs HEAD), library typescript `📜️script.ts`. Tools in `C/gen-tools/`.
