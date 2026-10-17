# Result: TS-TYPE-ERRORS-2 (prefix `tt2`)

Executor: Sonnet, 2026-10-16 17:37 to 19:10. No sub-agents, no modifying git command, no ticket tool, no `bun install` or download. Logs are in `🗑️generated/tt2-*`, tools in `tt2-tools/`, pre-edit snapshots in `🗑️generated/tt2-snap/` (every touched source file; the draw ones are mapped in `🗑️generated/tt2-edited.json`).

## Verdict

DONE for the five targets, with the leftovers listed under "Not done". The real `psc` project (`tt2-tsconfig-psc-live.json`, the same 313 roots as `tte-tsconfig-psc-live.json`, same command, 12 GB heap) went from **2454 to 2098 diagnostics** (`tte-psc-after.txt` to `tt2-psc-final.txt`; 423 gone, 67 new, the new ones being peers' concurrent edits in library, process and caching plus two draw diagnostics whose text changed but whose cause was already there). No syntax error, no TS6053 in either run, so both are full semantic runs.

| Family | Before | After | Where |
| --- | --- | --- | --- |
| draw (227 plus 4 `layout-drawing`) | 231 | **28** | see "Draw" for the 28 |
| draw Binary64 diagnostics | 24 | **0** | |
| draw JSON-literal casts (TS2352) | 46 | **0** | |
| puzzle diff + snapshot + dev parsers (tsc) | 0 | 0 | the 5d half-migration was invisible to `tsc`; proven by the test route |
| xml `✅️valid` | 6 (3 of them TS7006) | **0** | |
| generated stdio facets I touched (9 files) | 33 | **0** | measured alone in `tt2-gen9-b.txt` (4758 lines of other diagnostics, 0 in these files) |
| `psc` total | 2454 | 2098 | by area, before to after: draw 231 to 28, stdio 231 to 172, puzzle 18 to 17, library 601 to 573, framework 846 to 802, mit-bestand 161 to 161, hub 55 to 54, other 311 to 291 |

Test routes (per-file `bun test ./<file>`, 69 draw test files, `tt2-tools/tt2-run-draw-tests.sh`): **before 581 pass / 46 fail, after 582 pass / 45 fail**; the set of failing test names is 45 before and 44 after, nothing newly failing (`tt2-draw-fails-before.txt`, `tt2-draw-fails-after2.txt`). The single difference is `created round ellipse ... independent SVG coverage`, which took 5476 ms in the baseline run at load 50 and passed at load 25. I do not claim it as a fix. `dev/🧩️puzzle/🧪️tests/🔺️diff-parsers`: 24 pass before, 26 pass after (two new tests). xml mirror: 3 tests before, 6 after (run through `tt2-tools/tt2-xml-valid-harness.test.ts`, because the tests are in-source vitest).

## Ruling applied: Binary64

Binary64 words stay the representation in schemas, snapshots, diffs and mutations. Plain `number` appears only inside geometry evaluation, and the single conversion helper is the framework's `binary64Value` (`@semio-tech/framework/value/ieee754`); the wrappers I added only call it. No `number` copy of a model was added.

- `🧬️schema/🧮️geometry/📷️framing/🟦️.ts`: `drawingSceneBounds` read `artboard.width > 0` on a `Binary64`; it now converts once with `binary64Value` and works on the two numbers.
- `🧬️schema/🧮️geometry/↗️affine/🟦️.ts`: new pair `drawingTransformToWords` / `drawingTransformFromWords`. The numeric `DrawingTransform` of the mutation module and the persisted `DrawingTransform` of the schema share a name; the pair is the one place a computed transform enters or leaves the model.
- SVG import (`🚪️io/📥️import/…/🎨️svg/🔖️1.1/✳️any/📄️document/🟦️.ts`): the importer computes in `number` (CSS lengths, viewBox maths, gradient geometry) and captures Binary64 exactly once at emission: artboard, opacity, text `x/y/size`, colours, stroke width and dash, gradient endpoints and stops, path segments (through the existing `drawingPathFromGeometry`), transform (through the new pair). Before, it emitted plain numbers into a Binary64 model, which `parseDrawingArtifact` refuses, so a document it produced could not be parsed back. Groups now state `isolation: false` instead of omitting it (the type requires it). **Not run**: every importer test imports `@xmldom/xmldom`, which is not installed (see "Not done"); this change is verified by `tsc` only.
- `🧬️schema/🧮️geometry/🎯️picking/🎨️paint/🟦️.ts`: `PaintedPathHitJob.advance` and `own` accept the framework's readonly `PathSegment` (what a prepared plan hands over) instead of the mutable draw type; the job copies it anyway.
- `🎨️style/…/🌓️set-layer-blend-mode/↩️inverse/🟦️.ts`: `baseBlendMode` is typed `BlendMode`.
- `🏭️generator/📜️script.ts`: the catalog type gets `platform?: string` next to `oracle?`.

### Typed fixtures instead of casts (draw)

New shared test module `✳️any/🧪️tests/🧫️fixtures/🟦️.ts`, all built on the owning parsers or the owning JSON Schema:

- `drawingArtifactFixture`, `drawingLayersFixture`, `drawingAssetsFixture`, `pathSegmentsFixture`: plain JSON numbers (the schema's `Binary64Transport` number branch) become words once (`drawingWords`), `null` for an absent optional becomes an omitted key (`absentNulls`), then `parseDrawingArtifact` / `parseDrawingLayerNode` / `parseDrawingImageAsset` / `parsePathGeometrySegment` admit the result. This replaces the hand-copied untyped `lift` in 6 of the 14 draw tests that carry one (export-document, patch-layer, scene query, PDF write, SVG write, shape coordinates); the other 8 keep their local copy because they cast hostile or partial documents and had no diagnostic.
- `admitted(validate, value)`: a compiled Ajv `ValidateFunction<T>` is a type guard, so the schema that already validates the fixture also types it. Used for `PathRasterInput` (the 15 casts), `RasterSceneInput`, `PreparedScenePick`, `PreparedSceneNode`, `PathEdit`.
- `framingNodesFixture`, `paintedPathQueryFixture`, `svgNodesFixture`, `fillStyleFixture`, `strokeStyleFixture`, `matrixFixture`, `rasterSceneGroupFixture`, `rasterSceneAssetFixture`: field-by-field admission where no schema exists.
- `drawingNumbers`: the inverse of `drawingWords`, for the two tests that walk an imported model.
- `hostile<T>`: the one deliberately unchecked escape for refusal tests whose input the static type forbids (used once, in the raster-scene refusal test). It is a cast in a named place with a docstring, not a typed admission; I found no honest typed path for data that is invalid on purpose.
- 63 TS2769 were `expect(typed).toEqual(jsonExpectation)`: Bun types `expected` as the actual's type. They became `expect<unknown>(...)` where the expectation is the neutral JSON. This silences the compiler on those lines, so drift there now shows only at run time; the run above is the proof it does not hide a new failure.
- Real API drift found and fixed in tests: `admitFontSources(sources)` now needs `(sources, unicodeBytes)` (3 tests, `unicodeSourceUrl`); `new Database(bytes)` is not the typed API, `Database.deserialize(bytes)` is (4 sites in the sqlite snapshot test, as the other plugins do); the raster scene SVG oracle silently treated an `image` node as a path (it now throws); `previous`/`origin` typed as mutable tuples while the framework hands out readonly ones; duplicate `import Ajv` and `import type` lines in the interaction test.

## Target 2: 5d diff finished to the 3d end state

`🖐️5d/…/🧬️schema/🔺️diff/🟦️.ts` (539 to 551 lines, 80 added, 68 removed): `Puzzle5dVector3`/`Puzzle5dVector4`/`Binary64` in the patch interfaces; `GuardBinary64`, `GuardVector3`, `GuardVector4`; every `GuardNumber` call replaced (16 scalar sites, 6 vector sites, 20 index/from/to sites); `GuardNumber` removed. The diff JSON already referenced `Binary64`/`Vector3`/`Vector4` for these fields (tte repaired its `#`), so no JSON change was needed. Found on the way: `index`/`from`/`to` are `integer, minimum 0` in the schema but the 3d guard (tte) and the 5d guard accepted `-1`; both now take `{ minimum: 0 }`.

Test (independent oracle = Ajv on the sibling `🔣️.json`): new fixture `🖐️5d/…/🧪️tests/🧫️fixtures/🔺️diff/🔢️native-fields/🔣️.json` (same two corpora as the 3d one: zeros/infinity and NaN payloads), and in `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔺️diff-parsers/🟦️.ts`: a 5d native-words test (words in, wire out through the bigint replacer, Ajv admits the wire, plain `[1,2,3]` and `gap: 0.25` refused) and a 3d+5d index test (`-1`, `1.5`, `"0"` refused by both Ajv and the parser). The existing 5d patch fixtures (`gap: 0.25`, `origin: [0,0,0]`) were plain numbers that only passed while the parser took numbers; they now use patches without scalars. Result 26 pass, 0 fail (`tt2-diffparsers-after1.log`).

The Rust readers of the 2d/3d/5d diff JSON (`🦀️.rs` with `f64`) were not touched or compiled (compiler ingress refused).

## Target 3: xml doctype

`📰️xml/…/✅️valid/🧬️schema/🟦️.ts` is rewritten against the structured `XmlDoctype` (`name`, `externalId`, `declarations`, `prologPosition`) and mirrors the owned Rust validator (`🏅️conformance/🦀️.rs`) rule for rule: document element, boundary kinds, doctype position, declaration version, encoding name, doctype missing, root mismatch, standalone with external id, advisory. `snapshotWith` builds a complete `XmlDocument` (`prolog`/`epilog`). The mirror test now passes structured doctypes, covers the five added rules, and has typed dependencies instead of `any` (the 3 TS7006 of the baseline are gone). xml diagnostics 6 to 0.

## Target 4: the generator

The generator is the ticket script `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️08/SCOPE-OWNED-SCHEMA-CONTRACTS/wp4c-project-exports.py` (still not a permanent `📜️script.ts`; moving it is a coordinator decision, the AGENTS rule asks for one). Root cause: it emitted a caller for every `$ref` whose target was merely an *export of the document*, then refused to render the target (inline objects, nullable unions, `oneOf`, tuples, cross-document refs), so the call dangled (`ReferenceError` at run time, TS2304/TS2552 at check time).

Fixes in the generator (581 lines added, 102 removed):

- A parser is emitted only when every parser it calls is declared on disk or emitted in the same pass (fixed point, `settle`). A call can no longer dangle.
- `--closing` projects only what existing code already calls but never declared (and what that needs). `--replace A,B` re-projects named parsers that exist but are wrong (typed maps, tuple aliases) and drops a duplicate type declaration. Both are additive; without them the tool behaves as before.
- New renderers: nullable (`type: [T, "null"]`, `anyOf [T, null]`), `oneOf` with a `const` discriminator (switch), `oneOf` of single-field variants (externally tagged), `allOf [$ref]`, tuples (`items` list), inline objects, typed maps (`additionalProperties`), arrays without `items`, cross-document `$ref` (resolved through `$id`, import added), array `items` that are a list (this crashed the tool).
- Type-driven where the JSON Schema is opaque (`{"type": "object"}`) but the hand-written interface is not: the field follows the declared type (named parser located in this file or a sibling facet, `T[]`, number tuples, inline object types, alias unions). Without this the new `dxf` parsers failed to type-check against the hand-written `DxfBlock`/`DxfEntity`.
- `helper_prefix` read the old `$id` grammar (`/schema/`), so every new parser would have called `schemaGuard*` helpers that do not exist; it now reads `https://…/s/<path>/<facet>.json`, and prefers the prefix the file already states (213 of 227 existing files agree with the computed one; the 14 others are hand-written files whose class name differs).
- Private `type X`/`function parseX` declarations count as declared.

Route and result (additive, run from the restored pre-state, `--formats 🟦️typescript`):

1. `… apply --closing --formats 🟦️typescript --under ✏️s/🔌️plugins/🗄️stdio` (all of stdio; it found 9 files with dangling calls)
2. `… apply --closing --replace Vec3 … --under …/🖋️dxf` and `… --replace MdBlockAdded,MdListItemAdded … --under …/📝️md` (old wrongly typed outputs)
3. `… apply --replace DrawingTopology … --under ✏️s/🔌️plugins/🖍️draw` (typed map, draw inferences)

| File (stdio unless noted) | added | removed | new parsers |
| --- | --- | --- | --- |
| `🌐️html` diff | 28 | 3 | `HtmlChildAdded`, `HtmlNodeDiff`, `NullableAttrValue`, `NullableString` (2 of the 3 removed lines are the hand-written `HtmlAttrModified`/`HtmlAttrAdded` fix below) |
| `🌦️epw` diff | 9 | 0 | `EpwRecordAdded` |
| `📊️csv` diff | 16 | 0 | `CsvRecordAdded`, `CsvRecordDiff` |
| `📑️tsv` diff | 7 | 0 | `TsvRowDiff` |
| `📖️pdf` 1.4 base diff | 9 | 0 | `PdfPageAdded` |
| `📝️md` diff | 26 | 9 | `MdBlockDiff`, `NullableString`, `NullableUint32`; `MdBlockAdded`, `MdListItemAdded` replaced |
| `🖋️dxf` snapshot | 16 | 6 | `DxfGroupCode`, `DxfEntity`, `DxfValue`; `Vec3` replaced (and the duplicate `export type Vec3` removed) |
| `🖋️dxf` diff | 106 | 0 | 12: the `Dxf*sDiff` family, `DxfBlockDiff`, `DxfEntityDiff`, `DxfHeaderVarDiff`, … |
| `🧾️json` base diff | 22 | 0 | `JsonValueDiff`, `JsonObjectDiff`, `JsonArrayDiff` |
| `🖍️draw` inferences | 1 | 1 | `DrawingTopology` (`depth` is now a typed map, not an object guard) |

Totals over the nine stdio files: 239 added, 18 removed, 30 new parsers; the removed lines are the replaced functions and the two hand fixes. Nothing else in any file changed. Hand fixes to non-generated code in the same files: the html interfaces now say `value?: string | null` (the schema is tri-state; the docstring said "absent = valueless"); `🎞️animation/🚪️io/📝️text/🔺️diff/🟦️.ts` imported nothing for `AnimTimeline`, `AnimChannel`, `AnimChannelDiff` (TS2304, 3 diagnostics).

Oracle (independent: Ajv compiled from the sibling `🔣️.json`, `tt2-tools/tt2-generated-parsers.test.ts`, log `tt2-generated-parsers-oracle.log`): for each of the 30 new parsers, the schema's synthesized minimal instance must parse, and `42`, `null`, `"x"`, `true` must be accepted exactly when Ajv accepts them. **28 agree, 2 disagree on purpose**: `parseHtmlChildAdded` and `parseDxfEntityDiff` are stricter than their schema, because the schema leaves `item`/`entity` as an opaque `{"type": "object"}` and the parser follows the hand-written type. That is a schema defect (schema-first would replace the opaque nodes by `$ref`s), not a parser one.

## Target 5: remaining draw drift

Every item the previous result listed for draw is fixed except the ones under "Not done". The 28 draw diagnostics left, measured in the scoped project (`tt2-tsconfig-all.json`, 227 roots) and in `psc`:

| Count | Cause | Owner |
| --- | --- | --- |
| 7 | missing packages: `@xmldom/xmldom` ×6 (TS2307), `graphlib` types (TS7016) | the user's install decision |
| 7 | cascade of the missing `@xmldom/xmldom` in the SVG export unit test (`DOMParser` is `any`: TS2339 ×4, TS18046 ×3) | same |
| 4 | `CanvasSceneNode.text.fontFamily` is required by the renderer's Canvas2dHost (changed 13:15 today) but `DrawingSvgNode.text` has none | renderer (rhu2) |
| 6 | third-party typings: `Canvas` lacks `getContent` required by `SvgCanvas` (TS2741 ×3, napi canvas vs svg2pdf/pdfjs), pdfjs `isEvalSupported` (TS2353 ×2), opentype `RenderOptions.hinting` (TS2353 ×1) | dependency versions |
| 4 | `layout-drawing/🪶️sqlite/🟦️.ts` (TS2542 ×2, TS2769 ×2) | a peer edited it at 17:54, so I left it |

## Not done (evidence)

1. **Missing workspace packages** (reported, not touched): the six draw test files that import `@xmldom/xmldom` do not load (`error: Cannot find module '@xmldom/xmldom'` in `tt2-draw-tests-before.log`), so the SVG importer, its document/path/transform tests and the SVG export/write tests cannot run here. Everything I changed in those files is verified by `tsc` and by reading, not by running. The project-wide missing set is still the one in `RESULT-TS-TYPE-ERRORS-1.md` (`@semio-tech/cad-js*`, `stdio-*`, `brepjs`, `graphlib`, `@/lib/*`, …).
2. **Draw runtime failures that are not type errors** (44 failing test entries, unchanged; a test body registered from several files counts once per file): (a) 16 in `🔨️modules/🏠️host/🧰️owned/…/📋️native-owner` assert strings of `🦀️.rs` (`expect(initializer.includes("…")).toBe(true)`, 63 `readFileSync(join(owner, "🦀️.rs"))` reads) and the file was carved into children (`🏟️arena`, `🔏️digest`, `📐️footprint`, `🏗️initialization`, `♻️retirement`): the source-text-reader rule of the wave brief (`readModuleTreeSource`); (b) 3 in the sqlite snapshot test fail in `drawSnapshotFromSqliteDatabase` with `Draw row identity or fields` (a production reader of the Binary64 `draw_scalar` table, before and after my edit; not diagnosed); (c) 14 entries in the four scene query/paint/prepare test files fail on `ENOENT …/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/📦️data/🔢️properties.bin` (a data file that is not on disk; visible in the log after the `admitFontSources` fix lets the tests reach it); (d) `kinds-catalog`, `edit-selection`, `interaction`, `export-document`, `layout-drawing` one each (not investigated).
3. **Remaining typed-map parsers**: 14 more `💡️inferences` files carry the same untyped-map guard (flow, animate, shooting, sequence, forms, layout, playbook, procedure, raster, semio flow/mesh/graph, puzzle 2d, and two hub space configs; `Record<string, number>`-style TS2322). The generator now renders them correctly: `python3 wp4c-project-exports.py apply --replace DrawingTopology … --under <plugin>` for each owner (the `GuardObject` call is in `parse…Topology`, `parse…Inference`; names differ per file). I did not run it outside draw because those are other owners' files and some are in the hub.
4. **Opaque schema nodes**: the dxf, html and md diff schemas still say `{"type": "object"}` where the hand-written types are rich. The parsers now follow the types; the schemas should become `$ref`s.
5. **`SemioGraphPropertiesDiff`** (TS2304, 2 diagnostics) in `🧿️semio/…/🕸️graph/🧬️schema/🔺️diff/🟦️.ts`: edited at 17:55, left alone.
6. The `psc` root lists keep going stale as peers carve; `tt2-tsconfig-psc-live.json` has no missing root today (313 of 313 exist).
7. Rust, wasm and the GraphQL/protobuf projections of the generator were not run (compiler ingress refused; `--formats 🟦️typescript` only).

## Commands run (all from `/Users/ueli/Documents/semio`)

1. Baselines: draw scoped project `tt2-tsconfig-draw.json` (227 roots; `tt2-draw-base.txt`: 231 draw diagnostics, identical multiset to `tte-psc-after.txt`); draw tests `tt2-tools/tt2-run-draw-tests.sh before` (69 files, log `tt2-draw-tests-before.log`).
2. Edits: python with asserted anchors, one snapshot per file, refusing a file modified within an hour (`tt2-tools/tt2_edit.py`, `tt2-group*.py`, `tt2-5d-diff.py`, `tt2-3d-integer-min.py`, `tt2-diffparsers-test.py`, `tt2-xml-valid.py`).
3. `node --max-old-space-size=12288 node_modules/typescript/bin/tsc -p 🗑️generated/<config> --noEmit --pretty false` (`tt2-run-draw.sh`, `-fam`, `-all`, `-gen9`, `-psc`): `tt2-draw-base`, `tt2-fam-after1/2`, `tt2-all1/2/3`, `tt2-gen9-a/b`, `tt2-psc-final`. `tt2-fam-after1` and `tt2-gen9-a` were invalid runs (a generated-prefix bug I then fixed; a peer's syntax error in `📚️library/…/🧾️custody/🟦️.ts` that stopped tsc before the semantic phase) and are not used.
4. `bun test ./<file>` for the routes above; `tt2-tools/tt2-xml-valid-harness.test.ts`; `tt2-tools/tt2-generated-parsers.test.ts`.
5. Generator: `python3 …/wp4c-project-exports.py report|apply …` as listed under Target 4.
6. Read-only git: `git diff`, `git show HEAD:<path>` (to restore one file I had generated by mistake), `git grep`, `git ls-files`, `git status`.

No `[DEBUG]` logs were added. Nothing was deleted in any `🗑️generated` folder.

## Files changed

Draw (all under `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/`): new `✳️any/🧪️tests/🧫️fixtures/🟦️.ts`; source: `✳️any/🧬️schema/🧮️geometry/📷️framing/🟦️.ts`, `…/↗️affine/🟦️.ts`, `…/🎯️picking/🎨️paint/🟦️.ts`, `✳️any/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/📄️document/🟦️.ts`, `🎨️style/…/↩️inverse/🟦️.ts`, `✳️any/🏭️generator/📜️script.ts`, `✳️any/🧬️schema/💡️inferences/🟦️.ts` (regenerated); tests (41): fill, scene paint/prepare/query/view/placement/raster/assets/booleans/trace/text/identity, shape coordinates, stroke, five mutation tests, geometry bounds/editing/picking paint/framing/raster, add-layer, edit-selection, export-document, import-image, patch-layer, clipboard, interaction, svg export unit/write, pdf write, svg import document/path, image, sqlite snapshot, update-text, update-path-geometry, `🔨️modules/🏠️host/🧰️owned/…/📋️native-owner`; plus the framework wgpu `🖍️draw/🏷️types/↗️affine` test. Full list with snapshot mapping: `🗑️generated/tt2-edited.json`.

Puzzle: `🖐️5d/…/🔺️diff/🟦️.ts`, `🧊️3d/…/🔺️diff/🟦️.ts` (index minimum), new `🖐️5d/…/🧪️tests/🧫️fixtures/🔺️diff/🔢️native-fields/🔣️.json`, `✏️s/🧑‍💻dev/🧩️puzzle/🧪️tests/🔺️diff-parsers/🟦️.ts`.

stdio: `📰️xml/…/✅️valid/🧬️schema/🟦️.ts` and its mirror test; the nine generated files; `🌐️html` diff (interfaces); `🧿️semio/…/🎞️animation/🚪️io/📝️text/🔺️diff/🟦️.ts`.

Tooling: the generator above (ticket script), `tt2-tools/*`.
