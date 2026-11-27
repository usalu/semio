# GIS Dependency Closure (gismap lib test pulls the PDF plugin)

Method: `repo cargo ... tree` was queued behind the capacity gate (needs 1.5 GB, waited >8 min, killed, no output). Used a resolve-free static walk of the local `path` and `workspace = true` dependencies instead (normal edges everywhere, dev edges only of the root, as cargo does). Script: scratchpad `g.py`/`h.py`; numbers are local crates only (third-party crates excluded). Re-run the cargo tree query below once memory allows to confirm.

## Closure (gismap, `--lib` test build)

- 756 local crates, of which 512 framework, 230 stdio, 14 gis. 87 are `semio-s-artifact-stdio-pdf*`.
- bim / fem: none in the closure. gisterrain and gisterrain-editor: 838 / 837 local crates, 87 pdf crates, same two umbrellas.

## Chains to the heavy units

1. `gismap -> stdio-semio-drawing-export -> stdio-pdf-conformance-support -> stdio-pdf-model -> stdio-pdf-model-graphics | stdio-pdf-model-object`
2. `gismap -> stdio-semio-drawing-export -> stdio-pdf-lower -> (lift, fonts, model, ...)`
3. `gismap -> stdio-semio-drawing-import -> stdio-pdf (whole plugin incl. editor, viewer, page-editor) -> ...` (88 pdf crates alone)
(dev-dependency `stdio-pdf` of drawing-export does not apply to dependents.)

gismap crates carrying the edge: gismap, -snapshot, -schema, -mutations, -tests, -gismap (viewer). The only users are the leaf `📤️export/🧵️serializers/📖️pdf` (`encode_drawing(.., Pdf)`, registered as export composer row `EXPORT_PDF_DIALECT` in `io/🦀️.rs`) and, for import, only dxf/dwg readers.

## Findings

- The edge is a real product feature: gismap exports PDF 1.4 through the drawing umbrella. It is not a stray dev-dependency or laws crate.
- The umbrellas `drawing-export` / `drawing-import` are per-format umbrellas (dxf, dwg, svg, png, pdf) used by 70+ manifests (puzzle 5d/2d, shooting, raster, draw, ...); every consumer pays for pdf.
- The pdf plugin offers no narrow writer: `serialize_document` lives in the `conformance-support` crate (six conformance subsets), `lower_document` in `pdf-lower` which depends on `pdf-model`. Even `pdf-model` alone reaches `model-graphics` and `model-object` (closure 528, 7 pdf crates). Retargeting any single manifest edge therefore cannot drop those two units while PDF export stays in the gismap lib.
- Retargeting drawing-import from umbrella `stdio-pdf` (88 pdf crates) to `pdf-mut-family-11` (53) would trim ~35 pdf crates but not the named heavy ones.

## Fix (not applied, needs approval of scope)

No manifest was changed: a faithful fix is a cross-plugin restructure that cannot be validated under the current compile budget and would alter a shipped export feature.

1. Carve per-format crates from the umbrellas (precedent: `drawing-png-export`): `drawing-pdf-export` and `drawing-pdf-import`; `encode_drawing` / import dispatch stop naming pdf; consumers that offer pdf add the pdf crate explicitly.
2. In the pdf plugin carve a narrow `pdf-writer` (writer + lower + `PdfSnapshot`) out of conformance-support so the object/graphics models are not needed to serialise.
3. In gismap, move the `📖️pdf` export serializer and its composer row into its own crate (taxonomy: `🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/📦️packages`) which only the app composition root depends on; the lib then compiles without pdf.

Expected effect after 1+3: gismap local closure about 756 -> about 670 with 0 pdf crates (pdf share is 87 crates, but the removed set also drops 0 of the framework crates), and the two 1.6-1.8 GB units leave the lib test build.

## Before / after counts

Before (static, local crates): gismap 756 (87 pdf); gisterrain 838 (87 pdf); gisterrain-editor 837 (87 pdf). After: unchanged (no manifest edited). `repo cargo ... tree --prefix none | sort -u | wc -l` was not obtainable (capacity gate). Interim relief without restructure: compile the two heavy units once under the compile lock (shared store caches them), then lib tests reuse them.
