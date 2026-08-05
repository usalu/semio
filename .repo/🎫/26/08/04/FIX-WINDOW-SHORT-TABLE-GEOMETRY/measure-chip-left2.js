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

async function measure(pageNum, needle, out) {
  const page = await doc.getPage(pageNum);
  const scale = 8;
  const viewport = page.getViewport({ scale });
  const tc = await page.getTextContent();
  const hits = [];
  for (const item of tc.items) {
    if (String(item.str).includes(needle)) {
      const t = pdfjs.Util.transform(viewport.transform, item.transform);
      hits.push({ x: t[4], y: t[5], str: item.str, w: item.width * scale });
    }
  }
  // Prefer leftmost hit (title chip, not right number chip)
  hits.sort((a, b) => a.x - b.x || a.y - b.y);
  const hit = hits[0];
  if (!hit) {
    console.log("[DEBUG] miss", needle, pageNum);
    return null;
  }

  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const ctx = canvas.getContext("2d");
  await page.render({ canvasContext: ctx, viewport }).promise;
  const W = canvas.width;
  const H = canvas.height;
  const d = ctx.getImageData(0, 0, W, H).data;
  const L = (x, y) => lum(d, W, x, y);

  let cream = 0,
    n = 0;
  for (let x = Math.floor(W / 2); x < Math.floor(W / 2) + 40; x++) {
    cream += L(x, 30);
    n++;
  }
  cream /= n;
  const borderThr = cream - 45; // border grey ~128

  const cy = Math.floor(hit.y);
  // Chip vertical band: around title baseline
  const yChip0 = cy - Math.ceil(14 * scale);
  const yChip1 = cy + Math.ceil(2 * scale);
  // Body band just under shared baseline
  const yBody0 = cy + Math.ceil(10 * scale);
  const yBody1 = cy + Math.ceil(36 * scale);

  function leftmostBorder(y0, y1, xMax) {
    for (let x = 40; x < xMax; x++) {
      let ink = 0;
      for (let y = y0; y < y1; y++) if (L(x, y) < borderThr) ink++;
      if (ink > (y1 - y0) * 0.45) return x;
    }
    return -1;
  }

  // Also: from text, walk left through canvas until border (darker), then take leftmost of that run
  function chipLeftFromText() {
    let x = Math.floor(hit.x);
    // move left until we see a border-ish column
    while (x > 40) {
      let ink = 0;
      for (let y = yChip0; y < yChip1; y++) if (L(x, y) < borderThr) ink++;
      if (ink > (yChip1 - yChip0) * 0.4) break;
      x--;
    }
    // now on border or past; walk further left while still border
    while (x > 40) {
      let ink = 0;
      for (let y = yChip0; y < yChip1; y++) if (L(x - 1, y) < borderThr) ink++;
      if (ink > (yChip1 - yChip0) * 0.25) x--;
      else break;
    }
    return x;
  }

  const chipL = chipLeftFromText();
  const bodyL = leftmostBorder(yBody0, yBody1, Math.floor(hit.x) + 200);
  const dPx = chipL - bodyL;

  // Profile a few px around both
  const profile = [];
  const x0 = Math.min(chipL, bodyL) - 4;
  for (let x = x0; x < x0 + 20; x++) {
    let cInk = 0,
      bInk = 0;
    for (let y = yChip0; y < yChip1; y++) if (L(x, y) < borderThr) cInk++;
    for (let y = yBody0; y < yBody1; y++) if (L(x, y) < borderThr) bInk++;
    profile.push({ x, cInk, bInk, Lc: +L(x, Math.floor((yChip0 + yChip1) / 2)).toFixed(0) });
  }

  const result = {
    page: pageNum,
    needle,
    hit: { x: +hit.x.toFixed(1), y: +hit.y.toFixed(1), str: hit.str },
    cream: +cream.toFixed(1),
    chipL,
    bodyL,
    dPx,
    dPt: +(dPx / scale).toFixed(3),
    hairlinePx: +(0.75 * scale).toFixed(2),
  };
  console.log(JSON.stringify(result));

  const cx0 = Math.max(0, Math.min(chipL, bodyL) - 8);
  const cx1 = cx0 + 48;
  const cy0 = Math.max(0, yChip0 - 8);
  const cy1 = Math.min(H, yBody1 + 8);
  const z = 4;
  const crop = createCanvas((cx1 - cx0) * z, (cy1 - cy0) * z);
  const cctx = crop.getContext("2d");
  cctx.imageSmoothingEnabled = false;
  cctx.drawImage(canvas, cx0, cy0, cx1 - cx0, cy1 - cy0, 0, 0, (cx1 - cx0) * z, (cy1 - cy0) * z);
  cctx.strokeStyle = "red";
  cctx.beginPath();
  cctx.moveTo((chipL - cx0) * z + 0.5, 0);
  cctx.lineTo((chipL - cx0) * z + 0.5, crop.height);
  cctx.stroke();
  cctx.strokeStyle = "blue";
  cctx.beginPath();
  cctx.moveTo((bodyL - cx0) * z + 0.5, 0);
  cctx.lineTo((bodyL - cx0) * z + 0.5, crop.height);
  cctx.stroke();
  writeFileSync(`${ticket}/${out}`, crop.toBuffer("image/png"));
  writeFileSync(`${ticket}/${out.replace(".png", "-profile.json")}`, JSON.stringify(profile, null, 2));
  return result;
}

const results = [];
for (const row of [
  [78, "Tabelle", "fix-align-bbma.png"],
  [78, "Marktpl", "fix-align-markt.png"],
  [24, "P.K.1", "fix-align-pk1.png"],
  [19, "Risiken", "fix-align-risiken.png"],
  [18, "Meilensteine", "fix-align-meil.png"],
  [76, "Hürden", "fix-align-huerden.png"],
  [122, "Glossar", "fix-align-gloss.png"],
  [2, "Inhaltsverzeichnis", "fix-align-toc.png"],
]) {
  results.push(await measure(row[0], row[1], row[2]));
}
writeFileSync(`${ticket}/chip-left-align2.json`, JSON.stringify(results, null, 2));
