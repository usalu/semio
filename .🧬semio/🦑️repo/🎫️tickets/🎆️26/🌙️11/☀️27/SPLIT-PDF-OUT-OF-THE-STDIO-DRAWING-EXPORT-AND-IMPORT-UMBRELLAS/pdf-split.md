# Split PDF out of the stdio drawing export/import umbrellas

Plan followed: `26/11/22/GIS-PLUGIN-END-TO-END/gis-dependency-closure.md` (steps 1 and 3 done; step 2 reported below, not applied).

## Closure (measured with `repo cargo -- tree`, local crates only, `closure.sh`)

| package | before | after |
|---|---|---|
| gismap lib (`-e normal,build`) | 736 local, 87 pdf | 643 local, 0 pdf |
| gismap with dev edges (`--lib` test build) | 750 local, 87 pdf | 659 local, 0 pdf |
| gisterrain lib | 87 pdf (static walk 838) | 724 local, 0 pdf |
| `stdio-semio-drawing-export` umbrella | pdf-lower + conformance-support | 587 local, 0 pdf |
| `stdio-semio-drawing-pdf-export` (new) | — | 554 local, 41 pdf |

`pdf-model-graphics` and `pdf-model-object` (the 1.6–1.8 GB rustc units) are no longer in the gismap or gisterrain lib closure. Only the composition roots that offer pdf (hub gis/procedural/puzzle compositions, dev composition, the semio full catalog) pay for them.

## What changed

### stdio (schema-first)
- Authority `🧿️semio/🧩️composition/🔗️conversions/🔣️.json` is the source of the umbrella capability dependencies, the catalog `ALL` list and the catalog manifest. It now lists two more conversion crates:
  - `semio-s-artifact-stdio-semio-drawing-pdf-export` (`🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf`, lib = the 1.7 leaf): `SemioDrawingToPdf`, `drawing_to_pdf`, `encode_drawing_pdf(drawing, version)`, `entries()`, `CONVERSION`. Targets: `pdf-conformance-support`, `pdf-lower`.
  - `semio-s-artifact-stdio-semio-drawing-pdf-import` (`🖊️drawing/🚪️io/📥️import/🧩️deserializers/🗿️artifacts/📖️pdf`): `SemioDrawingFromPdf`, `decode_drawing_pdf(bytes)`, `entries()`, `CONVERSION`. Target: `stdio-pdf`.
  - The `drawing` umbrellas lost their pdf targets, leaves, composer rows and dispatch arms.
- Authority rule changed (`🔗️conversions/🟦️.ts`, schema, test oracle, fixture): a subset direction may be split into several conversion crates (one per format group) as long as they name disjoint targets; rendering vectors are keyed by package. The "names exactly its targets" check skips nested crates (a nested crate's leaves belong to it). New fixture case `split-subset-direction`; the old `repeated-subset-direction` became `split-subset-direction-repeats-target` (still rejected). `conversion-definition-check` passes (48 checks).
- `SemioDrawingFormat` no longer has `Pdf`; `encode_drawing` / `decode_drawing` cover svg, dxf, dwg, png.

### plugins (pdf route only at the composition root)
Each pdf serializer moved into its own crate; the artifact library declares its own leaves, and a `declaration_from` / `artifact_with` entry lets the pdf crate add its route:
- `semio-s-artifact-gis-gismap-pdf-export` → `declaration()` = gismap `definition()` + `s.gis.gismap.composer.pdf` capability + pdf composer row. gismap: `declaration_from`, `io_registry::rebuild_native_snapshot` public, pdf leaf mount removed from `gismap-mutations`, pdf re-exports removed.
- `semio-s-artifact-procedural-generation2d-pdf-export` → same pattern; generation2d: `declaration_from`, `io_registry::{GENERATION2D_DIALECT, rebuild_native_snapshot}` public.
- `semio-s-artifact-puzzle-2d-pdf-export` → `artifact::<PA>()` = puzzle2d `artifact_with(entries())` where `entries()` = `routes::rows()` + the pdf `IoEntry`. puzzle2d: `routes::rows()`, `artifact_with`, `standard(routes)` / `subset(routes)`; the dead debt-D1 `definition()` row for pdf removed and the `format-N` ids renumbered.
- Composition roots now use the pdf crates: `🌎️hub/🧩️compositions/{🌍️gis,🌀️procedural,🧩️puzzle}`, the puzzle surface tests, and `✏️s/🧑‍💻dev/🧩️composition` (generation2d example export test).
- The kinds' static `export_stdio_kinds` still list `stdio.pdf`: the plugin builder rejects an artifact kind whose descriptor differs between the plugin and its editor apps, so the kind keeps describing the product, and the composer row comes from the composition root.
- Tests: every pdf crate has its own unit test with lopdf as the independent reader (page count, version, visible labels for puzzle2d), plus a declaration test showing the pdf capability or route is present only through the pdf crate.

### Locks
19 workspaces whose `Cargo.lock` named the umbrellas or the moved crates were refreshed with `lock-refresh.sh`. `🌎️hub`, `🧹️fixture-sweep` and the shooting bridge did not finish (concurrent edits by other agents, and the warden). Any later `repo cargo` on them refreshes the lock ("is behind its manifests; refreshing").

## Step 2 (narrow pdf writer): not applied, finding
The drawing serializer builds a `PdfSnapshot` of `PdfOp`s (model-graphics) and lowers it to `PdfIndirectObject`s (model-object); `serialize_document` takes those objects. These two crates are the PDF data model itself, so no writer carved out of `conformance-support` can avoid them while the drawing writer stays typed. What a carve could still remove from `drawing-pdf-export` (41 pdf crates) is the 1.4 diff/mutation/snapshot crates pulled by `conformance-support` and the lift/fonts/inference crates pulled by `pdf-lower`. That is a restructure of the pdf plugin's "path-preserving carve" mirrors (writer + lexer + filters + encryption + xref keys into one crate; lowering without lift). Its benefit is now limited to composition roots that offer pdf, because no library pays for pdf any more.

## Verification state
- `conversion-definition-check`: passes.
- Closures: measured above.
- Rust compile/tests of the touched crates: see the ticket close summary (the host warden and an in-flight framework refactor by another session (`value/retained-clone`, `actor/wire-turn`, `os/store/operation-wire`) blocked the builds during this session).
