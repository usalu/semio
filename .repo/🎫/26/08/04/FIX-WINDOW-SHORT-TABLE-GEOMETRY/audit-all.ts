#!/usr/bin/env bun
/**
 * 📐 [DEBUG] Multi-zoom audit of table/window chrome consistency.
 * Measures: chip hairlines, cream gap under chips, L-border join notches,
 * text→border insets (left/top) for first body-ish text under the chip.
 *
 * Usage: bun audit-all.ts <pdf> <out.json> [scaleA,scaleB,...]
 */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const [pdfPath, outPath, scalesArg] = process.argv.slice(2);
const scales = (scalesArg ?? "3,6,9").split(",").map(Number);

type Sample = {
  id: string;
  page: number;
  phrase: string;
  kind: "window-table" | "long-table" | "project" | "toc" | "glossary" | "heading-window";
};

const samples: Sample[] = [
  { id: "toc", page: 3, phrase: "Inhaltsverzeichnis", kind: "toc" },
  { id: "meilensteine", page: 17, phrase: "Meilensteine", kind: "window-table" },
  { id: "risiken", page: 19, phrase: "Risiken und Maßnahmen", kind: "window-table" },
  { id: "kopfbau", page: 24, phrase: "Kopfbau Halle 118", kind: "project" },
  { id: "huerden", page: 76, phrase: "Hürden", kind: "long-table" },
  { id: "ueberblick", page: 77, phrase: "Überblick", kind: "long-table" },
  { id: "markt", page: 78, phrase: "Marktplätze · Zugang und Kanäle", kind: "long-table" },
  { id: "glossar", page: 121, phrase: "Glossar", kind: "glossary" },
];

const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({
  data: new Uint8Array(readFileSync(pdfPath)),
  useSystemFonts: true,
}).promise;

const TARGET_INSET_PT = 5.5; // bodypad / visible cell air
const HAIRLINE_PT_MAX = 1.1; // single hairline + AA
const TOL_PT = 1.25;

