#!/usr/bin/env bun
/**
 * 🔎 [DEBUG] Visual audit crops + measures for TOC register + project catalogue bands.
 * Usage: bun audit-ptoc-run.ts <pdf>
 */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const pdfPath = process.argv[2]!;
const ticketDir = dirname(fileURLToPath(import.meta.url));
const scale = 6;
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

type Sample = {
  id: string;
  kind: "toc" | "project" | "data";
  page: number;
  phrase: string;
};

const samples: Sample[] = [
  { id: "toc-p2", kind: "toc", page: 2, phrase: "Inhaltsverzeichnis" },
  { id: "toc-p3", kind: "toc", page: 3, phrase: "Inhaltsverzeichnis" },
  { id: "toc-p4", kind: "toc", page: 4, phrase: "Inhaltsverzeichnis" },
  { id: "project-kopfbau", kind: "project", page: 24, phrase: "Kopfbau Halle 118" },
  { id: "project-upcycle", kind: "project", page: 25, phrase: "Upcycle Studios" },
  { id: "project-recycling", kind: "project", page: 39, phrase: "Recyclinghaus Hannover" },
  { id: "data-meilensteine", kind: "data", page: 18, phrase: "Förderrechtliche Meilensteine" },
];

function near(a: readonly [number, number, number], t: { r: number; g: number; b: number }, tol = 16) {
  return Math.abs(a[0] - t.r) + Math.abs(a[1] - t.g) + Math.abs(a[2] - t.b) <= tol;
}

function saveCrop(
  src: any,
  out: string,
  x0: number,
  y0: number,
  x1: number,
  y1: number,
) {
  const w = Math.max(1, Math.floor(x1 - x0));
  const h = Math.max(1, Math.floor(y1 - y0));
  const c = createCanvas(w, h);
  c.getContext("2d").drawImage(src, Math.floor(x0), Math.floor(y0), w, h, 0, 0, w, h);
  writeFileSync(out, c.toBuffer("image/png"));
  return out;
}

const reports: Record<string, unknown>[] = [];
const cropIndex: Record<string, string> = {};

