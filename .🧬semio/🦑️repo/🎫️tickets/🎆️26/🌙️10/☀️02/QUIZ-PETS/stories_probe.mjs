/** 🔭️ Ticket tool: opens a running stories gallery in headless Chromium, records every response, console message and page error, and takes screenshots of the species view, the mirrored species view on the dark page, and the sandbox.
 *
 * Usage (from the repository root, with the gallery running):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/stories_probe.mjs" --url http://127.0.0.1:6071/ --out <directory> [--name <prefix>]
 *
 * Exit code 1 when a response failed, the page threw or the console reported an error.
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6071/");
const out = resolve(option("out", "."));
const name = option("name", "stories");
mkdirSync(out, { recursive: true });

const report = { url, responses: [], failed: [], console: [], errors: [], facts: {} };
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ viewport: { width: 1440, height: 900 }, locale: "en-GB" });
  page.on("response", (response) => report.responses.push({ status: response.status(), url: decodeURI(response.url()) }));
  page.on("requestfailed", (request) => report.failed.push({ url: decodeURI(request.url()), failure: request.failure()?.errorText }));
  page.on("console", (message) => report.console.push({ type: message.type(), text: message.text() }));
  page.on("pageerror", (error) => report.errors.push(String(error)));

  await page.goto(url, { waitUntil: "networkidle" });
  await page.waitForSelector(".story, .notice", { timeout: 20000 });
  await page.mouse.move(700, 420);
  await page.waitForTimeout(700);
  report.facts.title = await page.title();
  report.facts.language = await page.evaluate(() => document.documentElement.lang);
  report.facts.stories = await page.locator(".story").count();
  report.facts.pets = await page.locator("svg.pet").count();
  if (report.facts.stories === 0) {
    report.facts.notice = await page.locator(".notice").innerText();
    await page.screenshot({ path: resolve(out, `${name}-notice.png`), fullPage: false });
    throw new Error(`no menagerie shown: ${report.facts.notice}`);
  }
  report.facts.firstPet = await page.evaluate(() => {
    const pet = document.querySelector(".story svg.pet");
    if (!pet) return null;
    const style = getComputedStyle(pet);
    const body = pet.querySelector(".pet-fill-body");
    return {
      transform: pet.style.transform,
      styleAttribute: pet.getAttribute("style"),
      origin: style.transformOrigin,
      overflow: style.overflow,
      groups: pet.children.length,
      firstMatrix: pet.firstElementChild?.getAttribute("transform"),
      bodyFill: body ? getComputedStyle(body).fill : null,
      pupil: [...pet.querySelectorAll(".pet-pupil")].map((pupil) => [pupil.getAttribute("cx"), pupil.getAttribute("cy")]),
    };
  });
  await page.screenshot({ path: resolve(out, `${name}-species.png`), fullPage: false });
  await page.screenshot({ path: resolve(out, `${name}-species-full.png`), fullPage: true });

  await page.getByLabel("Face left").check();
  await page.getByLabel("Dark page").check();
  await page.getByLabel("Lid").fill("0.6");
  await page.getByLabel("Mood").fill("-1");
  await page.waitForTimeout(400);
  report.facts.mirrored = await page.evaluate(() => document.querySelector(".story svg.pet")?.style.transform);
  report.facts.dark = await page.evaluate(() => document.documentElement.classList.contains("dark"));
  await page.screenshot({ path: resolve(out, `${name}-species-mirrored-dark.png`), fullPage: false });

  await page.getByLabel("Language").selectOption("de");
  report.facts.german = { title: await page.title(), language: await page.evaluate(() => document.documentElement.lang) };
  await page.getByLabel("Sprache").selectOption("en");

  await page.getByRole("button", { name: "Sandbox", exact: true }).click();
  await page.waitForSelector(".room");
  await page.waitForTimeout(2500);
  report.facts.sandbox = await page.evaluate(() => ({
    surfaces: document.querySelectorAll("[data-pet-surface]").length,
    layer: document.querySelector(".pet-layer") !== null,
    layerHidden: document.querySelector(".pet-layer")?.getAttribute("aria-hidden") ?? null,
    pets: [...document.querySelectorAll(".pet-layer .pet")].map((pet) => [pet.getAttribute("data-pet"), pet.style.transform, pet.style.opacity]),
    notice: document.querySelector(".notice")?.textContent ?? null,
  }));
  await page.screenshot({ path: resolve(out, `${name}-sandbox.png`), fullPage: false });
  await page.getByRole("button", { name: "Rearrange the cards" }).click();
  await page.getByRole("button", { name: "Scroll the cards" }).click();
  await page.getByRole("button", { name: "Poke a pet" }).click();
  await page.waitForTimeout(2500);
  report.facts.sandboxAfter = await page.evaluate(() => [...document.querySelectorAll(".pet-layer .pet")].map((pet) => [pet.getAttribute("data-pet"), pet.style.transform, pet.style.opacity]));
  await page.screenshot({ path: resolve(out, `${name}-sandbox-rearranged.png`), fullPage: false });
} catch (error) {
  report.errors.push(String(error));
} finally {
  await browser.close();
}

const bad = report.responses.filter((response) => response.status >= 400);
const consoleErrors = report.console.filter((message) => message.type === "error");
writeFileSync(resolve(out, `${name}-report.json`), `${JSON.stringify(report, null, 2)}\n`);
console.log(JSON.stringify({ responses: report.responses.length, bad, failed: report.failed, consoleErrors, console: report.console.filter((message) => message.type !== "error").slice(0, 12), errors: report.errors, facts: report.facts }, null, 2));
process.exit(bad.length + report.failed.length + consoleErrors.length + report.errors.length === 0 ? 0 : 1);
