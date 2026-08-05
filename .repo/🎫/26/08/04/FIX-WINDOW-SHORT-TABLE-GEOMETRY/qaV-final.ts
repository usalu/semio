#!/usr/bin/env bun
/**
 * 🔬 [DEBUG] Final Visual QA metrics for TOC / Meilensteine / Risiken.
 */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const pdfPath = process.argv[2]!;
const scale = Number(process.argv[3] ?? 6);
const ticketDir = dirname(fileURLToPath(import.meta.url));

const samples = [
  { id: "toc", page: 3, title: "Inhaltsverzeichnis", bodyRe: /^2\.2\.3\.2$/ },
  { id: "meilensteine", page: 18, title: "Meilensteine", bodyRe: /^1$/ },
  { id: "risiken", page: 19, title: "Risiken und Maßnahmen", bodyRe: /^Formale/ },
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

const report: Record<string, unknown>[] = [];

for (const s of samples) {
  const page = await doc.getPage(s.page);
  const viewport = page.getViewport({ scale });
  const pts = (await page.getTextContent()).items.map((it: { str: string; transform: number[] }) => {
    const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
    return { str: it.str, x, y };
  });
  const title =
    pts
      .filter((c) => c.str && c.str.includes(s.title.slice(0, 8)))
      .sort((a, b) => b.str.length - a.str.length || a.y - b.y)[0] ?? null;
  if (!title) {
    report.push({ id: s.id, error: "no title" });
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
  const isInk = (x: number, y: number) => lum(x, y) < 90;
  const isPage = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.abs(r - 247) + Math.abs(g - 243) + Math.abs(b - 227) <= 14;
  };
  const isCanvas = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.abs(r - 240) + Math.abs(g - 236) + Math.abs(b - 221) <= 14;
  };

  let b0 = -1;
  let b1 = -1;
  const xL = Math.floor(title.x - 20 * scale);
  const xR = Math.floor(title.x + 180 * scale);
  for (let y = Math.floor(title.y) + 1; y < Math.floor(title.y) + 40 * scale; y++) {
    let n = 0;
    for (let x = xL; x < xR; x += 2) if (isRule(x, y)) n++;
    if (n > ((xR - xL) / 2) * 0.25) {
      if (b0 < 0) b0 = y;
      b1 = y;
    } else if (b0 >= 0) break;
  }

  let borderX = Math.floor(title.x);
  let best = 0;
  for (let x = Math.max(2, Math.floor(title.x) - 25 * scale); x < Math.floor(title.x) + 15 * scale; x++) {
    let n = 0;
    for (let y = b0; y < b0 + 250 * scale; y++) if (isRule(x, y)) n++;
    if (n > best) {
      best = n;
      borderX = x;
    }
  }

  const body = pts.filter((c) => c.str && s.bodyRe.test(c.str) && c.y > b1).sort((a, b) => a.y - b.y)[0];
  if (!body) {
    report.push({ id: s.id, error: "no body", title: title.str });
    console.log(`[DEBUG] ${s.id}: no body`);
    continue;
  }

  let ruleAbove = -1;
  for (let y = Math.floor(body.y) - 1; y > b1; y--) {
    let n = 0;
    for (let x = borderX + 10 * scale; x < borderX + 120 * scale; x += 2) if (isRule(x, y)) n++;
    if (n > 25) {
      ruleAbove = y;
      break;
    }
  }

  let inkTop = -1;
  let inkLeft = -1;
  for (let y = ruleAbove + 1; y < Math.floor(body.y) + 2; y++) {
    for (let x = borderX + 2; x < borderX + 90 * scale; x++) {
      if (isInk(x, y) && !isRule(x, y)) {
        let neigh = 0;
        for (let dx = 0; dx < 10; dx++) if (isInk(x + dx, y)) neigh++;
        if (neigh >= 4) {
          inkTop = y;
          inkLeft = x;
          break;
        }
      }
    }
    if (inkTop >= 0) break;
  }
  if (inkTop >= 0) {
    let left = inkLeft;
    for (let y = inkTop; y < inkTop + 8 * scale; y++) {
      for (let x = borderX + 1; x < inkLeft + 20 * scale; x++) {
        if (isInk(x, y) && !isRule(x, y)) {
          left = Math.min(left, x);
          break;
        }
      }
    }
    inkLeft = left;
  }

  const seamX = Math.floor(title.x + 15 * scale);
  let bands = 0;
  let inR = false;
  const ladder: { dyPt: number; kind: string; rgb: number[] }[] = [];
  for (let dy = -Math.round(1 * scale); dy <= Math.round(3 * scale); dy++) {
    const y = b0 + dy;
    const rule = isRule(seamX, y);
    if (rule && !inR) {
      bands++;
      inR = true;
    }
    if (!rule) inR = false;
    const kind = rule ? "RULE" : isCanvas(seamX, y) ? "CANVAS" : isPage(seamX, y) ? "PAGE" : "OTHER";
    if (dy >= -Math.round(0.5 * scale) && dy <= Math.round(2 * scale)) {
      ladder.push({ dyPt: +(dy / scale).toFixed(2), kind, rgb: [...rgb(seamX, y)] });
    }
  }

  // cream gap under chip: PAGE rows with broken border
  let creamGapPx = 0;
  for (let y = b1 + 1; y < b1 + Math.round(6 * scale); y++) {
    if (isRule(borderX, y)) break;
    if (isPage(borderX, y)) creamGapPx++;
    else break;
  }

  let tableBot = b1;
  let last = b1;
  for (let y = b1; y < Math.min(H - 2, b1 + 400 * scale); y++) {
    if (isRule(borderX, y)) last = y;
    else if (y - last > 3 * scale) break;
  }
  tableBot = last;

  const notches: number[] = [];
  for (let y = b1 + 6 * scale; y < tableBot - 2 * scale; y++) {
    if (isPage(borderX, y) && isRule(borderX, y - 2) && isRule(borderX, y + 2)) notches.push(y);
  }
  const notchClusters: { y0: number; y1: number; pt: number }[] = [];
  for (const y of notches) {
    const lastC = notchClusters[notchClusters.length - 1];
    if (lastC && y <= lastC.y1 + 3) lastC.y1 = y;
    else notchClusters.push({ y0: y, y1: y, pt: 0 });
  }
  for (const c of notchClusters) c.pt = +((c.y1 - c.y0 + 1) / scale).toFixed(2);

  // sample mid-row joins (RULE through border)
  const joinOk: { y: number; L: number; kind: string }[] = [];
  for (let y = b1 + 8 * scale; y < tableBot - 3 * scale; y++) {
    let mid = 0;
    for (let x = borderX + 30 * scale; x < borderX + 160 * scale; x += 2) if (isRule(x, y)) mid++;
    if (mid < 20) continue;
    joinOk.push({
      y,
      L: +lum(borderX, y).toFixed(0),
      kind: isRule(borderX, y) ? "RULE" : isPage(borderX, y) ? "PAGE" : "OTHER",
    });
    y += Math.round(4 * scale);
  }

  let hTop = -1;
  for (let y = b1 + 1; y < b1 + 30 * scale; y++) {
    for (let x = borderX + 2; x < borderX + 80 * scale; x++) {
      if (isInk(x, y) && !isRule(x, y)) {
        let n = 0;
        for (let dx = 0; dx < 8; dx++) if (isInk(x + dx, y)) n++;
        if (n >= 3) {
          hTop = y;
          break;
        }
      }
    }
    if (hTop >= 0) break;
  }

  const topPt = inkTop >= 0 ? +((inkTop - ruleAbove - 1) / scale).toFixed(2) : null;
  const leftPt = inkLeft >= 0 ? +((inkLeft - borderX - 1) / scale).toFixed(2) : null;
  const headerTopPt = hTop >= 0 ? +((hTop - b1 - 1) / scale).toFixed(2) : null;
  const hairlinePt = +((b1 - b0 + 1) / scale).toFixed(2);
  const creamGapPt = +(creamGapPx / scale).toFixed(2);

  const fail: string[] = [];
  if (bands !== 1) fail.push(`hairline-bands:${bands}`);
  if (hairlinePt > 1.25) fail.push(`hairline-thick:${hairlinePt}`);
  if (creamGapPt > 0.35) fail.push(`cream-gap:${creamGapPt}`);
  if (notchClusters.length) fail.push(`notches:${notchClusters.length}`);
  if (joinOk.some((j) => j.kind === "PAGE")) fail.push("join-page");
  if (leftPt != null && (leftPt < 3.5 || leftPt > 9.5)) fail.push(`left:${leftPt}`);
  if (topPt != null && (topPt < 3.5 || topPt > 10)) fail.push(`bodyTop:${topPt}`);
  if (headerTopPt != null && (headerTopPt < 3.5 || headerTopPt > 10)) fail.push(`headerTop:${headerTopPt}`);

  const crop = (name: string, x0: number, y0: number, w: number, h: number) => {
    const c = createCanvas(w, h);
    c.getContext("2d").drawImage(canvas, x0, y0, w, h, 0, 0, w, h);
    const p = join(ticketDir, `qaV-final-${s.id}-${name}.png`);
    writeFileSync(p, c.toBuffer("image/png"));
    return p.replace(/\\/g, "/");
  };

  const row = {
    id: s.id,
    page: s.page,
    scale,
    title: title.str,
    body: body.str,
    hairlinePt,
    bands,
    creamGapPt,
    joinNotches: notchClusters.length,
    joinSamples: joinOk.slice(0, 8),
    leftInsetPt: leftPt,
    bodyTopInsetPt: topPt,
    headerTopInsetPt: headerTopPt,
    ok: fail.length === 0,
    fail,
    ladder,
    crops: {
      seam: crop("seam", Math.max(0, borderX - 4), b0 - Math.round(14 * scale), Math.round(190 * scale), Math.round(42 * scale)),
      body: crop("body", Math.max(0, borderX - 3), ruleAbove - Math.round(2 * scale), Math.round(220 * scale), Math.round(40 * scale)),
      Ljoin: crop("Ljoin", Math.max(0, borderX - 2), b1 + Math.round(8 * scale), Math.round(32 * scale), Math.round(100 * scale)),
    },
  };
  report.push(row);
  console.log(
    `[DEBUG] ${s.id}@${scale} ok=${row.ok} hair=${hairlinePt} bands=${bands} cream=${creamGapPt} notches=${notchClusters.length} L=${leftPt} bodyT=${topPt} headT=${headerTopPt} fail=${fail.join("|") || "-"}`,
  );
  for (const l of ladder) console.log(`[DEBUG]   seam ${l.dyPt} ${l.kind} ${l.rgb.join(",")}`);
}

const out = join(ticketDir, `qaV-final-s${scale}.json`);
writeFileSync(out, JSON.stringify(report, null, 2));
console.log(`[DEBUG] wrote ${out} PASS ${report.filter((r) => (r as { ok?: boolean }).ok).length}/${report.length}`);