for (const sample of samples) {
  const page = await doc.getPage(sample.page);
  const viewport = page.getViewport({ scale });
  const content = await page.getTextContent();
  const items = content.items as { str: string; transform: number[] }[];
  const cands = items
    .filter((it) => it.str && it.str.includes(sample.phrase.slice(0, 8)))
    .map((it) => {
      const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
      return { x, y, str: it.str };
    })
    .filter((c) => c.y > 40 * scale && c.y < viewport.height - 40 * scale)
    .sort((a, b) => {
      const aHit = a.str.includes(sample.phrase) ? 0 : 1;
      const bHit = b.str.includes(sample.phrase) ? 0 : 1;
      // project cards: prefer lower-on-page title chip (below section banner)
      const yPref = sample.kind === "project" ? b.y - a.y : a.y - b.y;
      return aHit - bHit || b.str.length - a.str.length || yPref;
    });
  const anchor = cands[0];
  if (!anchor) {
    reports.push({ id: sample.id, error: "no anchor" });
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
  const kindAt = (x: number, y: number) => {
    const c = rgb(x, y);
    if (isRule(x, y)) return "RULE";
    if (near(c, PAGE)) return "PAGE";
    if (near(c, CANVAS)) return "CANVAS";
    if (lum(x, y) < 85) return "INK";
    const [r, g, b] = c;
    if (Math.max(r, g, b) - Math.min(r, g, b) > 40) return "PHOTO";
    return "OTHER";
  };

  // left border near chip
  let borderX = Math.floor(anchor.x - 4);
  {
    let best = 0;
    const xLo = Math.max(4, Math.floor(anchor.x - 90));
    const xHi = Math.min(W - 4, Math.floor(anchor.x + 20));
    for (let x = xLo; x < xHi; x++) {
      let n = 0;
      for (let y = Math.floor(anchor.y); y < Math.min(H - 2, Math.floor(anchor.y + 220 * scale)); y += 2) {
        if (isRule(x, y)) n++;
      }
      if (n > best) {
        best = n;
        borderX = x;
      }
    }
  }

  // chip baseline / first rule under title
  let base0 = -1;
  let base1 = -1;
  const xScan0 = Math.max(4, borderX + 8);
  const xScan1 = Math.min(W - 4, borderX + Math.round(420 * scale));
  for (let y = Math.floor(anchor.y) + 1; y < Math.floor(anchor.y) + Math.round(80 * scale); y++) {
    let n = 0;
    for (let x = xScan0; x < xScan1; x += 2) if (isRule(x, y)) n++;
    if (n > 40) {
      if (base0 < 0) base0 = y;
      base1 = y;
    } else if (base0 >= 0) break;
  }
  const baselineY = base1;

  // cream gap under chip: PAGE pixels immediately under baseline in mid-band
  let creamGapPx = 0;
  if (baselineY >= 0) {
    for (let dy = 1; dy <= Math.round(8 * scale); dy++) {
      const y = baselineY + dy;
      let pageN = 0;
      let canvasN = 0;
      for (let x = borderX + 20; x < borderX + Math.round(180 * scale); x += 2) {
        const k = kindAt(x, y);
        if (k === "PAGE") pageN++;
        if (k === "CANVAS") canvasN++;
      }
      if (pageN > canvasN && pageN > 8) creamGapPx++;
      else break;
    }
  }
  const creamGapPt = creamGapPx / scale;

  // join notches: PAGE cream *inside* the table just right of L border at hrule T-joins
  let joinNotches = 0;
  const joinSamples: { y: number; kind: string; L: number }[] = [];
  if (baselineY >= 0) {
    for (let y = baselineY; y < Math.min(H - 2, baselineY + Math.round(260 * scale)); y++) {
      let n = 0;
      for (let x = borderX + 10; x < borderX + Math.round(200 * scale); x += 3) if (isRule(x, y)) n++;
      if (n < 35) continue;
      // inside table: +1..+4 px from border pillar
      const insides = [1, 2, 3, 4].map((dx) => kindAt(borderX + dx, y));
      const L = lum(borderX, y);
      const hasPageInside = insides.includes("PAGE");
      joinSamples.push({ y, kind: insides.join("|"), L: Math.round(L) });
      if (hasPageInside) joinNotches++;
    }
  }

  // left inset: first body ink after baseline
  let leftInsetPt: number | null = null;
  let topInsetPt: number | null = null;
  let inkX = -1;
  let inkY = -1;
  if (baselineY >= 0) {
    outer: for (let y = baselineY + 2; y < baselineY + Math.round(40 * scale); y++) {
      for (let x = borderX + 2; x < borderX + Math.round(80 * scale); x++) {
        const L = lum(x, y);
        const c = rgb(x, y);
        if (L < 175 && !isRule(x, y) && !near(c, CANVAS) && !near(c, PAGE)) {
          let run = 0;
          for (let dx = 0; dx < 8; dx++) {
            const c2 = rgb(x + dx, y);
            const L2 = lum(x + dx, y);
            if (L2 < 175 && !near(c2, CANVAS) && !isRule(x + dx, y)) run++;
          }
          if (run >= 4) {
            inkX = x;
            inkY = y;
            break outer;
          }
        }
      }
    }
    if (inkX >= 0) {
      leftInsetPt = (inkX - borderX) / scale;
      topInsetPt = (inkY - baselineY) / scale;
    }
  }

  // photo pad for project: first PHOTO below baseline, measure canvas/air above it
  let photoPadPt: number | null = null;
  let photoFlush = false;
  if (sample.kind === "project" && baselineY >= 0) {
    let photoY = -1;
    for (let y = baselineY + 1; y < baselineY + Math.round(80 * scale); y++) {
      let photoN = 0;
      for (let x = borderX + 8; x < borderX + Math.round(120 * scale); x += 2) {
        if (kindAt(x, y) === "PHOTO") photoN++;
      }
      if (photoN > 18) {
        photoY = y;
        break;
      }
    }
    if (photoY >= 0) {
      photoPadPt = (photoY - baselineY - 1) / scale;
      photoFlush = photoPadPt < 2.5;
    }
  }

  // header chip left vs body border (page header chip)
  let headerChipAlignPt: number | null = null;
  {
    // find dark header chip left edge in top band
    let chipX = -1;
    for (let x = 20; x < Math.min(W / 2, 400); x++) {
      let dark = 0;
      for (let y = Math.round(18 * scale); y < Math.round(36 * scale); y++) {
        if (lum(x, y) < 100) dark++;
      }
      if (dark > 6) {
        chipX = x;
        break;
      }
    }
    if (chipX >= 0) headerChipAlignPt = (chipX - borderX) / scale;
  }

  // right border continuity sample
  let rightBorderX = borderX;
  {
    let best = 0;
    for (let x = Math.floor(W * 0.55); x < W - 8; x++) {
      let n = 0;
      for (let y = Math.max(0, baselineY); y < Math.min(H - 2, baselineY + Math.round(200 * scale)); y += 2) {
        if (isRule(x, y)) n++;
      }
      if (n > best) {
        best = n;
        rightBorderX = x;
      }
    }
  }
  let rightJoinNotches = 0;
  if (baselineY >= 0) {
    for (let y = baselineY; y < Math.min(H - 2, baselineY + Math.round(220 * scale)); y++) {
      let n = 0;
      for (let x = rightBorderX - Math.round(180 * scale); x < rightBorderX - 8; x += 3) if (isRule(x, y)) n++;
      if (n < 30) continue;
      const insides = [-1, -2, -3, -4].map((dx) => kindAt(rightBorderX + dx, y));
      if (insides.includes("PAGE")) rightJoinNotches++;
    }
  }

  const fail: string[] = [];
  if (creamGapPt > 0.75) fail.push(`creamGapUnderChip=${creamGapPt.toFixed(2)}pt`);
  if (joinNotches > 0) fail.push(`leftJoinNotches=${joinNotches}`);
  if (rightJoinNotches > 0) fail.push(`rightJoinNotches=${rightJoinNotches}`);
  if (photoFlush) fail.push(`photoFlushToTopRule pad=${photoPadPt?.toFixed(2)}pt`);
  if (sample.kind === "project" && photoPadPt != null && (photoPadPt < 4.0 || photoPadPt > 9.0)) {
    fail.push(`photoPadOutOfRange=${photoPadPt.toFixed(2)}pt (target~5.5)`);
  }
  if (headerChipAlignPt != null && Math.abs(headerChipAlignPt) > 2.5) {
    fail.push(`headerChipMisalign=${headerChipAlignPt.toFixed(2)}pt`);
  }
  // TOC half-pad internal consistency later across samples

  const prefix = `audit-ptoc-${sample.id}`;
  const crops: Record<string, string> = {};
  if (baselineY >= 0) {
    crops.chip = saveCrop(
      canvas,
      join(ticketDir, `${prefix}-chip.png`),
      borderX - 8,
      Math.floor(anchor.y - 28 * scale),
      Math.min(W - 2, borderX + Math.round(280 * scale)),
      baselineY + Math.round(18 * scale),
    );
    crops.Ljoin = saveCrop(
      canvas,
      join(ticketDir, `${prefix}-Ljoin.png`),
      borderX - 10,
      baselineY - 4,
      borderX + Math.round(70 * scale),
      baselineY + Math.round(90 * scale),
    );
    crops.Rjoin = saveCrop(
      canvas,
      join(ticketDir, `${prefix}-Rjoin.png`),
      rightBorderX - Math.round(70 * scale),
      baselineY - 4,
      rightBorderX + 10,
      baselineY + Math.round(90 * scale),
    );
    crops.body = saveCrop(
      canvas,
      join(ticketDir, `${prefix}-body.png`),
      borderX - 6,
      baselineY - 2,
      Math.min(W - 2, borderX + Math.round(320 * scale)),
      baselineY + Math.round(70 * scale),
    );
    if (sample.kind === "project") {
      crops.photo = saveCrop(
        canvas,
        join(ticketDir, `${prefix}-photo.png`),
        borderX - 6,
        baselineY - 4,
        borderX + Math.round(160 * scale),
        baselineY + Math.round(110 * scale),
      );
    }
    crops.head = saveCrop(
      canvas,
      join(ticketDir, `${prefix}-head.png`),
      Math.max(0, borderX - 20),
      Math.round(10 * scale),
      Math.min(W - 2, borderX + Math.round(200 * scale)),
      Math.round(50 * scale),
    );
  }
  Object.assign(cropIndex, Object.fromEntries(Object.entries(crops).map(([k, v]) => [`${sample.id}:${k}`, v])));

  reports.push({
    id: sample.id,
    kind: sample.kind,
    page: sample.page,
    scale,
    anchor: { x: Math.round(anchor.x), y: Math.round(anchor.y), str: anchor.str },
    borderX,
    rightBorderX,
    baselineY,
    creamGapPt: Number(creamGapPt.toFixed(2)),
    joinNotches,
    rightJoinNotches,
    leftInsetPt: leftInsetPt == null ? null : Number(leftInsetPt.toFixed(2)),
    topInsetPt: topInsetPt == null ? null : Number(topInsetPt.toFixed(2)),
    photoPadPt: photoPadPt == null ? null : Number(photoPadPt.toFixed(2)),
    photoFlush,
    headerChipAlignPt: headerChipAlignPt == null ? null : Number(headerChipAlignPt.toFixed(2)),
    joinSamples: joinSamples.slice(0, 12),
    crops,
    fail,
    ok: fail.length === 0,
  });
  console.log(
    `[DEBUG] ${sample.id} ok=${fail.length === 0} leftInset=${leftInsetPt?.toFixed(2)} topInset=${topInsetPt?.toFixed(2)} photoPad=${photoPadPt?.toFixed(2)} cream=${creamGapPt.toFixed(2)} Lnotch=${joinNotches} Rnotch=${rightJoinNotches}`,
  );
}

// TOC internal consistency: left/top insets across toc samples
const toc = reports.filter((r) => (r as any).kind === "toc" && (r as any).ok !== undefined) as any[];
const tocLeft = toc.map((r) => r.leftInsetPt).filter((v) => v != null) as number[];
const tocTop = toc.map((r) => r.topInsetPt).filter((v) => v != null) as number[];
const mean = (xs: number[]) => xs.reduce((a, b) => a + b, 0) / xs.length;
const spread = (xs: number[]) => (xs.length ? Math.max(...xs) - Math.min(...xs) : 0);
const tocConsistency = {
  leftInsetPt: tocLeft,
  topInsetPt: tocTop,
  leftSpreadPt: Number(spread(tocLeft).toFixed(2)),
  topSpreadPt: Number(spread(tocTop).toFixed(2)),
  leftMeanPt: tocLeft.length ? Number(mean(tocLeft).toFixed(2)) : null,
  topMeanPt: tocTop.length ? Number(mean(tocTop).toFixed(2)) : null,
  ok: spread(tocLeft) <= 1.25 && spread(tocTop) <= 1.25,
};
if (!tocConsistency.ok) {
  for (const r of reports) {
    if ((r as any).kind === "toc") {
      (r as any).fail = [...((r as any).fail ?? []), "tocInternalInsetSpread"];
      (r as any).ok = false;
    }
  }
}

const data = reports.find((r) => (r as any).kind === "data") as any;
const noteHalfPad = {
  tocLeftMeanPt: tocConsistency.leftMeanPt,
  dataLeftMeanPt: data?.leftInsetPt ?? null,
  expected: "TOC half-pad vs data full-pad (~5.5pt) — difference is intentional, not a bug",
  deltaPt:
    tocConsistency.leftMeanPt != null && data?.leftInsetPt != null
      ? Number((data.leftInsetPt - tocConsistency.leftMeanPt).toFixed(2))
      : null,
};

const summary = {
  verdict: reports.every((r) => (r as any).ok !== false) && tocConsistency.ok ? "PASS" : "FAIL",
  checkedAt: new Date().toISOString(),
  pdf: pdfPath,
  scale,
  tocConsistency,
  noteHalfPad,
  p125: {
    note: "p125 text-hit for Kopfbau is bibliography/Quellen register, not a project photo-band card",
    isProjectCard: false,
  },
  samples: reports,
  crops: cropIndex,
};

writeFileSync(join(ticketDir, "audit-agent-project-toc.json"), JSON.stringify(summary, null, 2));
console.log(`[DEBUG] verdict=${summary.verdict}`);
console.log(`[DEBUG] toc left mean=${tocConsistency.leftMeanPt} spread=${tocConsistency.leftSpreadPt}`);
console.log(`[DEBUG] noteHalfPad delta=${noteHalfPad.deltaPt}`);
