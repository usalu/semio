import { chromium } from "playwright";
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const base = process.argv[2] ?? "http://127.0.0.1:6291/";
const out = join(dirname(fileURLToPath(import.meta.url)), "..", "🗑️generated", "ub");
mkdirSync(out, { recursive: true });
const log = [];
const note = (ok, text) => { log.push(`${ok ? "PASS" : "FAIL"} ${text}`); console.log(`${ok ? "PASS" : "FAIL"} ${text}`); };
const errors = [];
const hash = (buffer) => createHash("sha1").update(buffer).digest("hex").slice(0, 10);

const browser = await chromium.launch(process.env.UB_CHROME ? { executablePath: process.env.UB_CHROME } : {});
async function open(locale, viewport, hash_) {
  const context = await browser.newContext({ locale, viewport, reducedMotion: "no-preference", hasTouch: viewport.width < 600 });
  const page = await context.newPage();
  page.on("console", (message) => { if (message.type() === "error") errors.push(`${locale} ${viewport.width}: ${message.text()}`); });
  page.on("pageerror", (error) => errors.push(`${locale} ${viewport.width} pageerror: ${error.message}`));
  await page.goto(`${base}#/${hash_}`);
  await page.waitForFunction(() => !/Loading|Lade/.test(document.body.innerText) && document.querySelector("[data-stage]"), null, { timeout: 120000 });
  await page.waitForTimeout(1500);
  const pause = page.getByRole("button", { name: /^(Pause|Pausieren|Anhalten)$/ });
  if ((await pause.count()) > 0) await pause.first().click();
  await page.waitForTimeout(500);
  return { context, page };
}
const stageShot = (page) => page.locator("[data-stage]").first().screenshot();

const assets = [["components/panel-radiator", "panel-radiator"], ["humans/walking-commuter", "walking-commuter"]];
for (const [route, name] of assets) {
  for (const [locale, viewport, tag] of [["en-US", { width: 1440, height: 900 }, "desktop-en"], ["de-DE", { width: 1440, height: 900 }, "desktop-de"], ["en-US", { width: 390, height: 844 }, "mobile-en"], ["de-DE", { width: 390, height: 844 }, "mobile-de"]]) {
    const { context, page } = await open(locale, viewport, `${route}?view=thermal&zoom=6`);
    const group = page.getByRole("radiogroup", { name: locale === "en-US" ? "Display range" : /Wertebereich|Anzeigebereich|Bereich/ });
    const count = await group.count();
    note(count === 1, `${name} ${tag}: range radiogroup found (${count})`);
    if (count === 1) {
      await group.scrollIntoViewIfNeeded();
      const labels = await group.getByRole("radio").allInnerTexts();
      const title = await group.getByRole("radio").last().getAttribute("title");
      note(true, `${name} ${tag}: radios ${JSON.stringify(labels)} span title ${JSON.stringify(title)}`);
      if (locale === "en-US") note(labels.includes("Span"), `${name} ${tag}: English label Span`);
      else note(labels.includes("Spanne"), `${name} ${tag}: German label Spanne`);
      await page.screenshot({ path: join(out, `${name}-${tag}-auto.png`) });
    }
    await context.close();
  }
}

for (const [route, name] of assets) {
  const { context, page } = await open("en-US", { width: 1440, height: 900 }, `${route}?view=thermal&zoom=6`);
  const group = page.getByRole("radiogroup", { name: "Display range" });
  const radios = group.getByRole("radio");
  await radios.first().focus();
  await page.keyboard.press("ArrowRight");
  await page.keyboard.press("ArrowRight");
  const checked = await radios.evaluateAll((nodes) => nodes.map((node) => node.getAttribute("aria-checked")));
  note(JSON.stringify(checked) === JSON.stringify(["false", "false", "true"]), `${name}: arrow keys select Span ${JSON.stringify(checked)}`);
  const focusedText = await page.evaluate(() => document.activeElement?.innerText);
  note(focusedText === "Span", `${name}: focus follows selection (${focusedText})`);
  await page.keyboard.press("Tab");
  const field = page.getByLabel(/Span around the air temperature/);
  note((await field.count()) === 1, `${name}: span field labelled`);
  const focusedField = await page.evaluate(() => document.activeElement?.id);
  note(focusedField === (await field.getAttribute("id")), `${name}: Tab reaches the span field`);
  const shots = {};
  for (const k of ["10", "25", "80"]) {
    await page.keyboard.press("Control+a");
    await page.keyboard.type(k);
    await page.waitForTimeout(1500);
    const url = page.url();
    note(url.includes(`span=${k}`), `${name}: URL carries span=${k} (${url.split("?")[1]})`);
    const shot = await stageShot(page);
    shots[k] = hash(shot);
    writeFileSync(join(out, `${name}-span-${k}.png`), shot);
  }
  note(new Set(Object.values(shots)).size === 3, `${name}: thermal picture differs between 10, 25, 80 K ${JSON.stringify(shots)}`);
  await page.keyboard.press("Control+a");
  await page.keyboard.type("0");
  await page.waitForTimeout(500);
  note(await field.getAttribute("aria-invalid") === "true", `${name}: 0 K marks invalid`);
  await page.keyboard.press("Control+a");
  await page.keyboard.type("80");
  await page.waitForTimeout(800);
  await page.screenshot({ path: join(out, `${name}-span-keyboard.png`) });
  const hashBefore = await page.evaluate(() => location.hash);
  await page.reload();
  await page.waitForFunction(() => !/Loading|Lade/.test(document.body.innerText) && document.querySelector("[data-stage]"), null, { timeout: 120000 });
  await page.waitForTimeout(1500);
  const after = page.getByRole("radiogroup", { name: "Display range" }).getByRole("radio");
  const checkedAfter = await after.evaluateAll((nodes) => nodes.map((node) => node.getAttribute("aria-checked")));
  const value = await page.getByLabel(/Span around the air temperature/).inputValue();
  note(JSON.stringify(checkedAfter) === JSON.stringify(["false", "false", "true"]) && value === "80" && (await page.evaluate(() => location.hash)) === hashBefore, `${name}: reload keeps span 80 (${value}, ${hashBefore.split("?")[1]})`);
  note(true, `${name}: reload legend text ${JSON.stringify((await page.locator("[data-stage]").first().innerText()).slice(0, 80))}`);
  await context.close();
}

{
  const { context, page } = await open("en-US", { width: 1440, height: 900 }, "components/panel-radiator?view=thermal&zoom=6&span=-5");
  const radios = page.getByRole("radiogroup", { name: "Display range" }).getByRole("radio");
  const checked = await radios.evaluateAll((nodes) => nodes.map((node) => node.getAttribute("aria-checked")));
  note(JSON.stringify(checked) === JSON.stringify(["true", "false", "false"]), `negative span falls back to automatic ${JSON.stringify(checked)}`);
  await context.close();
}

note(errors.length === 0, `console errors: ${errors.length} ${JSON.stringify(errors.slice(0, 5))}`);
writeFileSync(join(out, "result.txt"), log.join("\n"));
await browser.close();
