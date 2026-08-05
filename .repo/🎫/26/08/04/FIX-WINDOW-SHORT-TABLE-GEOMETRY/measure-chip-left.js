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

async function measureChipL(pageNum, needle, out) {
  const page = await doc.getPage(pageNum);
  const scale = 6;
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
    console.log("[DEBUG] miss", needle, pageNum);
    return null;
  }
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const ctx = canvas.getContext("2d");
  await page.render({ canvasContext: ctx, viewport }).promise;
  const { width: W, height: H } = canvas;
  const img = ctx.getImageData(0, 0, W, H).data;
  const Lof = (x, y) => {
    const i = (y * W + x) * 4;
    return lum(img[i], img[i + 1], img[i + 2]);
  };

  // Sample cream
  let cream = 0,
    n = 0;
  for (let x = Math.floor(W / 2); x < Math.floor(W / 2) + 40; x++) {
    cream += Lof(x, 40);
    n++;
  }
  cream /= n;

  const cy = Math.floor(hit.y);
  // Find chip left border: scan left from text for first dark col in chip band
  const chipY0 = cy - Math.ceil(18 * scale);
  const chipY1 = cy + Math.ceil(4 * scale);
  let chipL = -1;
  for (let x = Math.floor(hit.x); x > 40; x--) {
    let ink = 0;
    for (let y = chipY0; y < chipY1; y++) if (Lof(x, y) < cream - 40) ink++;
    if (ink > (chipY1 - chipY0) * 0.35) {
      chipL = x;
      // keep going left to find outermost
    } else if (chipL > 0) break;
  }
  // walk leftmost contiguous
  while (chipL > 1) {
    let ink = 0;
    for (let y = chipY0; y < chipY1; y++) if (Lof(chipL - 1, y) < cream - 40) ink++;
    if (ink > (chipY1 - chipY0) * 0.2) chipL--;
    else break;
  }

  // Table body left: below chip, find first dark vertical
  const bodyY0 = cy + Math.ceil(20 * scale);
  const bodyY1 = cy + Math.ceil(50 * scale);
  let bodyL = -1;
  for (let x = 40; x < Math.floor(hit.x) + 40; x++) {
    let ink = 0;
    for (let y = bodyY0; y < bodyY1; y++) if (Lof(x, y) < cream - 40) ink++;
    if (ink > (bodyY1 - bodyY0) * 0.4) {
      bodyL = x;
      break;
    }
  }

  const dPx = chipL - bodyL;
  const dPt = dPx / scale;
  const result = {
    page: pageNum,
    needle,
    cream: +cream.toFixed(1),
    chipL,
    bodyL,
    dPx,
    dPt: +dPt.toFixed(3),
  };
  console.log(JSON.stringify(result));

  // crop zoom
  const x0 = Math.max(0, Math.min(chipL, bodyL) - 20);
  const x1 = x0 + 80;
  const y0 = Math.max(0, cy - Math.ceil(30 * scale));
  const y1 = Math.min(H, cy + Math.ceil(55 * scale));
  const crop = createCanvas((x1 - x0) * 3, (y1 - y0) * 3);
  const cctx = crop.getContext("2d");
  cctx.imageSmoothingEnabled = false;
  cctx.drawImage(canvas, x0, y0, x1 - x0, y1 - y0, 0, 0, (x1 - x0) * 3, (y1 - y0) * 3);
  // mark lines
  cctx.strokeStyle = "rgba(255,0,0,0.8)";
  cctx.beginPath();
  cctx.moveTo((chipL - x0) * 3, 0);
  cctx.lineTo((chipL - x0) * 3, crop.height);
  cctx.stroke();
  cctx.strokeStyle = "rgba(0,128,255,0.8)";
  cctx.beginPath();
  cctx.moveTo((bodyL - x0) * 3, 0);
  cctx.lineTo((bodyL - x0) * 3, crop.height);
  cctx.stroke();
  writeFileSync(`${ticket}/${out}`, crop.toBuffer("image/png"));
  return result;
}

const results = [];
for (const [p, n, o] of [
  [78, "BB.M.a", "align-bbma.png"],
  [78, "Tabelle", "align-tabelle.png"],
  [24, "P.K.1", "align-pk1.png"],
  [19, "Risiken", "align-risiken.png"],
  [76, "Hürden", "align-huerden.png"],
  [122, "Glossar", "align-gloss.png"],
]) {
  results.push(await measureChipL(p, n, o));
}
writeFileSync(`${ticket}/chip-left-align.json`, JSON.stringify(results, null, 2));
