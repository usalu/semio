/** 🔎 The DOM chain from a surface node down to its <canvas> on 6018 — so an accessibility wrapper
 * can be inserted without disturbing the layout the scene host relies on. */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "react-i18n-a11y", process.env.SEMIO_PROBE_OUT ?? "canvas-dom");
mkdirSync(outDir, { recursive: true });
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 120; i++) { await page.waitForTimeout(1000); if (await page.locator('[data-surface-id="window:procedural-preview"]').count()) break; }
await page.waitForTimeout(6000);

const chains = await page.evaluate(() => {
  const describe = (el) => ({
    tag: el.tagName.toLowerCase(),
    cls: (el.getAttribute("class") || "").slice(0, 220),
    nodeId: el.getAttribute("data-ui-node-id"),
    nodeKey: el.getAttribute("data-ui-node-key"),
    surfaceId: el.getAttribute("data-surface-id"),
    display: getComputedStyle(el).display,
    position: getComputedStyle(el).position,
    rect: (() => { const r = el.getBoundingClientRect(); return [Math.round(r.width), Math.round(r.height)]; })(),
  });
  return [...document.querySelectorAll("canvas")].map((c) => {
    const chain = [];
    let el = c;
    for (let i = 0; i < 9 && el; i++) { chain.unshift(describe(el)); el = el.parentElement; }
    return chain;
  });
});
writeFileSync(join(outDir, "chains.json"), JSON.stringify(chains, null, 2));
chains.forEach((chain, i) => {
  console.log(`\n[DEBUG] === canvas ${i} ===`);
  chain.forEach((n, d) => console.log(`${"  ".repeat(d)}${n.tag} ${n.display}/${n.position} ${n.rect.join("x")} key=${n.nodeKey ?? ""} surf=${n.surfaceId ?? ""} cls=${n.cls}`));
});
console.log("DONE");
await browser.close();
