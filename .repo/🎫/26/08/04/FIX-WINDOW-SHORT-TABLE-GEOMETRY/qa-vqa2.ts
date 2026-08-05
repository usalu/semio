#!/usr/bin/env bun
/** 🔬 [DEBUG] Visual QA v2: corrected anchors, seam/join/cell crops, RGB metrics. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const outDir = "e:/semio/.repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";
const pdfPath = "e:/semio/mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf";
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);
const doc = await pdfjs.getDocument({
  data: new Uint8Array(readFileSync(pdfPath)),
  useSystemFonts: true,
}).promise;

type Cand = { str: string; x: number; y: number };
type Kind = "project" | "long-table" | "glossary";

const targets: {
  id: string;
  page: number;
  kind: Kind;
  pick: (c: Cand[]) => Cand | undefined;
}[] = [
  {
    id: "kopfbau",
    page: 24,
    kind: "project",
    pick: (c) => c.find((x) => x.str.includes("Kopfbau Halle")),
  },
  {
    id: "huerden",
    page: 76,
    kind: "long-table",
    pick: (c) => {
      const left = c.filter(
        (x) => x.x < 400 && x.str.length < 40 && !x.str.includes("Arbeitsübersicht"),
      );
      return left.sort((a, b) => a.y - b.y).find((x) => x.y > 140) ?? left[0];
    },
  },
  {
    id: "markt",
    page: 78,
    kind: "long-table",
    pick: (c) => c.find((x) => x.str.includes("Zugang")) ?? c.find((x) => x.str.includes("Marktplätze")),
  },
  {
    id: "glossar",
    page: 121,
    kind: "glossary",
    pick: (c) =>
      c.find((x) => x.str.trim() === "Glossar") ??
      c.find((x) => x.str.includes("Glossar") && x.str.length < 20),
  },
];

function metrics(
  data: Uint8ClampedArray,
  W: number,
  H: number,
  scale: number,
  anchor: Cand,
  kind: Kind,
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
    const [r, g, b] = rgb(x, y);
    const L = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    return L > 50 && L < 155 && Math.abs(r - g) < 35 && Math.abs(g - b) < 35;
  };
  const isCream = (x: number, y: number) => lum(x, y) > 200;
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
  const isInk = (x: number, y: number) => lum(x, y) < 85 && !isPhoto(x, y);

  const xL = Math.max(8, Math.floor(anchor.x - 20));
  const xR = Math.min(W - 8, Math.floor(anchor.x + (kind === "project" ? 280 : 900)));
  const ruleClusters: { y0: number; y1: number }[] = [];
  const yEnd = Math.floor(anchor.y) + Math.round((kind === "project" ? 50 : 90) * scale);
  for (let y = Math.floor(anchor.y) + 1; y < Math.min(H - 4, yEnd); y++) {
    let n = 0;
    for (let x = xL; x < xR; x++) if (isRule(x, y)) n++;
    const thr = (xR - xL) * (kind === "project" ? 0.45 : 0.28);
    if (n > thr) {
      const last = ruleClusters[ruleClusters.length - 1];
      if (last && y <= last.y1 + 2) last.y1 = y;
      else ruleClusters.push({ y0: y, y1: y });
    }
  }
  const baseY = ruleClusters[0]?.y0 ?? -1;
  const hairPt = ruleClusters[0]
    ? +((ruleClusters[0].y1 - ruleClusters[0].y0 + 1) / scale).toFixed(2)
    : 0;
  let hairCount = ruleClusters.length ? 1 : 0;
  let double = false;
  if (ruleClusters.length > 1) {
    const gap = (ruleClusters[1].y0 - ruleClusters[0].y1 - 1) / scale;
    if (gap < 2.5) {
      double = true;
      hairCount++;
    }
  }

  let bx = Math.max(4, Math.floor(anchor.x - 40));
  let best = 0;
  const y0 = baseY > 0 ? baseY : Math.floor(anchor.y);
  for (
    let x = Math.max(4, Math.floor(anchor.x) - Math.round(80 * scale));
    x < Math.floor(anchor.x);
    x++
  ) {
    let n = 0;
    for (let y = y0; y < Math.min(H, y0 + Math.round(420 * scale)); y++) if (isRule(x, y)) n++;
    if (n > best) {
      best = n;
      bx = x;
    }
  }
  const yAfter = (ruleClusters[0]?.y1 ?? baseY) + 1;

  let creamGap = 0;
  if (baseY > 0) {
    for (let y = yAfter; y < yAfter + Math.round(20 * scale); y++) {
      const broken = !isRule(bx, y) && isCream(bx, y);
      let pageish = 0;
      for (let x = bx + 8; x < bx + 160; x += 2) if (isPage(x, y)) pageish++;
      if (broken && pageish > 25) creamGap++;
      else break;
    }
  }

  const notches: number[] = [];
  if (baseY > 0) {
    for (let y = baseY + Math.round(8 * scale); y < Math.min(H - 3, baseY + Math.round(600 * scale)); y++) {
      if (isCream(bx, y) && isRule(bx, y - 2) && isRule(bx, y + 2)) notches.push(y);
    }
  }
  const nCl: { y0: number; y1: number }[] = [];
  for (const y of notches) {
    const last = nCl[nCl.length - 1];
    if (last && y <= last.y1 + 3) last.y1 = y;
    else nCl.push({ y0: y, y1: y });
  }

  const joins: { y: number; borderL: number; kind: string }[] = [];
  if (baseY > 0) {
    for (let y = yAfter + Math.round(12 * scale); y < Math.min(H - 2, baseY + Math.round(520 * scale)); y++) {
      let mid = 0;
      for (let x = bx + 30; x < bx + 220; x++) if (isRule(x, y)) mid++;
      if (mid < 50) continue;
      let bestL = 999;
      let bestK = "?";
      for (let dx = -2; dx <= 2; dx++) {
        const L = lum(bx + dx, y);
        const k = isRule(bx + dx, y) ? "RULE" : isCream(bx + dx, y) ? "CREAM" : "OTHER";
        if (L < bestL) {
          bestL = L;
          bestK = k;
        }
      }
      joins.push({ y, borderL: +bestL.toFixed(1), kind: bestK });
      y += Math.round(6 * scale);
    }
  }

  let photoPad: number | null = null;
  let canvasPadRows = 0;
  if (baseY > 0 && kind === "project") {
    for (let y = yAfter; y < yAfter + Math.round(60 * scale); y++) {
      let photo = 0;
      let canvasN = 0;
      for (let x = bx + 6; x < bx + 200; x++) {
        if (isPhoto(x, y)) photo++;
        else if (isCanvas(x, y)) canvasN++;
      }
      if (photo > 30) {
        photoPad = +((y - yAfter) / scale).toFixed(2);
        break;
      }
      if (canvasN > 80) canvasPadRows++;
    }
  }

  let left: number | null = null;
  let top: number | null = null;
  let ink: { x: number; y: number } | null = null;
  if (baseY > 0 && kind !== "project") {
    const xScanMax = bx + Math.round(50 * scale);
    for (let y = yAfter; y < yAfter + Math.round(70 * scale); y++) {
      for (let x = bx + 2; x < xScanMax; x++) {
        if (!isInk(x, y)) continue;
        let n = 0;
        for (let dx = 0; dx < 5; dx++) if (isInk(x + dx, y)) n++;
        if (n < 3) continue;
        ink = { x, y };
        break;
      }
      if (ink) break;
    }
    if (ink) {
      top = +((ink.y - yAfter) / scale).toFixed(2);
      left = +((ink.x - bx - 1) / scale).toFixed(2);
    }
  }

  let weld: Record<string, number | boolean> | null = null;
  if (baseY > 0) {
    let pageN = 0;
    let canvasN = 0;
    let photoN = 0;
    for (let x = bx + 4; x < bx + 180; x++) {
      if (isPhoto(x, yAfter)) photoN++;
      else if (isCanvas(x, yAfter)) canvasN++;
      else if (isPage(x, yAfter)) pageN++;
    }
    weld = {
      pageN,
      canvasN,
      photoN,
      borderL: +lum(bx, yAfter).toFixed(1),
      borderRule: isRule(bx, yAfter),
    };
  }

  return {
    baseY,
    bx,
    yAfter,
    hairPt,
    hairCount,
    double,
    creamGapPt: +(creamGap / scale).toFixed(2),
    notches: nCl.length,
    joins: joins.slice(0, 10),
    photoPad,
    canvasPadRows,
    left,
    top,
    ink,
    weld,
    ruleClusters: ruleClusters.slice(0, 3),
  };
}

const report: Record<string, unknown>[] = [];

for (const dpi of [144, 432]) {
  const scale = dpi / 72;
  for (const t of targets) {
    const page = await doc.getPage(t.page);
    const vp = page.getViewport({ scale });
    const items = (await page.getTextContent()).items as { str: string; transform: number[] }[];
    const cands: Cand[] = [];
    for (const it of items) {
      if (!it.str || it.str.length < 3) continue;
      const [x, y] = vp.convertToViewportPoint(it.transform[4], it.transform[5]);
      cands.push({ str: it.str, x, y });
    }
    // scale-adjust pick thresholds for huerden
    const pickCands =
      t.id === "huerden"
        ? cands.filter(
            (x) =>
              x.x < 200 * scale &&
              x.str.length < 40 &&
              !x.str.includes("Arbeitsübersicht") &&
              x.str.includes("Hürden"),
          )
        : cands;
    // chip is the lowest left short title (section heading sits above body copy)
    const anchor =
      t.id === "huerden"
        ? [...pickCands].sort((a, b) => b.y - a.y)[0]
        : t.pick(cands);
    if (!anchor) {
      report.push({ id: t.id, dpi, error: "no anchor" });
      continue;
    }

    const canvas = createCanvas(Math.ceil(vp.width), Math.ceil(vp.height));
    await page.render({ canvasContext: canvas.getContext("2d"), viewport: vp }).promise;
    const { data, width: W, height: H } = canvas.getContext("2d").getImageData(0, 0, canvas.width, canvas.height);
    const m = metrics(data, W, H, scale, anchor, t.kind);

    const crops: Record<string, string> = {};
    if (m.baseY > 0) {
      const save = (name: string, x0: number, y0: number, w: number, h: number) => {
        const c = createCanvas(w, h);
        c.getContext("2d").drawImage(canvas, x0, y0, w, h, 0, 0, w, h);
        const p = `${outDir}/vqa2-${t.id}-d${dpi}-${name}.png`;
        writeFileSync(p, c.toBuffer("image/png"));
        return p;
      };
      const bx = Math.max(0, m.bx - 8);
      crops.seam = save(
        "seam",
        bx,
        m.baseY - Math.round(24 * scale),
        Math.round(200 * scale),
        Math.round(60 * scale),
      );
      crops.join = save(
        "join",
        bx,
        m.baseY + Math.round(16 * scale),
        Math.round(80 * scale),
        Math.round(140 * scale),
      );
      crops.cell = save("cell", bx, m.yAfter, Math.round(140 * scale), Math.round(80 * scale));
      crops.chip = save(
        "chip",
        Math.max(0, Math.floor(anchor.x) - Math.round(8 * scale)),
        Math.max(0, Math.floor(anchor.y) - Math.round(12 * scale)),
        Math.round(200 * scale),
        Math.round(36 * scale),
      );
    }

    const creamJoins = m.joins.filter((j) => j.kind === "CREAM").length;
    const fail: string[] = [];
    if (m.hairCount !== 1) fail.push(`hairlines:${m.hairCount}`);
    if (m.double) fail.push("double-hairline");
    if (m.hairPt > 1.1) fail.push(`hairline-thick:${m.hairPt}`);
    if (m.creamGapPt > 0.2) fail.push(`cream-gap:${m.creamGapPt}`);
    if (m.notches > 0) fail.push(`join-notches:${m.notches}`);
    if (creamJoins > 0) fail.push(`cream-joins:${creamJoins}`);
    if (t.kind !== "project" && m.left != null && (m.left < 3.5 || m.left > 10.5)) {
      fail.push(`left-inset:${m.left}`);
    }
    if (t.kind !== "project" && m.top != null && (m.top < 3 || m.top > 12.5)) {
      fail.push(`top-inset:${m.top}`);
    }
    if (t.kind === "project") {
      if (m.photoPad == null) fail.push("photo-pad:missing");
      else if (Math.abs(m.photoPad - 5.5) > 1.0) fail.push(`photo-pad:${m.photoPad}`);
    }

    const row = {
      id: t.id,
      page: t.page,
      dpi,
      scale,
      anchor: anchor.str,
      anchorXY: [+anchor.x.toFixed(1), +anchor.y.toFixed(1)],
      ...m,
      creamJoins,
      crops,
      ok: fail.length === 0,
      fail,
    };
    report.push(row);
    console.log(
      `[DEBUG] ${t.id}@${dpi} ok=${row.ok} hair=${m.hairCount}x${m.hairPt}pt gap=${m.creamGapPt} notch=${m.notches} L=${m.left} T=${m.top} photo=${m.photoPad} joins=${m.joins.map((j) => j.kind).join("|")} ${fail.join(",")}`,
    );
  }
}

writeFileSync(`${outDir}/qa-vqa2-report.json`, JSON.stringify(report, null, 2));
console.log(`[DEBUG] done fails=${report.filter((r) => !r.ok).length}`);
