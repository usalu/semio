#!/usr/bin/env bun
/** 📊 Master consistency matrix across all Zwischenbericht table types. */
import { createRequire } from "node:module";
import { writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

const pdfjsEntry = fileURLToPath(
  new URL("../../../../../../node_modules/pdfjs-dist/legacy/build/pdf.mjs", import.meta.url),
);
const { createCanvas, loadImage } = createRequire(pdfjsEntry)("@napi-rs/canvas");
const ticket = ".repo/🎫/26/08/04/FIX-WINDOW-SHORT-TABLE-GEOMETRY";
const scale = 3; // 216dpi

type Row = {
  name: string;
  page: number;
  kind: string;
  dL: number;
  dR: number;
  creamL: number;
  creamR: number;
  jitterL: number;
  jitterR: number;
  seamCream: number;
  headerTopAirPt: number | null;
  bodyTopAirPt: number | null;
  leftInsetPt: number | null;
  photoPadPt: number | null;
  pass: boolean;
  notes: string[];
};

async function measure(name: string, page: number, kind: string, minY: number): Promise<Row> {
  const img = await loadImage(`${ticket}/allA-${String(page).padStart(3, "0")}.png`);
  const w = img.width;
  const h = img.height;
  const c = createCanvas(w, h);
  c.getContext("2d").drawImage(img, 0, 0);
  const { data } = c.getContext("2d").getImageData(0, 0, w, h);
  const L = (x: number, y: number) => {
    const i = (y * w + x) * 4;
    return 0.2126 * data[i] + 0.7152 * data[i + 1] + 0.0722 * data[i + 2];
  };

  let bodyL = -1;
  const y0 = Math.floor(h * 0.22);
  const y1 = Math.floor(h * 0.55);
  for (let x = Math.floor(w * 0.1); x < Math.floor(w * 0.28); x++) {
    let ink = 0;
    for (let y = y0; y < y1; y++) if (L(x, y) < 140) ink++;
    if (ink > (y1 - y0) * 0.35) {
      bodyL = x;
      break;
    }
  }
  let bodyR = -1;
  for (let x = Math.floor(w * 0.92); x > Math.floor(w * 0.68); x--) {
    let ink = 0;
    for (let y = y0; y < y1; y++) if (L(x, y) < 140) ink++;
    if (ink > (y1 - y0) * 0.35) {
      bodyR = x;
      break;
    }
  }

  let top = -1;
  for (let y = minY; y < Math.floor(h * 0.75); y++) {
    let ink = 0;
    for (let x = Math.floor(w * 0.16); x < Math.floor(w * 0.84); x += 2) if (L(x, y) < 130) ink++;
    if (ink > 220 && bodyL > 0 && L(bodyL, y) < 150) {
      top = y;
      break;
    }
  }

  let chipL = bodyL;
  for (let x = bodyL - 4; x <= bodyL + 4; x++) {
    let ink = 0;
    for (let y = Math.max(0, top - 36); y < top - 2; y++) if (L(x, y) < 145) ink++;
    if (ink > 12) {
      chipL = x;
      break;
    }
  }
  let chipR = bodyR;
  for (let x = bodyR + 4; x >= bodyR - 4; x--) {
    let ink = 0;
    for (let y = Math.max(0, top - 36); y < top - 2; y++) if (L(x, y) < 145) ink++;
    if (ink > 12) {
      chipR = x;
      break;
    }
  }

  const joins: number[] = [];
  for (let y = top + 20; y < Math.min(h - 20, top + 480); y++) {
    let ink = 0;
    for (let x = bodyL + 40; x < bodyL + 260; x += 2) if (L(x, y) < 130) ink++;
    if (ink > 100) joins.push(y);
  }
  const bands: number[] = [];
  if (joins.length) {
    let s = joins[0];
    let p0 = joins[0];
    for (const y of joins.slice(1)) {
      if (y <= p0 + 2) p0 = y;
      else {
        bands.push(s);
        s = y;
        p0 = y;
      }
    }
    bands.push(s);
  }

  let creamL = 0;
  let creamR = 0;
  let jitterL = 0;
  let jitterR = 0;
  for (const by of bands.slice(0, 7)) {
    for (let yy = by - 1; yy <= by + 1; yy++) {
      if (L(bodyL, yy) > 175) creamL++;
      if (L(bodyR, yy) > 175) creamR++;
    }
    const mid = Math.min(h - 1, by + 22);
    let bestL = bodyL;
    let bL = 999;
    let bestR = bodyR;
    let bR = 999;
    for (let x = bodyL - 2; x <= bodyL + 2; x++) {
      const l = L(x, mid);
      if (l < bL) {
        bL = l;
        bestL = x;
      }
    }
    for (let x = bodyR - 2; x <= bodyR + 2; x++) {
      const l = L(x, mid);
      if (l < bR) {
        bR = l;
        bestR = x;
      }
    }
    jitterL = Math.max(jitterL, Math.abs(bestL - bodyL));
    jitterR = Math.max(jitterR, Math.abs(bestR - bodyR));
  }

  // seam cream under chip in open connector zone (center) — expect some paper; measure near L border instead
  let seamCream = 0;
  for (let y = top; y < top + 4; y++) {
    let cream = 0;
    let n = 0;
    for (let x = bodyL + 8; x < bodyL + 40; x++) {
      n++;
      if (L(x, y) > 200) cream++;
    }
    if (cream / n > 0.85) seamCream++;
  }

  // header top air: first dark text px under top in col1
  const textDark = (x: number, y: number) => L(x, y) < 90;
  let headerTopAirPt: number | null = null;
  {
    let y = top + 2;
    while (y < top + 80) {
      let dark = 0;
      for (let x = bodyL + 12; x < bodyL + 80; x++) if (textDark(x, y)) dark++;
      if (dark > 3) {
        headerTopAirPt = +((y - top - 1) / scale).toFixed(2);
        break;
      }
      y++;
    }
  }
  let bodyTopAirPt: number | null = null;
  if (bands.length >= 2) {
    const rowTop = bands[0];
    let y = rowTop + 2;
    while (y < rowTop + 80) {
      let dark = 0;
      for (let x = bodyL + 12; x < bodyL + 80; x++) if (textDark(x, y)) dark++;
      if (dark > 3) {
        bodyTopAirPt = +((y - rowTop - 1) / scale).toFixed(2);
        break;
      }
      y++;
    }
  }
  let leftInsetPt: number | null = null;
  {
    const probeY = top + 30;
    for (let x = bodyL + 2; x < bodyL + 80; x++) {
      if (textDark(x, probeY)) {
        leftInsetPt = +((x - bodyL - 1) / scale).toFixed(2);
        break;
      }
    }
  }

  let photoPadPt: number | null = null;
  if (kind === "project") {
    let y = top + 1;
    while (y < top + 60) {
      let dark = 0;
      for (let x = bodyL + 15; x < bodyL + 180; x++) if (L(x, y) < 90) dark++;
      if (dark > 40) {
        photoPadPt = +((y - top - 1) / scale).toFixed(2);
        break;
      }
      y++;
    }
  }

  const notes: string[] = [];
  const dL = bodyL - chipL;
  const dR = bodyR - chipR;
  if (Math.abs(dL) > 2) notes.push(`chip/body dL=${dL}`);
  if (Math.abs(dR) > 2) notes.push(`chip/body dR=${dR}`);
  if (creamL > 0) notes.push(`creamL=${creamL}`);
  if (creamR > 0) notes.push(`creamR=${creamR}`);
  if (jitterL > 1) notes.push(`jitterL=${jitterL}`);
  if (jitterR > 1) notes.push(`jitterR=${jitterR}`);
  if (seamCream > 2) notes.push(`seamCream=${seamCream}`);
  if (kind === "project" && photoPadPt != null && (photoPadPt < 4 || photoPadPt > 8)) {
    notes.push(`photoPadPt=${photoPadPt}`);
  }

  const pass = notes.length === 0;
  return {
    name,
    page,
    kind,
    dL,
    dR,
    creamL,
    creamR,
    jitterL,
    jitterR,
    seamCream,
    headerTopAirPt,
    bodyTopAirPt,
    leftInsetPt,
    photoPadPt,
    pass,
    notes,
  };
}

const specs: Array<[string, number, string, number]> = [
  ["TOC", 3, "toc", 200],
  ["Meilensteine", 18, "window", 200],
  ["Risiken", 19, "window", 250],
  ["Akteure (p23)", 23, "window", 200],
  ["Kopfbau", 24, "project", 500],
  ["Huerden", 76, "long", 350],
  ["Ueberblick", 77, "window", 350],
  ["Marktplaetze", 78, "long", 200],
  ["Datenfelder p79", 79, "long", 200],
  ["Datenfelder p83", 83, "long", 200],
  ["Datenfelder p85", 85, "long", 200],
  ["Glossar", 121, "long", 200],
];

const rows: Row[] = [];
for (const [name, page, kind, minY] of specs) {
  const row = await measure(name, page, kind, minY);
  rows.push(row);
  console.log(
    `[DEBUG] ${name} p${page} pass=${row.pass} dL=${row.dL} dR=${row.dR} cream=${row.creamL}/${row.creamR} jit=${row.jitterL}/${row.jitterR} hAir=${row.headerTopAirPt} bAir=${row.bodyTopAirPt} inset=${row.leftInsetPt}` +
      (row.photoPadPt != null ? ` photo=${row.photoPadPt}` : "") +
      (row.notes.length ? ` NOTES:${row.notes.join(";")}` : ""),
  );
}

const insets = rows.map((r) => r.leftInsetPt).filter((x): x is number => x != null);
const airs = rows.map((r) => r.headerTopAirPt).filter((x): x is number => x != null);
const median = (a: number[]) => {
  const s = [...a].sort((x, y) => x - y);
  const m = Math.floor(s.length / 2);
  return s.length % 2 ? s[m] : +((s[m - 1] + s[m]) / 2).toFixed(2);
};
const medInset = median(insets);
const medAir = median(airs);

for (const r of rows) {
  if (r.kind === "toc") continue; // intentional half pad
  if (r.leftInsetPt != null && Math.abs(r.leftInsetPt - medInset) > 2.5) {
    r.notes.push(`insetOutlier ${r.leftInsetPt} vs med ${medInset}`);
    r.pass = false;
  }
}

const passed = rows.filter((r) => r.pass).length;
const failed = rows.filter((r) => !r.pass);

const md = [
  "# Master table consistency matrix",
  "",
  `Scale: 216dpi. Pass: ${passed}/${rows.length}. Median left inset: ${medInset}pt. Median header top air: ${medAir}pt.`,
  "",
  "| Table | Kind | dL | dR | cream L/R | jit L/R | hdr air | body air | inset | photo | PASS |",
  "| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |",
  ...rows.map(
    (r) =>
      `| ${r.name} | ${r.kind} | ${r.dL} | ${r.dR} | ${r.creamL}/${r.creamR} | ${r.jitterL}/${r.jitterR} | ${r.headerTopAirPt ?? "—"} | ${r.bodyTopAirPt ?? "—"} | ${r.leftInsetPt ?? "—"} | ${r.photoPadPt ?? "—"} | ${r.pass ? "PASS" : "FAIL"} |`,
  ),
  "",
  "## Failures / notes",
  ...(failed.length
    ? failed.map((r) => `- **${r.name}** (p${r.page}): ${r.notes.join("; ")}`)
    : ["- none"]),
  "",
  "TOC may differ in vertical air (half padding) — not counted as inset outlier.",
].join("\n");

writeFileSync(`${ticket}/audit-master-matrix.json`, JSON.stringify({ medInset, medAir, rows }, null, 2));
writeFileSync(`${ticket}/audit-master-matrix.md`, md);
console.log(md);
