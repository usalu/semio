#!/usr/bin/env bun
/**
 * [DEBUG] Cross-type consistency matrix for Zwischenbericht tables @216dpi.
 * Metrics: chip/body delta, join cream, border jitter, seam cream, text air/inset.
 */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const pdfPath =
  process.argv[2] ??
  join(ticketDir, "../../../../../../mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf");
const DPI = 216;
const scale = DPI / 72;

type Spec = {
  id: string;
  page: number;
  name: string;
  kind: "toc" | "window" | "project" | "long";
  needle: string;
};

const specs: Spec[] = [
  { id: "toc", page: 3, name: "TOC", kind: "toc", needle: "Inhaltsverzeichnis" },
  { id: "meilensteine", page: 18, name: "Meilensteine", kind: "window", needle: "Meilensteine" },
  { id: "risiken", page: 19, name: "Risiken", kind: "window", needle: "Risiken und" },
  { id: "kopfbau", page: 24, name: "Kopfbau", kind: "project", needle: "Kopfbau Halle 118" },
  { id: "huerden", page: 76, name: "Huerden", kind: "long", needle: "der Wiederverwendung" },
  { id: "ueberblick", page: 77, name: "Ueberblick", kind: "window", needle: "berblick" },
  { id: "marktplaetze", page: 78, name: "Marktplaetze", kind: "long", needle: "Zugang und Kan" },
  { id: "datenfelder", page: 79, name: "Datenfelder", kind: "long", needle: "Datenfelder und Beschaffung" },
  { id: "glossar", page: 121, name: "Glossar", kind: "long", needle: "Glossar" },
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

function median(xs: number[]): number {
  if (!xs.length) return NaN;
  const a = [...xs].sort((u, v) => u - v);
  const m = Math.floor(a.length / 2);
  return a.length % 2 ? a[m]! : (a[m - 1]! + a[m]!) / 2;
}

type Row = {
  id: string;
  page: number;
  name: string;
  kind: string;
  title: string;
  dpi: number;
  scale: number;
  chipBodyDL: number | null;
  chipBodyDR: number | null;
  joinCreamL: number;
  joinCreamR: number;
  borderJitterL: number;
  borderJitterR: number;
  seamCreamUnderChip: number;
  headerTopAirPt: number | null;
  bodyTopAirPt: number | null;
  leftTextInsetPt: number | null;
  chipL: number | null;
  chipR: number | null;
  bodyL: number | null;
  bodyR: number | null;
  outliers: string[];
  crop: string | null;
};

const rows: Row[] = [];

for (const spec of specs) {
  const page = await doc.getPage(spec.page);
  const viewport = page.getViewport({ scale });
  const items = (await page.getTextContent()).items as { str: string; transform: number[] }[];
  const cands = items
    .filter((it) => it.str && it.str.includes(spec.needle))
    .map((it) => {
      const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
      return { x, y, str: it.str };
    })
    .sort((a, b) => b.str.length - a.str.length || a.y - b.y);

  // Prefer chip title nearest a "Tabelle:" label (or lowest mid-page short match)
  const tableLabels = items
    .filter((it) => it.str && it.str.startsWith("Tabelle"))
    .map((it) => {
      const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
      return { x, y, str: it.str };
    });
  let title = cands[0] ?? null;
  if (cands.length) {
    const chipish = cands.filter((c) => c.str.length <= 60 && c.y > 40 * scale);
    const pool = chipish.length ? chipish : cands;
    if (tableLabels.length) {
      const tl = tableLabels.sort((a, b) => a.y - b.y)[0]!;
      title = [...pool].sort((a, b) => Math.abs(a.y - tl.y) - Math.abs(b.y - tl.y) || b.str.length - a.str.length)[0]!;
    } else {
      title = [...pool].sort((a, b) => a.y - b.y)[pool.length - 1]!;
    }
  }

  if (!title) {
    rows.push({
      id: spec.id,
      page: spec.page,
      name: spec.name,
      kind: spec.kind,
      title: spec.needle,
      dpi: DPI,
      scale,
      chipBodyDL: null,
      chipBodyDR: null,
      joinCreamL: 0,
      joinCreamR: 0,
      borderJitterL: 0,
      borderJitterR: 0,
      seamCreamUnderChip: 0,
      headerTopAirPt: null,
      bodyTopAirPt: null,
      leftTextInsetPt: null,
      chipL: null,
      chipR: null,
      bodyL: null,
      bodyR: null,
      outliers: ["no-title"],
      crop: null,
    });
    console.log("[DEBUG] " + spec.id + ": no title");
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
    if (x < 0 || y < 0 || x >= W || y >= H) return 255;
    const [r, g, b] = rgb(x, y);
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  };
  const isRule = (x: number, y: number) => {
    if (x < 0 || y < 0 || x >= W || y >= H) return false;
    const [r, g, b] = rgb(x, y);
    // hairline chrome ~ (123,130,125); avoid matching grey body glyphs
    return Math.abs(r - 123) + Math.abs(g - 130) + Math.abs(b - 125) <= 45;
  };
  const isPage = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.abs(r - PAGE.r) + Math.abs(g - PAGE.g) + Math.abs(b - PAGE.b) <= 14;
  };
  const isInk = (x: number, y: number) => {
    if (x < 0 || y < 0 || x >= W || y >= H) return false;
    if (isRule(x, y)) return false;
    const [r, g, b] = rgb(x, y);
    // grey body glyphs ~L145; reject cream/page and rule-core; border AA is L>~170
    if (Math.abs(r - PAGE.r) + Math.abs(g - PAGE.g) + Math.abs(b - PAGE.b) <= 18) return false;
    if (Math.abs(r - 240) + Math.abs(g - 236) + Math.abs(b - 221) <= 18) return false;
    if (Math.max(r, g, b) - Math.min(r, g, b) > 40) return false;
    return lum(x, y) < 155;
  };
  const isPhoto = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return Math.max(r, g, b) - Math.min(r, g, b) > 40;
  };

  // chip baseline under title
  const xScan0 = Math.max(4, Math.floor(title.x - 50 * scale));
  const xScan1 = Math.min(W - 4, Math.floor(title.x + 260 * scale));
  let chipBase0 = -1;
  let chipBase1 = -1;
  for (let y = Math.floor(title.y) + 1; y < Math.floor(title.y) + Math.round(50 * scale); y++) {
    let n = 0;
    for (let x = xScan0; x < xScan1; x += 2) if (isRule(x, y)) n++;
    if (n > ((xScan1 - xScan0) / 2) * 0.2) {
      if (chipBase0 < 0) chipBase0 = y;
      chipBase1 = y;
    } else if (chipBase0 >= 0) break;
  }

  // Find chip outer L/R from baseline row span (full-width rule under chips)
  let chipSpanL = -1;
  let chipSpanR = -1;
  if (chipBase0 >= 0) {
    const y = Math.floor((chipBase0 + chipBase1) / 2);
    for (let x = Math.max(2, Math.floor(title.x) - Math.round(40 * scale)); x < W - 2; x++) {
      if (isRule(x, y) || lum(x, y) < 160) {
        chipSpanL = x;
        break;
      }
    }
    for (let x = Math.min(W - 2, Math.floor(title.x) + Math.round(520 * scale)); x > 2; x--) {
      if (isRule(x, y) || lum(x, y) < 160) {
        chipSpanR = x;
        break;
      }
    }
  }

  // body L/R: darkest vertical rule columns under chip (center of hairline)
  const yBody0 = chipBase1 >= 0 ? chipBase1 + 4 : Math.floor(title.y) + 10;
  const yBody1 = Math.min(H - 2, yBody0 + Math.round(360 * scale));

  const darkestCol = (x0: number, x1: number) => {
    let bestX = x0;
    let bestScore = Infinity;
    const lo = Math.min(x0, x1);
    const hi = Math.max(x0, x1);
    for (let x = lo; x <= hi; x++) {
      let sum = 0;
      let n = 0;
      for (let y = yBody0; y < yBody1; y += 2) {
        sum += lum(x, y);
        n++;
      }
      const avg = sum / Math.max(1, n);
      // require mostly rule-like
      let ruleN = 0;
      for (let y = yBody0; y < yBody1; y += 3) if (isRule(x, y)) ruleN++;
      if (ruleN < ((yBody1 - yBody0) / 3) * 0.25) continue;
      if (avg < bestScore) {
        bestScore = avg;
        bestX = x;
      }
    }
    return bestScore < 200 ? bestX : -1;
  };

  const bodyL = darkestCol(
    Math.max(2, (chipSpanL >= 0 ? chipSpanL : Math.floor(title.x)) - Math.round(4 * scale)),
    Math.floor(title.x) + Math.round(8 * scale),
  );
  const bodyR = darkestCol(
    Math.min(W - 3, (chipSpanR >= 0 ? chipSpanR : Math.floor(W * 0.9)) + Math.round(4 * scale)),
    Math.max(bodyL + 40, Math.floor(title.x) + Math.round(80 * scale)),
  );

  // chip L/R in chip band: leftmost/rightmost dark near body edges
  const chipY0 = Math.max(0, chipBase0 - Math.round(12 * scale));
  const chipY1 = Math.max(chipY0 + 1, chipBase0 - 1);
  let chipL = bodyL;
  for (let x = Math.max(2, bodyL - Math.round(5 * scale)); x <= bodyL + Math.round(5 * scale); x++) {
    let ink = 0;
    for (let y = chipY0; y <= chipY1; y++) if (lum(x, y) < 155) ink++;
    if (ink >= Math.max(2, Math.floor((chipY1 - chipY0 + 1) * 0.4))) {
      chipL = x;
      break;
    }
  }
  let chipR = bodyR;
  if (bodyR >= 0) {
    for (let x = Math.min(W - 2, bodyR + Math.round(5 * scale)); x >= bodyR - Math.round(5 * scale); x--) {
      let ink = 0;
      for (let y = chipY0; y <= chipY1; y++) if (lum(x, y) < 155) ink++;
      if (ink >= Math.max(2, Math.floor((chipY1 - chipY0 + 1) * 0.4))) {
        chipR = x;
        break;
      }
    }
  }

  const chipBodyDL = bodyL >= 0 && chipL >= 0 ? bodyL - chipL : null;
  const chipBodyDR = bodyR >= 0 && chipR >= 0 ? chipR - bodyR : null;

  // interior hrules
  const midX0 = bodyL + Math.round(25 * scale);
  const midX1 = Math.min(
    bodyR > 0 ? bodyR - Math.round(15 * scale) : bodyL + Math.round(220 * scale),
    bodyL + Math.round(220 * scale),
  );
  const hrules: number[] = [];
  if (chipBase1 >= 0 && bodyL >= 0) {
    for (let y = chipBase1 + Math.round(8 * scale); y < Math.min(H - 2, chipBase1 + Math.round(400 * scale)); y++) {
      let n = 0;
      for (let x = midX0; x < midX1; x += 2) if (isRule(x, y)) n++;
      if (n > ((midX1 - midX0) / 2) * 0.4) {
        if (!hrules.length || y > hrules[hrules.length - 1]! + 2) hrules.push(y);
      }
    }
  }

  // join cream: page/cream pierce on border at hrule (strict, like final-compare)
  let joinCreamL = 0;
  let joinCreamR = 0;
  let borderJitterL = 0;
  let borderJitterR = 0;
  // only score joins where the outer border is expected continuous:
  // require a dark vertical neighbor above/below (pillar), else skip (inner-only midrule AA)
  for (const by of hrules.slice(0, 8)) {
    const minL = (bx: number, y: number) =>
      Math.min(lum(bx - 1, y), lum(bx, y), lum(bx + 1, y));
    const pillarOk = (bx: number) => minL(bx, by - 2) < 160 && minL(bx, by + 2) < 160;
    if (pillarOk(bodyL) && minL(bodyL, by) > 175) joinCreamL++;
    if (bodyR >= 0 && pillarOk(bodyR) && minL(bodyR, by) > 175) joinCreamR++;

    // jitter from mid-row (away from join AA): darkest in +/-2
    const midY = Math.min(H - 1, by + Math.round(10 * scale));
    // skip if midY lands on another rule
    let midRule = 0;
    for (let x = midX0; x < midX1; x += 2) if (isRule(x, midY)) midRule++;
    if (midRule > ((midX1 - midX0) / 2) * 0.35) continue;

    let bestLX = bodyL;
    let bestLL = 999;
    for (let x = bodyL - 2; x <= bodyL + 2; x++) {
      const L = lum(x, midY);
      if (L < bestLL) {
        bestLL = L;
        bestLX = x;
      }
    }
    if (bestLL < 150 && isRule(bestLX, midY)) {
      borderJitterL = Math.max(borderJitterL, Math.abs(bestLX - bodyL));
    }

    if (bodyR >= 0) {
      let bestRX = bodyR;
      let bestRL = 999;
      for (let x = bodyR - 2; x <= bodyR + 2; x++) {
        const L = lum(x, midY);
        if (L < bestRL) {
          bestRL = L;
          bestRX = x;
        }
      }
      if (bestRL < 150 && isRule(bestRX, midY)) {
        borderJitterR = Math.max(borderJitterR, Math.abs(bestRX - bodyR));
      }
    }
  }

  // seam cream under chip (welded => 0)
  let seamCreamUnderChip = 0;
  if (chipBase1 >= 0 && bodyL >= 0) {
    for (let y = chipBase1 + 1; y < chipBase1 + Math.round(6 * scale); y++) {
      const borderMin = Math.min(lum(bodyL - 1, y), lum(bodyL, y), lum(bodyL + 1, y));
      if (borderMin < 160 || isRule(bodyL, y)) break;
      if (borderMin > 200 && isPage(Math.max(0, bodyL - 6), y)) seamCreamUnderChip++;
      else break;
    }
  }

  let headerTopAirPt: number | null = null;
  let bodyTopAirPt: number | null = null;
  let leftTextInsetPt: number | null = null;
  const afterChip = chipBase1 + 1;
  const firstInterior = hrules[0] ?? afterChip + Math.round(30 * scale);

  if (spec.kind === "project") {
    for (let y = afterChip; y < afterChip + Math.round(40 * scale); y++) {
      let photo = 0;
      for (let x = bodyL + 8; x < bodyL + Math.round(90 * scale); x++) if (isPhoto(x, y)) photo++;
      if (photo > 20) {
        headerTopAirPt = +((y - afterChip) / scale).toFixed(2);
        bodyTopAirPt = headerTopAirPt;
        break;
      }
    }
  } else {
    let hTop = -1;
    let hLeft = -1;
    for (let y = afterChip + 1; y < firstInterior; y++) {
      let ruleN = 0;
      for (let x = midX0; x < midX1; x += 2) if (isRule(x, y)) ruleN++;
      if (ruleN > ((midX1 - midX0) / 2) * 0.35) continue;
      for (let x = bodyL + Math.round(2 * scale); x < bodyL + Math.round(120 * scale); x++) {
        if (!isInk(x, y)) continue;
        let run = 0;
        for (let dx = 0; dx < 8; dx++) if (isInk(x + dx, y)) run++;
        if (run >= 3) {
          hTop = y;
          hLeft = x;
          break;
        }
      }
      if (hTop >= 0) break;
    }
    if (hTop >= 0) {
      headerTopAirPt = +((hTop - afterChip) / scale).toFixed(2);
      let left = hLeft;
      for (let y = hTop; y < hTop + Math.round(7 * scale); y++) {
        for (let x = bodyL + Math.round(1.5 * scale); x <= hLeft; x++) {
          if (isInk(x, y)) {
            left = Math.min(left, x);
            break;
          }
        }
      }
      leftTextInsetPt = +((left - bodyL) / scale).toFixed(2);
    }

    // body top: after first interior hrule (header bottom)
    const bodyRule = hrules[0] ?? -1;
    if (bodyRule >= 0) {
      let bTop = -1;
      let bLeft = -1;
      for (let y = bodyRule + 2; y < bodyRule + Math.round(40 * scale); y++) {
        let ruleN = 0;
        for (let x = midX0; x < midX1; x += 2) if (isRule(x, y)) ruleN++;
        if (ruleN > ((midX1 - midX0) / 2) * 0.35) continue;
        for (let x = bodyL + Math.round(2 * scale); x < bodyL + Math.round(160 * scale); x++) {
          if (!isInk(x, y)) continue;
          let run = 0;
          for (let dx = 0; dx < 8; dx++) if (isInk(x + dx, y)) run++;
          if (run >= 3) {
            bTop = y;
            bLeft = x;
            break;
          }
        }
        if (bTop >= 0) break;
      }
      if (bTop >= 0) {
        bodyTopAirPt = +((bTop - bodyRule - 1) / scale).toFixed(2);
        if (leftTextInsetPt == null) {
          let left = bLeft;
          for (let y = bTop; y < bTop + Math.round(7 * scale); y++) {
            for (let x = bodyL + Math.round(1.5 * scale); x <= bLeft; x++) {
              if (isInk(x, y)) {
                left = Math.min(left, x);
                break;
              }
            }
          }
          leftTextInsetPt = +((left - bodyL) / scale).toFixed(2);
        }
      }
    }
  }

  const cx0 = Math.max(0, (bodyL >= 0 ? bodyL : Math.floor(title.x)) - 8);
  const cy0 = Math.max(0, (chipBase0 >= 0 ? chipBase0 : Math.floor(title.y)) - Math.round(18 * scale));
  const cw = Math.min(W - cx0, Math.round(240 * scale));
  const ch = Math.min(H - cy0, Math.round(100 * scale));
  const cropCanvas = createCanvas(cw, ch);
  cropCanvas.getContext("2d").drawImage(canvas, cx0, cy0, cw, ch, 0, 0, cw, ch);
  const cropPath = join(ticketDir, "audit-mtx-p" + spec.page + ".png");
  writeFileSync(cropPath, cropCanvas.toBuffer("image/png"));
  writeFileSync(
    join(ticketDir, "audit-mtx-full-p" + String(spec.page).padStart(3, "0") + ".png"),
    canvas.toBuffer("image/png"),
  );

  const row: Row = {
    id: spec.id,
    page: spec.page,
    name: spec.name,
    kind: spec.kind,
    title: title.str,
    dpi: DPI,
    scale,
    chipBodyDL,
    chipBodyDR,
    joinCreamL,
    joinCreamR,
    borderJitterL,
    borderJitterR,
    seamCreamUnderChip,
    headerTopAirPt,
    bodyTopAirPt,
    leftTextInsetPt,
    chipL,
    chipR,
    bodyL,
    bodyR,
    outliers: [],
    crop: cropPath.replace(/\\/g, "/"),
  };
  rows.push(row);
  console.log(
    "[DEBUG] " +
      spec.name +
      " p" +
      spec.page +
      " title=" +
      JSON.stringify(title.str) +
      " chip=" +
      chipBase0 +
      "-" +
      chipBase1 +
      " dL/dR=" +
      chipBodyDL +
      "/" +
      chipBodyDR +
      " creamL/R=" +
      joinCreamL +
      "/" +
      joinCreamR +
      " jitL/R=" +
      borderJitterL +
      "/" +
      borderJitterR +
      " seam=" +
      seamCreamUnderChip +
      " hTop=" +
      headerTopAirPt +
      " bTop=" +
      bodyTopAirPt +
      " inset=" +
      leftTextInsetPt +
      " bodyL/R=" +
      bodyL +
      "/" +
      bodyR +
      " hrules=" +
      JSON.stringify(hrules.slice(0, 6)),
  );
}

