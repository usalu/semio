#!/usr/bin/env bun
/**
 * 🔬 [DEBUG] Visual QA: chip seam + first body + L-border joins for TOC / Meilensteine / Risiken.
 * Usage: bun qaV-scan.ts <pdf> [scale]
 */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const pdfPath = process.argv[2]!;
const scale = Number(process.argv[3] ?? 6);
const ticketDir = dirname(fileURLToPath(import.meta.url));
const tag = `qaV-s${scale}`;

const samples = [
  { id: "toc", page: 3, phrase: "Inhaltsverzeichnis", kind: "toc" as const },
  { id: "meilensteine", page: 17, phrase: "Meilensteine", kind: "window" as const },
  { id: "risiken", page: 19, phrase: "Risiken", kind: "window" as const },
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
  const items = content.items as { str: string; transform: number[] }[];
  const word0 = sample.phrase.slice(0, 6);
  const cands = items
    .filter((it) => it.str && it.str.includes(word0))
    .map((it) => {
      const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
      return { x, y, str: it.str };
    })
    .filter((c) => c.y > 30 * scale && c.y < viewport.height - 30 * scale);
  cands.sort((a, b) => b.str.length - a.str.length || a.y - b.y);
  const preferred =
    cands.find((c) => c.str.includes(sample.phrase)) ||
    cands.find((c) => sample.phrase.split(/\s+/).every((w) => c.str.includes(w.slice(0, 4)))) ||
    cands[0];
  if (!preferred) {
    reports.push({ id: sample.id, error: "no anchor" });
    continue;
  }
  const anchor = preferred;

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
    return L > 45 && L < 175 && Math.abs(r - g) < 40 && Math.abs(g - b) < 40;
  };
  const isInk = (x: number, y: number) => lum(x, y) < 85;
  const kindAt = (x: number, y: number) => {
    const c = rgb(x, y);
    if (isRule(x, y)) return "RULE";
    if (near(c, PAGE)) return "PAGE";
    if (near(c, CANVAS)) return "CANVAS";
    if (lum(x, y) < 85) return "INK";
    return "OTHER";
  };

  const xL = Math.max(8, Math.floor(anchor.x - 80));
  const xR = Math.min(W - 8, Math.floor(anchor.x + 1100));

  // rule clusters under chip text
  const ruleClusters: { y0: number; y1: number }[] = [];
  for (let y = Math.floor(anchor.y) + 1; y < Math.min(H - 4, Math.floor(anchor.y) + Math.round(100 * scale)); y++) {
    let n = 0;
    for (let x = xL; x < xR; x += 2) if (isRule(x, y)) n++;
    if (n > ((xR - xL) / 2) * 0.28) {
      const last = ruleClusters[ruleClusters.length - 1];
      if (last && y <= last.y1 + 2) last.y1 = y;
      else ruleClusters.push({ y0: y, y1: y });
    }
  }
  const baselineY = ruleClusters[0]?.y0 ?? -1;
  const hairlineY1 = ruleClusters[0]?.y1 ?? -1;
  const hairlinePt = baselineY >= 0 ? (hairlineY1 - baselineY + 1) / scale : 0;
  let doubleHairline = false;
  let secondHairGapPt: number | null = null;
  if (ruleClusters.length >= 2) {
    const gapPt = (ruleClusters[1].y0 - hairlineY1 - 1) / scale;
    if (gapPt >= 0 && gapPt < 3.5) {
      doubleHairline = true;
      secondHairGapPt = +gapPt.toFixed(2);
    }
  }

  // left border
  let borderX = xL;
  let best = 0;
  const yScan0 = baselineY > 0 ? baselineY : Math.floor(anchor.y);
  for (let x = Math.max(2, xL - 50); x < xL + 120; x++) {
    let n = 0;
    for (let y = yScan0; y < Math.min(H, yScan0 + Math.round(500 * scale)); y++) if (isRule(x, y)) n++;
    if (n > best) {
      best = n;
      borderX = x;
    }
  }

  // cream gap under chips: PAGE-matching rows inside frame right after hairline
  let creamGapPx = 0;
  const yAfter = hairlineY1 + 1;
  if (baselineY > 0) {
    for (let y = yAfter; y < yAfter + Math.round(20 * scale); y++) {
      let pageish = 0;
      let samplesN = 0;
      for (let x = borderX + 10; x < borderX + Math.round(180 * scale); x += 2) {
        samplesN++;
        const c = rgb(x, y);
        const margin = rgb(Math.max(0, borderX - 14), y);
        if (
          near(c, { r: margin[0], g: margin[1], b: margin[2] }, 12) &&
          near(c, PAGE, 18) &&
          !isRule(borderX, y)
        ) {
          pageish++;
        }
      }
      // bad only if border broken AND interior matches page
      if (!isRule(borderX, y) && near(rgb(borderX, y), PAGE, 18) && pageish > samplesN * 0.45) creamGapPx++;
      else break;
    }
  }
  const creamGapPt = +(creamGapPx / scale).toFixed(2);

  // vertical seam RGB ladder under chip (mid of first chip-ish area)
  const seamX = Math.min(W - 2, Math.floor(anchor.x + 40 * scale));
  const seamLadder: { dy: number; kind: string; rgb: number[]; L: number }[] = [];
  if (baselineY > 0) {
    for (let dy = -4; dy <= Math.round(14 * scale); dy++) {
      const y = baselineY + dy;
      if (y < 0 || y >= H) continue;
      const c = rgb(seamX, y);
      seamLadder.push({ dy, kind: kindAt(seamX, y), rgb: [...c], L: +lum(seamX, y).toFixed(1) });
    }
  }

  // count distinct RULE bands in first 3pt under chip mid
  let ruleBandsInSeam = 0;
  if (baselineY > 0) {
    let inRule = false;
    for (let dy = 0; dy <= Math.round(4 * scale); dy++) {
      const r = isRule(seamX, baselineY + dy);
      if (r && !inRule) {
        ruleBandsInSeam++;
        inRule = true;
      }
      if (!r) inRule = false;
    }
  }

  // join notches
  const notches: number[] = [];
  if (baselineY > 0) {
    for (let y = baselineY + Math.round(6 * scale); y < Math.min(H - 3, baselineY + Math.round(650 * scale)); y++) {
      if (near(rgb(borderX, y), PAGE, 16) && isRule(borderX, y - 2) && isRule(borderX, y + 2)) notches.push(y);
    }
  }
  const notchClusters: { y0: number; y1: number; pt: number }[] = [];
  for (const y of notches) {
    const last = notchClusters[notchClusters.length - 1];
    if (last && y <= last.y1 + 3) last.y1 = y;
    else notchClusters.push({ y0: y, y1: y, pt: 0 });
  }
  for (const c of notchClusters) c.pt = +((c.y1 - c.y0 + 1) / scale).toFixed(2);

  // h-rule join samples: for each mid-table hrule, check L border continuity
  const joinSamples: { y: number; borderKinds: string; pageGap: boolean; Lrange: string }[] = [];
  if (baselineY > 0) {
    for (let y = baselineY + Math.round(10 * scale); y < Math.min(H - 4, baselineY + Math.round(600 * scale)); y++) {
      let midRule = 0;
      const x0 = borderX + Math.round(40 * scale);
      const x1 = borderX + Math.round(220 * scale);
      for (let x = x0; x < x1; x += 2) if (isRule(x, y)) midRule++;
      if (midRule < ((x1 - x0) / 2) * 0.35) continue;
      const kinds: string[] = [];
      let pageGap = false;
      let minL = 999;
      let maxL = 0;
      for (let dy = -Math.round(2 * scale); dy <= Math.round(2 * scale); dy++) {
        const k = kindAt(borderX, y + dy);
        kinds.push(`${dy}:${k}`);
        const L = lum(borderX, y + dy);
        minL = Math.min(minL, L);
        maxL = Math.max(maxL, L);
        if (k === "PAGE") pageGap = true;
      }
      joinSamples.push({
        y,
        borderKinds: kinds.join(" "),
        pageGap,
        Lrange: `${minL.toFixed(0)}-${maxL.toFixed(0)}`,
      });
      y += Math.round(4 * scale); // skip cluster
    }
  }

  // text insets: first ink in body
  let leftInsetPt: number | null = null;
  let topInsetPt: number | null = null;
  let inkX = -1;
  let inkY = -1;
  if (baselineY > 0) {
    for (let y = yAfter; y < yAfter + Math.round(120 * scale); y++) {
      for (let x = borderX + Math.round(2 * scale); x < borderX + Math.round(320 * scale); x++) {
        if (isInk(x, y)) {
          // skip if mostly rule-colored neighborhood (rule itself)
          if (isRule(x, y)) continue;
          inkY = y;
          inkX = x;
          break;
        }
      }
      if (inkY >= 0) break;
    }
    if (inkY >= 0) {
      topInsetPt = +((inkY - yAfter) / scale).toFixed(2);
      leftInsetPt = +((inkX - borderX - 1) / scale).toFixed(2);
    }
  }

  const crop = (name: string, x0: number, y0: number, w: number, h: number) => {
    const c = createCanvas(w, h);
    c.getContext("2d").drawImage(canvas, x0, y0, w, h, 0, 0, w, h);
    const p = join(ticketDir, `${tag}-${sample.id}-${name}.png`);
    writeFileSync(p, c.toBuffer("image/png"));
    return p;
  };

  const crops = {
    seam:
      baselineY > 0
        ? crop(
            "seam",
            Math.max(0, borderX - Math.round(6 * scale)),
            Math.max(0, baselineY - Math.round(18 * scale)),
            Math.round(220 * scale),
            Math.round(55 * scale),
          )
        : null,
    body:
      baselineY > 0
        ? crop(
            "body",
            Math.max(0, borderX - Math.round(4 * scale)),
            Math.max(0, baselineY - Math.round(4 * scale)),
            Math.round(280 * scale),
            Math.round(70 * scale),
          )
        : null,
    Ljoin:
      baselineY > 0
        ? crop(
            "Ljoin",
            Math.max(0, borderX - Math.round(3 * scale)),
            baselineY + Math.round(8 * scale),
            Math.round(40 * scale),
            Math.round(120 * scale),
          )
        : null,
    chip: crop(
      "chip",
      Math.max(0, Math.floor(anchor.x) - Math.round(10 * scale)),
      Math.max(0, Math.floor(anchor.y) - Math.round(12 * scale)),
      Math.round(200 * scale),
      Math.round(40 * scale),
    ),
  };

  const fail: string[] = [];
  if (doubleHairline || ruleBandsInSeam > 1) fail.push(`double-hairline(bands=${ruleBandsInSeam},gap=${secondHairGapPt})`);
  if (hairlinePt > 1.2) fail.push(`hairline-thick:${hairlinePt.toFixed(2)}`);
  if (creamGapPt > 0.35) fail.push(`cream-gap:${creamGapPt}`);
  if (notchClusters.length > 0) fail.push(`join-notches:${notchClusters.length}`);
  if (joinSamples.some((j) => j.pageGap)) fail.push("join-page-gap");
  if (leftInsetPt != null && (leftInsetPt < 3.5 || leftInsetPt > 9.5)) fail.push(`left-inset:${leftInsetPt}`);
  if (topInsetPt != null && (topInsetPt < 3.5 || topInsetPt > 10)) fail.push(`top-inset:${topInsetPt}`);

  const row = {
    id: sample.id,
    page: sample.page,
    scale,
    anchor: anchor.str,
    baselineY,
    borderX,
    hairlinePt: +hairlinePt.toFixed(2),
    ruleBandsInSeam,
    doubleHairline,
    secondHairGapPt,
    creamGapPt,
    joinNotches: notchClusters.length,
    notchClusters: notchClusters.slice(0, 8),
    joinSamples: joinSamples.slice(0, 8),
    leftInsetPt,
    topInsetPt,
    inkX,
    inkY,
    seamLadder: seamLadder.filter((s) => s.dy >= -2 && s.dy <= Math.round(6 * scale)),
    crops,
    ok: fail.length === 0,
    fail,
  };
  reports.push(row);
  console.log(
    `[DEBUG] ${sample.id}@${scale} ok=${row.ok} hair=${row.hairlinePt}pt bands=${ruleBandsInSeam} gap=${creamGapPt} notches=${notchClusters.length} L=${leftInsetPt} T=${topInsetPt} fail=${fail.join("|") || "-"}`,
  );
  for (const j of joinSamples.slice(0, 4)) {
    console.log(`[DEBUG]   join y=${j.y} pageGap=${j.pageGap} L=${j.Lrange} ${j.borderKinds}`);
  }
}

const out = join(ticketDir, `${tag}-report.json`);
writeFileSync(out, JSON.stringify(reports, null, 2));
console.log(`[DEBUG] wrote ${out}`);
const bad = reports.filter((r) => (r as { ok?: boolean }).ok === false);
console.log(`[DEBUG] ${reports.length - bad.length}/${reports.length} PASS`);
