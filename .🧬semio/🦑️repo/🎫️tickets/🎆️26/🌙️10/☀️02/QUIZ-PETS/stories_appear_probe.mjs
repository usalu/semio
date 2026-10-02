/** 🌱️ Ticket tool: proves that a stories gallery started for a menagerie file that does not exist yet shows the file as soon as it appears, and says so again when it goes away.
 *
 * Usage (from the repository root, with the gallery running for `PETS_MENAGERIE=<file>`, a path that names no file yet):
 *   node ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/stories_appear_probe.mjs" --url http://127.0.0.1:6072/ --file <file>
 *
 * The file it writes wraps `reference-species.json`; it is removed again before the tool ends.
 */
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium } from "playwright";

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const index = args.indexOf(`--${name}`);
  return index < 0 ? fallback : args[index + 1];
};
const url = option("url", "http://127.0.0.1:6072/");
const file = resolve(option("file"));
const species = relative(dirname(file), resolve(dirname(fileURLToPath(import.meta.url)), "reference-species.json")).replaceAll("\\", "/");
const facts = {};
const browser = await chromium.launch();
try {
  const page = await browser.newPage({ viewport: { width: 1200, height: 800 }, locale: "en-GB" });
  const errors = [];
  page.on("pageerror", (error) => errors.push(String(error)));
  await page.goto(url, { waitUntil: "networkidle" });
  facts.before = { stories: await page.locator(".story").count(), notice: await page.locator(".notice").innerText() };
  mkdirSync(dirname(file), { recursive: true });
  writeFileSync(file, `import species from ${JSON.stringify(species.startsWith(".") ? species : `./${species}`)};\nexport const LATE = { schema: "semio.pets.menagerie/v1", id: "late", title: { en: "Late", de: "Spät" }, species: [species], bonds: [], casts: [{ scene: "home", core: [species.id], rotation: [] }] };\n`);
  await page.waitForSelector(".story", { timeout: 20000 });
  facts.appeared = { stories: await page.locator(".story").count(), heading: await page.locator("h1").innerText() };
  rmSync(file);
  await page.waitForSelector(".notice", { timeout: 20000 });
  facts.gone = { stories: await page.locator(".story").count(), notice: await page.locator(".notice").innerText() };
  facts.errors = errors;
} finally {
  rmSync(file, { force: true });
  await browser.close();
}
console.log(JSON.stringify(facts, null, 2));
process.exit(facts.appeared?.stories === 1 && facts.gone?.stories === 0 ? 0 : 1);