const insets = rows
  .map((r) => r.leftTextInsetPt)
  .filter((v): v is number => v != null && Number.isFinite(v));
const insetMedian = median(insets);
const insetMadTol = 1.25;

for (const r of rows) {
  if (r.outliers.includes("no-title")) continue;
  const o: string[] = [];
  if (r.chipBodyDL != null && Math.abs(r.chipBodyDL) > 2) o.push("chipBodyDL:" + r.chipBodyDL);
  if (r.chipBodyDR != null && Math.abs(r.chipBodyDR) > 2) o.push("chipBodyDR:" + r.chipBodyDR);
  if (r.joinCreamL > 0) o.push("joinCreamL:" + r.joinCreamL);
  if (r.joinCreamR > 0) o.push("joinCreamR:" + r.joinCreamR);
  if (r.borderJitterL > 1) o.push("borderJitterL:" + r.borderJitterL);
  if (r.borderJitterR > 1) o.push("borderJitterR:" + r.borderJitterR);
  if (r.seamCreamUnderChip > 0) o.push("seamCreamUnderChip:" + r.seamCreamUnderChip);
  if (r.leftTextInsetPt != null && Math.abs(r.leftTextInsetPt - insetMedian) > insetMadTol) {
    o.push("leftTextInsetPt:" + r.leftTextInsetPt + " (median " + insetMedian.toFixed(2) + ")");
  }
  r.outliers = o;
}

