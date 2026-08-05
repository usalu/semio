#!/usr/bin/env bun
/**
 * 🔬 [DEBUG] Body-cell left/top pad via text items + rule scan (scale 6).
 */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const pdfPath = process.argv[2]!;
const scale = 6;
const ticketDir = dirname(fileURLToPath(import.meta.url));
const samples = [
  { id: "toc", page: 3, title: "Inhaltsverzeichnis", bodyNeedle: "Lebenszyklusanalyse" },
  { id: "meilensteine", page: 18, title: "Meilensteine", bodyNeedle: "Aus dem Erfahrungswissen" },
  { id: "risiken", page: 19, title: "Risiken und Maßnahmen", bodyNeedle: "Formale Fristen" },
];

const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({
  data: new Uint8Array(readFileSync(pdfPath)),
  useSystemFonts: true,
}).promise;

for (const s of samples) {
  const page = await doc.getPage(s.page);
  const viewport = page.getViewport({ scale });
  const pts = (await page.getTextContent()).items.map((it: { str: string; transform: number[]; height?: number; width?: number }) => {
    const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
    // pdf.js text baseline; approx glyph top ≈ y - fontHeight*scale-ish via transform
    const fontH = Math.abs(it.transform[3]) * scale; // viewport scale already applied via convert? transform is in PDF space
    const fontHpx = Math.abs(it.transform[3]) * scale;
    return { str: it.str, x, y, fontHpx };
  });
  // fix font height: convertToViewportPoint doesn't scale transform[3]; multiply by scale
  const title = pts.filter((c) => c.str.includes(s.title.slice(0, 8))).sort((a, b) => b.str.length - a.str.length)[0]!;
  const bodyParts = pts.filter((c) => c.str && s.bodyNeedle.includes(c.str.slice(0, Math.min(8, c.str.length))));
  // better: find item whose str is prefix of needle or needle includes str
  const body = pts
    .filter((c) => c.str && (s.bodyNeedle.startsWith(c.str) || c.str.startsWith(s.bodyNeedle.slice(0, 6))))
    .sort((a, b) => a.y - b.y)[0];

  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
  const W = canvas.width;
  const data = canvas.getContext("2d").getImageData(0, 0, W, canvas.height).data;
  const rgb = (x: number, y: number) => {
    const i = (y * W + x) * 4;
    return [data[i], data[i + 1], data[i + 2]] as const;
  };
  const lum = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  };
  const isRule = (x: number, y: number) => {
    const L = lum(x, y);
    const [r, g, b] = rgb(x, y);
    return L > 45 && L < 175 && Math.abs(r - g) < 45 && Math.abs(g - b) < 45;
  };
  const isInk = (x: number, y: number) => lum(x, y) < 100;

  // chip baseline + border
  let b0 = -1;
  let b1 = -1;
  for (let y = Math.floor(title.y) + 1; y < Math.floor(title.y) + 40 * scale; y++) {
    let n = 0;
    for (let x = Math.floor(title.x); x < Math.floor(title.x) + 160 * scale; x += 2) if (isRule(x, y)) n++;
    if (n > 40) {
      if (b0 < 0) b0 = y;
      b1 = y;
    } else if (b0 >= 0) break;
  }
  let borderX = Math.floor(title.x);
  let best = 0;
  for (let x = Math.max(2, Math.floor(title.x) - 25 * scale); x < Math.floor(title.x) + 15 * scale; x++) {
    let n = 0;
    for (let y = b0; y < b0 + 220 * scale; y++) if (isRule(x, y)) n++;
    if (n > best) {
      best = n;
      borderX = x;
    }
  }

  if (!body) {
    console.log(JSON.stringify({ id: s.id, error: "no body", near: pts.filter((p) => p.y > b1 && p.y < b1 + 120 * scale).slice(0, 20) }));
    continue;
  }

  // rule above body baseline
  let ruleAbove = -1;
  let ruleAboveEnd = -1;
  for (let y = Math.floor(body.y) - 1; y > b1; y--) {
    let n = 0;
    for (let x = borderX + 8 * scale; x < borderX + 140 * scale; x += 2) if (isRule(x, y)) n++;
    if (n > 25) {
      if (ruleAboveEnd < 0) ruleAboveEnd = y;
      ruleAbove = y;
    } else if (ruleAboveEnd >= 0) break;
  }

  // scan for ink top in a band to the right of border, starting just below rule
  const scanX0 = borderX + Math.round(3 * scale);
  const scanX1 = Math.min(W - 2, Math.floor(body.x) + Math.round(80 * scale));
  let inkTop = -1;
  let inkLeft = -1;
  for (let y = ruleAboveEnd + 1; y < Math.floor(body.y) + Math.round(2 * scale); y++) {
    for (let x = scanX0; x < scanX1; x++) {
      // darker than canvas, not rule-gray
      const L = lum(x, y);
      const [r, g, b] = rgb(x, y);
      const ruleish = L > 45 && L < 175 && Math.abs(r - g) < 45;
      if (L < 110 && !ruleish) {
        let run = 0;
        for (let dx = 0; dx < Math.round(4 * scale); dx++) if (lum(x + dx, y) < 110) run++;
        if (run >= Math.round(2 * scale)) {
          inkTop = y;
          inkLeft = x;
          break;
        }
      }
    }
    if (inkTop >= 0) break;
  }
  if (inkTop >= 0) {
    let left = inkLeft;
    for (let y = inkTop; y < inkTop + Math.round(6 * scale); y++) {
      for (let x = scanX0; x <= inkLeft; x++) {
        if (lum(x, y) < 110 && !isRule(x, y)) {
          left = Math.min(left, x);
          break;
        }
      }
    }
    inkLeft = left;
  }

  // also: header cell under chip (first ink after chip baseline)
  let headTop = -1;
  let headLeft = -1;
  for (let y = b1 + 1; y < b1 + Math.round(28 * scale); y++) {
    for (let x = borderX + 2; x < borderX + Math.round(70 * scale); x++) {
      if (isInk(x, y) && !isRule(x, y)) {
        let run = 0;
        for (let dx = 0; dx < 8; dx++) if (isInk(x + dx, y)) run++;
        if (run >= 3) {
          headTop = y;
          headLeft = x;
          break;
        }
      }
    }
    if (headTop >= 0) break;
  }
  if (headTop >= 0) {
    let left = headLeft;
    for (let y = headTop; y < headTop + Math.round(6 * scale); y++) {
      for (let x = borderX + 1; x <= headLeft; x++) {
        if (isInk(x, y) && !isRule(x, y)) {
          left = Math.min(left, x);
          break;
        }
      }
    }
    headLeft = left;
  }

  const out = {
    id: s.id,
    page: s.page,
    body: body.str,
    bodyY: Math.floor(body.y),
    ruleAbove,
    ruleAboveEnd,
    inkTop,
    inkLeft,
    bodyTopInsetPt: inkTop >= 0 ? +((inkTop - ruleAboveEnd - 1) / scale).toFixed(2) : null,
    bodyLeftInsetPt: inkLeft >= 0 ? +((inkLeft - borderX - 1) / scale).toFixed(2) : null,
    headerTopInsetPt: headTop >= 0 ? +((headTop - b1 - 1) / scale).toFixed(2) : null,
    headerLeftInsetPt: headLeft >= 0 ? +((headLeft - borderX - 1) / scale).toFixed(2) : null,
    borderX,
  };
  console.log(JSON.stringify(out, null, 2));

  // zoom crop of body cell corner
  if (inkTop >= 0) {
    const c = createCanvas(Math.round(80 * scale), Math.round(30 * scale));
    c.getContext("2d").drawImage(
      canvas,
      borderX - 2,
      ruleAboveEnd - Math.round(2 * scale),
      Math.round(80 * scale),
      Math.round(30 * scale),
      0,
      0,
      Math.round(80 * scale),
      Math.round(30 * scale),
    );
    writeFileSync(join(ticketDir, `qaV-pad-${s.id}.png`), c.toBuffer("image/png"));
  }
}
