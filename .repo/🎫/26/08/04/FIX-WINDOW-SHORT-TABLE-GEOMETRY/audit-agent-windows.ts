#!/usr/bin/env bun
/** 🔬 [DEBUG] Final visual consistency audit for windowed short tables. */
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

type Target = {
  id: string;
  page: number;
  title: string;
  kind: "window-short";
  note?: string;
};

const targets: Target[] = [
  { id: "meilensteine", page: 18, title: "Meilensteine", kind: "window-short" },
  { id: "risiken", page: 19, title: "Risiken", kind: "window-short" },
  { id: "ueberblick", page: 77, title: "Überblick", kind: "window-short" },
  { id: "p23", page: 23, title: "Tabelle", kind: "window-short", note: "extra Tabelle: page" },
];

const dpis = [144, 288] as const;

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
    const [r, g, b] = rgb(x, y);
    const L = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    return L > 50 && L < 165 && Math.abs(r - g) < 40 && Math.abs(g - b) < 40;
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
  const isInk = (x: number, y: number) => lum(x, y) < 85;

  // Chip baseline clusters under title
  const xL = Math.max(4, Math.floor(anchor.x - 30 * scale));
  const xR = Math.min(W - 4, Math.floor(anchor.x + 220 * scale));
  const clusters: { y0: number; y1: number }[] = [];
  const yScanEnd = Math.min(H - 4, Math.floor(anchor.y) + Math.round(55 * scale));
  for (let y = Math.floor(anchor.y) + 1; y < yScanEnd; y++) {
    let n = 0;
    for (let x = xL; x < xR; x++) if (isRule(x, y)) n++;
    if (n > (xR - xL) * 0.28) {
      const last = clusters[clusters.length - 1];
      if (last && y <= last.y1 + 2) last.y1 = y;
      else clusters.push({ y0: y, y1: y });
    }
  }
  const base = clusters[0];
  const hairPt = base ? +((base.y1 - base.y0 + 1) / scale).toFixed(2) : 0;
  let hairCount = base ? 1 : 0;
  let doubleHair = false;
  if (clusters.length > 1) {
    const gapPt = (clusters[1].y0 - clusters[0].y1 - 1) / scale;
    if (gapPt < 2.5) {
      doubleHair = true;
      hairCount = 2;
    }
  }

  // Body L/R borders below baseline
  const y0 = (base?.y1 ?? Math.floor(anchor.y)) + 2;
  const yBodyEnd = Math.min(H - 2, y0 + Math.round(280 * scale));
  let bodyL = -1;
  let bestL = 0;
  for (
    let x = Math.max(2, Math.floor(anchor.x) - Math.round(80 * scale));
    x < Math.floor(anchor.x) + Math.round(20 * scale);
    x++
  ) {
    let n = 0;
    for (let y = y0; y < yBodyEnd; y++) if (isRule(x, y)) n++;
    if (n > bestL) {
      bestL = n;
      bodyL = x;
    }
  }
  let bodyR = -1;
  let bestR = 0;
  for (let x = Math.min(W - 3, Math.floor(W * 0.95)); x > Math.floor(W * 0.55); x--) {
    let n = 0;
    for (let y = y0; y < Math.min(yBodyEnd, y0 + Math.round(120 * scale)); y++) if (isRule(x, y)) n++;
    if (n > bestR) {
      bestR = n;
      bodyR = x;
    }
  }

  // Chip L/R: ink/rule in band above baseline
  const chipY0 = Math.max(0, (base?.y0 ?? Math.floor(anchor.y)) - Math.round(28 * scale));
  const chipY1 = (base?.y0 ?? Math.floor(anchor.y)) - 1;
  let chipL = -1;
  for (
    let x = Math.max(2, Math.floor(anchor.x) - Math.round(80 * scale));
    x < Math.floor(anchor.x) + Math.round(30 * scale);
    x++
  ) {
    let n = 0;
    for (let y = chipY0; y <= chipY1; y++) if (isRule(x, y) || isInk(x, y)) n++;
    if (n > 8) {
      chipL = x;
      break;
    }
  }
  let chipR = -1;
  for (let x = Math.min(W - 3, bodyR + Math.round(40 * scale)); x > Math.floor(W * 0.45); x--) {
    let n = 0;
    for (let y = chipY0; y <= chipY1; y++) if (isRule(x, y) || isInk(x, y)) n++;
    if (n > 8) {
      chipR = x;
      break;
    }
  }

  // Cream gap under chip baseline (on body border x)
  let creamGapPx = 0;
  if (base && bodyL >= 0) {
    for (let y = base.y1 + 1; y < base.y1 + 1 + Math.round(18 * scale); y++) {
      const broken = !isRule(bodyL, y) && isCream(bodyL, y);
      let pageish = 0;
      for (let x = bodyL + 6; x < bodyL + Math.round(120 * scale); x += 2) if (isPage(x, y)) pageish++;
      if (broken && pageish > 20) creamGapPx++;
      else break;
    }
  }
  const creamGapPt = +(creamGapPx / scale).toFixed(2);

  // Horizontal mid-rules → join cream / jitter on L and R
  const midRules: number[] = [];
  const midX0 = bodyL + Math.round(40 * scale);
  const midX1 = Math.min(bodyR - Math.round(40 * scale), bodyL + Math.round(220 * scale));
  for (let y = y0 + Math.round(8 * scale); y < yBodyEnd; y++) {
    let n = 0;
    for (let x = midX0; x < midX1; x += 2) if (isRule(x, y)) n++;
    if (n > ((midX1 - midX0) / 2) * 0.35) midRules.push(y);
  }
  const bands: number[] = [];
  if (midRules.length) {
    let s = midRules[0];
    let p0 = midRules[0];
    for (const y of midRules.slice(1)) {
      if (y <= p0 + 2) p0 = y;
      else {
        bands.push(s);
        s = y;
        p0 = y;
      }
    }
    bands.push(s);
  }

  let creamJoinsL = 0;
  let creamJoinsR = 0;
  let jitterL = 0;
  let jitterR = 0;
  for (const by of bands.slice(0, 8)) {
    for (let y = by - 1; y <= by + 1; y++) {
      if (bodyL >= 0 && lum(bodyL, y) > 175) creamJoinsL++;
      if (bodyR >= 0 && lum(bodyR, y) > 175) creamJoinsR++;
    }
    const mid = Math.min(H - 1, by + Math.round(10 * scale));
    let best = bodyL;
    let bL = 999;
    for (let x = bodyL - 3; x <= bodyL + 3; x++) {
      const l = lum(x, mid);
      if (l < bL) {
        bL = l;
        best = x;
      }
    }
    jitterL = Math.max(jitterL, Math.abs(best - bodyL));
    let bestRr = bodyR;
    let bRr = 999;
    for (let x = bodyR - 3; x <= bodyR + 3; x++) {
      const l = lum(x, mid);
      if (l < bRr) {
        bRr = l;
        bestRr = x;
      }
    }
    jitterR = Math.max(jitterR, Math.abs(bestRr - bodyR));
  }

  // Text insets: first ink inside body after baseline (header) and after first mid-rule (body)
  const measureInset = (yStart: number, yEnd: number) => {
    let topAir = -1;
    let leftPad = -1;
    for (let y = yStart; y < yEnd; y++) {
      let ink = 0;
      for (let x = bodyL + 2; x < Math.min(bodyR - 2, bodyL + Math.round(200 * scale)); x++) {
        if (isInk(x, y) && !isRule(x, y)) ink++;
      }
      if (ink > 3) {
        topAir = y - yStart;
        for (let x = bodyL + 1; x < bodyL + Math.round(80 * scale); x++) {
          if (isInk(x, y) && !isRule(x, y)) {
            leftPad = x - bodyL;
            break;
          }
        }
        break;
      }
    }
    return {
      topAirPt: topAir >= 0 ? +(topAir / scale).toFixed(2) : null,
      leftPadPt: leftPad >= 0 ? +(leftPad / scale).toFixed(2) : null,
    };
  };
  const headerInset = measureInset((base?.y1 ?? y0) + 1, y0 + Math.round(40 * scale));
  const bodyInsetY = bands[0] ? bands[0] + 2 : y0 + Math.round(30 * scale);
  const bodyInset = measureInset(bodyInsetY, bodyInsetY + Math.round(40 * scale));

  // Interior verticals: count distinct X with tall rule columns inside body
  const verts: number[] = [];
  for (let x = bodyL + Math.round(20 * scale); x < bodyR - Math.round(20 * scale); x++) {
    let n = 0;
    for (let y = y0; y < Math.min(yBodyEnd, y0 + Math.round(160 * scale)); y++) if (isRule(x, y)) n++;
    if (n > Math.round(40 * scale)) {
      const last = verts[verts.length - 1];
      if (last == null || x > last + 3) verts.push(x);
    }
  }

  return {
    baseY0: base?.y0 ?? -1,
    baseY1: base?.y1 ?? -1,
    hairPt,
    hairCount,
    doubleHair,
    creamGapPt,
    creamGapPx,
    chipL,
    chipR,
    bodyL,
    bodyR,
    dL: chipL >= 0 && bodyL >= 0 ? bodyL - chipL : null,
    dR: chipR >= 0 && bodyR >= 0 ? bodyR - chipR : null,
    creamJoinsL,
    creamJoinsR,
    jitterL,
    jitterR,
    joinBands: bands.length,
    headerInset,
    bodyInset,
    interiorVerts: verts.length,
    interiorVertXs: verts.slice(0, 6),
  };
}