function analyze(
  data: Uint8ClampedArray,
  W: number,
  H: number,
  scale: number,
  anchor: { x: number; y: number },
) {
  const rgb = (x: number, y: number) => {
    const i = (y * W + x) * 4;
    return [data[i], data[i + 1], data[i + 2]] as const;
  };
  const lum = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  };
  const isRule = (x: number, y: number) => {
    const L = lum(x, y);
    return L > 45 && L < 155;
  };
  const isCream = (x: number, y: number) => lum(x, y) > 200;
  const isInk = (x: number, y: number) => lum(x, y) < 90;
  const isPhoto = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.max(r, g, b) - Math.min(r, g, b) > 40;
  };

  const xL = Math.max(8, Math.floor(anchor.x - 60));
  const xR = Math.min(W - 8, Math.floor(anchor.x + 1000));

  // baseline under chip text
  let baselineY = -1;
  const ruleClusters: { y0: number; y1: number }[] = [];
  for (let y = Math.floor(anchor.y) + 1; y < Math.min(H - 4, Math.floor(anchor.y) + Math.round(90 * scale)); y++) {
    let n = 0;
    for (let x = xL; x < xR; x += 2) if (isRule(x, y)) n++;
    if (n > ((xR - xL) / 2) * 0.32) {
      const last = ruleClusters[ruleClusters.length - 1];
      if (last && y <= last.y1 + 2) last.y1 = y;
      else ruleClusters.push({ y0: y, y1: y });
    }
  }
  // first cluster under text = chip baseline
  if (ruleClusters.length) {
    baselineY = ruleClusters[0].y0;
  }

  // second hairline within 3pt?
  let doubleHairline = false;
  let hairlinePt = 0;
  if (ruleClusters.length) {
    const c0 = ruleClusters[0];
    hairlinePt = (c0.y1 - c0.y0 + 1) / scale;
    for (let i = 1; i < ruleClusters.length; i++) {
      const gapPt = (ruleClusters[i].y0 - c0.y1 - 1) / scale;
      if (gapPt >= 0 && gapPt < 3.5) {
        doubleHairline = true;
        break;
      }
      if (gapPt >= 3.5) break; // next section rule, stop
    }
  }

  // left border under baseline
  let borderX = xL;
  let best = 0;
  const yScan0 = baselineY > 0 ? baselineY : Math.floor(anchor.y);
  for (let x = Math.max(4, xL - 40); x < xL + 100; x++) {
    let n = 0;
    for (let y = yScan0; y < Math.min(H, yScan0 + Math.round(420 * scale)); y++) {
      if (isRule(x, y)) n++;
    }
    if (n > best) {
      best = n;
      borderX = x;
    }
  }

  // cream gap under baseline (page-like = matches margin left of border)
  let creamGapPx = 0;
  if (baselineY > 0) {
    for (let y = baselineY + (ruleClusters[0]?.y1 ?? baselineY) - baselineY + 1; y < baselineY + Math.round(40 * scale); y++) {
      // start after hairline cluster
      break;
    }
    const yAfter = (ruleClusters[0]?.y1 ?? baselineY) + 1;
    for (let y = yAfter; y < yAfter + Math.round(36 * scale); y++) {
      let pageish = 0;
      let canvasish = 0;
      for (let x = borderX + 12; x < borderX + 220; x += 2) {
        const [r, g, b] = rgb(x, y);
        const margin = rgb(Math.max(0, borderX - 12), y);
        if (
          Math.abs(r - margin[0]) < 10 &&
          Math.abs(g - margin[1]) < 10 &&
          Math.abs(b - margin[2]) < 10 &&
          lum(x, y) > 200
        ) {
          pageish++;
        }
        // canvas inside table often same RGB as page — treat continuous cream as OK weld if border continues
        if (isCream(x, y)) canvasish++;
      }
      // only count as BAD gap if border also broken (page showing through frame)
      const borderBroken = !isRule(borderX, y) && isCream(borderX, y);
      if (borderBroken && pageish > 40) creamGapPx++;
      else break;
    }
  }

  // join notches: cream on border sandwiched by rule
  const notches: number[] = [];
  if (baselineY > 0) {
    const y1 = Math.min(H - 3, baselineY + Math.round(700 * scale));
    for (let y = baselineY + Math.round(8 * scale); y < y1; y++) {
      if (isCream(borderX, y) && isRule(borderX, y - 2) && isRule(borderX, y + 2)) {
        notches.push(y);
      }
    }
  }
  const notchClusters: { y0: number; y1: number }[] = [];
  for (const y of notches) {
    const last = notchClusters[notchClusters.length - 1];
    if (last && y <= last.y1 + 3) last.y1 = y;
    else notchClusters.push({ y0: y, y1: y });
  }

  // text insets: first ink below baseline in content area
  let leftInsetPt: number | null = null;
  let topInsetPt: number | null = null;
  let photoPadPt: number | null = null;
  if (baselineY > 0) {
    const yAfter = (ruleClusters[0]?.y1 ?? baselineY) + 1;
    // photo pad
    for (let y = yAfter; y < yAfter + Math.round(80 * scale); y++) {
      let photo = 0;
      for (let x = borderX + 10; x < borderX + 220; x++) if (isPhoto(x, y)) photo++;
      if (photo > 25) {
        photoPadPt = +((y - yAfter) / scale).toFixed(2);
        break;
      }
    }
    // first text ink (dark, not photo)
    let inkY = -1;
    let inkX = -1;
    for (let y = yAfter; y < yAfter + Math.round(100 * scale); y++) {
      for (let x = borderX + 4; x < borderX + Math.round(280 * scale); x++) {
        if (isInk(x, y) && !isPhoto(x, y)) {
          inkY = y;
          inkX = x;
          break;
        }
      }
      if (inkY >= 0) break;
    }
    if (inkY >= 0) {
      topInsetPt = +((inkY - yAfter) / scale).toFixed(2);
      // walk left from ink to border
      let x = inkX;
      while (x > borderX && !isRule(x, inkY)) x--;
      leftInsetPt = +((inkX - borderX - 1) / scale).toFixed(2);
    }
  }

  return {
    baselineY,
    hairlinePt: +hairlinePt.toFixed(2),
    doubleHairline,
    creamGapPt: +(creamGapPx / scale).toFixed(2),
    joinNotches: notchClusters.length,
    leftInsetPt,
    topInsetPt,
    photoPadPt,
    borderX,
    ruleClustersUnderChip: ruleClusters.slice(0, 3),
  };
}

