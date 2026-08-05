#!/usr/bin/env bun
/** 📏 [DEBUG] Full-page inset/seam/join compare at multiple scales. */
import { createRequire } from "node:module";
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const pdfPath = process.argv[2];
const outJson = process.argv[3];
const scales = (process.argv[4] ?? "2,4,6").split(",").map(Number);

const targets = [
  { id: "toc", page: 3, title: "Inhaltsverzeichnis" },
  { id: "meilensteine", page: 18, title: "Förderrechtliche Meilensteine" },
  { id: "risiken", page: 19, title: "Risiken und Maßnahmen" },
  { id: "erfolg", page: 22, title: "Erfolgsindikatoren" },
  { id: "kopfbau", page: 24, title: "Kopfbau Halle 118" },
  { id: "huerden", page: 76, title: "Hürden der Wiederverwendung" },
  { id: "ueberblick", page: 77, title: "Überblick" },
  { id: "markt", page: 78, title: "Marktplätze · Zugang und Kanäle" },
  { id: "datenfelder", page: 79, title: "Datenfelder und Beschaffung" },
  { id: "glossar", page: 121, title: "Glossar" },
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

const rows: Record<string, unknown>[] = [];

for (const t of targets) {
  const page = await doc.getPage(t.page);
  const content = await page.getTextContent();
  const items = content.items as { str: string; transform: number[] }[];

  for (const scale of scales) {
    const viewport = page.getViewport({ scale });
    const cands = items
      .filter((it) => it.str && (it.str === t.title || it.str.includes(t.title.slice(0, 12))))
      .map((it) => {
        const [x, y] = viewport.convertToViewportPoint(it.transform[4], it.transform[5]);
        return { x, y, str: it.str };
      })
      .sort((a, b) => b.str.length - a.str.length);
    const anchor = cands[0];
    if (!anchor) {
      rows.push({ id: t.id, scale, error: "no anchor" });
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

    // chip baseline: first nearly-full rule under title
    let base0 = -1;
    let base1 = -1;
    const x0 = Math.max(10, Math.floor(anchor.x - 30));
    const x1 = Math.min(W - 10, Math.floor(anchor.x + 900));
    for (let y = Math.floor(anchor.y) + 1; y < Math.floor(anchor.y) + Math.round(70 * scale); y++) {
      let n = 0;
      for (let x = x0; x < x1; x += 2) {
        const L = lum(x, y);
        if (L > 50 && L < 155) n++;
      }
      if (n > ((x1 - x0) / 2) * 0.35) {
        if (base0 < 0) base0 = y;
        base1 = y;
      } else if (base0 >= 0) break;
    }

    // second hairline within 3pt?
    let double = false;
    let hairPt = base0 >= 0 ? (base1 - base0 + 1) / scale : 0;
    if (base1 >= 0) {
      for (let y = base1 + 1; y < base1 + Math.round(4 * scale); y++) {
        let n = 0;
        for (let x = x0; x < x1; x += 2) {
          const L = lum(x, y);
          if (L > 50 && L < 155) n++;
        }
        if (n > ((x1 - x0) / 2) * 0.35) {
          double = true;
          hairPt = (y - base0 + 1) / scale;
          break;
        }
      }
    }

    // border
    let borderX = x0;
    let best = 0;
    for (let x = Math.max(4, x0 - 50); x < x0 + 80; x++) {
      let n = 0;
      for (let y = base0; y < Math.min(H, base0 + Math.round(500 * scale)); y++) {
        const L = lum(x, y);
        if (L > 50 && L < 155) n++;
      }
      if (n > best) {
        best = n;
        borderX = x;
      }
    }

    // cream gap: border broken + page-like under baseline
    let creamGap = 0;
    if (base1 >= 0) {
      for (let y = base1 + 1; y < base1 + Math.round(20 * scale); y++) {
        const borderCream = lum(borderX, y) > 200;
        if (borderCream) creamGap++;
        else break;
      }
    }

    // join notches
    let notches = 0;
    if (base1 >= 0) {
      for (let y = base1 + Math.round(10 * scale); y < Math.min(H - 2, base1 + Math.round(600 * scale)); y++) {
        if (lum(borderX, y) > 200 && lum(borderX, y - 2) < 155 && lum(borderX, y - 2) > 50 && lum(borderX, y + 2) < 155 && lum(borderX, y + 2) > 50) {
          notches++;
        }
      }
    }

    // first header/body text ink below baseline
    let inkY = -1;
    let inkX = -1;
    if (base1 >= 0) {
      for (let y = base1 + 1; y < base1 + Math.round(80 * scale); y++) {
        for (let x = borderX + 6; x < borderX + Math.round(200 * scale); x++) {
          if (lum(x, y) < 70) {
            // skip full-width rule rows
            let ruleN = 0;
            for (let xx = x0; xx < x1; xx += 2) {
              const L = lum(xx, y);
              if (L > 50 && L < 155) ruleN++;
            }
            if (ruleN > ((x1 - x0) / 2) * 0.35) break;
            inkY = y;
            inkX = x;
            break;
          }
        }
        if (inkY >= 0) break;
      }
    }

    const topPt = inkY >= 0 ? +((inkY - base1 - 1) / scale).toFixed(2) : null;
    const leftPt = inkY >= 0 ? +((inkX - borderX - 1) / scale).toFixed(2) : null;

    // photo pad for project
    let photoPt: number | null = null;
    if (t.id === "kopfbau" && base1 >= 0) {
      for (let y = base1 + 1; y < base1 + Math.round(40 * scale); y++) {
        let photo = 0;
        for (let x = borderX + 10; x < borderX + 200; x++) {
          const [r, g, b] = rgb(x, y);
          if (Math.max(r, g, b) - Math.min(r, g, b) > 40) photo++;
        }
        if (photo > 25) {
          photoPt = +((y - base1 - 1) / scale).toFixed(2);
          break;
        }
      }
    }

    const fail: string[] = [];
    if (double) fail.push("double-hairline");
    if (hairPt > 1.15) fail.push(`thick:${hairPt.toFixed(2)}`);
    if (creamGap / scale > 0.4) fail.push(`cream-gap:${(creamGap / scale).toFixed(2)}`);
    if (notches > 0) fail.push(`notches:${notches}`);
    if (topPt != null && (topPt < 4.0 || topPt > 9.5)) fail.push(`top:${topPt}`);
    if (leftPt != null && (leftPt < 4.0 || leftPt > 9.5)) fail.push(`left:${leftPt}`);
    if (photoPt != null && Math.abs(photoPt - 5.5) > 1.2) fail.push(`photo:${photoPt}`);

    const row = {
      id: t.id,
      page: t.page,
      scale,
      title: anchor.str,
      hairPt: +hairPt.toFixed(2),
      double,
      creamGapPt: +(creamGap / scale).toFixed(2),
      notches,
      topPt,
      leftPt,
      photoPt,
      ok: fail.length === 0,
      fail,
    };
    rows.push(row);
    console.log(
      `[DEBUG] ${t.id}@${scale} hair=${row.hairPt} dbl=${double} gap=${row.creamGapPt} notches=${notches} top=${topPt} left=${leftPt} photo=${photoPt} ok=${row.ok} ${fail.join(",")}`,
    );
  }
}

writeFileSync(outJson, JSON.stringify(rows, null, 2));
const bad = rows.filter((r) => r.ok === false);
console.log(`[DEBUG] ${rows.length} rows, ${bad.length} failures`);
