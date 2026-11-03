# r2-dn: depiction options model text as optional

## Design
`DepictionOptions = DepictionCommon & (TextualDepiction | TextlessDepiction)` in `S/🔨️modules/🔗️contracts/🟦️.ts` (discriminated by `language`).
- `TextualDepiction`: `language: Language` required, all annotations and diagrams free.
- `TextlessDepiction`: `language?: undefined`, annotations `dimensions/scaleBar/legend/labels: false` (only `ground` free), `diagrams.freeBody: false`. The type forbids every text path.
- `S/🔨️modules/🎨️depiction/🟦️.ts`: `defaultDepictionOptions(view, language, ...)` now returns the textual branch; new `textlessDepictionOptions(view, ...)` returns the textless branch (shared `commonOptions`). `Ctx.language` is a getter calling `requireLanguage`, which throws "a textless depiction has no language" (no fallback). `depictAll` uses a distributive `WithoutView`. `textlessDepictionOptions` re-exported from the package root.
- `transport-view`: the language was read eagerly and tag texts were resolved before the labels gate; now resolved lazily inside the gate.
- Cast exporter `S/🔨️modules/📤️export/🎭️cast/🟦️.ts`: `TEXTLESS_LANGUAGE` removed, uses `textlessDepictionOptions`. The `[DEBUG] pf` hook untouched.
- Callers that render text (CLI, React pipeline/figure/element, tests) already pass their language and compile unchanged.

## Tests
- New `S/🧪️tests/🤐️textless-depiction` (vitest + feature): textless options carry no language; 7 views with no text item; `depictAll`; forced text throws; two `@ts-expect-error` type-level refusals; textual branch requires language and formats en `1.73 m` vs de `1,73 m`.
- Library vitest (round.sh c, alias config): 55 files, 1119 passed, 11 skipped. Includes cast-export (69 passed with the new file).
- React vitest: 17 files pass, 1 failure, `catalog-loading` "ready 132, failed 15 vs 147", an asset catalog problem from concurrent asset edits, unrelated to depiction.
- tsc staffage package: no errors from this change. Remaining: `d3-format` types, `fixtures/thermal:58` (`front` missing), and tests `front-projection:134,151` and `plan-projection:196` still pass `language` to `CastOptions` (leftover of r2-lg, they are not type-checked by vitest; someone should drop `language` there). React tsc errors only in framework infra (`@semio-tech/framework-transient`).