const overall = rows.every((r) => r.outliers.length === 0) ? "PASS" : "FAIL";
const outlierList = rows
  .filter((r) => r.outliers.length)
  .map((r) => ({ id: r.id, page: r.page, name: r.name, outliers: r.outliers }));

const report = {
  pdf: pdfPath.replace(/\\/g, "/"),
  dpi: DPI,
  scale,
  overall,
  medianLeftTextInsetPt: +insetMedian.toFixed(2),
  insetOutlierTolPt: insetMadTol,
  outlierList,
  tables: rows,
};

writeFileSync(join(ticketDir, "audit-agent-matrix.json"), JSON.stringify(report, null, 2));

const na = "-";
const mdLines: string[] = [];
mdLines.push("# Cross-Type Consistency Matrix - Zwischenbericht Tables");
mdLines.push("");
mdLines.push("PDF: `" + report.pdf + "`");
mdLines.push("Raster: **" + DPI + "dpi** (scale " + scale.toFixed(3) + ") via `@napi-rs/canvas` / pdfjs.");
mdLines.push("Measured: " + new Date().toISOString());
mdLines.push("");
mdLines.push("## Overall: **" + overall + "**");
mdLines.push("");
mdLines.push(
  "| Table | Page | Kind | chipBodyDL | chipBodyDR | joinCreamL | joinCreamR | jitterL | jitterR | seamCream | headerTopAirPt | bodyTopAirPt | leftInsetPt | Flags |",
);
mdLines.push("| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |");
for (const r of rows) {
  const f = r.outliers.length ? r.outliers.join("; ") : na;
  mdLines.push(
    "| " +
      [
        r.name,
        r.page,
        r.kind,
        r.chipBodyDL ?? na,
        r.chipBodyDR ?? na,
        r.joinCreamL,
        r.joinCreamR,
        r.borderJitterL,
        r.borderJitterR,
        r.seamCreamUnderChip,
        r.headerTopAirPt ?? na,
        r.bodyTopAirPt ?? na,
        r.leftTextInsetPt ?? na,
        f,
      ].join(" | ") +
      " |",
  );
}
mdLines.push("");
mdLines.push("## Outlier rules");
mdLines.push("");
mdLines.push("- chipBodyDL/DR: fail if abs(delta) > 2 px");
mdLines.push("- joinCream* / seamCreamUnderChip: fail if > 0");
mdLines.push("- borderJitter*: fail if > 1 px");
mdLines.push(
  "- leftTextInsetPt: fail if abs(x - median) > " +
    insetMadTol +
    " pt (median = **" +
    insetMedian.toFixed(2) +
    "** pt)",
);
mdLines.push("");
mdLines.push("## Outliers");
mdLines.push("");
if (!outlierList.length) {
  mdLines.push("_None._");
  mdLines.push("");
} else {
  for (const o of outlierList) {
    mdLines.push("- **" + o.name + "** (p" + o.page + "): " + o.outliers.join("; "));
  }
  mdLines.push("");
}
const headerMed = median(rows.map((r) => r.headerTopAirPt).filter((v): v is number => v != null));
const bodyMed = median(rows.map((r) => r.bodyTopAirPt).filter((v): v is number => v != null));
mdLines.push("## Medians");
mdLines.push("");
mdLines.push("- leftTextInsetPt median: **" + insetMedian.toFixed(2) + "** pt");
mdLines.push("- headerTopAirPt median: **" + headerMed.toFixed(2) + "** pt");
mdLines.push("- bodyTopAirPt median: **" + bodyMed.toFixed(2) + "** pt");
mdLines.push("");
mdLines.push("## Notes");
mdLines.push("");
mdLines.push(
  "- chipBodyDR is consistently 1px (hairline AA / sub-pixel); within the 2px tolerance.",
);
mdLines.push(
  "- TOC headerTopAirPt ~5pt is expected (half strut). Kopfbau header/body air is photo-pad.",
);
mdLines.push(
  "- Marktplaetze headerTopAirPt **11.33pt** sits above the peer cluster (~8pt); not an auto-fail under the inset/cream/jitter rules, but the largest cross-type air delta.",
);
mdLines.push("- Crops: `audit-mtx-p*.png` in this ticket folder.");
mdLines.push("");

writeFileSync(join(ticketDir, "audit-agent-matrix.md"), mdLines.join("\n"));
console.log(
  "[DEBUG] overall=" + overall + " medianInset=" + insetMedian.toFixed(2) + " outliers=" + outlierList.length,
);
console.log(JSON.stringify({ overall, medianLeftTextInsetPt: +insetMedian.toFixed(2), outlierList }, null, 2));
