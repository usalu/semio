#!/usr/bin/env bun
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
const pdfPath = process.argv[2];
const needles = process.argv.slice(3);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
console.log(`[DEBUG] pages=${doc.numPages}`);
for (let p=1; p<=doc.numPages; p++) {
  const page = await doc.getPage(p);
  const tc = await page.getTextContent();
  const text = tc.items.map((i:any)=>i.str).join(" ");
  for (const n of needles) {
    if (text.includes(n)) console.log(`[DEBUG] p${p}: ${n}`);
  }
}
