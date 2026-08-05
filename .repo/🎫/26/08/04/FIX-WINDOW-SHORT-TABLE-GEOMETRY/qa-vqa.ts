#!/usr/bin/env bun
/** 🔬 [DEBUG] Visual QA: crop seam/join/cell + RGB metrics at dpi 144 & 432. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const pdfPath =
  process.argv[2] ??
  "e:/semio/mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf";
const outDir =
  process.argv[3] ??
  "e:/semio/.repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";

const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({
  data: new Uint8Array(readFileSync(pdfPath)),
  useSystemFonts: true,
}).promise;

type Target = {
  id: string;
  page: number;
  phrase: string;
  kind: "project" | "long-table" | "glossary";
};

const targets: Target[] = [
  { id: "kopfbau", page: 24, phrase: "Kopfbau Halle 118", kind: "project" },
  { id: "huerden", page: 76, phrase: "Hürden", kind: "long-table" },
  { id: "markt", page: 78, phrase: "Marktplätze · Zugang und Kanäle", kind: "long-table" },
  { id: "glossar", page: 121, phrase: "Glossar", kind: "glossary" },
];

const dpiList = [144, 432];

const isRuleL = (L: number) => L > 50 && L < 155;
const isCreamL = (L: number) => L > 200;
const isInkL = (L: number) => L < 85;

function findAnchor(
  items: { str: string; transform: number[] }[],
  viewport: { convertToViewportPoint: (x: number, y: number) => number[] },
  phrase: string,
  pageH: number,
  scale: number,
) {
  const words = phrase.split(/\s+/);
  const cands = items
    .filter((it) => it.str && it.str.length >= 3)
    .map((it) => {
      const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
      let score = 0;
      if (it.str.includes(phrase)) score += 100 + it.str.length;
      else {
        for (const w of words) if (it.str.includes(w)) score += 10 + w.length;
      }
      // prefer shorter title-like strings near mid-upper page
      if (it.str.length <= phrase.length + 8) score += 20;
      if (y > 50 * scale && y < pageH * 0.55) score += 15;
      return { x, y, str: it.str, score };
    })
    .filter((c) => c.score >= 10)
    .sort((a, b) => b.score - a.score || a.y - b.y);
  // Hürden: prefer exact short chip title
  if (phrase === "Hürden") {
    const exact = cands.find((c) => c.str.trim() === "Hürden" || c.str.includes("Hürden") && c.str.length < 20);
    if (exact) return exact;
  }
  if (phrase.startsWith("Marktplätze")) {
    const exact = cands.find((c) => c.str.includes("Zugang"));
    if (exact) return exact;
  }
  if (phrase === "Glossar") {
    const exact = cands.find((c) => c.str.trim() === "Glossar" || (c.str.includes("Glossar") && c.str.length < 16));
    if (exact) return exact;
  }
  return cands[0] ?? null;
}

function analyze(
  data: Uint8ClampedArray,
  W: number,
  H: number,
  scale: number,
  anchor: { x: number; y: number },
  kind: Target["kind"],
) {
  const rgb = (x: number, y: number) => {
    const i = (y * W + x) * 4;
    return [data[i], data[i + 1], data[i + 2]] as const;
  };
  const lum = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  };
  const isPhoto = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.max(r, g, b) - Math.min(r, g, b) > 40;
  };
  const isCanvas = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.abs(r - 240) + Math.abs(g - 236) + Math.abs(b - 221) <= 18;
  };
  const isPage = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.abs(r - 247) + Math.abs(g - 243) + Math.abs(b - 227) <= 16;
  };

  const xL = Math.max(8, Math.floor(anchor.x - 80));
  const xR = Math.min(W - 8, Math.floor(anchor.x + 1100));

  const ruleClusters: { y0: number; y1: number }[] = [];
  for (let y = Math.floor(anchor.y) + 1; y < Math.min(H - 4, Math.floor(anchor.y) + Math.round(100 * scale)); y++) {
    let n = 0;
    for (let x = xL; x < xR; x += 2) if (isRuleL(lum(x, y))) n++;
    if (n > ((xR - xL) / 2) * 0.30) {
      const last = ruleClusters[ruleClusters.length - 1];
      if (last && y <= last.y1 + 2) last.y1 = y;
      else ruleClusters.push({ y0: y, y1: y });
    }
  }

  let baselineY = ruleClusters[0]?.y0 ?? -1;
  let hairlinePt = 0;
  let doubleHairline = false;
  if (ruleClusters.length) {
    const c0 = ruleClusters[0];
    hairlinePt = +((c0.y1 - c0.y0 + 1) / scale).toFixed(2);
    for (let i = 1; i < ruleClusters.length; i++) {
      const gapPt = (ruleClusters[i].y0 - c0.y1 - 1) / scale;
      if (gapPt >= 0 && gapPt < 2.5) {
        doubleHairline = true;
        break;
      }
      if (gapPt >= 2.5) break;
    }
  }

  let borderX = xL;
  let best = 0;
  const yScan0 = baselineY > 0 ? baselineY : Math.floor(anchor.y);
  for (let x = Math.max(4, xL - 50); x < xL + 120; x++) {
    let n = 0;
    for (let y = yScan0; y < Math.min(H, yScan0 + Math.round(500 * scale)); y++) {
      if (isRuleL(lum(x, y))) n++;
    }
    if (n > best) {
      best = n;
      borderX = x;
    }
  }

  const yAfter = (ruleClusters[0]?.y1 ?? baselineY) + 1;

  // cream gap: page cream under baseline with broken border
  let creamGapPx = 0;
  if (baselineY > 0) {
    for (let y = yAfter; y < yAfter + Math.round(24 * scale); y++) {
      const borderBroken = !isRuleL(lum(borderX, y)) && (isCreamL(lum(borderX, y)) || isPage(borderX, y));
      let pageish = 0;
      for (let x = borderX + 10; x < borderX + 180; x += 2) {
        if (isPage(x, y) && isPage(Math.max(0, borderX - 12), y)) pageish++;
      }
      if (borderBroken && pageish > 30) creamGapPx++;
      else break;
    }
  }

  // weld check: first row under hairline should be canvas or photo, not page through frame
  let weldOk = true;
  let weldSample: string | null = null;
  if (baselineY > 0) {
    const y = yAfter;
    let pageN = 0;
    let canvasN = 0;
    let photoN = 0;
    for (let x = borderX + 8; x < borderX + 200; x++) {
      if (isPhoto(x, y)) photoN++;
      else if (isCanvas(x, y)) canvasN++;
      else if (isPage(x, y)) pageN++;
    }
    weldSample = `y${y} page=${pageN} canvas=${canvasN} photo=${photoN} borderL=${lum(borderX, y).toFixed(0)}`;
    // cream gap to body = page cream with broken L border
    if (creamGapPx > 0) weldOk = false;
  }

  // join notches
  const notches: number[] = [];
  if (baselineY > 0) {
    const y1 = Math.min(H - 3, baselineY + Math.round(650 * scale));
    for (let y = baselineY + Math.round(6 * scale); y < y1; y++) {
      if (isCreamL(lum(borderX, y)) && isRuleL(lum(borderX, y - 2)) && isRuleL(lum(borderX, y + 2))) {
        notches.push(y);
      }
    }
  }
  const notchClusters: { y0: number; y1: number; pt: number }[] = [];
  for (const y of notches) {
    const last = notchClusters[notchClusters.length - 1];
    if (last && y <= last.y1 + 3) last.y1 = y;
    else notchClusters.push({ y0: y, y1: y, pt: 0 });
  }
  for (const c of notchClusters) c.pt = +((c.y1 - c.y0 + 1) / scale).toFixed(2);

  // join samples at H-rules
  const joinSamples: { y: number; borderL: number; kind: string }[] = [];
  if (baselineY > 0) {
    for (let y = yAfter + Math.round(10 * scale); y < Math.min(H - 2, baselineY + Math.round(500 * scale)); y++) {
      let midRule = 0;
      for (let x = borderX + 40; x < borderX + 200; x++) if (isRuleL(lum(x, y))) midRule++;
      if (midRule < 40) continue;
      const L = lum(borderX, y);
      const k = isRuleL(L) ? "RULE" : isCreamL(L) ? "CREAM" : "OTHER";
      joinSamples.push({ y, borderL: +L.toFixed(1), kind: k });
      y += Math.round(8 * scale);
    }
  }

  // photo pad
  let photoPadPt: number | null = null;
  if (baselineY > 0 && kind === "project") {
    for (let y = yAfter; y < yAfter + Math.round(80 * scale); y++) {
      let photo = 0;
      for (let x = borderX + 8; x < borderX + 220; x++) if (isPhoto(x, y)) photo++;
      if (photo > 30) {
        photoPadPt = +((y - yAfter) / scale).toFixed(2);
        break;
      }
    }
  }

  // text insets: first dark ink that is NOT photo, within left content column
  let leftInsetPt: number | null = null;
  let topInsetPt: number | null = null;
  let firstInk: { x: number; y: number } | null = null;
  if (baselineY > 0) {
    const xMax = borderX + Math.round(120 * scale); // first cell only
    for (let y = yAfter; y < yAfter + Math.round(90 * scale); y++) {
      for (let x = borderX + 3; x < xMax; x++) {
        if (isInkL(lum(x, y)) && !isPhoto(x, y) && !isRuleL(lum(x, y))) {
          // require a few neighbors to avoid noise
          let n = 0;
          for (let dx = 0; dx < 6; dx++) if (isInkL(lum(x + dx, y))) n++;
          if (n < 3) continue;
          firstInk = { x, y };
          break;
        }
      }
      if (firstInk) break;
    }
    if (firstInk) {
      topInsetPt = +((firstInk.y - yAfter) / scale).toFixed(2);
      leftInsetPt = +((firstInk.x - borderX - 1) / scale).toFixed(2);
    }
  }

  // hairline count under chip: distinct clusters within 3pt of first
  let hairlineCount = ruleClusters.length ? 1 : 0;
  if (ruleClusters.length > 1) {
    const c0 = ruleClusters[0];
    for (let i = 1; i < ruleClusters.length; i++) {
      const gapPt = (ruleClusters[i].y0 - c0.y1 - 1) / scale;
      if (gapPt < 2.5) hairlineCount++;
      else break;
    }
  }

  return {
    baselineY,
    borderX,
    yAfter,
    hairlinePt,
    hairlineCount,
    doubleHairline,
    creamGapPx,
    creamGapPt: +(creamGapPx / scale).toFixed(2),
    weldOk,
    weldSample,
    joinNotches: notchClusters.length,
    notchClusters: notchClusters.slice(0, 12),
    joinSamples: joinSamples.slice(0, 8),
    leftInsetPt,
    topInsetPt,
    photoPadPt,
    firstInk,
    ruleClustersUnderChip: ruleClusters.slice(0, 4),
  };
}

const crop = (
  canvas: ReturnType<typeof createCanvas>,
  name: string,
  x0: number,
  y0: number,
  w: number,
  h: number,
) => {
  const c = createCanvas(w, h);
  c.getContext("2d").drawImage(canvas, x0, y0, w, h, 0, 0, w, h);
  const p = `${outDir}/${name}.png`;
  writeFileSync(p, c.toBuffer("image/png"));
  return p;
};

const report: Record<string, unknown>[] = [];

for (const dpi of dpiList) {
  const scale = dpi / 72;
  for (const t of targets) {
    const page = await doc.getPage(t.page);
    const viewport = page.getViewport({ scale });
    const content = await page.getTextContent();
    const items = content.items as { str: string; transform: number[] }[];
    const anchor = findAnchor(items, viewport, t.phrase, viewport.height, scale);
    if (!anchor) {
      report.push({ id: t.id, dpi, scale, error: "no anchor" });
      continue;
    }
    const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
    const ctx = canvas.getContext("2d");
    await page.render({ canvasContext: ctx, viewport }).promise;
    const img = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const m = analyze(img.data, canvas.width, canvas.height, scale, anchor, t.kind);

    const fail: string[] = [];
    if (m.hairlineCount !== 1) fail.push(`hairlines:${m.hairlineCount}`);
    if (m.doubleHairline) fail.push("double-hairline");
    if (m.hairlinePt > 1.1) fail.push(`hairline-thick:${m.hairlinePt}`);
    if (m.creamGapPt > 0.2) fail.push(`cream-gap:${m.creamGapPt}`);
    if (!m.weldOk) fail.push("weld-broken");
    if (m.joinNotches > 0) fail.push(`join-notches:${m.joinNotches}`);
    // join samples must be RULE
    const creamJoins = m.joinSamples.filter((j) => j.kind === "CREAM");
    if (creamJoins.length) fail.push(`cream-joins:${creamJoins.length}`);
    if (m.leftInsetPt != null && (m.leftInsetPt < 3.5 || m.leftInsetPt > 10.5)) {
      fail.push(`left-inset:${m.leftInsetPt}`);
    }
    if (m.topInsetPt != null && (m.topInsetPt < 3.0 || m.topInsetPt > 12.5)) {
      fail.push(`top-inset:${m.topInsetPt}`);
    }
    if (t.kind === "project") {
      if (m.photoPadPt == null) fail.push("photo-pad:missing");
      else if (Math.abs(m.photoPadPt - 5.5) > 1.0) fail.push(`photo-pad:${m.photoPadPt}`);
    }

    const tag = `${t.id}-d${dpi}`;
    const crops: Record<string, string | null> = { seam: null, join: null, cell: null, chip: null };
    if (m.baselineY > 0) {
      const bx = Math.max(0, m.borderX - 10);
      crops.seam = crop(canvas, `vqa-${tag}-seam`, bx, m.baselineY - Math.round(28 * scale), Math.round(220 * scale), Math.round(70 * scale));
      crops.join = crop(canvas, `vqa-${tag}-join`, bx, m.baselineY + Math.round(20 * scale), Math.round(90 * scale), Math.round(160 * scale));
      crops.cell = crop(canvas, `vqa-${tag}-cell`, bx, m.yAfter, Math.round(160 * scale), Math.round(90 * scale));
      crops.chip = crop(
        canvas,
        `vqa-${tag}-chip`,
        Math.max(0, Math.floor(anchor.x) - Math.round(10 * scale)),
        Math.max(0, Math.floor(anchor.y) - Math.round(14 * scale)),
        Math.round(220 * scale),
        Math.round(40 * scale),
      );
    }

    const row = {
      id: t.id,
      kind: t.kind,
      page: t.page,
      dpi,
      scale,
      anchor: anchor.str,
      ...m,
      crops,
      ok: fail.length === 0,
      fail,
    };
    report.push(row);
    console.log(
      `[DEBUG] ${t.id}@${dpi} ok=${fail.length === 0} hair=${m.hairlineCount}x${m.hairlinePt}pt gap=${m.creamGapPt} notches=${m.joinNotches} L=${m.leftInsetPt} T=${m.topInsetPt} photo=${m.photoPadPt} joins=${m.joinSamples.map((j) => j.kind).join("|")} ${fail.join(",")}`,
    );
  }
}

const outJson = `${outDir}/qa-vqa-report.json`;
writeFileSync(outJson, JSON.stringify(report, null, 2));
const bad = report.filter((r) => (r as { ok?: boolean }).ok === false);
console.log(`[DEBUG] done ${report.length} rows, ${bad.length} fail → ${outJson}`);
