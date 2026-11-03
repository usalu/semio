// usage: bun sheet.mjs out.png cell_px cols svg1 svg2 ...   (renders SVGs into a labelled contact sheet via chromium)
import { chromium } from "playwright";
import { readFileSync, writeFileSync } from "node:fs";
import { basename } from "node:path";
const [out, cell, cols, ...files] = process.argv.slice(2);
const c = Number(cell), n = Number(cols);
const items = files.map((f) => {
  const svg = readFileSync(f, "utf8");
  return `<div style="width:${c}px;height:${c + 18}px;display:inline-block;vertical-align:top;background:#fff"><img style="width:${c}px;height:${c}px;object-fit:contain;background:#0b1a22" src="data:image/svg+xml;base64,${Buffer.from(svg).toString("base64")}"><div style="font:11px sans-serif">${basename(f).replace(/-1to\d+-frame-000000\.svg$/, "")}</div></div>`;
});
const html = `<body style="margin:0;width:${c * n}px">${items.join("")}</body>`;
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: c * n, height: 400 } });
await page.setContent(html);
await page.screenshot({ path: out, fullPage: true });
await browser.close();
