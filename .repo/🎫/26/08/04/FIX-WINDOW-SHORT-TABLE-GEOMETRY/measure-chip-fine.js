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

function lum(d, W, x, y) {
  const i = (y * W + x) * 4;
  return 0.2126 * d[i] + 0.7152 * d[i + 1] + 0.0722 * d[i + 2];
}

async function fine(pageNum, needle) {
  const page = await doc.getPage(pageNum);
  const scale = 12; // 864 dpi
  const viewport = page.getViewport({ scale });
  const tc = await page.getTextContent();
  let hit = null;
  for (const item of tc.items) {
    if (String(item.str).includes(needle)) {
      const t = pdfjs.Util.transform(viewport.transform, item.transform);
      const cand = { x: t[4], y: t[5], str: item.str };
      if (!hit || cand.x < hit.x) hit = cand;
    }
  }
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
  const W = canvas.width,
    H = canvas.height;
  const d = canvas.getContext("2d").getImageData(0, 0, W, H).data;
  const L = (x, y) => lum(d, W, x, y);
  let cream = 0,
    n = 0;
  for (let x = Math.floor(W / 2); x < Math.floor(W / 2) + 20; x++) {
    cream += L(x, 40);
    n++;
  }
  cream /= n;
  const thr = cream - 50;
  const cy = Math.floor(hit.y);
  const yC0 = cy - Math.ceil(12 * scale),
    yC1 = cy + Math.ceil(1 * scale);
  const yB0 = cy + Math.ceil(12 * scale),
    yB1 = cy + Math.ceil(40 * scale);

  // Find all border columns near left of text
  const xMax = Math.floor(hit.x) + 20;
  const chipCols = [],
    bodyCols = [];
  for (let x = 40; x < xMax; x++) {
    let c = 0,
      b = 0;
    for (let y = yC0; y < yC1; y++) if (L(x, y) < thr) c++;
    for (let y = yB0; y < yB1; y++) if (L(x, y) < thr) b++;
    if (c > (yC1 - yC0) * 0.5) chipCols.push(x);
    if (b > (yB1 - yB0) * 0.5) bodyCols.push(x);
  }
  // cluster first runs
  const firstRun = (arr) => {
    if (!arr.length) return null;
    let s = arr[0],
      e = arr[0];
    for (const x of arr.slice(1)) {
      if (x === e + 1) e = x;
      else break;
    }
    return { s, e, mid: (s + e) / 2 };
  };
  const chip = firstRun(chipCols);
  const body = firstRun(bodyCols);
  const dPx = chip && body ? chip.s - body.s : null;
  console.log(
    JSON.stringify({
      page: pageNum,
      needle,
      hitX: +hit.x.toFixed(1),
      cream: +cream.toFixed(1),
      chip,
      body,
      dPx,
      dPt: dPx != null ? +(dPx / scale).toFixed(4) : null,
      hairlinePx: +(0.75 * scale).toFixed(2),
    }),
  );

  if (chip && body) {
    const x0 = Math.min(chip.s, body.s) - 6;
    const x1 = x0 + 30;
    const y0 = yC0 - 4;
    const y1 = yB1 + 4;
    const z = 6;
    const crop = createCanvas((x1 - x0) * z, (y1 - y0) * z);
    const ctx = crop.getContext("2d");
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(canvas, x0, y0, x1 - x0, y1 - y0, 0, 0, (x1 - x0) * z, (y1 - y0) * z);
    writeFileSync(`${ticket}/fine-${pageNum}-${needle.slice(0, 8)}.png`, crop.toBuffer("image/png"));
  }
}

await fine(78, "Marktpl");
await fine(19, "Risiken");
await fine(24, "Kopfbau"); // project card left?
await fine(18, "Meilensteine");
await fine(2, "Inhaltsverzeichnis");

// also find any "Tabelle:" strings
for (let p = 1; p <= doc.numPages; p++) {
  const page = await doc.getPage(p);
  const tc = await page.getTextContent();
  for (const it of tc.items) {
    if (/^Tabelle/.test(it.str)) {
      console.log("[DEBUG] Tabelle@", p, it.str);
    }
  }
}
