import assert from "node:assert/strict";
import { readFileSync, writeFileSync, mkdirSync, mkdtempSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { pathToFileURL } from "node:url";
import { createRequire } from "node:module";
import { spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { renderVizGalleryDocument } from "C:/git/semio/🧰️framework/🛍️products/📓️print/🔨️modules/📊️visualization-gallery/🟦️.ts";
import { preparedTectonic } from "C:/git/semio/🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts";
import { preparedPrintBundle } from "C:/git/semio/🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📚️bundle/📜️script.ts";
import { printFontSearchPaths } from "C:/git/semio/🧰️framework/🛍️products/📓️print/🔨️modules/🔤print-font-catalog/🟦️.ts";
const workspaceRoot="C:/git/semio", mode=process.argv[2] ?? "before";
if (import.meta.main) assert.ok(["before","candidate"].includes(mode), "probe mode");
const ticket=join(import.meta.dir,"../.."), outputRoot=join(ticket,"🗑️generated/fleet-scientific-layout",mode!), library=join(outputRoot,"library");
const canvas=createRequire(join(workspaceRoot,"node_modules/pdfjs-dist/legacy/build/pdf.mjs"))("@napi-rs/canvas");
Object.assign(globalThis,{DOMMatrix:canvas.DOMMatrix,Path2D:canvas.Path2D,ImageData:canvas.ImageData});
async function openPrintPdf(path:string) { const {getDocument}=await import("pdfjs-dist/legacy/build/pdf.mjs"); return await getDocument({data:new Uint8Array(readFileSync(path)),useSystemFonts:true}).promise; }
export async function compileFrozenPrintTex(tex:string,out:string,work:string,library:string,signal?:AbortSignal) {
  mkdirSync(out,{recursive:true});
  const args=["--bundle",pathToFileURL(preparedPrintBundle(workspaceRoot)).href,"--keep-logs","--keep-intermediates","-Z","deterministic-mode",...[library,work,out,...printFontSearchPaths(workspaceRoot)].flatMap(path=>["-Z",`search-path=${path}`]),"--outdir",out,relative(work,tex).replaceAll("\\","/")];
  const env={...process.env,SOURCE_DATE_EPOCH:"1782864000",TZ:"UTC",TEXINPUTS:[library,work,out].map(path=>`${path}//;${path};`).join("")};
  await new Promise<void>((accept,reject)=>{const child=spawn(preparedTectonic(workspaceRoot),args,{cwd:work,env,stdio:"inherit",timeout:300000,signal});child.once("error",reject);child.once("close",code=>code===0?accept():reject(Error(`Frozen native compiler ${code}`)));});
}
export async function verifyPrintGalleryCarrier(beforePdfPath?: string): Promise<void> {
  const control = JSON.parse(readFileSync(join(import.meta.dir, "🔣️carrier.json"), "utf8"));
  const { extent } = await import("d3-array"), { OPS, Util } = await import("pdfjs-dist/legacy/build/pdf.mjs");
  const validate = new (createRequire(import.meta.url)("ajv").default)().compile({ type: "object", required: ["width", "explicitHeight", "tolerancePt", "cases"], properties: { width: { const: 80 }, explicitHeight: { const: 40 }, tolerancePt: { type: "number", minimum: 0, maximum: 2 }, cases: { type: "array", minItems: 4, items: { type: "object", required: ["slug", "en", "de"], properties: { slug: { type: "string" }, en: { type: "string" }, de: { type: "string" } }, additionalProperties: false } } }, additionalProperties: false });
  assert.ok(validate(control), JSON.stringify(validate.errors));
  const title = { en: "Gallery carrier controls", de: "Galerierahmen-Kontrollen" };
  let source = renderVizGalleryDocument(title, control.cases.map((entry: any) => ({ group: title, leafId: entry.slug, slug: entry.slug })));
  const failures: string[] = [];
  const mark = (name: string) => `\\special{pdf:literal direct /${name} BMC}`;
  const end = "\\special{pdf:literal direct EMC}";
  const reference = (index: number | string) => `\\coordinate (SemioCarrierSW) at (current bounding box.south west);\\coordinate (SemioCarrierNE) at (current bounding box.north east);\n${mark(`SemioFrame${index}`)}\n\\begin{pgfinterruptboundingbox}\\draw[line width=0.01pt] (SemioCarrierSW) rectangle (SemioCarrierNE);\\end{pgfinterruptboundingbox}\n${end}`;
  for (const [index, entry] of control.cases.entries()) source = source.replace(`\\SemioVizChart{${entry.slug}}`, `${mark(`SemioCarrier${index}`)}\n\\SemioVizChart{${entry.slug}}\n${end}\n${reference(index)}`);
  const explicit = `\\begin{VizFigure}[title={\\SemioVizLocalized{Explicit height}{Explizite Höhe}},width=${control.width},height=${control.explicitHeight}]\n${mark("SemioCarrierExplicit")}\n\\draw (0,0) rectangle (10,10);\n${end}\n${reference("Explicit")}\n\\end{VizFigure}`;
  source = source.replace("\\end{document}", explicit + "\n\\end{document}");
  mkdirSync(outputRoot, { recursive: true });
  const root = mkdtempSync(join(outputRoot, ".gallery-carrier-"));
  const box = (points: number[][]) => [...extent(points.map(point => point[0]!)), ...extent(points.map(point => point[1]!))] as [number, number, number, number];
  const documents = beforePdfPath ? [["en", "light"]] as const : [["en", "light"], ["de", "dark"]] as const;
  for (const [language, appearance] of documents) {
    const work = join(root, `${language}-${appearance}`), out = join(work, "out"), tex = join(work, "carrier.tex");
    mkdirSync(work, { recursive: true });
    const documentSource = source.replace("theme=light,language=de", `theme=${appearance},language=${language}`);
    if (beforePdfPath) assert.equal(readFileSync(join(dirname(dirname(beforePdfPath)), "carrier.tex"), "utf8"), documentSource, "retained carrier TeX differs from neutral input");
    else { writeFileSync(tex, documentSource); await compilePrintTexOnce(tex, out, work); }
    const pdf = await openPrintPdf(beforePdfPath ?? join(out, "carrier.pdf"));
    const frames = new Map<string, number[]>(), origins = new Map<string, number>(), pages = new Map<string, number>(), pageWidths = new Map<number, number>(), panels: { page: number; bounds: number[] }[] = [], geometry = new Map<string, number[][]>(), texts = new Map<string, string[]>();
    try {
      for (let number = 1; number <= pdf.numPages; number++) {
        const page = await pdf.getPage(number), operators = await page.getOperatorList(), content = await page.getTextContent({ includeMarkedContent: true });
        pageWidths.set(number, page.getViewport({ scale: 1 }).width);
        const graphics: { matrix: number[]; width: number; join: number; cap: number; limit: number }[] = [], marked: string[] = [];
        let matrix = [1, 0, 0, 1, 0, 0], width = 1, joinStyle = 0, cap = 0, limit = 10;
        for (const [index, id] of operators.fnArray.entries()) {
          const args = operators.argsArray[index];
          if (id === OPS.save || id === OPS.paintFormXObjectBegin) { graphics.push({ matrix: [...matrix], width, join: joinStyle, cap, limit }); if (id === OPS.paintFormXObjectBegin && args[0]) matrix = Util.transform(matrix, args[0]); }
          else if (id === OPS.restore || id === OPS.paintFormXObjectEnd) { const saved = graphics.pop(); assert.ok(saved, "gallery PDF graphics stack underflow"); matrix = saved.matrix; width = saved.width; joinStyle = saved.join; cap = saved.cap; limit = saved.limit; }
          else if (id === OPS.transform) matrix = Util.transform(matrix, args);
          else if (id === OPS.setLineWidth) width = Number(args[0]);
          else if (id === OPS.setLineJoin) joinStyle = Number(args[0]);
          else if (id === OPS.setLineCap) cap = Number(args[0]);
          else if (id === OPS.setMiterLimit) limit = Number(args[0]);
          else if (id === OPS.setGState) { for (const state of args[0]) { if (state[0] === "LW") width = Number(state[1]); else if (state[0] === "LJ") joinStyle = Number(state[1]); else if (state[0] === "LC") cap = Number(state[1]); else if (state[0] === "ML") limit = Number(state[1]); } }
          else if (id === OPS.beginMarkedContent || id === OPS.beginMarkedContentProps) marked.push(String(args[0]?.name ?? args[0]));
          else if (id === OPS.endMarkedContent) marked.pop();
          else if (id === OPS.constructPath && args[2]) {
            const owner = [...marked].reverse().find(name => /^(?:SemioFrame|SemioCarrier)/.test(name)) ?? "", bounds = Array.from(args[2], Number);
            const points = [[bounds[0]!, bounds[1]!], [bounds[2]!, bounds[1]!], [bounds[2]!, bounds[3]!], [bounds[0]!, bounds[3]!]];
            points.forEach(point => Util.applyTransform(point, matrix));
            if (owner.startsWith("SemioFrame")) { assert.ok(!frames.has(owner), `duplicate ${owner}`); frames.set(owner, box(points)); origins.set(owner, matrix[5]!); pages.set(owner, number); }
            else if (owner.startsWith("SemioCarrier")) {
              const stroked = [OPS.stroke, OPS.closeStroke, OPS.fillStroke, OPS.eoFillStroke, OPS.closeFillStroke, OPS.closeEOFillStroke].includes(args[0]);
              const reservation = Math.max(joinStyle === 0 ? limit : 1, cap === 2 ? Math.SQRT2 : 1);
              const horizontal = stroked ? reservation * width / 2 * Math.hypot(matrix[0]!, matrix[2]!) : 0, vertical = stroked ? reservation * width / 2 * Math.hypot(matrix[1]!, matrix[3]!) : 0;
              geometry.set(owner, [...(geometry.get(owner) ?? []), ...points.flatMap(point => [[point[0]! - horizontal, point[1]! - vertical], [point[0]! + horizontal, point[1]! + vertical]])]);
            }
            else if ([OPS.fill, OPS.eoFill].includes(args[0])) panels.push({ page: number, bounds: box(points) });
          }
        }
        assert.equal(graphics.length, 0, "unbalanced gallery PDF graphics stack");
        const textMarks: string[] = [];
        for (const item of content.items) {
          if (!("str" in item)) { if (item.type === "beginMarkedContent" || item.type === "beginMarkedContentProps") textMarks.push(String((item as { tag?: string }).tag)); else if (item.type === "endMarkedContent") textMarks.pop(); continue; }
          const owner = [...textMarks].reverse().find(name => name.startsWith("SemioCarrier")) ?? "";
          if (!owner.startsWith("SemioCarrier")) continue;
          texts.set(owner, [...(texts.get(owner) ?? []), item.str]);
          const style = content.styles[item.fontName], ascent = style?.ascent ?? 0.8, descent = style?.descent ?? -0.2;
          const [a, b, c, d, x, y] = item.transform, length = Math.hypot(a!, b!);
          const points = [descent, ascent].flatMap(vertical => [0, item.width].map(horizontal => [x! + a! / length * horizontal + c! * vertical, y! + b! / length * horizontal + d! * vertical]));
          geometry.set(owner, [...(geometry.get(owner) ?? []), ...points]);
        }
      }
      for (const [index, entry] of control.cases.entries()) {
        const frame = frames.get(`SemioFrame${index}`), points = geometry.get(`SemioCarrier${index}`);
        assert.ok(frame && points?.length, `${language}/${entry.slug}: marked frame or content missing`);
        const panel = panels.filter(candidate => candidate.page === pages.get(`SemioFrame${index}`) && candidate.bounds[0]! < frame[0]! && candidate.bounds[1]! > frame[1]! && candidate.bounds[2]! < frame[2]! && candidate.bounds[3]! > frame[3]!).sort((left, right) => (left.bounds[1]! - left.bounds[0]!) * (left.bounds[3]! - left.bounds[2]!) - (right.bounds[1]! - right.bounds[0]!) * (right.bounds[3]! - right.bounds[2]!))[0]?.bounds;
        assert.ok(panel && panel[1]! - panel[0]! < pageWidths.get(pages.get(`SemioFrame${index}`)!)! - 0.1, `${language}/${entry.slug}: actual figure panel missing`);
        const bounds = box(points), tolerance = control.tolerancePt;
        if (bounds[0] < panel[0]! - tolerance || bounds[1] > panel[1]! + tolerance || bounds[2] < panel[2]! - tolerance || bounds[3] > panel[3]! + tolerance) failures.push(`${language}/${entry.slug}: actual content ${JSON.stringify(bounds)} escapes figure body ${JSON.stringify(panel)}`);
        console.log(`[DEBUG] Gallery carrier ${language}/${entry.slug}: frame=${JSON.stringify(frame)} body=${JSON.stringify(panel)} ink=${JSON.stringify(bounds)} textItems=${texts.get(`SemioCarrier${index}`)?.length ?? 0}`);
      }
      const explicitFrame = frames.get("SemioFrameExplicit");
      assert.ok(explicitFrame, "explicit-height reference missing");
      assert.ok(Math.abs(explicitFrame[1]! - explicitFrame[0]! - control.width * 72 / 25.4) < 0.2 && Math.abs(explicitFrame[3]! - explicitFrame[2]! - control.explicitHeight * 72 / 25.4) < 0.2, "explicit width/height contract changed");
      assert.ok(Math.min(...geometry.get("SemioCarrier2")!.map(point => point[1]!)) < origins.get("SemioFrame2")! - 6, "negative legend ink was not measured");
      console.log(`[print] Gallery carrier ${language}/${appearance}: ${control.cases.length} actual frames, marked path/text extents, explicit-height control, ${pdf.numPages} PDF.js pages measured`);
    } finally { await pdf.destroy(); }
  }
  if (failures.length) throw Error(failures.join("\n"));
  assert.doesNotMatch(source, /width=80, height=40/, "gallery must select intrinsic height");
}


async function compilePrintTexOnce(tex:string,out:string,work:string) { await compileFrozenPrintTex(tex,out,work,library); }
if (import.meta.main) try { await verifyPrintGalleryCarrier(); console.log(`[DEBUG] Frozen ${mode} native layout carrier PASS`); }
finally { writeFileSync(join(outputRoot,"probe-binding.json"),JSON.stringify({mode,styleSha256:createHash("sha256").update(readFileSync(join(library,"semio-viz-scientific-field.sty"))).digest("hex"),fixtureSha256:createHash("sha256").update(readFileSync(join(import.meta.dir,"🔣️carrier.json"))).digest("hex")},null,2)); }