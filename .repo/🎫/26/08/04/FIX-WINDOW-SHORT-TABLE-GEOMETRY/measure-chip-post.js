import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { createRequire } from "node:module";

const ticket = ".repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);

function lum(d, W, x, y) {
  const i = (y * W + x) * 4;
  return 0.2126 * d[i] + 0.7152 * d[i + 1] + 0.0722 * d[i + 2];
}

async function measure(pdfPath, pageNum, needle, tag) {
  const doc = await pdfjs.getDocument({
    data: new Uint8Array(readFileSync(pdfPath)),
    useSystemFonts: true,
  }).promise;
  const page = await doc.getPage(pageNum);
  const scale = 8;
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
  if (!hit) {
    console.log("[DEBUG] miss", tag, needle);
    return null;
  }
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
  const W = canvas.width,
    H = canvas.height;
  const d = canvas.getContext("2d").getImageData(0, 0, W, H).data;
  const L = (x, y) => lum(d, W, x, y);

  // Theme-aware: light cream~243, dark bg~17
  let bg = 0,
    n = 0;
  for (let x = Math.floor(W / 2); x < Math.floor(W / 2) + 30; x++) {
    bg += L(x, 40);
    n++;
  }
  bg /= n;
  const dark = bg < 80;
  const isBorder = (x, y) => {
    const v = L(x, y);
    return dark ? v > bg + 40 && v < 200 : v < bg - 40;
  };

  const cy = Math.floor(hit.y);
  const yC0 = cy - Math.ceil(12 * scale),
    yC1 = cy + Math.ceil(1 * scale);
  const yB0 = cy + Math.ceil(12 * scale),
    yB1 = cy + Math.ceil(40 * scale);

  function firstBorder(y0, y1, xMax) {
    for (let x = 40; x < xMax; x++) {
      let ink = 0;
      for (let y = y0; y < y1; y++) if (isBorder(x, y)) ink++;
      if (ink > (y1 - y0) * 0.45) return x;
    }
    return -1;
  }
  function borderRun(y0, y1, x0) {
    let x = x0;
    while (x > 40) {
      let ink = 0;
      for (let y = y0; y < y1; y++) if (isBorder(x - 1, y)) ink++;
      if (ink > (y1 - y0) * 0.25) x--;
      else break;
    }
    let e = x0;
    while (e < W - 40) {
      let ink = 0;
      for (let y = y0; y < y1; y++) if (isBorder(e + 1, y)) ink++;
      if (ink > (y1 - y0) * 0.25) e++;
      else break;
    }
    return { s: x, e };
  }

  // chip L from text
  let chipX = Math.floor(hit.x);
  while (chipX > 40) {
    let ink = 0;
    for (let y = yC0; y < yC1; y++) if (isBorder(chipX, y)) ink++;
    if (ink > (yC1 - yC0) * 0.4) break;
    chipX--;
  }
  const chip = borderRun(yC0, yC1, chipX);
  const bodyX = firstBorder(yB0, yB1, Math.floor(hit.x) + 100);
  const body = bodyX >= 0 ? borderRun(yB0, yB1, bodyX) : null;

  const dPx = body ? chip.s - body.s : null;
  const result = {
    tag,
    page: pageNum,
    needle,
    dark,
    bg: +bg.toFixed(1),
    chip,
    body,
    dPx,
    dPt: dPx != null ? +(dPx / scale).toFixed(3) : null,
    hairlinePx: +(0.75 * scale).toFixed(2),
  };
  console.log(JSON.stringify(result));

  if (body) {
    const x0 = Math.min(chip.s, body.s) - 6;
    const x1 = x0 + 36;
    const y0 = yC0 - 4;
    const y1 = yB1 + 4;
    const z = 5;
    const crop = createCanvas((x1 - x0) * z, (y1 - y0) * z);
    const ctx = crop.getContext("2d");
    ctx.imageSmoothingEnabled = false;
    ctx.drawImage(canvas, x0, y0, x1 - x0, y1 - y0, 0, 0, (x1 - x0) * z, (y1 - y0) * z);
    ctx.strokeStyle = "red";
    ctx.beginPath();
    ctx.moveTo((chip.s - x0) * z + 0.5, 0);
    ctx.lineTo((chip.s - x0) * z + 0.5, crop.height);
    ctx.stroke();
    ctx.strokeStyle = "cyan";
    ctx.beginPath();
    ctx.moveTo((body.s - x0) * z + 0.5, 0);
    ctx.lineTo((body.s - x0) * z + 0.5, crop.height);
    ctx.stroke();
    writeFileSync(`${ticket}/post-${tag}.png`, crop.toBuffer("image/png"));
  }
  return result;
}

const light = "mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf";
const dark = "mit-bestand/bericht/zwischenbericht/dist/zwischenbericht-dark.pdf";
const results = [];
for (const pdf of [light, dark]) {
  if (!existsSync(pdf)) {
    console.log("[DEBUG] missing", pdf);
    continue;
  }
  const theme = pdf.includes("dark") ? "dark" : "light";
  for (const [p, n] of [
    [78, "Marktpl"],
    [19, "Risiken"],
    [18, "Meilensteine"],
    [24, "Kopfbau"],
    [2, "Inhaltsverzeichnis"],
    [122, "Glossar"],
  ]) {
    results.push(await measure(pdf, p, n, `${theme}-p${p}`));
  }
}
writeFileSync(`${ticket}/chip-left-post.json`, JSON.stringify(results, null, 2));
