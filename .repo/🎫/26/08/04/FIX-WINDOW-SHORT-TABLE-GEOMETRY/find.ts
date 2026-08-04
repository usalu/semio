#!/usr/bin/env bun
/** 🔍 [DEBUG] temp: locates pages containing a phrase and dumps x-extents of text + vector edges. */
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, phrase] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
for (let number = 1; number <= doc.numPages; number += 1) {
  const page = await doc.getPage(number);
  const content = await page.getTextContent();
  const text = content.items.map((item: { str: string }) => item.str).join(" ");
  if (text.includes(phrase)) console.log(`[DEBUG] page ${number}: ${phrase}`);
}
