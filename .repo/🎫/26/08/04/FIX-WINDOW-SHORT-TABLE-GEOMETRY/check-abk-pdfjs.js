import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";

const pdfPath = "mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf";
const ticket = ".repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({
  data: new Uint8Array(readFileSync(pdfPath)),
  useSystemFonts: true,
}).promise;

const hits = [];
for (let p = 1; p <= doc.numPages; p++) {
  const page = await doc.getPage(p);
  const tc = await page.getTextContent();
  const text = tc.items.map((i) => i.str).join(" ");
  if (/Test-Case|Abkürzungs|Glossar|Abkürzung/.test(text)) {
    hits.push({
      p,
      glossTitle: /Glossar/.test(text),
      abkTitle: /Abkürzungsverzeichnis/.test(text),
      abkHeader: /Abkürzung/.test(text),
      test: /Test-Case/.test(text),
      AP: /\bAP\b/.test(text),
      API: /\bAPI\b/.test(text),
      snippet: text.replace(/\s+/g, " ").slice(0, 280),
    });
  }
}
console.log(JSON.stringify(hits, null, 2));
writeFileSync(`${ticket}/abk-text-hits.json`, JSON.stringify(hits, null, 2));

// Render relevant pages for visual QA
for (const h of hits) {
  if (!(h.test || h.abkTitle || h.AP)) continue;
  const page = await doc.getPage(h.p);
  const scale = 2.5;
  const viewport = page.getViewport({ scale });
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const ctx = canvas.getContext("2d");
  await page.render({ canvasContext: ctx, viewport }).promise;
  const out = `${ticket}/qa-p${h.p}.png`;
  writeFileSync(out, canvas.toBuffer("image/png"));
  console.log("[DEBUG] wrote", out);
}
