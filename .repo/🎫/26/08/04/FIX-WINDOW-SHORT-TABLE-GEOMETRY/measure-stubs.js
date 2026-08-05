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

  // Find table L/R as dark vertical runs in mid-page
  const midY = Math.floor(H * 0.45);
  const darkCols = [];
  for (let x = 40; x < W - 40; x++) {
    let dark = 0;
    for (let y = midY; y < midY + 80; y++) {
      const i = (y * W + x) * 4;
      if (lum(img[i], img[i + 1], img[i + 2]) < 120) dark++;
    }
    if (dark > 40) darkCols.push(x);
  }
  const L = darkCols[0];
  const R = darkCols[darkCols.length - 1];

  // Scan bottom half for last horizontal rule (many dark pixels across L..R)
  let lastRuleY = -1;
  const ruleYs = [];
  for (let y = Math.floor(H * 0.2); y < H - 80; y++) {
    let dark = 0;
    for (let x = L; x <= R; x++) {
      const i = (y * W + x) * 4;
      const LUM = lum(img[i], img[i + 1], img[i + 2]);
      if (LUM > 90 && LUM < 170) dark++;
    }
    if (dark > (R - L) * 0.45) {
      ruleYs.push(y);
      lastRuleY = y;
    }
  }

  // Count dark pixels on L and R columns in band below last rule
  function stubBelow(x, fromY) {
    let stub = 0;
    for (let y = fromY + 1; y < fromY + Math.ceil(8 * scale); y++) {
      const i = (y * W + x) * 4;
      if (lum(img[i], img[i + 1], img[i + 2]) < 130) stub++;
    }
    return stub;
  }
  function stubAbove(x, fromY) {
    let stub = 0;
    for (let y = fromY - 1; y > fromY - Math.ceil(8 * scale); y--) {
      const i = (y * W + x) * 4;
      if (lum(img[i], img[i + 1], img[i + 2]) < 130) stub++;
    }
    return stub;
  }

  // First strong rule near top (table top under chip)
  const firstRuleY = ruleYs.find((y) => y > 80) ?? -1;

  // Chip: look for dark band above first rule near L
  let chipBot = -1;
  for (let y = firstRuleY - 1; y > 40; y--) {
    let dark = 0;
    for (let x = L; x < L + 200; x++) {
      const i = (y * W + x) * 4;
      if (lum(img[i], img[i + 1], img[i + 2]) < 140) dark++;
    }
    if (dark > 30) {
      chipBot = y;
      break;
    }
  }

  const result = {
    label,
    page: pageNum,
    L,
    R,
    firstRuleY,
    lastRuleY,
    chipBot,
    chipToRuleGapPx: chipBot >= 0 && firstRuleY >= 0 ? firstRuleY - chipBot - 1 : null,
    chipToRuleGapPt:
      chipBot >= 0 && firstRuleY >= 0 ? (firstRuleY - chipBot - 1) / scale : null,
    stubBelowL: stubBelow(L, lastRuleY),
    stubBelowR: stubBelow(R, lastRuleY),
    stubAboveL_first: stubAbove(L, firstRuleY),
    stubAboveR_first: stubAbove(R, firstRuleY),
    ruleCount: ruleYs.length,
  };

  // Crop bottom of table
  const cy0 = Math.max(0, lastRuleY - Math.ceil(40 * scale));
  const cy1 = Math.min(H, lastRuleY + Math.ceil(20 * scale));
  const crop = createCanvas(R - L + 40, cy1 - cy0);
  const cctx = crop.getContext("2d");
  cctx.drawImage(canvas, L - 20, cy0, R - L + 40, cy1 - cy0, 0, 0, R - L + 40, cy1 - cy0);
  writeFileSync(`${ticket}/stub-${label}-bottom.png`, crop.toBuffer("image/png"));

  // Crop top (chip join)
  if (firstRuleY > 0) {
    const ty0 = Math.max(0, firstRuleY - Math.ceil(50 * scale));
    const ty1 = Math.min(H, firstRuleY + Math.ceil(30 * scale));
    const top = createCanvas(R - L + 40, ty1 - ty0);
    const tctx = top.getContext("2d");
    tctx.drawImage(canvas, L - 20, ty0, R - L + 40, ty1 - ty0, 0, 0, R - L + 40, ty1 - ty0);
    writeFileSync(`${ticket}/stub-${label}-top.png`, top.toBuffer("image/png"));
  }

  console.log(JSON.stringify(result, null, 2));
  return result;
}

const results = [];
results.push(await analyze(122, "gloss"));
results.push(await analyze(123, "abk"));
results.push(await analyze(2, "toc"));
writeFileSync(`${ticket}/stub-metrics.json`, JSON.stringify(results, null, 2));
