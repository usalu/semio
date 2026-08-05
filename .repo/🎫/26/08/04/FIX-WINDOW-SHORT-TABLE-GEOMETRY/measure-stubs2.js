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

async function analyze(pageNum, label) {
  const page = await doc.getPage(pageNum);
  const scale = 4;
  const viewport = page.getViewport({ scale });
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const ctx = canvas.getContext("2d");
  await page.render({ canvasContext: ctx, viewport }).promise;
  const { width: W, height: H } = canvas;
  const img = ctx.getImageData(0, 0, W, H).data;

  // Sample page cream
  let cream = 0;
  let n = 0;
  for (let y = 20; y < 60; y++) {
    for (let x = Math.floor(W / 2) - 20; x < Math.floor(W / 2) + 20; x++) {
      const i = (y * W + x) * 4;
      cream += lum(img[i], img[i + 1], img[i + 2]);
      n++;
    }
  }
  cream /= n;

  // Find left/right border columns: persistent ink darker than cream-40 across many rows
  const y0 = Math.floor(H * 0.25);
  const y1 = Math.floor(H * 0.7);
  const scores = [];
  for (let x = 20; x < W - 20; x++) {
    let ink = 0;
    for (let y = y0; y < y1; y += 2) {
      const i = (y * W + x) * 4;
      if (lum(img[i], img[i + 1], img[i + 2]) < cream - 35) ink++;
    }
    scores.push({ x, ink });
  }
  scores.sort((a, b) => b.ink - a.ink);
  const candidates = scores.filter((s) => s.ink > (y1 - y0) / 2 / 2 * 0.35);
  candidates.sort((a, b) => a.x - b.x);
  // cluster
  const cols = [];
  for (const c of candidates) {
    const last = cols[cols.length - 1];
    if (!last || c.x - last > 6) cols.push(c.x);
  }
  const L = cols[0];
  const R = cols[cols.length - 1];
  console.log(`[DEBUG] ${label} cream=${cream.toFixed(1)} borderCols=${cols.slice(0, 8)}... L=${L} R=${R}`);

  if (L == null || R == null) {
    writeFileSync(`${ticket}/stub-${label}-full.png`, canvas.toBuffer("image/png"));
    return { label, error: "no borders" };
  }

  // Horizontal rules: rows where many x in [L,R] are border-ish (cream-35..cream-5 or darker grey)
  const ruleYs = [];
  for (let y = Math.floor(H * 0.12); y < H - 100; y++) {
    let hit = 0;
    for (let x = L; x <= R; x++) {
      const i = (y * W + x) * 4;
      const LUM = lum(img[i], img[i + 1], img[i + 2]);
      if (LUM < cream - 25) hit++;
    }
    if (hit > (R - L) * 0.4) ruleYs.push(y);
  }
  // collapse contiguous
  const rules = [];
  for (const y of ruleYs) {
    const last = rules[rules.length - 1];
    if (last && y === last.y1 + 1) last.y1 = y;
    else rules.push({ y0: y, y1: y });
  }
  const firstRule = rules[0];
  const lastRule = rules[rules.length - 1];

  function colInk(x, yFrom, yTo) {
    let c = 0;
    const a = Math.min(yFrom, yTo);
    const b = Math.max(yFrom, yTo);
    for (let y = a; y <= b; y++) {
      const i = (y * W + x) * 4;
      if (lum(img[i], img[i + 1], img[i + 2]) < cream - 35) c++;
    }
    return c;
  }

  const lastY = lastRule?.y1 ?? -1;
  const firstY = firstRule?.y0 ?? -1;
  const stubBelowL = lastY >= 0 ? colInk(L, lastY + 1, lastY + Math.ceil(6 * scale)) : 0;
  const stubBelowR = lastY >= 0 ? colInk(R, lastY + 1, lastY + Math.ceil(6 * scale)) : 0;
  const stubAboveL = firstY >= 0 ? colInk(L, firstY - Math.ceil(6 * scale), firstY - 1) : 0;
  const stubAboveR = firstY >= 0 ? colInk(R, firstY - Math.ceil(6 * scale), firstY - 1) : 0;

  // cream gap inside table: consecutive rules with large cream-only band between
  const internalGaps = [];
  for (let i = 1; i < rules.length; i++) {
    const gapPx = rules[i].y0 - rules[i - 1].y1 - 1;
    const gapPt = gapPx / scale;
    if (gapPt > 14) {
      // sample mid of gap
      const mid = Math.floor((rules[i - 1].y1 + rules[i].y0) / 2);
      let avg = 0;
      let m = 0;
      for (let x = L + 10; x < R - 10; x += 3) {
        const idx = (mid * W + x) * 4;
        avg += lum(img[idx], img[idx + 1], img[idx + 2]);
        m++;
      }
      avg /= m;
      internalGaps.push({
        afterRule: i - 1,
        gapPt: +gapPt.toFixed(2),
        midLum: +avg.toFixed(1),
        isPageCream: avg > cream - 8,
      });
    }
  }

  const result = {
    label,
    page: pageNum,
    cream: +cream.toFixed(1),
    L,
    R,
    ruleBands: rules.length,
    firstRuleY: firstY,
    lastRuleY: lastY,
    stubBelowLpx: stubBelowL,
    stubBelowRpx: stubBelowR,
    stubAboveLpx: stubAboveL,
    stubAboveRpx: stubAboveR,
    internalGaps,
  };

  if (lastY >= 0) {
    const cy0 = Math.max(0, lastY - Math.ceil(35 * scale));
    const cy1 = Math.min(H, lastY + Math.ceil(18 * scale));
    const crop = createCanvas(R - L + 40, cy1 - cy0);
    crop.getContext("2d").drawImage(canvas, L - 20, cy0, R - L + 40, cy1 - cy0, 0, 0, R - L + 40, cy1 - cy0);
    writeFileSync(`${ticket}/stub-${label}-bottom.png`, crop.toBuffer("image/png"));
  }
  if (firstY >= 0) {
    const ty0 = Math.max(0, firstY - Math.ceil(45 * scale));
    const ty1 = Math.min(H, firstY + Math.ceil(25 * scale));
    const top = createCanvas(R - L + 40, ty1 - ty0);
    top.getContext("2d").drawImage(canvas, L - 20, ty0, R - L + 40, ty1 - ty0, 0, 0, R - L + 40, ty1 - ty0);
    writeFileSync(`${ticket}/stub-${label}-top.png`, top.toBuffer("image/png"));
  }

  console.log(JSON.stringify(result, null, 2));
  return result;
}

const results = [];
for (const [p, l] of [
  [122, "gloss"],
  [123, "abk"],
  [2, "toc"],
]) {
  results.push(await analyze(p, l));
}
writeFileSync(`${ticket}/stub-metrics.json`, JSON.stringify(results, null, 2));
