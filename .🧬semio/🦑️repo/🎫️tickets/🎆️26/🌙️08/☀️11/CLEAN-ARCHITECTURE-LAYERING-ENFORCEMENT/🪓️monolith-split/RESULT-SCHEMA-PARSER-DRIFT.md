# Result: SCHEMA-PARSER-DRIFT (prefix `spd`)

Executor: Sonnet, started 2026-10-17 22:15. Foreground, no sub-agents, no modifying git command, no ticket tool. Logs `🗑️generated/spd-*`, tools `spd-tools/`. Continues `RESULT-TS-TYPE-ERRORS-5.md` section 4B. Concurrent: `tt6` (finished its parser cases: html, xml, ifc 2x3, pdf, equation, semio graph and model text, wav, gif; its unowned registry rows and the nullable renderer change are in the tree), `tsp`, `mwu`.

## 0. Verdict

IN PROGRESS (interim, written 23:55). Refused positions of `schema parsers report`: 1,295 (brief) -> 888 (after tt6's nullable change, my start) -> 7 in `✏️s`, 0 in `🌎️hub` (was 6). `schema parsers check`: `fresh: pending=0 dangling=0 helperless=0 drift=0 orphans=0 unowned=186`. Tests: 252 pass (Ajv oracle 212, rendered forms 39, nullable fields 1).

## 1. What the 1,295 refusals really were

Not stub schemas against Rust models. A refused position is one export in one format in one leaf for which a renderer returned nothing; the report prints only the first 60 of them, so the full list comes from `spd-tools/spd-dump.ts` (calls `projectParsers`, report mode) and the causes from `spd-cause.py`, `spd-ts-why.ts`, `spd-parser-why.ts` (descend to the smallest unrenderable node). Causes of the 888 (before): GraphQL/protobuf had no form for a property that is an enum, a constant, a nested object, an array of objects, a map, a union, a tuple or any value (about 460 rows); TypeScript had none for unions, nested objects, tuples, `{}`, `allOf`, constants, cross-document references (131); the parser renderer had none for untagged unions, `type` lists, `{}`, `null`, numeric enums, non-scalar constants, multi-part `allOf`, references into whole documents, into pointers, into relative files and into the framework (294, of which 227 only said "needs parseX" because a callee was refused: 57 times `parseBinary64Transport`). In addition `exportsOf` saw only `$defs`, never the draft-07 `definitions` of the stdio schemas, so their exports and every reference to them were invisible.

## 2. Generator changes (all in `🪞️parser-projection`, monotonic: no parser that was generated before changes by one byte)

- `📐️type-render`: total GraphQL and protobuf renderers (nested object = named declaration `<Parent><Field>`; enums and constants = scalars; typed map = `map<string, V>`; array = `repeated`; what the language cannot express = `scalar JSON` / `bytes` carrying the JSON text; non-identifier names get `_`; a taken name is never claimed). TypeScript: unions, `allOf` intersections, tuples, nested object types, constants, `{}` = `unknown`, `type` lists, numeric enums, and a `TypeScope` that follows references (pointer into this or another document rendered in place, export of another document through `import("<specifier>").Name`, so no import statement is needed).
- `🧾️parser-render`: first-match alternatives (`anyOf`, untagged `oneOf`, `type` lists; each alternative guarded by its JSON class), `null`, any value (a required one still refuses an absent key), scalar enums of any kind, constants that are arrays or objects, `allOf` of objects (merge), whole-document and relative-file references, pointer references rendered in place in a view of the referenced document (`Context.viewOf`, shared cycle guard), framework references (`Owner.shared`: called as imports through the package specifier, in-place when the framework exports no parser, refused when recursive).
- `🗂️documents`: `definitions` read like `$defs`; facet folders that state an `$id` next to a TypeScript leaf (`📍️feature`, `📝️definition`, ... 426 documents seen, the promoted ones are projected); framework documents and package manifests are indexed (`frameworkSpecifiers`: entry file -> specifier, following the re-exports of the package facades, so `@semio-tech/framework-os/modules/store/child/schema` is derived, not configured).
- `🟦️.ts`: renderers get the document scope and a `Taken` predicate; imports of framework callees use the package specifier.
- tt6's `🫥️nullable-fields` corpus rows for a free-form object and for nullable array items now expect `bytes` instead of "no form".

## 3. Proofs

| Check | Result |
| --- | --- |
| `bun test` Ajv oracle (`🎯️ajv-oracle`) | 212 pass (was 156): new case `alternatives` (13 exports: anyOf, untagged oneOf, type list, null, any, any field, numeric and mixed enum, constant list and object, merged allOf, in-place pointer, pointer and whole-document references into another document) agrees with Ajv instance by instance, type-checks under `strict`, audit and second projection rewrite nothing |
| `bun test` rendered forms (`🧩️rendered-forms`, new) | 39 pass: 13 cases x (GraphQL parsed by graphql-js, protobuf by protobufjs, TypeScript compiled by the TypeScript API under `strict`) with exact expected text |
| Render validation over every export of `✏️s` and `🌎️hub` (`spd-render-validate.ts`) | 5,531 exports, GraphQL and protobuf text parsed by graphql-js and protobufjs: 0 failures, 0 refusals (one family refuses where two fields make one identifier) |
| `schema parsers check` | fresh, drift 0 (before and after every generator change) |
| Sandbox apply of the TypeScript projection to a copy of all schema leaves (`spd-make-sandbox.py`, `spd-sandbox-apply.ts`) | 754 leaves, 2,604 parsers and 665 types written (old generator: 2,032 and 423) |
| tsc of those leaves, old generator vs new | 774 vs 1,030 diagnostics over a baseline of 258; the extra 256 are the hand-declared-type disagreements of the 570 additional parsers (section 4), not defects of the new forms; no diagnostic in an oracle-covered form |
| Ajv differential of 1,207 of the new parsers (<= 6 per leaf), 67,107 single-point mutations (`spd-verify.ts`) | 66,729 agree; 285 parser stricter than schema, 87 lenient, 6 lenient on unknown keys in 48 parsers of 16 artifacts (section 4) |

## 4. Open: schema stubs behind the new parsers

(filled in below as the schema fixes land)