const report: Record<string, unknown>[] = [];

for (const sample of samples) {
  if (sample.page > doc.numPages) {
    report.push({ ...sample, error: "page out of range" });
    continue;
  }
  const page = await doc.getPage(sample.page);
  const content = await page.getTextContent();
  const items = content.items as { str: string; transform: number[] }[];

  for (const scale of scales) {
    const viewport = page.getViewport({ scale });
    // find best text match
    let anchor: { x: number; y: number; str: string } | null = null;
    for (const item of items) {
      if (!item.str) continue;
      if (item.str.includes(sample.phrase) || sample.phrase.split(/\s+/).every((w) => item.str.includes(w) || sample.phrase.includes(item.str))) {
        if (item.str.length < 3) continue;
        const [x, y] = viewport.convertToViewportPoint(item.transform[4], item.transform[5]);
        // prefer longer / fuller matches
        if (!anchor || item.str.length > anchor.str.length) {
          // skip navbar-ish top matches when a lower fuller title exists
          anchor = { x, y, str: item.str };
        }
      }
    }
    // refine: among matches containing first word, pick the one closest to mid-page with longest str
    const word0 = sample.phrase.split(/\s+/)[0]!.slice(0, 6);
    const cands = items
      .filter((it) => it.str && it.str.includes(word0))
      .map((it) => {
        const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
        return { x, y, str: it.str };
      })
      .filter((c) => c.y > 40 * scale && c.y < viewport.height - 40 * scale);
    if (cands.length) {
      cands.sort((a, b) => b.str.length - a.str.length || a.y - b.y);
      // for markt, prefer full title
      const preferred =
        cands.find((c) => c.str.includes("Zugang")) ||
        cands.find((c) => c.str.includes(sample.phrase.slice(0, 10))) ||
        cands[0];
      anchor = preferred;
    }

    if (!anchor) {
      report.push({ id: sample.id, scale, page: sample.page, error: "no text anchor" });
      continue;
    }

    const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
    const ctx = canvas.getContext("2d");
    await page.render({ canvasContext: ctx, viewport }).promise;
    const img = ctx.getImageData(0, 0, canvas.width, canvas.height);
    const m = analyze(img.data, canvas.width, canvas.height, scale, anchor);

    const fail: string[] = [];
    if (m.doubleHairline) fail.push("double-hairline");
    if (m.hairlinePt > HAIRLINE_PT_MAX) fail.push(`hairline-thick:${m.hairlinePt}`);
    if (m.creamGapPt > 0.35) fail.push(`cream-gap:${m.creamGapPt}`);
    if (m.joinNotches > 0) fail.push(`join-notches:${m.joinNotches}`);
    if (m.leftInsetPt != null && Math.abs(m.leftInsetPt - TARGET_INSET_PT) > TOL_PT + 2) {
      // left inset includes chrome pad + cellpad (~ chrome+cell); allow wider band 3.5–9
      if (m.leftInsetPt < 3.2 || m.leftInsetPt > 10.5) fail.push(`left-inset:${m.leftInsetPt}`);
    }
    if (m.topInsetPt != null && (m.topInsetPt < 3.0 || m.topInsetPt > 12)) {
      fail.push(`top-inset:${m.topInsetPt}`);
    }
    if (sample.kind === "project" && m.photoPadPt != null) {
      if (Math.abs(m.photoPadPt - TARGET_INSET_PT) > 1.0) fail.push(`photo-pad:${m.photoPadPt}`);
    }

    report.push({
      id: sample.id,
      kind: sample.kind,
      page: sample.page,
      scale,
      anchor: anchor.str,
      ...m,
      ok: fail.length === 0,
      fail,
    });
    console.log(
      `[DEBUG] ${sample.id}@${scale}x page${sample.page} ok=${fail.length === 0} hair=${m.hairlinePt} gap=${m.creamGapPt} notches=${m.joinNotches} L=${m.leftInsetPt} T=${m.topInsetPt} photo=${m.photoPadPt} ${fail.join(",")}`,
    );
  }
}

writeFileSync(outPath, JSON.stringify(report, null, 2));
const bad = report.filter((r) => r && (r as { ok?: boolean }).ok === false);
console.log(`[DEBUG] done: ${report.length} rows, ${bad.length} failures → ${outPath}`);
