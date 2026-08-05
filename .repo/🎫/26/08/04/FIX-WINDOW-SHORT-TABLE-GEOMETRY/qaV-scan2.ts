#!/usr/bin/env bun
/**
 * 🔬 [DEBUG] Tight Visual QA for TOC / Meilensteine / Risiken window tables.
 * Usage: bun qaV-scan2.ts <pdf> [scale]
 */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const pdfPath = process.argv[2]!;
const scale = Number(process.argv[3] ?? 6);
const ticketDir = dirname(fileURLToPath(import.meta.url));
const tag = `qaV2-s${scale}`;

const samples = [
  { id: "toc", page: 3, phrase: "Inhaltsverzeichnis", bodyPhrase: "Schlüssel" },
  { id: "meilensteine", page: 18, phrase: "Meilensteine", bodyPhrase: "Meilenstein" },
  { id: "risiken", page: 19, phrase: "Risiken und Maßnahmen", bodyPhrase: "Risiko" },
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

const PAGE = { r: 247, g: 243, b: 227 };
const CANVAS = { r: 240, g: 236, b: 221 };

function near(a: readonly [number, number, number], t: { r: number; g: number; b: number }, tol = 14) {
  return Math.abs(a[0] - t.r) + Math.abs(a[1] - t.g) + Math.abs(a[2] - t.b) <= tol;
}

const reports: Record<string, unknown>[] = [];

for (const sample of samples) {
  const page = await doc.getPage(sample.page);
  const viewport = page.getViewport({ scale });
  const content = await page.getTextContent();
  const items = (content.items as { str: string; transform: number[] }[])
    .filter((it) => it.str)
    .map((it) => {
      const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
      return { x, y, str: it.str };
    });

  const titleCands = items
    .filter((c) => c.str.includes(sample.phrase.slice(0, 8)) || c.str === sample.phrase)
    .filter((c) => c.y > 40 * scale && c.y < viewport.height * 0.55);
  titleCands.sort((a, b) => (b.str === sample.phrase ? 1 : 0) - (a.str === sample.phrase ? 1 : 0) || b.str.length - a.str.length || a.y - b.y);
  const anchor = titleCands.find((c) => c.str.includes(sample.phrase)) || titleCands[0];
  if (!anchor) {
    reports.push({ id: sample.id, error: "no title anchor", page: sample.page });
    console.log(`[DEBUG] ${sample.id}: no title`);
    continue;
  }

  const canvas = createCanvas(Math.ceil(viewport.width), Math.ceil(viewport.height));
  const ctx = canvas.getContext("2d");
  await page.render({ canvasContext: ctx, viewport }).promise;
  const W = canvas.width;
  const H = canvas.height;
  const data = ctx.getImageData(0, 0, W, H).data;
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
    const [r, g, b] = rgb(x, y);
    return L > 45 && L < 175 && Math.abs(r - g) < 45 && Math.abs(g - b) < 45;
  };
  const isInk = (x: number, y: number) => lum(x, y) < 95;
  const kindAt = (x: number, y: number) => {
    const c = rgb(x, y);
    if (isRule(x, y)) return "RULE";
    if (near(c, PAGE)) return "PAGE";
    if (near(c, CANVAS)) return "CANVAS";
    if (lum(x, y) < 85) return "INK";
    return "OTHER";
  };

  const xL = Math.max(8, Math.floor(anchor.x - 40 * scale));
  const xR = Math.min(W - 8, Math.floor(anchor.x + 200 * scale));

  const ruleClusters: { y0: number; y1: number }[] = [];
  for (let y = Math.floor(anchor.y) + 1; y < Math.min(H - 4, Math.floor(anchor.y) + Math.round(50 * scale)); y++) {
    let n = 0;
    for (let x = xL; x < xR; x += 2) if (isRule(x, y)) n++;
    if (n > ((xR - xL) / 2) * 0.25) {
      const last = ruleClusters[ruleClusters.length - 1];
      if (last && y <= last.y1 + 2) last.y1 = y;
      else ruleClusters.push({ y0: y, y1: y });
    }
  }
  const baselineY = ruleClusters[0]?.y0 ?? -1;
  const hairlineY1 = ruleClusters[0]?.y1 ?? -1;
  const hairlinePt = baselineY >= 0 ? +((hairlineY1 - baselineY + 1) / scale).toFixed(2) : 0;

  let doubleHairline = false;
  let secondHairGapPt: number | null = null;
  if (ruleClusters.length >= 2) {
    const gapPt = (ruleClusters[1].y0 - hairlineY1 - 1) / scale;
    if (gapPt >= 0 && gapPt < 3.5) {
      doubleHairline = true;
      secondHairGapPt = +gapPt.toFixed(2);
    }
  }

  // left border under table
  let borderX = Math.floor(anchor.x);
  let best = 0;
  const yScan0 = baselineY > 0 ? baselineY : Math.floor(anchor.y);
  for (let x = Math.max(2, Math.floor(anchor.x) - Math.round(30 * scale)); x < Math.floor(anchor.x) + Math.round(20 * scale); x++) {
    let n = 0;
    for (let y = yScan0; y < Math.min(H, yScan0 + Math.round(280 * scale)); y++) if (isRule(x, y)) n++;
    if (n > best) {
      best = n;
      borderX = x;
    }
  }

  // cream gap under chip: only if interior matches PAGE while border broken
  let creamGapPx = 0;
  const yAfter = hairlineY1 + 1;
  if (baselineY > 0) {
    for (let y = yAfter; y < yAfter + Math.round(8 * scale); y++) {
      if (isRule(borderX, y)) break;
      if (near(rgb(borderX, y), PAGE, 18)) {
        let pageish = 0;
        let tot = 0;
        for (let x = borderX + 8; x < borderX + Math.round(100 * scale); x += 2) {
          tot++;
          if (near(rgb(x, y), PAGE, 16)) pageish++;
        }
        if (pageish > tot * 0.5) creamGapPx++;
        else break;
      } else break;
    }
  }
  const creamGapPt = +(creamGapPx / scale).toFixed(2);

  // seam ladder under chip text baseline (x through chip body)
  const seamX = Math.min(W - 2, Math.floor(anchor.x + 20 * scale));
  const seamLadder: { dyPt: number; kind: string; rgb: number[] }[] = [];
  let ruleBandsInSeam = 0;
  if (baselineY > 0) {
    let inRule = false;
    for (let dy = 0; dy <= Math.round(3.5 * scale); dy++) {
      const r = isRule(seamX, baselineY + dy);
      if (r && !inRule) {
        ruleBandsInSeam++;
        inRule = true;
      }
      if (!r) inRule = false;
    }
    for (let dy = -Math.round(1.5 * scale); dy <= Math.round(8 * scale); dy++) {
      const y = baselineY + dy;
      if (y < 0 || y >= H) continue;
      seamLadder.push({ dyPt: +(dy / scale).toFixed(2), kind: kindAt(seamX, y), rgb: [...rgb(seamX, y)] });
    }
  }

  // find table bottom (last continuous border RULE before long PAGE)
  let tableBottomY = baselineY > 0 ? baselineY + Math.round(40 * scale) : 0;
  if (baselineY > 0) {
    let lastRule = baselineY;
    for (let y = baselineY; y < Math.min(H - 2, baselineY + Math.round(420 * scale)); y++) {
      if (isRule(borderX, y)) lastRule = y;
      else if (y - lastRule > Math.round(3 * scale) && near(rgb(borderX, y), PAGE, 16)) break;
    }
    tableBottomY = lastRule;
  }

  // mid-row join samples only inside table (exclude bottom edge / below)
  const joinSamples: { y: number; pageGap: boolean; Lmin: number; Lmax: number; kinds: string }[] = [];
  const notchClusters: { y0: number; y1: number; pt: number }[] = [];
  if (baselineY > 0) {
    const notches: number[] = [];
    for (let y = baselineY + Math.round(4 * scale); y < tableBottomY - Math.round(2 * scale); y++) {
      if (near(rgb(borderX, y), PAGE, 16) && isRule(borderX, y - 2) && isRule(borderX, y + 2)) notches.push(y);
    }
    for (const y of notches) {
      const last = notchClusters[notchClusters.length - 1];
      if (last && y <= last.y1 + 3) last.y1 = y;
      else notchClusters.push({ y0: y, y1: y, pt: 0 });
    }
    for (const c of notchClusters) c.pt = +((c.y1 - c.y0 + 1) / scale).toFixed(2);

    for (let y = baselineY + Math.round(8 * scale); y < tableBottomY - Math.round(3 * scale); y++) {
      let midRule = 0;
      const x0 = borderX + Math.round(30 * scale);
      const x1 = Math.min(W - 4, borderX + Math.round(200 * scale));
      for (let x = x0; x < x1; x += 2) if (isRule(x, y)) midRule++;
      if (midRule < ((x1 - x0) / 2) * 0.32) continue;
      const kinds: string[] = [];
      let pageGap = false;
      let Lmin = 999;
      let Lmax = 0;
      for (let dy = -Math.round(1.5 * scale); dy <= Math.round(1.5 * scale); dy++) {
        const k = kindAt(borderX, y + dy);
        kinds.push(`${(dy / scale).toFixed(1)}:${k}`);
        const L = lum(borderX, y + dy);
        Lmin = Math.min(Lmin, L);
        Lmax = Math.max(Lmax, L);
        if (k === "PAGE") pageGap = true;
      }
      joinSamples.push({ y, pageGap, Lmin: +Lmin.toFixed(0), Lmax: +Lmax.toFixed(0), kinds: kinds.join(" ") });
      y += Math.round(3 * scale);
    }
  }

  // body text insets from text items (first bodyPhrase or first content under baseline)
  let leftInsetPt: number | null = null;
  let topInsetPt: number | null = null;
  let bodyAnchor: { x: number; y: number; str: string } | null = null;
  if (baselineY > 0) {
    const bodyCands = items
      .filter((c) => c.y > baselineY + 2 * scale && c.y < baselineY + 80 * scale)
      .filter((c) => c.x > borderX && c.x < borderX + 120 * scale)
      .filter((c) => c.str.length >= 3);
    bodyCands.sort((a, b) => a.y - b.y || a.x - b.x);
    bodyAnchor =
      bodyCands.find((c) => c.str.includes(sample.bodyPhrase)) ||
      bodyCands.find((c) => /[A-Za-zÄÖÜäöüß0-9]/.test(c.str) && !c.str.includes("Tabelle")) ||
      bodyCands[0] ||
      null;

    // Prefer first ink after header row: find first dark glyph near left after yAfter+header
    // Use text item y as top of glyph baseline — measure top of ink above baseline
    if (bodyAnchor) {
      // walk up from text baseline to find top of ink
      let topInk = Math.floor(bodyAnchor.y);
      for (let y = Math.floor(bodyAnchor.y); y > yAfter; y--) {
        let ink = 0;
        for (let x = Math.floor(bodyAnchor.x); x < Math.floor(bodyAnchor.x) + Math.round(40 * scale); x++) {
          if (isInk(x, y)) ink++;
        }
        if (ink < 2) {
          topInk = y + 1;
          break;
        }
        topInk = y;
      }
      // left of first ink in that row
      let leftInk = Math.floor(bodyAnchor.x);
      for (let x = Math.floor(bodyAnchor.x); x > borderX; x--) {
        if (isInk(x, Math.floor(bodyAnchor.y) - Math.round(0.5 * scale))) leftInk = x;
        else if (x < leftInk - 2) break;
      }
      // refine left: first ink scanning right from border on topInk row band
      outer: for (let x = borderX + 1; x < borderX + Math.round(80 * scale); x++) {
        for (let y = topInk; y < topInk + Math.round(8 * scale); y++) {
          if (isInk(x, y) && !isRule(x, y)) {
            leftInk = x;
            break outer;
          }
        }
      }
      leftInsetPt = +((leftInk - borderX - 1) / scale).toFixed(2);
      // top inset from rule under chips OR from nearest hrule above body text
      let ruleAbove = yAfter - 1;
      for (let y = Math.floor(bodyAnchor.y) - 1; y > baselineY; y--) {
        let n = 0;
        for (let x = borderX + 20; x < borderX + Math.round(120 * scale); x += 2) if (isRule(x, y)) n++;
        if (n > 20) {
          ruleAbove = y;
          break;
        }
      }
      topInsetPt = +((topInk - ruleAbove - 1) / scale).toFixed(2);
    }
  }

  const crop = (name: string, x0: number, y0: number, w: number, h: number) => {
    const c = createCanvas(Math.max(1, w), Math.max(1, h));
    c.getContext("2d").drawImage(canvas, x0, y0, w, h, 0, 0, w, h);
    const p = join(ticketDir, `${tag}-${sample.id}-${name}.png`);
    writeFileSync(p, c.toBuffer("image/png"));
    return p.replace(/\\/g, "/");
  };

  const crops = {
    seam:
      baselineY > 0
        ? crop("seam", Math.max(0, borderX - Math.round(4 * scale)), Math.max(0, baselineY - Math.round(16 * scale)), Math.round(200 * scale), Math.round(48 * scale))
        : null,
    body:
      baselineY > 0
        ? crop("body", Math.max(0, borderX - Math.round(3 * scale)), Math.max(0, baselineY - Math.round(2 * scale)), Math.round(260 * scale), Math.round(72 * scale))
        : null,
    Ljoin:
      baselineY > 0
        ? crop("Ljoin", Math.max(0, borderX - Math.round(2 * scale)), baselineY + Math.round(6 * scale), Math.round(36 * scale), Math.round(110 * scale))
        : null,
    chip: crop("chip", Math.max(0, Math.floor(anchor.x) - Math.round(8 * scale)), Math.max(0, Math.floor(anchor.y) - Math.round(10 * scale)), Math.round(180 * scale), Math.round(36 * scale)),
  };

  const fail: string[] = [];
  if (doubleHairline || ruleBandsInSeam > 1) fail.push(`double-hairline(bands=${ruleBandsInSeam},gap=${secondHairGapPt})`);
  if (hairlinePt > 1.25) fail.push(`hairline-thick:${hairlinePt}`);
  if (creamGapPt > 0.35) fail.push(`cream-gap:${creamGapPt}`);
  if (notchClusters.length > 0) fail.push(`join-notches:${notchClusters.length}`);
  if (joinSamples.some((j) => j.pageGap)) fail.push("join-page-gap");
  if (leftInsetPt != null && (leftInsetPt < 3.5 || leftInsetPt > 9.5)) fail.push(`left-inset:${leftInsetPt}`);
  if (topInsetPt != null && (topInsetPt < 3.5 || topInsetPt > 10)) fail.push(`top-inset:${topInsetPt}`);
  if (baselineY < 0) fail.push("no-baseline");

  const row = {
    id: sample.id,
    page: sample.page,
    scale,
    anchor: anchor.str,
    bodyAnchor: bodyAnchor?.str ?? null,
    baselineY,
    borderX,
    tableBottomY,
    hairlinePt,
    ruleBandsInSeam,
    doubleHairline,
    secondHairGapPt,
    creamGapPt,
    joinNotches: notchClusters.length,
    notchClusters,
    joinSamples: joinSamples.slice(0, 10),
    leftInsetPt,
    topInsetPt,
    seamLadder: seamLadder.filter((s) => s.dyPt >= -0.5 && s.dyPt <= 2.5),
    crops,
    ok: fail.length === 0,
    fail,
  };
  reports.push(row);
  console.log(
    `[DEBUG] ${sample.id}@${scale} ok=${row.ok} hair=${hairlinePt}pt bands=${ruleBandsInSeam} cream=${creamGapPt} notches=${notchClusters.length} joins=${joinSamples.length} pageGaps=${joinSamples.filter((j) => j.pageGap).length} L=${leftInsetPt} T=${topInsetPt} fail=${fail.join("|") || "-"}`,
  );
  for (const s of row.seamLadder.slice(0, 12)) {
    console.log(`[DEBUG]   seam dy=${s.dyPt} ${s.kind} rgb=${s.rgb.join(",")}`);
  }
  for (const j of joinSamples.slice(0, 5)) {
    console.log(`[DEBUG]   join y=${j.y} pageGap=${j.pageGap} L=${j.Lmin}-${j.Lmax}`);
  }
}

const out = join(ticketDir, `${tag}-report.json`);
writeFileSync(out, JSON.stringify(reports, null, 2));
const bad = reports.filter((r) => (r as { ok?: boolean }).ok === false);
console.log(`[DEBUG] wrote ${out}  PASS ${reports.length - bad.length}/${reports.length}`);
