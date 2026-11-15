import * as pdfjs from "pdfjs-dist/legacy/build/pdf.mjs";
import { readFileSync, readdirSync } from "node:fs";
const root = process.argv[2]!;
const OPS = pdfjs.OPS as Record<string, number>;
const paintOps = new Set([OPS.fill, OPS.eoFill, OPS.fillStroke, OPS.eoFillStroke, OPS.stroke, OPS.closeStroke, OPS.closeFillStroke, OPS.closeEOFillStroke]);
const textOps = new Set([OPS.showText, OPS.showSpacedText, OPS.nextLineShowText, OPS.nextLineSetSpacingShowText]);
for (const dir of readdirSync(root).filter(name => /^viz-\d+$/.test(name))) for (const file of readdirSync(`${root}/${dir}`).filter(f => f.endsWith(".pdf"))) {
  const pdf = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(`${root}/${dir}/${file}`)), useSystemFonts: false }).promise;
  const rec = new Map<string, { paths: number; images: number; text: number }>();
  for (let p = 1; p <= pdf.numPages; p++) {
    const page = await pdf.getPage(p), ol = await page.getOperatorList(), stack: string[] = [];
    for (const [i, id] of ol.fnArray.entries()) {
      const a = ol.argsArray[i];
      if (id === OPS.beginMarkedContent || id === OPS.beginMarkedContentProps) { const n = typeof a?.[0] === "object" ? String(a[0].name) : String(a?.[0]); stack.push(n); if (n.startsWith("SemioVizLeaf")) rec.set(n, { paths: 0, images: 0, text: 0 }); }
      else if (id === OPS.endMarkedContent) stack.pop();
      else { const n = [...stack].reverse().find(s => s.startsWith("SemioVizLeaf")); if (!n) continue; const r = rec.get(n)!;
        if (id === OPS.constructPath && paintOps.has(a?.[0])) r.paths++; else if (id === OPS.paintImageXObject || id === OPS.paintInlineImageXObject) r.images++; else if (textOps.has(id)) r.text++; }
    }
  }
  for (const [n, r] of rec) if (!r.paths && !r.images) console.log(`${dir}/${file}`, Buffer.from(n.slice(12), "hex").toString(), JSON.stringify(r));
  console.log(`# ${dir}/${file}: ${rec.size} leaves`);
}
