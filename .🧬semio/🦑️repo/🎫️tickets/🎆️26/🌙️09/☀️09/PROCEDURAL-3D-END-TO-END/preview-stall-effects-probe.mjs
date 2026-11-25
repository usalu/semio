import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🧭️ Capped probe: rewrites ONLY the served (not the repo) turn-dispatch module so every guest turn result's effects are logged with a timestamp; proves which effects the guest returned after the contributions install. One browser, hard cap. */
const out = join(import.meta.dir, "🗑️generated/preview-stall"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const tag = process.env.PROBE_TAG ?? "e";
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, 280000);
const ctx = await browser.newContext({ viewport: { width: 1440, height: 900 } });
let patched = 0;
await ctx.route((u) => decodeURIComponent(u.toString()).includes("PluginRuntime/🧵️turn-dispatch/🟦️.ts"), async (route) => {
  const res = await route.fetch(); let body = await res.text();
  const needle = "row.effects += result?.effects.length ?? 0;";
  if (body.includes(needle)) { patched++; body = body.replace(needle, needle + ` if (result?.effects?.length) { (globalThis.__fx ??= []).push(Math.round(performance.now()) + " " + key + " " + JSON.stringify(result.effects).slice(0, 700)); }`); }
  await route.fulfill({ response: res, body });
});
const page = await ctx.newPage();
const lines = []; const t0 = Date.now(); const stamp = () => `+${Date.now() - t0}ms`;
const state = () => page.evaluate(() => { const g = (id) => document.querySelector(`[data-surface-id="${id}"]`); const p = JSON.parse(g("window:procedural-preview")?.getAttribute("data-status-json") ?? "{}"); return `${p.phase} units=${p.progress?.unitsDone}/${p.progress?.unitsTotal}`; });
await page.goto(url, { waitUntil: "load" });
await page.waitForTimeout(25000);
lines.push(`${stamp()} patched=${patched} PRE-ASK ${await state()}`);
for (const k of ["brep.solid.extrude", "math.vector"]) lines.push(`${stamp()} ASK ${k} ${await page.evaluate((n) => window.semioReactActivateOnDemand?.(JSON.stringify({ kind: "node-kind", nodeKind: n })), k)}`);
for (let t = 0; t < 130; t += 10) { lines.push(`${stamp()} ${await state()}`); await page.waitForTimeout(10000); }
if (process.env.PROBE_GESTURE === "1") {
  lines.push(`${stamp()} GESTURE-BEGIN fx=${await page.evaluate(() => (globalThis.__fx ?? []).length)}`);
  const combo = page.locator("[role=combobox]").first();
  lines.push(`${stamp()} combo=${await combo.count()}`);
  await combo.click({ timeout: 5000 }).catch(e => lines.push(`${stamp()} combo-click-failed ${e.message.slice(0, 80)}`));
  await page.waitForTimeout(800);
  const options = page.locator("[role=option]");
  lines.push(`${stamp()} options=${await options.count()} ${(await options.allTextContents()).join("|").slice(0, 300)}`);
  await options.nth(1).click({ timeout: 5000 }).catch(e => lines.push(`${stamp()} option-click-failed ${e.message.slice(0, 80)}`));
  for (let t = 0; t < 60; t += 10) { lines.push(`${stamp()} ${await state()}`); await page.waitForTimeout(10000); }
}
const fx = await page.evaluate(() => (globalThis.__fx ?? []).join("\n"));
writeFileSync(join(out, `${tag}-effects.txt`), lines.join("\n") + "\n--- effects (performance.now ms since navigation) ---\n" + fx);
console.log(lines.join("\n")); console.log(fx.slice(0, 9000));
clearTimeout(cap); await browser.close();
