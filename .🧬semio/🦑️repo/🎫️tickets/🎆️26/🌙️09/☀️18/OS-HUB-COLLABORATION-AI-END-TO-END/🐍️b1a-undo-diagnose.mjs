/** 🩻️ Why is `framework.history.undo` mounted but unclickable after the Actions pane is opened?
 * Replays the probe's sequence and reports the undo row's geometry, computed visibility and every
 * ancestor that hides it. Usage: SEMIO_B1A_PORT=6090 SEMIO_B1A_PLUGIN=architect bun 🐍️b1a-undo-diagnose.mjs
 */
import { chromium } from "playwright";

const plugin = process.env.SEMIO_B1A_PLUGIN ?? "architect";
const port = process.env.SEMIO_B1A_PORT ?? "6090";

const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.goto(`http://127.0.0.1:${port}/?plugin=${plugin}`, { waitUntil: "domcontentloaded", timeout: 60000 });
for (let i = 0; i < 60 && !(await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready"))); i++) await page.waitForTimeout(1000);
await page.waitForTimeout(4000);

const geometry = (label) => page.evaluate((tag) => {
  const element = document.getElementById("framework.history.undo");
  if (!element) return { tag, present: false };
  const rect = element.getBoundingClientRect();
  const hiders = [];
  for (let node = element; node && node !== document.documentElement; node = node.parentElement) {
    const style = getComputedStyle(node);
    const box = node.getBoundingClientRect();
    if (style.display === "none" || style.visibility === "hidden" || style.opacity === "0" || box.width === 0 || box.height === 0 || node.hasAttribute("hidden") || node.getAttribute("aria-hidden") === "true") {
      hiders.push(`${node.id || node.tagName.toLowerCase()}#${node.className?.toString?.().slice(0, 40)} display=${style.display} vis=${style.visibility} opacity=${style.opacity} box=${Math.round(box.width)}x${Math.round(box.height)} hidden=${node.hasAttribute("hidden")} ariaHidden=${node.getAttribute("aria-hidden")}`);
    }
  }
  return { tag, present: true, rect: { x: Math.round(rect.x), y: Math.round(rect.y), w: Math.round(rect.width), h: Math.round(rect.height) }, hiders };
}, label);

const click = async (id) => page.locator(`[id="${id}"]`).first().click({ force: true, timeout: 6000 }).then(() => "ok").catch((error) => String(error).split("\n")[0]);

process.stdout.write(`${JSON.stringify(await geometry("boot"))}\n`);
process.stdout.write(`panel=${await click("framework.panel.history")}\n`);
await page.waitForTimeout(1500);
process.stdout.write(`${JSON.stringify(await geometry("history-open"))}\n`);
const toggles = await page.evaluate(() => [...document.querySelectorAll('[id$=".engagement.toggle"]')].map((element) => element.id));
process.stdout.write(`toggles=${JSON.stringify(toggles)}\n`);
for (const toggle of toggles) {
  process.stdout.write(`toggle ${toggle}=${await click(toggle)}\n`);
  await page.waitForTimeout(1200);
  process.stdout.write(`${JSON.stringify(await geometry(`after ${toggle}`))}\n`);
}
process.stdout.write(`commands=${await click("framework.history.commands")}\n`);
await page.waitForTimeout(1000);
process.stdout.write(`${JSON.stringify(await geometry("commands-expanded"))}\n`);
await browser.close();
