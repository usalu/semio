/** 🌳️ Prints the element tree of the introduction box of the dev page at a given step: boxes, slots, classes that size things and the inline style. Usage: node intro-tree.mjs [step] [width] [height] [focus] */
import { createRequire } from "node:module";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width: Number(process.argv[3] ?? 1280), height: Number(process.argv[4] ?? 800) } });
await page.goto("http://127.0.0.1:6061/");
const box = page.locator('[data-slot="introduction-info-box"]');
await box.waitFor({ timeout: 180000 });
for (let step = 1; step < Number(process.argv[2] ?? 1); step += 1) { await page.waitForTimeout(900); await page.keyboard.press("Enter"); }
await page.waitForTimeout(1500);
if (process.argv[5] === "focus") { await page.keyboard.press("Tab"); await page.keyboard.press("Tab"); await page.waitForTimeout(300); }
console.log(await box.evaluate((root) => {
  const lines = [];
  const walk = (e, level) => {
    const r = e.getBoundingClientRect();
    const s = getComputedStyle(e);
    if (e.tagName === "path" || e.tagName === "svg" && level > 2) return;
    lines.push("  ".repeat(level) + [r.x, r.y, r.width, r.height].map((v) => Math.round(v * 10) / 10).join(",") + " " + e.tagName.toLowerCase() + (e.getAttribute("data-slot") ? "[" + e.getAttribute("data-slot") + "]" : "") + (e.id ? "#" + e.id : "") + " d=" + s.display + (s.position !== "static" ? " pos=" + s.position : "") + " w=" + (e.style.width || "-") + " cls=" + String(e.className?.baseVal ?? e.className).split(" ").filter((c) => /^(w-|max-w|min-w|flex|grid|shrink|grow|ms-|col-|justify|h-|min-h|!h|p[xy]?-|gap|items)/.test(c)).join(" ").slice(0, 150) + (e.children.length === 0 ? " " + JSON.stringify((e.textContent ?? "").slice(0, 24)) : ""));
    if (level < 9) for (const child of e.children) walk(child, level + 1);
  };
  walk(root, 0);
  return "active=" + (document.activeElement?.id || document.activeElement?.tagName) + "\n" + lines.join("\n");
}));
await browser.close();
