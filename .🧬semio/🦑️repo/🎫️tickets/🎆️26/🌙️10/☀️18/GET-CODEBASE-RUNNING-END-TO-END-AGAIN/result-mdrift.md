# Result mdrift: stdio mesh diff parser drift (`stale SemioMeshDiff`)

Executor `mdrift`, foreground, no cargo, no sub-agents, no modifying git command, no `package.json` / `bun.lock` / storage / revision edit.

## Verdict
The hand cast is gone and the root cause is fixed in the parser-projection generator: a parser whose schema states no structure while its declared type instantiates a generic of the codebase is now refused, never emitted with a type error or a cast. The mesh diff parser is exactly that case, so it no longer exists. Full parser gate `fresh`, drift=0; strict `tsc` over all of stdio shows no mesh error.

## Root cause
- `🔺️mesh/🧬️schema/🔺️diff/🔣️.json` states `meshes`, `materials`, `textures` as bare `{"type": "object"}`, while the TypeScript leaf declares `NamedTripleDiff<string, SemioMeshItemDiff, SemioMesh>` etc. (Rust, GraphQL, proto agree with the TypeScript).
- The generator fell back to its opaque object read (`Readonly<Record<string, unknown>>`) for those fields, which is not assignable to the declared generic: 3 x TS2322. The previous executor (`tscp`) hand-cast the three members (`as unknown as ...`), which the freshness gate then (correctly) flagged `stale`.
- The schema cannot be made precise enough for the generator to own the parser today: the payloads reach `SemioMesh`/`SemioPrimitive`/`SemioMaterial` and `SemioPoint3`/`SemioRgba`/`Binary32`, whose TypeScript types are the in-memory model (`Binary32 = { bits }`) while the schemas state JSON numbers. That is the registry reason `in-memory-model` (a schema binding to the framework value parsers does not exist yet). `parseSemioMeshDiff` had no caller anywhere (grep over stdio/framework/hub), so a parser that cannot be sound is removed rather than hand-owned.

## Changes
Generator (`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🧬️schema/🪞️parser-projection/`):
- `📖️declared-types/🟦️.ts`: new `instantiatesGeneric(text, aliasOf)`: true for a generic of the codebase (`Box<Point>`, arrays/unions/tuples of it, aliases that resolve to it, library generics such as `Record<string, Box<Point>>` through their arguments); `Record`, `Array`, `Readonly`, ... of plain types are not.
- `🧾️parser-render/🟦️.ts`: in `parserExpression`, after the typed-declaration lookup fails for an opaque/any node, `instantiatesGeneric(declared)` returns `undefined` (parser refused). Docstring updated.
- `🟦️.ts` (projection): module docstring states the rule.
- `📦️packages/🟦️typescript/📜️script.ts` (library): `test parser-projection` now also runs the new suite.
- Tests (language-agnostic corpus + Ajv + TypeScript compiler oracle): `🧪️tests/🧷️generic-types/🟦️.ts` and `🔣️.json`: 3 refusal cases (generic instance, alias of one, array of one), 2 control cases that still project (`Record<string, unknown>`, structured schema for `Box<string>`) type-check under strict TS, contain no cast and agree with Ajv; plus a unit test of `instantiatesGeneric`. Negative control: with the guard removed the 3 refusal tests fail.
Generated leaf:
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🧬️schema/🔺️diff/🟦️.ts`: the `//#region 🚪️Parsers` block (guard helpers) and `parseSemioMeshDiff` (with the three casts) removed; the leaf is the pure type declaration like its brep/flow/presentation siblings. Regeneration (`projectParsers` apply, closing, restate all, scope = the mesh subset) writes 0 files and reports `REFUSED parser SemioMeshDiff` and `SemioMeshItemDiff` (the same generic case).
Tool: `C/gen-tools/mdrift-apply.ts` (scoped report+apply run).

## Verification
- `bun C/gen-tools/check-all.ts` (= `checkParsers` over `✏️s` + `🌎️hub`): before the change `drift=1 stale SemioMeshDiff`; with only the guard `drift=1 unrenderable SemioMeshDiff` (no other parser in either scope is affected, so zero blast radius); final `[schema parsers check] fresh: pending=0 dangling=0 helperless=0 drift=0 orphans=0 unowned=186`.
- `bun test` of ajv-oracle, split-leaves, rendered-forms, nullable-fields, generic-types: 272 pass / 0 fail.
- `tsc` (strict, noUnusedLocals, 340 files, Check time 0.9 s) over generator, declared-types, parser-render, new test, split-leaves test and the mesh diff leaf: clean.
- `bunx tsc -p <scratch config extending ✏️s/tsconfig.json, include ✏️s/🔌️plugins/🗄️stdio/**>`: 5,375 files, Check time 22 s, 8 errors, all missing packages (`brepjs` x3, `pngjs` declarations x5): the dependency-freeze items already listed by `tscp`; no error in any mesh file.
- `bun test` of all 182 bun test files under `✏️s/🔌️plugins/🗄️stdio`: 1616 pass / 2 skip / 4 fail. The 4 are unrelated and pre-existing (listed in `result-tst2-suites.md`): 2 import errors (`d3-dsv` in csv/tsv snapshot tests, missing package), `PDF neutral multiobject replay law` (`tree-sitter-wasms` missing), `Semio Base Native matched eighteen actual defaults` (5 s timeout, native lane). The mesh sqlite snapshot suite passes.

## Open
- The mesh diff JSON schema still states the three collections as bare objects. Making the whole diff projectable needs the schema binding to the framework value parsers (`in-memory-model`), after which `SemioMeshesDiff` & co. can be stated as `$ref` triple definitions (pattern of `🌊️flow` diff) and `parseSemioMeshDiff` regenerated; until then the generator refuses it by construction.
- Other leaves with a hand-written parser over a generic (listed in `🧱️unowned`) are untouched.
