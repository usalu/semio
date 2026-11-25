import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/** 🧭️ Capped probe: after on-demand activation lands, list the tool-run controls and, with PROBE_START=1, press the preview run start to prove whether the guest then emits invokeExtension. One browser, hard cap. */
const out = join(import.meta.dir, "🗑️generated/preview-stall"); mkdirSync(out, { recursive: true });
const url = process.env.SEMIO_BATTERY_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const tag = process.env.PROBE_TAG ?? "a";
const browser = await chromium.launch({ headless: true, args: ["--js-flags=--max-old-space-size=1024", "--renderer-process-limit=1", "--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const cap = setTimeout(() => { browser.close().finally(() => process.exit(3)); }, 280000);
const page = await (await browser.newContext({ viewport: { width: 1440, height: 900 } })).newPage();
const lines = []; const t0 = Date.now();
const stamp = () => `+${Date.now() - t0}ms`;
page.on("console", m => { const t = m.text(); if (/toolRun|invokeExtension|flowEval|setContributions|on-demand|panick|DEBUG\]/i.test(t) && !/receiving quote/.test(t)) lines.push(`${stamp()} ${m.type()} ${t.slice(0, 400)}`); });
page.on("pageerror", e => lines.push(`${stamp()} pageerror ${e.message}`));
page.on("request", r => { if (/core\.wasm|brep|math/i.test(r.url()) && !/\.json/.test(r.url())) lines.push(`${stamp()} req ${decodeURIComponent(r.url()).slice(-110)}`); });
const state = () => page.evaluate(() => { const g = (id) => document.querySelector(`[data-surface-id="${id}"]`); const p = JSON.parse(g("window:procedural-preview")?.getAttribute("data-status-json") ?? "{}"); return `${p.phase} ${p.fault?.code ?? ""} units=${p.progress?.unitsDone}/${p.progress?.unitsTotal} nodes=${p.progress?.nodesDone}/${p.progress?.nodesTotal} cancellable=${p.cancellable}`; });
await page.goto(url, { waitUntil: "load" });
await page.waitForTimeout(25000);
for (const k of ["brep.solid.extrude", "math.vector"]) lines.push(`${stamp()} ASK ${k} ${await page.evaluate((n) => window.semioReactActivateOnDemand?.(JSON.stringify({ kind: "node-kind", nodeKind: n })), k)}`);
for (let t = 0; t < 100; t += 5) { const s = await state(); lines.push(`${stamp()} STATE ${s}`); if (/^computing/.test(s) && t > 20) break; await page.waitForTimeout(5000); }
const controls = await page.evaluate(() => [...document.querySelectorAll("button,[role=button],[role=menuitem],[data-tool-run],[data-testid*=tool]")].map(e => `${e.tagName} ${e.getAttribute("aria-label") ?? ""} | ${(e.textContent ?? "").trim().slice(0, 60)} | ${e.getAttribute("data-testid") ?? ""}`).filter(s => /run|start|tool|evaluat|play|compute/i.test(s)).slice(0, 40));
writeFileSync(join(out, `${tag}-controls.txt`), controls.join("\n"));
await page.screenshot({ path: join(out, `${tag}-before.png`) });
if (process.env.PROBE_START === "1") {
  await page.getByRole("button", { name: "Tool runs" }).first().click({ timeout: 4000 }).catch(e => lines.push(`${stamp()} toolruns-click-failed ${e.message.slice(0, 100)}`));
  await page.waitForTimeout(1500);
  const panel = await page.evaluate(() => [...document.querySelectorAll("button,[role=button],[role=menuitem],[role=dialog] *,[role=listitem],[role=row]")].map(e => `${e.tagName} ${e.getAttribute("aria-label") ?? ""} | ${(e.textContent ?? "").trim().slice(0, 80)}`).filter(s => /run|start|abort|preview|pause|step|final/i.test(s)).slice(0, 40));
  lines.push(`${stamp()} PANEL ${JSON.stringify(panel)}`);
  await page.screenshot({ path: join(out, `${tag}-panel.png`) });
  const start = page.locator("[role=menuitem],button,[role=button]").filter({ hasText: /^\s*(start|run|play|evaluate)/i }).first();
  lines.push(`${stamp()} START-CLICK ${await start.count()} ${await start.textContent().catch(() => "?")}`);
  await start.click({ timeout: 4000 }).catch(e => lines.push(`${stamp()} click-failed ${e.message.slice(0, 120)}`));
  for (let t = 0; t < 60; t += 5) { lines.push(`${stamp()} STATE ${await state()}`); await page.waitForTimeout(5000); }
}
await page.screenshot({ path: join(out, `${tag}-after.png`) });
writeFileSync(join(out, `${tag}-console.txt`), lines.join("\n"));
console.log(lines.join("\n")); console.log(controls.join("\n"));
clearTimeout(cap); await browser.close();
