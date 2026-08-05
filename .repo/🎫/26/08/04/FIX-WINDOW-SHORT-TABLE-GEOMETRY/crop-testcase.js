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

function lum(r, g, b) {
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

async function cropAround(pageNum, needle, outName) {
  const page = await doc.getPage(pageNum);
  const scale = 3;
  const viewport = page.getViewport({ scale });
  const tc = await page.getTextContent();
  let hit = null;
  for (const item of tc.items) {
    if (String(item.str).includes(needle)) {
      const t = pdfjs.Util.transform(viewport.transform, item.transform);
      hit = { x: t[4], y: t[5], str: item.str };
      break;
    }
  }
  if (!hit) {
    console.log("[DEBUG] miss", needle, "on", pageNum);
    return;
  }
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const ctx = canvas.getContext("2d");
  await page.render({ canvasContext: ctx, viewport }).promise;
  const { width: W, height: H } = canvas;
  const img = ctx.getImageData(0, 0, W, H).data;

  // PDF.js text y is from bottom in pdf space; after viewport transform y grows down
  const cx = Math.floor(hit.x);
  const cy = Math.floor(hit.y);
  console.log(`[DEBUG] ${needle} @ ${cx},${cy} page ${pageNum}`);

  const x0 = Math.max(0, cx - 80);
  const x1 = Math.min(W, cx + 1400);
  const y0 = Math.max(0, cy - 40);
  const y1 = Math.min(H, cy + 120);
  const crop = createCanvas(x1 - x0, y1 - y0);
  crop.getContext("2d").drawImage(canvas, x0, y0, x1 - x0, y1 - y0, 0, 0, x1 - x0, y1 - y0);
  writeFileSync(`${ticket}/${outName}`, crop.toBuffer("image/png"));

  // Measure ink on left edge of table near this row: find L near cx-ish
  let cream = 0,
    n = 0;
  for (let x = Math.floor(W / 2); x < Math.floor(W / 2) + 30; x++) {
    const i = (Math.floor(H * 0.1) * W + x) * 4;
    cream += lum(img[i], img[i + 1], img[i + 2]);
    n++;
  }
  cream /= n;

  // Find last horizontal rule below cy within 80px
  let lastRule = -1;
  for (let y = cy; y < cy + 80 && y < H; y++) {
    let hitCount = 0;
    for (let x = x0; x < x1; x++) {
      const i = (y * W + x) * 4;
      if (lum(img[i], img[i + 1], img[i + 2]) < cream - 30) hitCount++;
    }
    if (hitCount > (x1 - x0) * 0.35) lastRule = y;
  }
  // Find L/R near row
  let L = -1,
    R = -1;
  for (let x = 40; x < W - 40; x++) {
    let ink = 0;
    for (let y = cy - 10; y < cy + 40; y++) {
      const i = (y * W + x) * 4;
      if (lum(img[i], img[i + 1], img[i + 2]) < cream - 40) ink++;
    }
    if (ink > 20) {
      if (L < 0) L = x;
      R = x;
    }
  }
  let belowL = 0,
    belowR = 0;
  if (lastRule > 0 && L > 0) {
    for (let y = lastRule + 1; y <= lastRule + 18; y++) {
      const iL = (y * W + L) * 4;
      const iR = (y * W + R) * 4;
      if (lum(img[iL], img[iL + 1], img[iL + 2]) < cream - 40) belowL++;
      if (lum(img[iR], img[iR + 1], img[iR + 2]) < cream - 40) belowR++;
    }
  }
  console.log(
    JSON.stringify({
      needle,
      pageNum,
      lastRule,
      L,
      R,
      stubBelowLpx: belowL,
      stubBelowRpx: belowR,
      cream: +cream.toFixed(1),
    }),
  );
}

await cropAround(122, "Test-Case", "qa-testcase-end.png");
await cropAround(123, "Abkürzungsverzeichnis", "qa-abk-chrome.png");
await cropAround(123, "AP", "qa-abk-ap.png");
await cropAround(2, "Inhaltsverzeichnis", "qa-toc-chrome.png");
