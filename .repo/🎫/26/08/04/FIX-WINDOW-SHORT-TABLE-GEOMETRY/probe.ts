#!/usr/bin/env bun
/** 📐 [DEBUG] temp: dumps text-item x/y extents and vector line segments for one page, in pt. */
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, pageArg, yMinArg, yMaxArg] = process.argv.slice(2);
const pdfjsEntry = fileURLToPath(new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url));
createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({ data: new Uint8Array(readFileSync(pdfPath)), useSystemFonts: true }).promise;
const page = await doc.getPage(Number(pageArg));
const yMin = Number(yMinArg ?? 0);
const yMax = Number(yMaxArg ?? 1e6);
const content = await page.getTextContent();
type Item = { str: string; transform: number[]; width: number; height: number };
const rows = new Map<number, { x: number; right: number; str: string }[]>();
for (const raw of content.items as Item[]) {
  if (!raw.str.trim()) continue;
  const x = raw.transform[4];
  const y = raw.transform[5];
  if (y < yMin || y > yMax) continue;
  const key = Math.round(y * 2) / 2;
  const bucket = rows.get(key) ?? [];
  bucket.push({ x, right: x + raw.width, str: raw.str });
  rows.set(key, bucket);
}
const keys = [...rows.keys()].sort((a, b) => b - a);
for (const key of keys) {
  const bucket = (rows.get(key) ?? []).sort((a, b) => a.x - b.x);
  const cells = bucket.map((item) => `${item.x.toFixed(1)}→${item.right.toFixed(1)} ${JSON.stringify(item.str)}`);
  console.log(`y=${key.toFixed(1)}  ${cells.join("  |  ")}`);
}
