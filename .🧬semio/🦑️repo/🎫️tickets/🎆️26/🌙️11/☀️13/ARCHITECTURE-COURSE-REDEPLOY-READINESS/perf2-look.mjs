/** 👀️ Screenshots of the home screen at rest and after a pan, in the default look, forced colours and reduced motion, for comparing two builds pixel by pixel. Env: SITE, OUT (folder). */
import { createRequire } from "node:module";
import { mkdirSync } from "node:fs";
const require = createRequire("/Users/ueli/Documents/semio/.cache/b/d/m/j/b87aa6c238609ac86876/node_modules/@playwright/test/");
const { chromium } = require("@playwright/test");
const site = process.env.SITE ?? "http://127.0.0.1:6069", out = process.env.OUT ?? "look";
mkdirSync(out, { recursive: true });
const browser = await chromium.launch();
for (const [name, options] of Object.entries({ default: {}, forced: { forcedColors: "active" }, reduced: { reducedMotion: "reduce" }, dark: { colorScheme: "dark" } })) {
  const context = await browser.newContext({ viewport: { width: 1280, height: 800 }, locale: "en", ...options });
  const page = await context.newPage();
  page.setDefaultTimeout(120_000);
  await page.goto(site + "/", { waitUntil: "commit" });
  await page.locator('[data-slot="introduction-close"]').click();
  await page.locator('[data-slot="introduction-info-box"]').waitFor({ state: "detached" });
  const calm = () => page.evaluate(() => { for (const e of document.querySelectorAll(".pet-layer")) e.style.display = "none"; for (const e of document.querySelectorAll("*")) { e.style.caretColor = "transparent"; } });
  await page.waitForTimeout(3500);
  await page.mouse.move(2, 2);
  await calm();
  await page.screenshot({ path: `${out}/${name}-rest.png`, animations: "disabled" });
  const box = await page.locator("[data-layered-overview]").first().boundingBox();
  for (let step = 0; step <= 30; step++) { await page.mouse.move(box.x + box.width * (0.1 + 0.8 * (step / 30)), box.y + box.height * 0.5); await page.waitForTimeout(20); }
  await page.mouse.move(2, 2);
  await page.waitForTimeout(3500);
  await calm();
  await page.screenshot({ path: `${out}/${name}-panned.png`, animations: "disabled" });
  await context.close();
}
await browser.close();
