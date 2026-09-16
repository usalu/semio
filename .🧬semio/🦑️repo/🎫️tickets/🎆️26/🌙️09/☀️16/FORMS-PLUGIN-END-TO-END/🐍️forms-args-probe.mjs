/** 🔍️ Forms binding-args probe: which authored `ActionBinding.args` survive to the host? Boots, then
 * (1) clicks the Catalogue panel's "Number" row (`addQuestion {kind}`), (2) types into the Try window's
 * first text input (`setTryValue {key, windowId, windowKindId}` + change value), (3) clicks Try "Next"
 * (`nextStep {windowId, windowKindId}`), logging every `action failed` / `performInvocation` line.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=forms-args-1 bun 🐍️forms-args-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6058/?plugin=forms";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "forms-args");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));
const text = () => page.evaluate(() => document.body.innerText.replace(/\s+/g, " "));
const report = [];
const note = async (step, detail, from) => {
  const relevant = lines.slice(from).filter((l) => /action failed|refused|performInvocation|ingress crossed|Fault|trapped/.test(l)).map((l) => l.slice(0, 500));
  report.push({ step, detail, relevant });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 600)}\n${relevant.map((l) => "    " + l.slice(0, 300)).join("\n")}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.length}-${step}.png`) }).catch(() => {});
};
await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 180; i++) { await page.waitForTimeout(1000); const ready = await page.evaluate(() => document.documentElement.getAttribute("data-semio-os-ready")); if (ready && (await text()).includes("Step 1 / 2") && i > 5) break; }
await note("boot", { body: (await text()).slice(0, 200) }, 0);
{
  const from = lines.length;
  const tab = page.locator('button:has-text("Catalogue"), [role="tab"]:has-text("Catalogue")').first();
  if (await tab.count()) await tab.click({ force: true }).catch(() => {});
  await page.waitForTimeout(1500);
  const row = page.locator('[role="treeitem"]:has-text("Number"), [id="forms-play-catalogue.number"]').first();
  let clicked = "absent";
  if (await row.count()) clicked = await row.click({ force: true, timeout: 5000 }).then(() => "ok").catch((e) => String(e).slice(0, 100));
  await page.waitForTimeout(4000);
  const body = await text();
  await note("catalogue-add-number", { clicked, questionsNumber: (body.match(/\bnumber\b/g) ?? []).length, bodyLen: body.length }, from);
}
{
  const from = lines.length;
  const input = page.locator('input[placeholder="Column A"], input[type="text"]').first();
  let typed = "absent";
  if (await input.count()) typed = await input.fill("Probe column").then(() => "ok").catch((e) => String(e).slice(0, 100));
  await page.keyboard.press("Tab");
  await page.waitForTimeout(4000);
  await note("try-set-value", { typed }, from);
}
{
  const from = lines.length;
  const next = page.locator('button:has-text("Next")').first();
  let clicked = "absent";
  if (await next.count()) clicked = await next.click({ timeout: 5000 }).then(() => "ok").catch((e) => String(e).slice(0, 100));
  await page.waitForTimeout(4000);
  await note("try-next", { clicked, step2: (await text()).includes("Step 2 / 2") }, from);
}
await browser.close();
