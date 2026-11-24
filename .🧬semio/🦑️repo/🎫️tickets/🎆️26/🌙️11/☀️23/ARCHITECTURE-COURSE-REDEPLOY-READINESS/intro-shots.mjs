/** 📸️ Photographs every step of the introduction of the dev page: the box with a margin around it, one picture per step, and says what each of its chips measures. Usage: node intro-shots.mjs <output directory> [width] [height] */
import { createRequire } from "node:module";
import { mkdirSync } from "node:fs";
import { join } from "node:path";
const require = createRequire("C:/semio/.cache/b/d/m/j/c642535c86e40717744c/node_modules/");
const { chromium } = require("playwright");
const out = process.argv[2];
mkdirSync(out, { recursive: true });
const [width, height] = [Number(process.argv[3] ?? 1280), Number(process.argv[4] ?? 800)];
const browser = await chromium.launch();
const page = await browser.newPage({ viewport: { width, height } });
await page.goto("http://127.0.0.1:6061/");
const box = page.locator('[data-slot="introduction-info-box"]');
await box.waitFor({ timeout: 180000 });
for (let step = 1; step <= 12; step += 1) {
  await page.waitForTimeout(1200);
  if ((await box.count()) === 0) break;
  const at = await box.boundingBox();
  const clip = { x: Math.max(0, at.x - 24), y: Math.max(0, at.y - 24), width: Math.min(width - Math.max(0, at.x - 24), at.width + 48), height: Math.min(height - Math.max(0, at.y - 24), at.height + 48) };
  await page.screenshot({ path: join(out, `intro-${width}-${step}.png`), clip });
  console.log(step + " box " + [at.x, at.y, at.width, at.height].map(Math.round).join(",") + " | " + (await box.evaluate((element) => [...element.querySelectorAll("[data-window-frame-chip], [data-slot=introduction-info-box-title], button")].map((chip) => { const r = chip.getBoundingClientRect(); return (chip.getAttribute("data-slot") ?? chip.tagName.toLowerCase()).slice(0, 26) + "[" + (chip.getAttribute("data-dock") ?? "") + "]" + [r.x, r.y, r.width, r.height].map(Math.round).join(",") + " " + JSON.stringify((chip.textContent ?? "").trim().slice(0, 28)); }).join(" · "))));
  await page.keyboard.press("Enter");
}
await browser.close();
