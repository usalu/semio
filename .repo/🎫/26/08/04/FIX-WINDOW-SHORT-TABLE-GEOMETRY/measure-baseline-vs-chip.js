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

async function check(pageNum, needle) {
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
  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  await page.render({ canvasContext: canvas.getContext("2d"), viewport }).promise;
  const W = canvas.width,
    H = canvas.height;
  const d = canvas.getContext("2d").getImageData(0, 0, W, H).data;
  const L = (x, y) => lum(d, W, x, y);
  let cream = 0,
    n = 0;
  for (let x = Math.floor(W / 2); x < Math.floor(W / 2) + 30; x++) {
    cream += L(x, 40);
    n++;
  }
  cream /= n;
  const thr = cream - 40;
  const cy = Math.floor(hit.y);

  // Find full-width horizontal rules near chip
  const rules = [];
  for (let y = cy - Math.ceil(40 * scale); y < cy + Math.ceil(30 * scale); y++) {
    if (y < 0 || y >= H) continue;
    let mid = 0;
    for (let x = Math.floor(W * 0.15); x < Math.floor(W * 0.85); x += 2) {
      if (L(x, y) < thr) mid++;
    }
    if (mid > ((W * 0.7) / 2) * 0.4) rules.push(y);
  }
  const bands = [];
  for (const y of rules) {
    const last = bands[bands.length - 1];
    if (last && y === last.y1 + 1) last.y1 = y;
    else bands.push({ y0: y, y1: y });
  }

  function ruleLeft(y0, y1) {
    const y = Math.floor((y0 + y1) / 2);
    for (let x = 40; x < W / 2; x++) {
      if (L(x, y) < thr) return x;
    }
    return -1;
  }
  function ruleRight(y0, y1) {
    const y = Math.floor((y0 + y1) / 2);
    for (let x = W - 40; x > W / 2; x--) {
      if (L(x, y) < thr) return x;
    }
    return -1;
  }

  // Chip left: near hit text
  const yChip = cy - Math.ceil(8 * scale);
  let chipL = -1;
  for (let x = Math.floor(hit.x); x > 40; x--) {
    if (L(x, yChip) < thr) {
      chipL = x;
    } else if (chipL > 0) break;
  }
  while (chipL > 40 && L(chipL - 1, yChip) < thr) chipL--;

  const info = bands.slice(0, 5).map((b, i) => ({
    i,
    y0: b.y0,
    y1: b.y1,
    L: ruleLeft(b.y0, b.y1),
    R: ruleRight(b.y0, b.y1),
    wPt: +((ruleRight(b.y0, b.y1) - ruleLeft(b.y0, b.y1)) / scale).toFixed(2),
  }));

  // body L below
  const yBody = cy + Math.ceil(20 * scale);
  let bodyL = -1;
  for (let x = 40; x < hit.x; x++) {
    let ink = 0;
    for (let y = yBody; y < yBody + Math.ceil(20 * scale); y++) if (L(x, y) < thr) ink++;
    if (ink > 10) {
      bodyL = x;
      break;
    }
  }

  console.log(
    JSON.stringify({
      page: pageNum,
      needle,
      cream: +cream.toFixed(1),
      chipL,
      bodyL,
      dChipBody: chipL - bodyL,
      dChipBodyPt: +((chipL - bodyL) / scale).toFixed(3),
      rules: info,
      dChipVsFirstFullRule: info[0] ? chipL - info[0].L : null,
      dChipVsFirstFullRulePt: info[0] ? +((chipL - info[0].L) / scale).toFixed(3) : null,
    }),
  );
}

for (const [p, n] of [
  [78, "Marktpl"],
  [19, "Risiken"],
  [18, "Meilensteine"],
  [24, "Kopfbau"],
  [2, "Inhaltsverzeichnis"],
  [77, "Bauteilbörsen"],
]) {
  try {
    await check(p, n);
  } catch (e) {
    console.log("[DEBUG] fail", p, n, String(e));
  }
}
