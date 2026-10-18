# Result CENSUS5-H (prefix `c5h`): architect `🔺️diff` parsers leave the leaf

## 0. Verdict

Done and verified statically plus by running the pure bun/ts generator and audit (no cargo, no build).

| File | Before | After |
| --- | --- | --- |
| `…/✳️any/🧬️schema/🔺️diff/🟦️.ts` (leaf) | 872 | 70 |
| `🔺️diff/🩹️patch-types/🚪️parsers/🟦️.ts` (new) | - | 283 (69 parsers) |
| `🔺️diff/🗃️deltas-records/🚪️parsers/🟦️.ts` (new) | - | 248 (30) |
| `🔺️diff/📋️deltas-requirements/🚪️parsers/🟦️.ts` (new) | - | 208 (25) |
| `🔺️diff/🧱️deltas-program-structure/🚪️parsers/🟦️.ts` (new) | - | 96 (11) |
| type children (`📋️`, `🗃️` 859, `🧱️`, `🩹️`, `🧾️`) | unchanged | byte-identical |

Largest file of the package is the untouched type child `🗃️deltas-records` (859). The leaf keeps the hand-written `parseProgramDiff` (above the region, not audited) and the guard helper block; 135 generated parsers moved, 136 `parse…` exports are reachable through the leaf.

## 1. Generator change (`📚️library/🎛️commands/🧬️schema/🪞️parser-projection`)

- New component `📍️parser-files`: the parser home rule. A parser lives in the file that already defines it; a new one goes to `<dir of the file declaring its type>/🚪️parsers/🟦️.ts`, or to the leaf when the leaf declares the type (so every existing single-file leaf behaves exactly as before). Also the symbol table of a leaf's module tree, import supply, region append, leaf re-export statements and unused-import pruning.
- New component `✍️leaf-writer` (`LeafWriter`): applies one leaf's settled parsers to all files of its tree (add, restate in place in the defining file, `distribute`). `restateParser`, `withImports`, `importLines` moved here from the main file.
- Leaf stays the public face: it re-exports every parser its folders hold (`export { parse… } from "./<component>/🚪️parsers/🟦️.ts"`), so importers (`parseProgramDiff` users, sibling documents) are untouched.
- `SourceStore.tree()` (module tree of a leaf); `declarations()` now built from it. `settle`, `locateParser` and the plan loop test "declared/defined/imported" on the whole tree, not the leaf text.
- Audit (`auditLeaves`) walks every file of each leaf's module tree below its `//#region 🚪️Parsers` marker; drift rows carry `path` (the file holding the parser) and `leaf`; new strict reason `duplicate` (a parser projected in two files); `helperless` also fires when a parser folder exists but the leaf has no helper block, and counts `…Closed(` calls in the whole tree. No exemptions added; the unowned registry still keys by the file holding the hand-written parser (the 72 architect snapshot rows are unchanged and still drift as listed).
- New option/flag `--distribute <leaf folder>[,…]` (`ProjectionOptions.distribute`): moves the fresh (not drifted, not restated) generated parsers of the leaves below that folder verbatim into their component folders; `schema parsers apply --distribute …` is accepted without `--closing` (guard in `🧬️schema/🟦️.ts`). No new executable command: `schema-parsers-apply` forwards args, so `launch.json` needs no entry.

## 2. Regeneration

`apply --closing --formats 🟦️typescript --distribute <…/🧬️schema/🔺️diff>` (driven by `F/c5h-tools/run-diff.ts` with a narrowed catalog for speed). Touched: the leaf and the four parser folders only; the snapshot leaf is byte-identical.

## 3. Oracles and results

- Lossless: `F/c5h-tools/lossless.py` — the multiset of the 135 generated function texts before (copy in `/Users/ueli/.cache/c5h/orig`) and after is equal; all five type children byte-identical; the leaf gained only 4 re-export lines and lost only the moved functions and the now-unused `import type` names.
- Audit/gate on the architect scope (`F/c5h-tools/check-narrow.ts`): `pending=[] dangling=[] helperless=[] drift=[] orphans=0 unowned=72`, identical to the baseline `unowned=72`.
- TypeScript strict + `noUnusedLocals` over leaf + 4 parser folders + the document-contract test (`F/c5h-tools/tsc.ts`): 0 diagnostics in `🔺️diff` (16 unrelated unresolved `@semio-tech/*` modules in this environment).
- Runtime (`F/c5h-tools/runtime.ts`): importing the leaf exposes 136 `parse…`; `parseProgramStakeholdersPatchEntry` accepts a row and refuses `extra` (`$.extra: unknown property`); `parseProgramRisksPatchEntry({id:1})` refused.
- Tests (bun, ajv/typescript from the scratch node_modules): existing `🎯️ajv-oracle` 220 pass (single-file leaves unchanged); new `🧩️split-leaves` 3 pass: fresh split leaf (parsers in component folders, re-exports, imports, audit, check, idempotent re-run, strict tsc, parsers equal Ajv instance by instance), drift in a parser folder found and restated in place plus `duplicate`, `distribute` of a leaf that held types and parsers. `🫥️nullable-fields` cannot load here (package `graphql` missing), not related.
- `schema-catalog.json`: regenerated with the generator's own `inventorySchemaScopes`/`renderSchemaCatalog` (`F/c5h-tools/catalog.ts`, output compared, not blindly written); the only architect change is the hash of `🔺️diff/🟦️.ts` `42e796bd…` -> `84d9163c…`, applied as that single line (86 other diff lines are peers' in-flight changes, left alone). Parser folders are not facet leaves and are not hashed.

## 4. Text readers

None read the parsers as text: importers use `parseProgramDiff` (stays in the leaf). `📦️packages/🦀️rust/📜️script.ts` `program-document-contract` runs tsc on the leaves and follows imports (clean).

## 5. Open risks

1. Child parser files import the guard helpers from the leaf while the leaf re-exports the children: a runtime import cycle between leaf and parser folder (all uses are inside function bodies, loads fine under bun). A helper child would remove it but would change every existing leaf.
2. A leaf's tree contains the leaves of other schema documents it imports (for example a root leaf re-exporting its snapshot facet). The generator therefore uses `SourceStore.owned`/`parsers` (tree minus other documents' leaves) for everything about parsers, and the full tree (`declarations`) only for type declarations; the first full-gate run without that split reported 3 `block` leaves pending and 9 `stdio` leaves helperless, the fix cleared both.
3. `🗃️deltas-records` (859) and `📋️deltas-requirements` (703) are type children, left as they are.
4. Peers hold uncommitted edits in the same generator files (`📐️type-render`, `🧾️parser-render`, `🗂️documents`, main); my edits are anchored and additive.

## 6. Full gate

`F/c5h-tools/check-all.ts` (= `checkParsers` over `✏️s` and `🌎️hub`, real catalog, run through bun with the scratch node_modules): `[schema parsers check] fresh: pending=0 dangling=0 helperless=0 drift=0 orphans=0 unowned=186`. Run last, after the final code state; `ajv-oracle` + `split-leaves` re-run green afterwards (223 pass).

## 7. Coordinator compile list

No Rust change, nothing to `cargo check`. Optional: `bun repo run @semio-tech/repo-lib:test-parser-projection` (runs the ajv oracle) and add `🧩️split-leaves` to that target when the dependencies are installed.
