#!/usr/bin/env bun
import { createRequire } from "node:module";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas, loadImage } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const ticket = ".repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";

async function measure(prefix: string, p: number, minY: number) {
  const img = await loadImage(`${ticket}/${prefix}-${String(p).padStart(3, "0")}.png`);
  const w = img.width;
  const h = img.height;
  const c = createCanvas(w, h);
  c.getContext("2d").drawImage(img, 0, 0);
  const { data } = c.getContext("2d").getImageData(0, 0, w, h);
  const L = (x: number, y: number) => {
    const i = (y * w + x) * 4;
    return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
  };
  let top = -1;
  for (let y = minY; y < Math.floor(h * 0.75); y++) {
    let ink = 0;
    for (let x = Math.floor(w * 0.14); x < Math.floor(w * 0.86); x += 2) {
      if (L(x, y) < 130) ink++;
    }
    if (ink > 200) {
      top = y;
      break;
    }
  }
  let bodyL = -1;
  for (let x = Math.floor(w * 0.08); x < Math.floor(w * 0.3); x++) {
    let ink = 0;
    for (let y = top + 8; y < top + 100; y++) if (L(x, y) < 140) ink++;
    if (ink > 60) {
      bodyL = x;
      break;
    }
  }
  let chipL = -1;
  for (let x = Math.floor(w * 0.08); x < Math.floor(w * 0.3); x++) {
    let ink = 0;
    for (let y = Math.max(0, top - 38); y < top - 2; y++) if (L(x, y) < 140) ink++;
    if (ink > 16) {
      chipL = x;
      break;
    }
  }
  let bodyR = -1;
  for (let x = Math.floor(w * 0.92); x > Math.floor(w * 0.68); x--) {
    let ink = 0;
    for (let y = top + 8; y < top + 100; y++) if (L(x, y) < 140) ink++;
    if (ink > 60) {
      bodyR = x;
      break;
    }
  }
  let chipR = -1;
  for (let x = Math.floor(w * 0.92); x > Math.floor(w * 0.68); x--) {
    let ink = 0;
    for (let y = Math.max(0, top - 38); y < top - 2; y++) if (L(x, y) < 140) ink++;
    if (ink > 16) {
      chipR = x;
      break;
    }
  }
  const rules: number[] = [];
  for (let y = top + 15; y < top + 420; y++) {
    let ink = 0;
    for (let x = bodyL + 35; x < bodyL + 260; x += 2) if (L(x, y) < 130) ink++;
    if (ink > 90) rules.push(y);
  }
  const bands: number[] = [];
  if (rules.length) {
    let s = rules[0];
    let p0 = rules[0];
    for (const y of rules.slice(1)) {
      if (y <= p0 + 2) p0 = y;
      else {
        bands.push(s);
        s = y;
        p0 = y;
      }
    }
    bands.push(s);
  }
  let cream = 0;
  let jitter = 0;
  for (const by of bands.slice(0, 6)) {
    for (let y = by - 1; y <= by + 1; y++) if (L(bodyL, y) > 175) cream++;
    const mid = Math.min(h - 1, by + 25);
    let best = bodyL;
    let bL = 999;
    for (let x = bodyL - 2; x <= bodyL + 2; x++) {
      const l = L(x, mid);
      if (l < bL) {
        bL = l;
        best = x;
      }
    }
    jitter = Math.max(jitter, Math.abs(best - bodyL));
  }
  let photoPad: number | null = null;
  if (p === 24) {
    let y = top + 1;
    while (y < top + 50) {
      let dark = 0;
      for (let x = bodyL + 15; x < bodyL + 180; x++) if (L(x, y) < 90) dark++;
      if (dark > 40) {
        photoPad = +((y - top - 1) / 3).toFixed(2);
        break;
      }
      y++;
    }
  }
  return {
    img,
    w,
    top,
    chipL,
    bodyL,
    chipR,
    bodyR,
    dL: bodyL - chipL,
    dR: bodyR - chipR,
    cream,
    jitter,
    photoPad,
  };
}