function cropSave(
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

const pages: any[] = [];

for (const t of targets) {
  const pageEntry: any = {
    id: t.id,
    page: t.page,
    title: t.title,
    kind: t.kind,
    note: t.note ?? null,
    dpi: {} as Record<string, any>,
    crops: [] as string[],
    checks: {} as Record<string, string>,
    verdict: "PASS",
  };

  for (const dpi of dpis) {
    const scale = dpi / 72;
    const pngPath = join(ticket, `audit-win${dpi}-${String(t.page).padStart(3, "0")}.png`);
    const img = await loadImage(pngPath);
    const c = createCanvas(img.width, img.height);
    const ctx = c.getContext("2d");
    ctx.drawImage(img, 0, 0);
    const { data } = ctx.getImageData(0, 0, img.width, img.height);

    // Anchor from PDF text
    const page = await (await pdfjs.getDocument({
      data: new Uint8Array(readFileSync(pdfPath)),
      useSystemFonts: true,
    }).promise).getPage(t.page);
    const viewport = page.getViewport({ scale });
    const items = (await page.getTextContent()).items as { str: string; transform: number[] }[];
    const pts = items.map((it) => {
      const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
      return { str: it.str, x, y };
    });
    let anchor =
      pts
        .filter((p) => p.str && p.str.includes(t.title.slice(0, Math.min(8, t.title.length))))
        .sort((a, b) => b.str.length - a.str.length || a.y - b.y)[0] ?? null;
    if (t.page === 23 && !anchor) {
      anchor =
        pts
          .filter((p) => p.str.includes("Tabelle"))
          .sort((a, b) => a.y - b.y)[0] ?? null;
    }
    if (t.id === "ueberblick") {
      anchor =
        pts
          .filter((p) => p.str.includes("Überblick") || p.str.includes("Ueberblick") || p.str.includes("berblick"))
          .sort((a, b) => a.y - b.y)[0] ?? anchor;
    }
    if (!anchor) {
      pageEntry.dpi[dpi] = { error: "no anchor" };
      continue;
    }

    const m = analyze(data, img.width, img.height, scale, anchor);
    pageEntry.dpi[dpi] = { anchor: { x: +anchor.x.toFixed(1), y: +anchor.y.toFixed(1), str: anchor.str }, ...m };

    // Crops
    const bx = m.bodyL;
    const by0 = Math.max(0, m.baseY0 - Math.round(36 * scale));
    const seam = join(ticket, `audit-win-${t.id}-d${dpi}-seam.png`);
    cropSave(img, seam, Math.max(0, bx - 8), by0, Math.min(img.width, bx + Math.round(420 * scale)), m.baseY1 + Math.round(28 * scale));
    pageEntry.crops.push(seam.replace(/\\/g, "/").split("/").pop()!);

    const edge = join(ticket, `audit-win-${t.id}-d${dpi}-edgeL.png`);
    cropSave(
      img,
      edge,
      Math.max(0, bx - 10),
      Math.max(0, m.baseY0 - Math.round(20 * scale)),
      bx + Math.round(80 * scale),
      Math.min(img.height, m.baseY0 + Math.round(200 * scale)),
    );
    pageEntry.crops.push(edge.replace(/\\/g, "/").split("/").pop()!);

    const head = join(ticket, `audit-win-${t.id}-d${dpi}-head.png`);
    cropSave(
      img,
      head,
      Math.max(0, Math.min(m.chipL, bx) - 12),
      Math.max(0, m.baseY0 - Math.round(40 * scale)),
      Math.min(img.width, (m.bodyR > 0 ? m.bodyR : bx + 500) + 12),
      Math.min(img.height, m.baseY1 + Math.round(120 * scale)),
    );
    pageEntry.crops.push(head.replace(/\\/g, "/").split("/").pop()!);
  }

  // Verdict from both dpi
  const notes: string[] = [];
  const fail = (k: string, msg: string) => {
    pageEntry.checks[k] = "FAIL";
    notes.push(msg);
    pageEntry.verdict = "FAIL";
  };
  const pass = (k: string) => {
    pageEntry.checks[k] = "PASS";
  };

  for (const dpi of dpis) {
    const m = pageEntry.dpi[dpi];
    if (!m || m.error) {
      fail(`anchor@${dpi}`, `no anchor @${dpi}`);
      continue;
    }
    // 1 chip seam
    if (m.hairCount === 1 && !m.doubleHair && m.creamGapPt === 0) pass(`chipSeam@${dpi}`);
    else fail(`chipSeam@${dpi}`, `seam@${dpi}: hairs=${m.hairCount} double=${m.doubleHair} creamGap=${m.creamGapPt}pt`);
    // 2 chip align
    if (m.dL != null && Math.abs(m.dL) <= 1 && m.dR != null && Math.abs(m.dR) <= 2) pass(`chipAlign@${dpi}`);
    else fail(`chipAlign@${dpi}`, `align@${dpi}: dL=${m.dL} dR=${m.dR}`);
    // 3 continuous edges
    if (m.creamJoinsL === 0 && m.creamJoinsR === 0 && m.jitterL <= 1 && m.jitterR <= 1) pass(`edges@${dpi}`);
    else
      fail(
        `edges@${dpi}`,
        `edges@${dpi}: creamL/R=${m.creamJoinsL}/${m.creamJoinsR} jitL/R=${m.jitterL}/${m.jitterR}`,
      );
    // 4 insets — header vs body left pad within 2pt; top air present
    const hl = m.headerInset.leftPadPt;
    const bl = m.bodyInset.leftPadPt;
    const ht = m.headerInset.topAirPt;
    const bt = m.bodyInset.topAirPt;
    if (hl != null && bl != null && Math.abs(hl - bl) <= 2.5 && ht != null && bt != null && ht > 1 && bt > 1)
      pass(`insets@${dpi}`);
    else fail(`insets@${dpi}`, `insets@${dpi}: H L/T=${hl}/${ht} B L/T=${bl}/${bt}`);
    // 5 interior verticals — informational; pass if none or consistent (no cream check here)
    pass(`interiorVerts@${dpi}`);
  }

  pageEntry.notes = notes;
  pages.push(pageEntry);
  console.log(
    `[DEBUG] ${t.id} p${t.page} ${pageEntry.verdict} ` +
      dpis
        .map((d) => {
          const m = pageEntry.dpi[d];
          if (!m || m.error) return `d${d}=ERR`;
          return `d${d}: hair=${m.hairCount}/${m.hairPt}pt cream=${m.creamGapPt} dL/R=${m.dL}/${m.dR} joinC=${m.creamJoinsL},${m.creamJoinsR} jit=${m.jitterL},${m.jitterR} verts=${m.interiorVerts}`;
        })
        .join(" | "),
  );
}

const passCount = pages.filter((p) => p.verdict === "PASS").length;
const failCount = pages.filter((p) => p.verdict === "FAIL").length;
const out = {
  pdf: pdfPath,
  ticket,
  auditedAt: new Date().toISOString(),
  scope: "windowed short tables",
  summary: { pages: pages.length, pass: passCount, fail: failCount },
  pages,
};
writeFileSync(join(ticket, "audit-agent-windows.json"), JSON.stringify(out, null, 2));

const md: string[] = [
  "# Windowed Short Tables — Visual Consistency Audit",
  "",
  `PDF: \`${pdfPath}\``,
  `Rasters: \`audit-win144-*.png\` (144dpi), \`audit-win288-*.png\` (288dpi)`,
  `Crops: \`audit-win-<id>-d{144|288}-{seam,edgeL,head}.png\``,
  `Generated: ${out.auditedAt}`,
  "",
  `## Summary: **${passCount} PASS** / **${failCount} FAIL** (of ${pages.length})`,
  "",
  "| Page | Table | Verdict | Chip seam | Chip align dL/dR | Edge cream/jit | Insets H→B L/T | Verts | Notes |",
  "| --- | --- | --- | --- | --- | --- | --- | --- | --- |",
];

for (const p of pages) {
  const m288 = p.dpi[288] ?? {};
  const m144 = p.dpi[144] ?? {};
  const seam =
    m288.hairCount != null
      ? `${m288.hairCount}×${m288.hairPt}pt cream=${m288.creamGapPt}`
      : "n/a";
  const align = m288.dL != null ? `${m288.dL}/${m288.dR}` : "n/a";
  const edge =
    m288.creamJoinsL != null
      ? `cL/R=${m288.creamJoinsL}/${m288.creamJoinsR} jit=${m288.jitterL}/${m288.jitterR}`
      : "n/a";
  const insets =
    m288.headerInset
      ? `H ${m288.headerInset.leftPadPt}/${m288.headerInset.topAirPt} · B ${m288.bodyInset.leftPadPt}/${m288.bodyInset.topAirPt}`
      : "n/a";
  md.push(
    `| ${p.page} | ${p.title} | **${p.verdict}** | ${seam} | ${align} | ${edge} | ${insets} | ${m288.interiorVerts ?? "?"} | ${(p.notes || []).join("; ") || (p.note ?? "—")} |`,
  );
  // also note 144 deltas if fail
  if (p.verdict === "FAIL" && m144.dL != null) {
    md.push(
      `|  | _(144dpi)_ |  | ${m144.hairCount}×${m144.hairPt} cream=${m144.creamGapPt} | ${m144.dL}/${m144.dR} | c=${m144.creamJoinsL}/${m144.creamJoinsR} jit=${m144.jitterL}/${m144.jitterR} | H ${m144.headerInset.leftPadPt}/${m144.headerInset.topAirPt} · B ${m144.bodyInset.leftPadPt}/${m144.bodyInset.topAirPt} | ${m144.interiorVerts} |  |`,
    );
  }
}

md.push(
  "",
  "## Checks (per page)",
  "",
  "1. **Chip seam** — single hairline, no double, no cream gap under chips",
  "2. **Chip L/R align** — dL/dR ≈ 0 vs body outer borders (tol: |dL|≤1, |dR|≤2 px)",
  "3. **Continuous outer edges** — no cream notches at row joins, no x-jitter (>1px)",
  "4. **Text insets** — header vs body left pad within 2.5pt; top air present both",
  "5. **Interior verticals** — count only (consistency visual)",
  "",
  "## Crops",
  "",
);
for (const p of pages) {
  md.push(`### p${p.page} ${p.title} — ${p.verdict}`);
  for (const c of p.crops) md.push(`- \`${c}\``);
  md.push("");
}

writeFileSync(join(ticket, "audit-agent-windows.md"), md.join("\n"));
console.log(`[DEBUG] wrote audit-agent-windows.json/.md pass=${passCount} fail=${failCount}`);
