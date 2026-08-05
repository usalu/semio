#!/usr/bin/env bun
/** 🔬 [DEBUG] Corrected mid-rule / inset metrics + crops for window short tables. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const ticket = dirname(fileURLToPath(import.meta.url));
const pdfPath = "e:/semio/mit-bestand/bericht/zwischenbericht/dist/zwischenbericht.pdf";
const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas, loadImage } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const pdfjs = await import(pdfjsEntry);

type Target = { id: string; page: number; needle: string; title: string };

const targets: Target[] = [
  { id: "meilensteine", page: 18, needle: "Meilensteine", title: "Meilensteine" },
  { id: "risiken", page: 19, needle: "Risiken und", title: "Risiken" },
  { id: "erfolgsindikatoren", page: 23, needle: "Erfolgsindikatoren", title: "Erfolgsindikatoren" },
  { id: "ueberblick", page: 77, needle: "Überblick", title: "Überblick" },
  { id: "interviewinfo", page: 101, needle: "Interviewinformationen", title: "Interviewinformationen" },
  { id: "uebersicht", page: 111, needle: "Übersicht", title: "Übersicht" },
];

function saveCrop(
  img: Awaited<ReturnType<typeof loadImage>>,
  out: string,
  x0: number,
  y0: number,
  x1: number,
  y1: number,
) {
  const w = Math.max(1, x1 - x0);
  const h = Math.max(1, y1 - y0);
  const c = createCanvas(w, h);
  c.getContext("2d").drawImage(img, x0, y0, w, h, 0, 0, w, h);
  writeFileSync(out, c.toBuffer("image/png"));
}

async function measure(t: Target, dpi: number) {
  const scale = dpi / 72;
  const png = join(ticket, `audit-win${dpi}-${String(t.page).padStart(3, "0")}.png`);
  const img = await loadImage(png);
  const doc = await pdfjs.getDocument({
    data: new Uint8Array(readFileSync(pdfPath)),
    useSystemFonts: true,
  }).promise;
  const page = await doc.getPage(t.page);
  const vp = page.getViewport({ scale });
  const items = (await page.getTextContent()).items as { str: string; transform: number[] }[];
  const pts = items.map((it) => {
    const [x, y] = vp.convertToViewportPoint(it.transform[4], it.transform[5]);
    return { str: it.str, x, y };
  });
  const anchor = pts
    .filter((p) => p.str && p.str.includes(t.needle))
    .sort((a, b) => a.y - b.y)[0];
  if (!anchor) throw new Error(`no anchor ${t.id}@${dpi}`);

  const c = createCanvas(img.width, img.height);
  const ctx = c.getContext("2d");
  ctx.drawImage(img, 0, 0);
  const { data } = ctx.getImageData(0, 0, img.width, img.height);
  const rgb = (x: number, y: number) => {
    const i = (y * img.width + x) * 4;
    return [data[i], data[i + 1], data[i + 2]] as const;
  };
  const lum = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    return 0.2126 * r + 0.7152 * g + 0.0722 * b;
  };
  const isRule = (x: number, y: number) => {
    const [r, g, b] = rgb(x, y);
    const L = lum(x, y);
    return L > 50 && L < 165 && Math.abs(r - g) < 40 && Math.abs(g - b) < 40;
  };
  const isInk = (x: number, y: number) => lum(x, y) < 85;

  let b0 = -1;
  let b1 = -1;
  const xL = Math.floor(anchor.x - 20 * scale);
  const xR = Math.floor(anchor.x + 200 * scale);
  for (let y = Math.floor(anchor.y) + 1; y < Math.floor(anchor.y) + 50 * scale; y++) {
    let n = 0;
    for (let x = xL; x < xR; x++) if (isRule(x, y)) n++;
    if (n > (xR - xL) * 0.25) {
      if (b0 < 0) b0 = y;
      b1 = y;
    } else if (b0 >= 0) break;
  }

  let bodyL = -1;
  let best = 0;
  for (
    let x = Math.max(2, Math.floor(anchor.x) - 80 * scale);
    x < Math.floor(anchor.x) + 20 * scale;
    x++
  ) {
    let n = 0;
    for (let y = b1 + 2; y < b1 + 200 * scale; y++) if (isRule(x, y)) n++;
    if (n > best) {
      best = n;
      bodyL = x;
    }
  }
  let bodyR = -1;
  let bestR = 0;
  for (let x = img.width - 3; x > img.width * 0.55; x--) {
    let n = 0;
    for (let y = b1 + 2; y < b1 + 120 * scale; y++) if (isRule(x, y)) n++;
    if (n > bestR) {
      bestR = n;
      bodyR = x;
    }
  }

  let chipL = -1;
  for (let x = Math.max(2, bodyL - 40); x < bodyL + 40; x++) {
    let n = 0;
    for (let y = b0 - Math.round(26 * scale); y < b0; y++) if (isRule(x, y) || isInk(x, y)) n++;
    if (n > 8) {
      chipL = x;
      break;
    }
  }
  let chipR = -1;
  for (let x = Math.min(img.width - 2, bodyR + 40); x > bodyR - 40; x--) {
    let n = 0;
    for (let y = b0 - Math.round(26 * scale); y < b0; y++) if (isRule(x, y) || isInk(x, y)) n++;
    if (n > 8) {
      chipR = x;
      break;
    }
  }

  let cream = 0;
  for (let y = b1 + 1; y < b1 + 1 + 18 * scale; y++) {
    if (!isRule(bodyL, y) && lum(bodyL, y) > 200) cream++;
    else break;
  }

  const bands: number[] = [];
  let run: { y0: number; y1: number } | null = null;
  for (let y = b1 + Math.round(10 * scale); y < b1 + Math.round(260 * scale); y++) {
    let n = 0;
    const xa = bodyL + Math.round(30 * scale);
    const xb = bodyR - Math.round(30 * scale);
    for (let x = xa; x < xb; x += 2) if (isRule(x, y)) n++;
    const thr = ((xb - xa) / 2) * 0.55;
    if (n > thr) {
      if (!run) run = { y0: y, y1: y };
      else run.y1 = y;
    } else if (run) {
      bands.push(run.y0);
      run = null;
    }
  }
  if (run) bands.push(run.y0);

  let creamJoins = 0;
  let jit = 0;
  for (const by of bands.slice(0, 12)) {
    const up = isRule(bodyL, by - 2);
    const dn = isRule(bodyL, by + 2);
    const mid = !isRule(bodyL, by) && lum(bodyL, by) > 200;
    if (up && dn && mid) creamJoins++;
    let bestX = bodyL;
    let bL = 999;
    const midY = Math.min(img.height - 1, by + Math.round(8 * scale));
    for (let x = bodyL - 2; x <= bodyL + 2; x++) {
      const l = lum(x, midY);
      if (l < bL) {
        bL = l;
        bestX = x;
      }
    }
    jit = Math.max(jit, Math.abs(bestX - bodyL));
  }

  // continuous border sample inside body span
  let creamOnBorder = 0;
  let ruleOnBorder = 0;
  for (let y = b1 + 2; y < Math.min(img.height, b1 + Math.round(160 * scale)); y++) {
    if (isRule(bodyL, y)) ruleOnBorder++;
    else if (lum(bodyL, y) > 200) creamOnBorder++;
  }

  const inset = (y0: number, y1: number) => {
    for (let y = y0; y < y1; y++) {
      for (let x = bodyL + 2; x < bodyL + Math.round(100 * scale); x++) {
        if (isInk(x, y) && !isRule(x, y)) {
          return { topAirPt: +((y - y0) / scale).toFixed(2), leftPadPt: +((x - bodyL) / scale).toFixed(2) };
        }
      }
    }
    return { topAirPt: null as number | null, leftPadPt: null as number | null };
  };
  const headerInset = inset(b1 + 1, b1 + Math.round(36 * scale));
  const firstBody = bands.find((y) => y > b1 + Math.round(18 * scale)) ?? bands[0];
  const bodyInset = firstBody
    ? inset(firstBody + 2, firstBody + Math.round(40 * scale))
    : { topAirPt: null as number | null, leftPadPt: null as number | null };

  // interior verts
  let verts = 0;
  for (let x = bodyL + Math.round(25 * scale); x < bodyR - Math.round(25 * scale); x++) {
    let n = 0;
    for (let y = b1 + 4; y < b1 + Math.round(140 * scale); y++) if (isRule(x, y)) n++;
    if (n > Math.round(50 * scale)) {
      verts++;
      x += 4;
    }
  }

  saveCrop(
    img,
    join(ticket, `audit-win-${t.id}-d${dpi}-seam.png`),
    Math.max(0, bodyL - 8),
    Math.max(0, b0 - Math.round(36 * scale)),
    Math.min(img.width, bodyL + Math.round(420 * scale)),
    b1 + Math.round(28 * scale),
  );
  saveCrop(
    img,
    join(ticket, `audit-win-${t.id}-d${dpi}-edgeL.png`),
    Math.max(0, bodyL - 10),
    Math.max(0, b0 - Math.round(20 * scale)),
    bodyL + Math.round(90 * scale),
    Math.min(img.height, b0 + Math.round(180 * scale)),
  );
  saveCrop(
    img,
    join(ticket, `audit-win-${t.id}-d${dpi}-head.png`),
    Math.max(0, Math.min(chipL, bodyL) - 12),
    Math.max(0, b0 - Math.round(40 * scale)),
    Math.min(img.width, (bodyR || bodyL + 500) + 12),
    Math.min(img.height, b1 + Math.round(140 * scale)),
  );

  return {
    dpi,
    hairPt: +((b1 - b0 + 1) / scale).toFixed(2),
    hairCount: 1,
    creamGapPt: +(cream / scale).toFixed(2),
    dL: bodyL - chipL,
    dR: bodyR - chipR,
    creamJoins,
    jitterPx: jit,
    creamOnBorder,
    ruleOnBorder,
    bands: bands.length,
    headerInset,
    bodyInset,
    interiorVerts: verts,
    bodyL,
    bodyR,
    chipL,
    chipR,
  };
}

const all: Record<string, unknown> = {};
for (const t of targets) {
  const entry: any = { id: t.id, page: t.page, title: t.title, dpi: {} as any };
  for (const dpi of [144, 288]) {
    entry.dpi[dpi] = await measure(t, dpi);
    const m = entry.dpi[dpi];
    console.log(
      `[DEBUG] ${t.id}@${dpi} hair=${m.hairPt} creamGap=${m.creamGapPt} dL/R=${m.dL}/${m.dR} joins=${m.creamJoins} jit=${m.jitterPx} borderCream/rule=${m.creamOnBorder}/${m.ruleOnBorder} H=${m.headerInset.leftPadPt}/${m.headerInset.topAirPt} B=${m.bodyInset.leftPadPt}/${m.bodyInset.topAirPt} verts=${m.interiorVerts}`,
    );
  }
  all[t.id] = entry;
}
writeFileSync(join(ticket, "audit-agent-windows-metrics.json"), JSON.stringify(all, null, 2));
console.log("[DEBUG] wrote metrics");