const specs: Array<[number, number, string]> = [
  [3, 200, "TOC"],
  [18, 200, "Meilensteine (window)"],
  [19, 250, "Risiken (window)"],
  [24, 500, "Kopfbau (project)"],
  [76, 350, "Huerden (long)"],
  [77, 350, "Ueberblick (window)"],
  [78, 200, "Marktplaetze (long)"],
  [121, 200, "Glossar (long)"],
];

const report: any[] = [];
for (const [p, minY, name] of specs) {
  const ref = await measure("ref16", p, minY);
  const cur = await measure("cur16", p, minY);
  report.push({
    name,
    p,
    ref: {
      dL: ref.dL,
      dR: ref.dR,
      cream: ref.cream,
      jitter: ref.jitter,
      photoPadPt: ref.photoPad,
    },
    cur: {
      dL: cur.dL,
      dR: cur.dR,
      cream: cur.cream,
      jitter: cur.jitter,
      photoPadPt: cur.photoPad,
    },
  });
  console.log(
    `[DEBUG] ${name} REF dL/dR/cream/jit=${[ref.dL, ref.dR, ref.cream, ref.jitter].join("/")} CUR=${[cur.dL, cur.dR, cur.cream, cur.jitter].join("/")}` +
      (cur.photoPad != null ? ` padPt=${cur.photoPad}` : ""),
  );

  const bx = Math.min(ref.bodyL, cur.bodyL);
  const hx0 = Math.max(0, bx - 6);
  const hx1 = Math.min(ref.w, bx + 560);
  const hy0r = Math.max(0, ref.top - 48);
  const hy1r = ref.top + 190;
  const hy0c = Math.max(0, cur.top - 48);
  const hy1c = cur.top + 190;
  const hw = hx1 - hx0;
  const hhr = hy1r - hy0r;
  const hhc = hy1c - hy0c;
  const head = createCanvas(hw, hhr + hhc + 12);
  const hc = head.getContext("2d");
  hc.fillStyle = "#c8c8c8";
  hc.fillRect(0, 0, head.width, head.height);
  hc.drawImage(ref.img, hx0, hy0r, hw, hhr, 0, 0, hw, hhr);
  hc.drawImage(cur.img, hx0, hy0c, hw, hhc, 0, hhr + 12, hw, hhc);
  writeFileSync(`${ticket}/FINAL-cmp-p${p}.png`, head.toBuffer("image/png"));
}

writeFileSync(`${ticket}/FINAL-compare-report.json`, JSON.stringify(report, null, 2));

const lines = [
  "# Final compare vs Kinan Zwischenbericht branch",
  "",
  "Reference: commit `16` / sty blob from commit `15` (last clean Kinan sty before continuous-edge pass).",
  "Current: commit `17` working tree (flush chips + inner mid-rules + side pillars).",
  "",
  "Crops: top = REF, bottom = CURRENT (`FINAL-cmp-p*.png`).",
  "",
  "| Table | REF dL | CUR dL | REF dR | CUR dR | REF cream | CUR cream | REF jit | CUR jit |",
  "| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |",
  ...report.map(
    (r) =>
      `| ${r.name} | ${r.ref.dL} | ${r.cur.dL} | ${r.ref.dR} | ${r.cur.dR} | ${r.ref.cream} | ${r.cur.cream} | ${r.ref.jitter} | ${r.cur.jitter} |`,
  ),
  "",
  `- Kopfbau photo pad (pt): REF=${report.find((r) => r.p === 24)?.ref.photoPadPt} CUR=${report.find((r) => r.p === 24)?.cur.photoPadPt}`,
  "",
  "Legend: dL/dR = chip−body outer edge px @216dpi (0=aligned). cream = cream pixels on L border at joins. jitter = max L-border x drift across rows.",
];
writeFileSync(`${ticket}/FINAL-compare.md`, lines.join("\n"));
console.log(lines.join("\n"));
