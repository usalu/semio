/** 🔍 How a user reaches the `error` node status, and what the outline row says when they do.
 *
 * `status-states · *:error` records `attempts: 0` — `driveError`'s widget-row lookup finds nothing to
 * type into after the outline moved into the Artifact panel. This dumps the real row ids, the real
 * Inspection field ids, and then drives a degenerate value and polls `data-status-json`, so the step
 * can be written against what is there instead of against a remembered id shape.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=react-sweep/error-recon bun 🐍️error-status-recon.mjs
 * @see 🐍️status-states-probe.mjs, 🐍️inspection-i18n-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6023/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "error-recon");
mkdirSync(outDir, { recursive: true });
const MAIN = "window:procedural-main";
const lines = [];
const t0 = Date.now();
const report = { url, steps: [] };

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 700)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 700)}`));
const say = (step, detail) => {
  report.steps.push({ step, detail, t: Date.now() - t0 });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1800)}`);
};

const openTab = async (id, marker) => {
  const tab = page.locator(`button#${id.replace(/\./gu, "\\.")}`);
  if ((await tab.count()) === 0) return false;
  for (let attempt = 0; attempt < 3; attempt += 1) {
    const box = await tab.first().boundingBox().catch(() => null);
    await tab.first().click({ position: { x: 8, y: Math.round((box?.height ?? 22) / 2) }, timeout: 8000 }).catch((e) => lines.push(`tab ${id} ${String(e).slice(0, 120)}`));
    await page.waitForTimeout(2500);
    if ((await page.evaluate((m) => document.querySelectorAll(`[id*="${m}"]`).length, marker)) > 0) return true;
  }
  return false;
};

const statuses = () =>
  page.evaluate((main) => {
    const host = document.querySelector(`[data-surface-id="${main}"]`);
    try {
      return JSON.parse(host?.getAttribute("data-status-json") ?? "null");
    } catch {
      return null;
    }
  }, MAIN);

const rowTexts = () => page.evaluate(() => [...document.querySelectorAll('[data-slot="panel"] [role="treeitem"]')].map((el) => ({ id: el.id, text: (el.textContent ?? "").replace(/\s+/gu, " ").trim().slice(0, 70) })));

await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 200; i += 1) {
  await page.waitForTimeout(1000);
  if ((await page.locator(`[data-surface-id="${MAIN}"]`).count()) > 0) break;
}
await page.waitForTimeout(10000);

say("artifact-open", { opened: await openTab("framework.panel.artifact", "procedural-play-graph") });
say("rows", { rows: await rowTexts() });

const rows = await rowTexts();
const victim = rows.find((r) => /Radius|Side Count|Column Height/u.test(r.text));
await page.locator(`[data-slot="panel"] [id="${victim?.id}"]`).first().click({ timeout: 8000 }).catch((e) => lines.push(`victim ${String(e).slice(0, 140)}`));
await page.waitForTimeout(3000);
say("selected", { victim });

say("inspection-open", { opened: await openTab("framework.panel.inspection", "procedural-play-inspector") });
const fields = await page.evaluate(() =>
  [...document.querySelectorAll('[data-slot="panel"] input, [data-slot="panel"] [role="slider"], [data-slot="panel"] [role="spinbutton"]')].map((el) => ({
    id: el.id || null,
    tag: el.tagName,
    type: el.getAttribute("type"),
    role: el.getAttribute("role"),
    value: el.getAttribute("value") ?? el.value ?? null,
    min: el.getAttribute("min"),
    max: el.getAttribute("max"),
  })),
);
say("fields", { fields });

for (const value of ["0", "-1", "1e9"]) {
  const field = page.locator('[data-slot="panel"] input[type="number"], [data-slot="panel"] input[type="text"]').first();
  if ((await field.count()) === 0) {
    say(`type-${value}`, { field: false });
    break;
  }
  await field.fill(value, { timeout: 8000 }).catch((e) => lines.push(`fill ${String(e).slice(0, 120)}`));
  await page.keyboard.press("Enter");
  await page.waitForTimeout(1200);
  await openTab("framework.panel.artifact", "procedural-play-graph");
  let tags = null;
  for (let i = 0; i < 50; i += 1) {
    await page.waitForTimeout(600);
    const s = await statuses();
    tags = s ? Object.fromEntries(Object.entries(s).map(([k, v]) => [k, v?.status])) : null;
    if (tags && Object.values(tags).includes("error")) break;
  }
  say(`type-${value}`, { field: true, tags, rows: (await rowTexts()).filter((r) => /Error|Fehler|Radius|Extrude/u.test(r.text)) });
  if (tags && Object.values(tags).includes("error")) break;
  await openTab("framework.panel.inspection", "procedural-play-inspector");
}

await page.screenshot({ path: join(outDir, "error.png") });
writeFileSync(join(outDir, "recon.json"), JSON.stringify(report, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log(`[DEBUG] ERROR RECON DONE -> ${outDir}`);
await browser.close();
