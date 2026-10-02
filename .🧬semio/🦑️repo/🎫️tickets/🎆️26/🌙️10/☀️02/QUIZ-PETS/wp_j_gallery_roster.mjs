/** 🖼️ Ticket tool of work package J: photographs the roster strip of a running stories gallery at three times the
 * size, on the light and on the dark page, and the sandbox for each scene of the menagerie after it has lived for a
 * while, so every species and every cast can be judged in one look each.
 *
 * Usage (from the repository root, with the gallery running):
 *   node ".../wp_j_gallery_roster.mjs" --url http://127.0.0.1:6074/ --out <directory>
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6074/");
const out = resolve(option("out", "."));
mkdirSync(out, { recursive: true });

const facts = { console: [], errors: [], roster: [], scenes: {} };
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 3, locale: "en-GB" });
  page.on("console", (message) => {
    if (message.type() === "error") facts.console.push(message.text());
  });
  page.on("pageerror", (error) => facts.errors.push(String(error)));
  await page.goto(url, { waitUntil: "networkidle" });
  await page.waitForSelector(".story", { timeout: 30000 });
  await page.waitForTimeout(800);
  facts.roster = await page.evaluate(() => [...document.querySelectorAll(".story h2, .story h3")].map((heading) => heading.textContent));
  const strip = await page.evaluate(() => {
    const first = document.querySelector(".story");
    return first === null ? 300 : Math.round(first.getBoundingClientRect().top);
  });
  await page.screenshot({ path: resolve(out, "roster-light.png"), clip: { x: 0, y: 90, width: 1440, height: Math.max(60, strip - 90) } });
  await page.getByLabel("Dark page").check();
  await page.waitForTimeout(400);
  await page.screenshot({ path: resolve(out, "roster-dark.png"), clip: { x: 0, y: 90, width: 1440, height: Math.max(60, strip - 90) } });
  await page.getByLabel("Dark page").uncheck();
  await page.close();

  const room = await browser.newPage({ viewport: { width: 1440, height: 900 }, locale: "en-GB" });
  room.on("pageerror", (error) => facts.errors.push(String(error)));
  await room.goto(url, { waitUntil: "networkidle" });
  await room.getByRole("button", { name: "Sandbox", exact: true }).click();
  await room.waitForSelector(".room");
  const scenes = await room.getByLabel("Scene").locator("option").evaluateAll((options) => options.map((option) => option.value));
  for (const scene of scenes) {
    await room.getByLabel("Scene").selectOption(scene);
    await room.getByLabel("Capacity").fill("6");
    await room.waitForTimeout(9000);
    facts.scenes[scene] = await room.evaluate(() => [...document.querySelectorAll(".pet-layer .pet")].map((pet) => `${pet.getAttribute("data-pet")} ${pet.style.transform} ${pet.style.opacity}`));
    await room.screenshot({ path: resolve(out, `sandbox-${scene}.png`) });
  }
} catch (error) {
  facts.errors.push(String(error));
} finally {
  await browser.close();
}
writeFileSync(resolve(out, "roster-report.json"), `${JSON.stringify(facts, null, 2)}\n`);
process.stdout.write(`${JSON.stringify(facts, null, 1)}\n`);
