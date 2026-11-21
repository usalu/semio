/** 🖼️ Stage bounded proof for retained sampled and DCT image resources and scalar vector programs. */
import assert from "node:assert/strict";
import {join,resolve} from "node:path";
import {createHash} from "node:crypto";
const base = process.argv[2];
assert.ok(base?.endsWith("/🧱️base"));
const root = join(base,"✏️editor/💾️native-save");
const path = join(root,"🔏️proof.rs");
const original = await Bun.file(path).text();
const compare = await Bun.file(join(import.meta.dir,"📜️pdf-save-image-lanes.rs")).text();
function exact(source:string, anchor:string, value:string) {
    assert.equal(source.split(anchor).length,2,anchor);
    return source.replace(anchor,value);
}
let text=exact(original,'ObjRef, PdfDictEntry, PdfFont, PdfFontKind, PdfObject,','ObjRef, PdfColorSpace, PdfDictEntry, PdfFont, PdfFontKind, PdfImage, PdfImageBody, PdfObject,');
text=exact(text,'    page: usize,\n    op: usize,','    image: usize,\n    sample: usize,\n    page: usize,\n    op: usize,');
text=exact(text,'source: None, page: 0, op: 0,','source: None, image: 0, sample: 0, page: 0, op: 0,');
text=exact(text,'            self.document = true;\n        } else if let Some(page)',`            self.document = true;
        } else if let Some(image) = projected.images.get(self.image) {
            let current = expected.images.get(self.image).ok_or(ExactGraphRefusal::Semantic)?;
            if compare_image(image, current, &mut self.sample)? {
                self.image += 1;
                self.sample = 0;
            }
        } else if let Some(page)`);
text=exact(text,'        pdf.images.is_empty() && pdf.forms.is_empty()', '        pdf.forms.is_empty()');
text=exact(text,'        && projected.pages.len() == current.pages.len() && projected.fonts == current.fonts',
    '        && projected.pages.len() == current.pages.len() && projected.fonts == current.fonts\n        && projected.images.len() == current.images.len()');
text=exact(text,'fn narrow_font(font: &PdfFont)', compare+'\nfn narrow_font(font: &PdfFont)');
text=exact(text,'        SetFont { name, .. } => name.len() <= TEXT_CAP,',`        SetFont { name, .. } | PaintXObject { name } => name.len() <= TEXT_CAP,
        SetRenderingIntent { intent } => intent.len() <= TEXT_CAP,
        SetDash { array, .. } => array.len() <= TJ_ITEMS,`);
text=exact(text,'        | SetFillGray { .. } | SetFillRgb { .. } | SetFillCmyk { .. } => true,',`        | SetFillGray { .. } | SetFillRgb { .. } | SetFillCmyk { .. }
        | SetStrokeGray { .. } | SetStrokeRgb { .. } | SetStrokeCmyk { .. }
        | SetLineWidth { .. } | SetLineCap { .. } | SetLineJoin { .. } | SetMiterLimit { .. }
        | SetFlatness { .. } | MoveTo { .. } | LineTo { .. } | CurveTo { .. } | CurveToInitial { .. }
        | CurveToFinal { .. } | ClosePath | Rectangle { .. } | Stroke | CloseStroke | Fill | FillEvenOdd
        | FillStroke | FillStrokeEvenOdd | CloseFillStroke | CloseFillStrokeEvenOdd | EndPath | Clip | ClipEvenOdd => true,`);
const {measureRustCrate}=await import(resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🗺️plan/📏️measure/🟦️.ts"));
const pkg=resolve(root,"📦️packages/🦀️rust");
for (const os of ["linux","macos","windows","wasi"]) {
    const cfg = [`target_os="${os}"`,os === "wasi" ? 'target_arch="wasm32"' : os === "macos" ? 'target_arch="aarch64"' : 'target_arch="x86_64"',os === "wasi" ? 'target_family="wasm"' : os === "windows" ? "windows" : "unix"];
    const row=await measureRustCrate(process.cwd(),resolve(pkg,"🦀️.rs"),false,new Map([[resolve(path),text]]),pkg,cfg);
    assert.ok(row.files<=32&&row.lines<=4000&&row.bytes<=262144);
}
assert.equal(await Bun.file(path).text(),original);
const output=join(import.meta.dir,"🗑️generated","pdf-save-visible-graph-overlay.json");
await Bun.write(output,JSON.stringify([[path,createHash("sha256").update(original).digest("hex"),text]],null,2)+"\n");
console.log("[DEBUG] Staged image and vector proof in the actual native Save package");
