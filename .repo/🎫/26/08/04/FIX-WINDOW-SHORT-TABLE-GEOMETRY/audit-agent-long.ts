#!/usr/bin/env bun
/** 🔬 [DEBUG] Final visual consistency audit for LONG DATA TABLES. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const outDir = "e:/semio/.repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";
const pdfPath =
  "e:/semio/mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf";
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
type Role = "first" | "cont" | "last" | "single";

type Target = {
  id: string;
  family: string;
  page: number;
  role: Role;
  tableId: string;
  titleNeedle: string;
  pick?: (c: Cand[], scale: number) => Cand | undefined;
};

const targets: Target[] = [
  {
    id: "huerden-p76",
    family: "Hürden",
    page: 76,
    role: "single",
    tableId: "H.a",
    titleNeedle: "Hürden der Wiederverwendung",
    pick: (c, scale) => {
      const hits = c.filter(
        (x) =>
          x.str.includes("Hürden") &&
          x.x < 220 * scale &&
          x.str.length < 48 &&
          !x.str.includes("Arbeitsübersicht"),
      );
      return [...hits].sort((a, b) => b.y - a.y)[0];
    },
  },
  {
    id: "markt-zugang-p78",
    family: "Marktplätze · Zugang",
    page: 78,
    role: "single",
    tableId: "BB.M.a",
    titleNeedle: "Marktplätze · Zugang",
  },
  {
    id: "datenfelder-markt-p79",
    family: "Marktplätze · Datenfelder",
    page: 79,
    role: "first",
    tableId: "BB.M.b",
    titleNeedle: "Marktplätze · Datenfelder",
  },
  {
    id: "datenfelder-markt-p80",
    family: "Marktplätze · Datenfelder",
    page: 80,
    role: "cont",
    tableId: "BB.M.b",
    titleNeedle: "Marktplätze · Datenfelder",
  },
  {
    id: "datenfelder-markt-p81",
    family: "Marktplätze · Datenfelder",
    page: 81,
    role: "last",
    tableId: "BB.M.b",
    titleNeedle: "Marktplätze · Datenfelder",
  },
  {
    id: "datenfelder-depot-p83",
    family: "Depot-Shops · Datenfelder",
    page: 83,
    role: "first",
    tableId: "BB.D.b",
    titleNeedle: "Depot-Shops · Datenfelder",
  },
  {
    id: "datenfelder-depot-p84",
    family: "Depot-Shops · Datenfelder",
    page: 84,
    role: "last",
    tableId: "BB.D.b",
    titleNeedle: "Depot-Shops · Datenfelder",
  },
  {
    id: "datenfelder-vermitt-p85",
    family: "Vermittlungsplattformen · Datenfelder",
    page: 85,
    role: "single",
    tableId: "BB.V.b",
    titleNeedle: "Vermittlungsplattformen · Datenfelder",
  },
  {
    id: "glossar-p121",
    family: "Glossar",
    page: 121,
    role: "single",
    tableId: "Glossar",
    titleNeedle: "Glossar",
    pick: (c) =>
      c.find((x) => x.str.trim() === "Glossar") ??
      c.find((x) => x.str.includes("Glossar") && x.str.length < 20),
  },
];

const DPIS = [144, 288, 432];

function analyze(
  data: Uint8ClampedArray,
  W: number,
  H: number,
  scale: number,
  anchor: Cand,
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
    return L > 50 && L < 155 && Math.abs(r - g) < 40 && Math.abs(g - b) < 40;
  };
  const isCream = (x: number, y: number) => lum(x, y) > 200;
  const isPage = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.abs(r - 247) + Math.abs(g - 243) + Math.abs(b - 227) <= 16;
  };
  const isCanvas = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.abs(r - 240) + Math.abs(g - 236) + Math.abs(b - 221) <= 18;
  };
  // body copy is medium-gray (~208); headers are near-black
  const isInk = (x: number, y: number) => lum(x, y) < 165;
  const isDarkInk = (x: number, y: number) => lum(x, y) < 100;

  const xL = Math.max(8, Math.floor(anchor.x - 24 * scale));
  const xR = Math.min(W - 8, Math.floor(anchor.x + 980 * scale));
  const ruleClusters: { y0: number; y1: number }[] = [];
  const yEnd = Math.min(H - 4, Math.floor(anchor.y) + Math.round(90 * scale));
  for (let y = Math.floor(anchor.y) + 1; y < yEnd; y++) {
    let n = 0;
    for (let x = xL; x < xR; x++) if (isRule(x, y)) n++;
    if (n > (xR - xL) * 0.28) {
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
  let doubleHairline = false;
  if (ruleClusters.length > 1) {
    const gap = (ruleClusters[1].y0 - ruleClusters[0].y1 - 1) / scale;
    if (gap < 2.5) {
      doubleHairline = true;
      hairCount = 2;
    }
  }
  const yAfter = (ruleClusters[0]?.y1 ?? baseY) + 1;

  // outer L border under body
  let borderL = Math.max(4, Math.floor(anchor.x - 40));
  let bestL = 0;
  const yScan0 = baseY > 0 ? baseY : Math.floor(anchor.y);
  const yScan1 = Math.min(H - 2, yScan0 + Math.round(620 * scale));
  for (
    let x = Math.max(4, Math.floor(anchor.x) - Math.round(90 * scale));
    x < Math.floor(anchor.x);
    x++
  ) {
    let n = 0;
    for (let y = yScan0; y < yScan1; y++) if (isRule(x, y)) n++;
    if (n > bestL) {
      bestL = n;
      borderL = x;
    }
  }

  // outer R border
  let borderR = Math.min(W - 4, Math.floor(anchor.x + 900 * scale));
  let bestR = 0;
  for (
    let x = Math.min(W - 4, Math.floor(anchor.x + 200 * scale));
    x < Math.min(W - 2, Math.floor(anchor.x + 980 * scale));
    x++
  ) {
    let n = 0;
    for (let y = yScan0; y < yScan1; y++) if (isRule(x, y)) n++;
    if (n > bestR) {
      bestR = n;
      borderR = x;
    }
  }

  // chip L/R verticals in band above baseline
  const chipTop = Math.max(0, Math.floor(anchor.y) - Math.round(14 * scale));
  const chipBot = baseY > 0 ? baseY - 1 : Math.floor(anchor.y) + Math.round(10 * scale);
  const vertXs: number[] = [];
  for (let x = Math.max(0, borderL - Math.round(20 * scale)); x < Math.min(W, borderR + Math.round(20 * scale)); x++) {
    let n = 0;
    for (let y = chipTop; y <= chipBot; y++) if (isRule(x, y)) n++;
    if (n > (chipBot - chipTop + 1) * 0.35) vertXs.push(x);
  }
  const vertBands: { x0: number; x1: number }[] = [];
  for (const x of vertXs) {
    const last = vertBands[vertBands.length - 1];
    if (last && x === last.x1 + 1) last.x1 = x;
    else vertBands.push({ x0: x, x1: x });
  }
  const chipL = vertBands[0] ? Math.round((vertBands[0].x0 + vertBands[0].x1) / 2) : -1;
  const chipR = vertBands.length
    ? Math.round((vertBands[vertBands.length - 1].x0 + vertBands[vertBands.length - 1].x1) / 2)
    : -1;
  const chipAlignLpt =
    chipL >= 0 ? +((chipL - borderL) / scale).toFixed(2) : null;
  const chipAlignRpt =
    chipR >= 0 ? +((chipR - borderR) / scale).toFixed(2) : null;

  // cream gap under baseline (page showing through frame)
  let creamGapPx = 0;
  if (baseY > 0) {
    for (let y = yAfter; y < yAfter + Math.round(20 * scale); y++) {
      const broken = !isRule(borderL, y) && isCream(borderL, y);
      let pageish = 0;
      for (let x = borderL + 8; x < borderL + 160; x += 2) if (isPage(x, y)) pageish++;
      if (broken && pageish > 25) creamGapPx++;
      else break;
    }
  }
  const creamGapPt = +(creamGapPx / scale).toFixed(2);

  // weld sample just under baseline
  let weldPage = 0;
  let weldCanvas = 0;
  if (baseY > 0) {
    for (let x = borderL + 4; x < Math.min(borderL + 180, borderR - 4); x++) {
      if (isCanvas(x, yAfter)) weldCanvas++;
      else if (isPage(x, yAfter)) weldPage++;
    }
  }

  // L/R join continuity: cream notches + border x jitter at midrules
  const joinYs: number[] = [];
  if (baseY > 0) {
    for (let y = yAfter + Math.round(10 * scale); y < yScan1; y++) {
      let mid = 0;
      const mid0 = borderL + Math.round(40 * scale);
      const mid1 = Math.min(borderR - Math.round(40 * scale), borderL + Math.round(260 * scale));
      for (let x = mid0; x < mid1; x++) if (isRule(x, y)) mid++;
      if (mid < (mid1 - mid0) * 0.35) continue;
      joinYs.push(y);
      y += Math.round(5 * scale);
    }
  }

  let creamJoinsL = 0;
  let creamJoinsR = 0;
  let notchL = 0;
  let notchR = 0;
  const borderXsL: number[] = [];
  const borderXsR: number[] = [];
  const midMeetGapsL: number[] = [];
  const midMeetGapsR: number[] = [];
  const doubleOuterHits: number[] = [];

  for (const y of joinYs) {
    // cream at border?
    let bestKindL = "OTHER";
    let bestLL = 999;
    for (let dx = -2; dx <= 2; dx++) {
      const L = lum(borderL + dx, y);
      const k = isRule(borderL + dx, y) ? "RULE" : isCream(borderL + dx, y) ? "CREAM" : "OTHER";
      if (L < bestLL) {
        bestLL = L;
        bestKindL = k;
      }
    }
    if (bestKindL === "CREAM") creamJoinsL++;

    let bestKindR = "OTHER";
    let bestLR = 999;
    for (let dx = -2; dx <= 2; dx++) {
      const L = lum(borderR + dx, y);
      const k = isRule(borderR + dx, y) ? "RULE" : isCream(borderR + dx, y) ? "CREAM" : "OTHER";
      if (L < bestLR) {
        bestLR = L;
        bestKindR = k;
      }
    }
    if (bestKindR === "CREAM") creamJoinsR++;

    // find actual border ink x near expected
    let bxL = borderL;
    let minL = 999;
    for (let x = borderL - 4; x <= borderL + 4; x++) {
      const L = lum(x, y);
      if (L < minL) {
        minL = L;
        bxL = x;
      }
    }
    borderXsL.push(bxL);
    let bxR = borderR;
    let minR = 999;
    for (let x = borderR - 4; x <= borderR + 4; x++) {
      const L = lum(x, y);
      if (L < minR) {
        minR = L;
        bxR = x;
      }
    }
    borderXsR.push(bxR);

    // mid-rule meet: walk inward from outer face until cream/canvas after rule band
    // gap = cream pixels between outer border outer face and midrule start (should be ~0 inside)
    let gapL = 0;
    for (let x = bxL + 1; x < bxL + Math.round(8 * scale); x++) {
      if (isRule(x, y)) break;
      if (isCream(x, y) || isPage(x, y)) gapL++;
      else break;
    }
    midMeetGapsL.push(gapL);

    let gapR = 0;
    for (let x = bxR - 1; x > bxR - Math.round(8 * scale); x--) {
      if (isRule(x, y)) break;
      if (isCream(x, y) || isPage(x, y)) gapR++;
      else break;
    }
    midMeetGapsR.push(gapR);

    // double outer edge: two rule runs separated by cream/page (not AA thickening)
    const runs: { x0: number; x1: number }[] = [];
    let run: { x0: number; x1: number } | null = null;
    for (let x = bxL - Math.round(5 * scale); x <= bxL + Math.round(5 * scale); x++) {
      if (isRule(x, y)) {
        if (!run) run = { x0: x, x1: x };
        else run.x1 = x;
      } else if (run) {
        runs.push(run);
        run = null;
      }
    }
    if (run) runs.push(run);
    if (runs.length >= 2) {
      const gap = runs[1].x0 - runs[0].x1 - 1;
      let creamSep = 0;
      for (let x = runs[0].x1 + 1; x < runs[1].x0; x++) {
        if (isCream(x, y) || isPage(x, y)) creamSep++;
      }
      if (gap >= 2 && creamSep >= 2) doubleOuterHits.push(y);
    }
  }

  // notch scan full column
  if (baseY > 0) {
    for (let y = yAfter + Math.round(6 * scale); y < yScan1; y++) {
      if (isCream(borderL, y) && isRule(borderL, y - 2) && isRule(borderL, y + 2)) notchL++;
      if (isCream(borderR, y) && isRule(borderR, y - 2) && isRule(borderR, y + 2)) notchR++;
    }
  }
  // cluster notches
  const clusterCount = (n: number, step: number) => {
    // approximate: notch pixels / (~3px cluster)
    return Math.round(n / Math.max(1, step));
  };
  const notchClustersL = clusterCount(notchL, 3);
  const notchClustersR = clusterCount(notchR, 3);

  const jitterLpx =
    borderXsL.length > 1 ? Math.max(...borderXsL) - Math.min(...borderXsL) : 0;
  const jitterRpx =
    borderXsR.length > 1 ? Math.max(...borderXsR) - Math.min(...borderXsR) : 0;
  const jitterLpt = +(jitterLpx / scale).toFixed(2);
  const jitterRpt = +(jitterRpx / scale).toFixed(2);
  const maxMeetGapLpt = midMeetGapsL.length
    ? +(Math.max(...midMeetGapsL) / scale).toFixed(2)
    : 0;
  const maxMeetGapRpt = midMeetGapsR.length
    ? +(Math.max(...midMeetGapsR) / scale).toFixed(2)
    : 0;

  // first-column right edge (internal vertical) — keep inset scan inside col 1
  let col1R = borderL + Math.round(55 * scale);
  if (baseY > 0) {
    for (let x = borderL + Math.round(12 * scale); x < borderL + Math.round(80 * scale); x++) {
      let n = 0;
      for (let y = yAfter; y < yAfter + Math.round(80 * scale); y++) if (isRule(x, y)) n++;
      if (n > Math.round(25 * scale)) {
        col1R = x;
        break;
      }
    }
  }

  // text insets: leftmost ink in first column (min x across band) — avoids wrapped col2 headers
  const measureInk = (y0: number, y1: number, darkOnly: boolean) => {
    let best: { x: number; y: number; leftPt: number; topPt: number } | null = null;
    for (let y = y0; y < y1; y++) {
      for (let x = borderL + 2; x < col1R - 2; x++) {
        const hit = darkOnly ? isDarkInk(x, y) : isInk(x, y);
        if (!hit) continue;
        let n = 0;
        for (let dx = 0; dx < 5; dx++) {
          if (darkOnly ? isDarkInk(x + dx, y) : isInk(x + dx, y)) n++;
        }
        if (n < 3) continue;
        const cand = {
          x,
          y,
          leftPt: +((x - borderL - 1) / scale).toFixed(2),
          topPt: +((y - y0) / scale).toFixed(2),
        };
        if (!best || cand.x < best.x) best = cand;
        break;
      }
    }
    return best;
  };
  const headerInk = baseY > 0 ? measureInk(yAfter, yAfter + Math.round(70 * scale), true) : null;
  let bodyInk: ReturnType<typeof measureInk> = null;
  if (joinYs[0]) {
    const by0 = joinYs[0] + 1;
    bodyInk = measureInk(by0, by0 + Math.round(80 * scale), false);
  } else if (baseY > 0) {
    bodyInk = measureInk(yAfter + Math.round(40 * scale), yAfter + Math.round(140 * scale), false);
  }

  // mid-rule meet: re-evaluate using best y in cluster (min gap)
  let maxMeetGapLpt2 = maxMeetGapLpt;
  let maxMeetGapRpt2 = maxMeetGapRpt;
  if (joinYs.length) {
    const gapsL: number[] = [];
    const gapsR: number[] = [];
    for (const y0 of joinYs) {
      let bestGL = 999;
      let bestGR = 999;
      for (let y = y0 - 2; y <= y0 + 2; y++) {
        let gL = 0;
        for (let x = borderL + 1; x < borderL + Math.round(6 * scale); x++) {
          if (isRule(x, y)) break;
          if (isCream(x, y) || isPage(x, y)) gL++;
          else break;
        }
        let gR = 0;
        for (let x = borderR - 1; x > borderR - Math.round(6 * scale); x--) {
          if (isRule(x, y)) break;
          if (isCream(x, y) || isPage(x, y)) gR++;
          else break;
        }
        bestGL = Math.min(bestGL, gL);
        bestGR = Math.min(bestGR, gR);
      }
      gapsL.push(bestGL === 999 ? 0 : bestGL);
      gapsR.push(bestGR === 999 ? 0 : bestGR);
    }
    maxMeetGapLpt2 = gapsL.length ? +(Math.max(...gapsL) / scale).toFixed(2) : 0;
    maxMeetGapRpt2 = gapsR.length ? +(Math.max(...gapsR) / scale).toFixed(2) : 0;
  }

  // continuation chrome: for cont pages, chip/title present and seam ok
  const hasTitleChrome = baseY > 0 && hairCount >= 1;

  return {
    baseY,
    yAfter,
    borderL,
    borderR,
    chipL,
    chipR,
    chipAlignLpt,
    chipAlignRpt,
    hairPt,
    hairCount,
    doubleHairline,
    creamGapPt,
    weldPage,
    weldCanvas,
    joinCount: joinYs.length,
    creamJoinsL,
    creamJoinsR,
    notchClustersL,
    notchClustersR,
    jitterLpt,
    jitterRpt,
    maxMeetGapLpt: maxMeetGapLpt2,
    maxMeetGapRpt: maxMeetGapRpt2,
    doubleOuterCount: doubleOuterHits.length,
    headerInsetL: headerInk?.leftPt ?? null,
    headerInsetT: headerInk?.topPt ?? null,
    bodyInsetL: bodyInk?.leftPt ?? null,
    bodyInsetT: bodyInk?.topPt ?? null,
    hasTitleChrome,
    ruleClusters: ruleClusters.slice(0, 4),
    joinYs: joinYs.slice(0, 8),
  };
}

const report: Record<string, unknown>[] = [];
const familyInsets: Record<string, number[]> = {};

for (const dpi of DPIS) {
  const scale = dpi / 72;
  for (const t of targets) {
    const page = await doc.getPage(t.page);
    const vp = page.getViewport({ scale });
    const items = (await page.getTextContent()).items as {
      str: string;
      transform: number[];
    }[];
    const cands: Cand[] = [];
    for (const it of items) {
      if (!it.str || it.str.length < 2) continue;
      const [x, y] = vp.convertToViewportPoint(it.transform[4], it.transform[5]);
      cands.push({ str: it.str, x, y });
    }
    const anchor =
      t.pick?.(cands, scale) ??
      cands
        .filter(
          (x) =>
            x.str.includes(t.titleNeedle) ||
            x.str.includes(t.titleNeedle.slice(0, 14)),
        )
        .sort((a, b) => b.str.length - a.str.length)[0] ??
      cands.find((x) => x.str.includes("Tabelle:"));
    if (!anchor) {
      report.push({ id: t.id, family: t.family, page: t.page, dpi, ok: false, fail: ["no-anchor"] });
      continue;
    }

    const canvas = createCanvas(Math.ceil(vp.width), Math.ceil(vp.height));
    await page.render({ canvasContext: canvas.getContext("2d"), viewport: vp }).promise;
    const { data, width: W, height: H } = canvas
      .getContext("2d")
      .getImageData(0, 0, canvas.width, canvas.height);
    const m = analyze(data, W, H, scale, anchor);

    const crops: Record<string, string> = {};
    if (m.baseY > 0) {
      const save = (name: string, x0: number, y0: number, w: number, h: number) => {
        const c = createCanvas(Math.max(1, w), Math.max(1, h));
        c.getContext("2d").drawImage(
          canvas,
          Math.max(0, x0),
          Math.max(0, y0),
          w,
          h,
          0,
          0,
          w,
          h,
        );
        const p = `${outDir}/audit-long-${t.id}-d${dpi}-${name}.png`;
        writeFileSync(p, c.toBuffer("image/png"));
        return `audit-long-${t.id}-d${dpi}-${name}.png`;
      };
      const bx = Math.max(0, m.borderL - 10);
      crops.seam = save(
        "seam",
        bx,
        m.baseY - Math.round(28 * scale),
        Math.min(W - bx, Math.round(260 * scale)),
        Math.round(70 * scale),
      );
      crops.joinL = save(
        "joinL",
        bx,
        m.baseY + Math.round(12 * scale),
        Math.round(90 * scale),
        Math.round(160 * scale),
      );
      crops.joinR = save(
        "joinR",
        Math.max(0, m.borderR - Math.round(50 * scale)),
        m.baseY + Math.round(12 * scale),
        Math.round(90 * scale),
        Math.round(160 * scale),
      );
      crops.chip = save(
        "chip",
        Math.max(0, Math.floor(anchor.x) - Math.round(10 * scale)),
        Math.max(0, Math.floor(anchor.y) - Math.round(14 * scale)),
        Math.round(280 * scale),
        Math.round(40 * scale),
      );
      crops.cell = save(
        "cell",
        bx,
        m.yAfter,
        Math.round(180 * scale),
        Math.round(90 * scale),
      );
      if (m.joinYs[1]) {
        crops.midmeet = save(
          "midmeet",
          bx,
          m.joinYs[1] - Math.round(8 * scale),
          Math.round(100 * scale),
          Math.round(24 * scale),
        );
      }
      // full table strip for continuation
      if (t.role !== "single" || dpi === 288) {
        crops.head = save(
          "head",
          bx,
          Math.max(0, m.baseY - Math.round(36 * scale)),
          Math.min(W - bx, m.borderR - bx + 20),
          Math.round(120 * scale),
        );
      }
    }

    const fail: string[] = [];
    const checks: Record<string, { ok: boolean; detail: string }> = {};

    // 1) seam
    const seamOk =
      m.hairCount === 1 &&
      !m.doubleHairline &&
      m.hairPt <= 1.1 &&
      m.creamGapPt <= 0.25 &&
      m.weldPage < 20;
    checks.seam = {
      ok: seamOk,
      detail: `hair=${m.hairCount}x${m.hairPt}pt creamGap=${m.creamGapPt} weldPage=${m.weldPage} weldCanvas=${m.weldCanvas}`,
    };
    if (!seamOk) {
      if (m.hairCount !== 1) fail.push(`hairlines:${m.hairCount}`);
      if (m.doubleHairline) fail.push("double-hairline");
      if (m.hairPt > 1.1) fail.push(`hairline-thick:${m.hairPt}`);
      if (m.creamGapPt > 0.25) fail.push(`cream-gap:${m.creamGapPt}`);
      if (m.weldPage >= 20) fail.push(`weld-page:${m.weldPage}`);
    }

    // 2) chip L/R vs body
    const alignTol = dpi >= 288 ? 1.25 : 1.75;
    const alignLok =
      m.chipAlignLpt == null || Math.abs(m.chipAlignLpt) <= alignTol;
    const alignRok =
      m.chipAlignRpt == null || Math.abs(m.chipAlignRpt) <= alignTol;
    // chip R often is title-chip right, not full table width — only require L for title chips
    // For full-width chrome chips (glossar style), both; for window title chips spanning full, both.
    // Long tables: title chip spans full table width typically.
    const alignOk = alignLok && (m.chipR < 0 || alignRok || Math.abs(m.chipAlignRpt ?? 99) > 30);
    // if chipR is far from borderR (>30pt), it's an inner divider — ignore
    const chipRIsOuter = m.chipAlignRpt != null && Math.abs(m.chipAlignRpt) <= 8;
    const alignOk2 = alignLok && (!chipRIsOuter || alignRok);
    checks.chipAlign = {
      ok: alignOk2,
      detail: `chipL-borderL=${m.chipAlignLpt}pt chipR-borderR=${m.chipAlignRpt}pt outerR=${chipRIsOuter}`,
    };
    if (!alignOk2) {
      if (!alignLok) fail.push(`chip-align-L:${m.chipAlignLpt}`);
      if (chipRIsOuter && !alignRok) fail.push(`chip-align-R:${m.chipAlignRpt}`);
    }

    // 3) continuous outer edges
    const jitterTol = dpi <= 144 ? 1.25 : 0.85;
    const edgeOk =
      m.creamJoinsL === 0 &&
      m.creamJoinsR === 0 &&
      m.notchClustersL === 0 &&
      m.notchClustersR === 0 &&
      m.jitterLpt <= jitterTol &&
      m.jitterRpt <= jitterTol;
    checks.outerEdges = {
      ok: edgeOk,
      detail: `creamL/R=${m.creamJoinsL}/${m.creamJoinsR} notchL/R=${m.notchClustersL}/${m.notchClustersR} jitterL/R=${m.jitterLpt}/${m.jitterRpt}pt joins=${m.joinCount}`,
    };
    if (!edgeOk) {
      if (m.creamJoinsL) fail.push(`cream-joins-L:${m.creamJoinsL}`);
      if (m.creamJoinsR) fail.push(`cream-joins-R:${m.creamJoinsR}`);
      if (m.notchClustersL) fail.push(`notch-L:${m.notchClustersL}`);
      if (m.notchClustersR) fail.push(`notch-R:${m.notchClustersR}`);
      if (m.jitterLpt > jitterTol) fail.push(`jitter-L:${m.jitterLpt}`);
      if (m.jitterRpt > jitterTol) fail.push(`jitter-R:${m.jitterRpt}`);
    }

    // 4) mid-rule meets inner face
    const meetOk =
      m.maxMeetGapLpt <= 1.25 &&
      m.maxMeetGapRpt <= 1.25 &&
      m.doubleOuterCount === 0;
    checks.midruleMeet = {
      ok: meetOk,
      detail: `maxGapL/R=${m.maxMeetGapLpt}/${m.maxMeetGapRpt}pt doubleOuter=${m.doubleOuterCount}`,
    };
    if (!meetOk) {
      if (m.maxMeetGapLpt > 1.0) fail.push(`mid-gap-L:${m.maxMeetGapLpt}`);
      if (m.maxMeetGapRpt > 1.0) fail.push(`mid-gap-R:${m.maxMeetGapRpt}`);
      if (m.doubleOuterCount) fail.push(`double-outer:${m.doubleOuterCount}`);
    }

    // 5) text insets
    const leftOk =
      m.headerInsetL != null &&
      m.headerInsetL >= 3.5 &&
      m.headerInsetL <= 10.5 &&
      (m.bodyInsetL == null ||
        (m.bodyInsetL >= 3.5 &&
          m.bodyInsetL <= 10.5 &&
          Math.abs(m.bodyInsetL - m.headerInsetL) <= 2.5));
    const topOk =
      m.headerInsetT != null && m.headerInsetT >= 3 && m.headerInsetT <= 12.5;
    checks.insets = {
      ok: !!(leftOk && topOk),
      detail: `header L/T=${m.headerInsetL}/${m.headerInsetT} body L/T=${m.bodyInsetL}/${m.bodyInsetT}`,
    };
    if (!(leftOk && topOk)) {
      if (!leftOk) fail.push(`inset-L:h=${m.headerInsetL},b=${m.bodyInsetL}`);
      if (!topOk) fail.push(`inset-T:${m.headerInsetT}`);
    }
    if (m.headerInsetL != null) {
      (familyInsets[t.family] ??= []).push(m.headerInsetL);
    }

    // 6) continuation chrome
    let contOk = true;
    let contDetail = "n/a (single-page table)";
    if (t.role === "cont" || t.role === "last" || t.role === "first") {
      contOk = m.hasTitleChrome && m.creamGapPt <= 0.25 && m.hairCount === 1;
      contDetail = `role=${t.role} titleChrome=${m.hasTitleChrome} hair=${m.hairCount} gap=${m.creamGapPt}`;
      if (!contOk) fail.push(`cont-chrome:${t.role}`);
    }
    checks.continuation = { ok: contOk, detail: contDetail };

    const row = {
      id: t.id,
      family: t.family,
      page: t.page,
      role: t.role,
      tableId: t.tableId,
      dpi,
      scale,
      anchor: anchor.str,
      anchorXY: [+anchor.x.toFixed(1), +anchor.y.toFixed(1)],
      metrics: {
        hairPt: m.hairPt,
        hairCount: m.hairCount,
        doubleHairline: m.doubleHairline,
        creamGapPt: m.creamGapPt,
        chipAlignLpt: m.chipAlignLpt,
        chipAlignRpt: m.chipAlignRpt,
        creamJoinsL: m.creamJoinsL,
        creamJoinsR: m.creamJoinsR,
        notchClustersL: m.notchClustersL,
        notchClustersR: m.notchClustersR,
        jitterLpt: m.jitterLpt,
        jitterRpt: m.jitterRpt,
        maxMeetGapLpt: m.maxMeetGapLpt,
        maxMeetGapRpt: m.maxMeetGapRpt,
        doubleOuterCount: m.doubleOuterCount,
        headerInsetL: m.headerInsetL,
        headerInsetT: m.headerInsetT,
        bodyInsetL: m.bodyInsetL,
        bodyInsetT: m.bodyInsetT,
        joinCount: m.joinCount,
        borderL: m.borderL,
        borderR: m.borderR,
        weldPage: m.weldPage,
        weldCanvas: m.weldCanvas,
      },
      checks,
      crops,
      ok: fail.length === 0,
      fail,
    };
    report.push(row);
    console.log(
      `[DEBUG] ${t.id}@${dpi} ok=${row.ok} ${fail.join(",") || "PASS"} | ${checks.seam.detail} | ${checks.outerEdges.detail}`,
    );
  }
}

// Abkürzungen probe
{
  let found = false;
  for (let p = 1; p <= doc.numPages; p++) {
    const page = await doc.getPage(p);
    const text = ((await page.getTextContent()).items as { str: string }[])
      .map((i) => i.str)
      .join(" ");
    if (/Abkürzungen|Abkürzungsverzeichnis|Abbreviat/i.test(text)) {
      found = true;
      report.push({
        id: "abkuerzungen",
        family: "Abkürzungen",
        page: p,
        role: "single",
        tableId: "?",
        dpi: null,
        ok: false,
        fail: ["present-but-not-audited-as-long-table"],
        note: "Abkürzungen text found; inspect if tabular",
      });
      break;
    }
  }
  if (!found) {
    report.push({
      id: "abkuerzungen",
      family: "Abkürzungen",
      page: null,
      role: "absent",
      tableId: null,
      dpi: null,
      ok: true,
      fail: [],
      checks: {
        presence: { ok: true, detail: "no Abkürzungen table in PDF" },
      },
      note: "N/A — not present",
    });
  }
}

const summary = {
  generatedAt: new Date().toISOString(),
  pdf: pdfPath,
  dpis: DPIS,
  targets: targets.map((t) => ({
    id: t.id,
    family: t.family,
    page: t.page,
    role: t.role,
    tableId: t.tableId,
  })),
  passCount: report.filter((r) => r.ok).length,
  failCount: report.filter((r) => !r.ok).length,
  inconsistentFamilies: [
    ...new Set(
      report.filter((r) => !r.ok && r.family).map((r) => r.family as string),
    ),
  ],
  familyInsetRanges: Object.fromEntries(
    Object.entries(familyInsets).map(([k, vs]) => [
      k,
      { min: Math.min(...vs), max: Math.max(...vs), n: vs.length },
    ]),
  ),
};

writeFileSync(
  `${outDir}/audit-agent-long.json`,
  JSON.stringify({ summary, rows: report }, null, 2),
);
console.log(
  `[DEBUG] done pass=${summary.passCount} fail=${summary.failCount} inconsistent=${summary.inconsistentFamilies.join("|") || "none"}`,
);
