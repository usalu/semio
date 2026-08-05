#!/usr/bin/env bun
/** 📐 [DEBUG] temp: find Seite bbox on a PDF page. */
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs
  .getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true })
  .promise;
const page = await doc.getPage(Number(pageArg));
const tc = await page.getTextContent();
const viewport = page.getViewport({ scale: 1 });
for (const item of tc.items as { str: string; transform: number[]; width: number; height: number }[]) {
  if (!/Seite|^1$/.test(item.str)) continue;
  const [, , , , x, y] = item.transform;
  console.log(JSON.stringify({ str: item.str, x, y, w: item.width, h: item.height, pageH: viewport.height }));
}
