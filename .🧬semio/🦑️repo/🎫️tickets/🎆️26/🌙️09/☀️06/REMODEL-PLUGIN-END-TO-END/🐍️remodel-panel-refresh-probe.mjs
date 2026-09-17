/** 🔄️ Remodel panel-refresh probe: boots the remodel react playground, opens the Media panel, runs `addStream`
 * from the Frames Actions pane, then reads the Media summary row (a) right after the edit settles, (b) after
 * closing and reopening the Media panel, (c) after a full page reload — to tell "the panel body never
 * re-rendered" from "the mutation never reached the document".
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=remodel-panel-refresh-1 bun 🐍️remodel-panel-refresh-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6063/?plugin=remodel";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "remodel-panel-refresh");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 800)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const state = () => page.evaluate(() => {
  const treeRows = [...document.querySelectorAll('[role="treeitem"]')].map((el) => el.innerText.replace(/\s+/g, " ").trim().slice(0, 80));
  return {
    ready: document.documentElement.getAttribute("data-semio-os-ready"),
    streamSummary: treeRows.find((t) => /^Streams: \d+/.test(t)) ?? null,
    streamRows: treeRows.filter((t) => /Image Sequence|Video|Bildsequenz/.test(t) && !/^Media /.test(t)),
    mediaPressed: document.getElementById("remodeling.media")?.getAttribute("aria-pressed") ?? null,
    treeRows: treeRows.slice(0, 12),
  };
});
const note = async (step, detail) => { report.steps.push({ step, t: Date.now() - t0, detail }); console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 800)}`); await page.screenshot({ path: join(outDir, `${report.steps.length}-${step}.png`) }).catch(() => {}); };
const waitReady = async () => { let s = null; for (let i = 0; i < 240; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && i > 8) break; } return s; };
const openMedia = async () => { const t = page.locator('[id="remodeling.media"]').first(); if ((await t.getAttribute("aria-pressed")) !== "true") await t.click({ timeout: 8000, force: true }); await page.waitForTimeout(2500); };

if (process.env.SEMIO_PROBE_GUEST_DIAGNOSTICS === "1") await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
await page.goto(url, { waitUntil: "domcontentloaded" });
await waitReady();
await openMedia();
await note("boot-media", await state());

await page.locator('[id="framework.window.remodelingFrames.engagement.toggle"]').first().click({ timeout: 8000, force: true });
await page.waitForTimeout(1500);
await page.locator('[id="action.addStream"]').first().click({ timeout: 8000, force: true });
await page.waitForTimeout(1500);
await page.locator('[id$=".action.addStream.execute"]').first().click({ timeout: 8000 });
const from = lines.length;
for (let i = 0; i < 30; i++) { await page.waitForTimeout(500); if (lines.slice(from).some((l) => /history patch applied.*create-stream/.test(l))) break; }
await page.waitForTimeout(6000);
await note("after-add-stream", { ...(await state()), created: lines.slice(from).filter((l) => /create-stream/.test(l)).length });

// (a2) a second verb addressing the new stream — proves whether the GUEST document carries stream-1
{
  const from2 = lines.length;
  await page.locator('[id="action.setStreamSync"]').first().click({ timeout: 8000, force: true }).catch(() => {});
  await page.waitForTimeout(1200);
  const inputs = await page.evaluate(() => [...document.querySelectorAll('[id*="setStreamSync"] input, input[id*="streamId"], input[name*="streamId"]')].map((el) => ({ id: el.id, name: el.getAttribute("name"), value: el.value })));
  const input = page.locator('input[id*="streamId"], input[name*="streamId"], [id$=".action.setStreamSync"] input').first();
  let filled = "absent";
  if (await input.count()) filled = await input.fill("stream-1").then(() => "ok").catch((e) => String(e).slice(0, 100));
  const exec = page.locator('[id$=".action.setStreamSync.execute"]').first();
  let executed = "absent";
  if (await exec.count()) executed = await exec.click({ timeout: 8000 }).then(() => "ok").catch((e) => String(e).slice(0, 100));
  await page.waitForTimeout(5000);
  const sync = lines.slice(from2).filter((l) => /change-stream-sync|setStreamSync|stream-sync/.test(l)).map((l) => l.slice(0, 300));
  const faults = lines.slice(from2).filter((l) => /refused|failed|Fault|fault|rejected/.test(l)).map((l) => l.slice(0, 300));
  await note("set-stream-sync", { inputs, filled, executed, sync: sync.slice(0, 6), faults: faults.slice(0, 6) });
}

// (b) close + reopen the Media panel → forces the host to mount the panel body again
await page.locator('[id="remodeling.media"]').first().click({ timeout: 8000, force: true });
await page.waitForTimeout(1500);
await openMedia();
await note("after-media-reopen", await state());

// (c) another app panel (Results) then back
await page.locator('[id="remodeling.results"]').first().click({ timeout: 8000, force: true }).catch(() => {});
await page.waitForTimeout(2000);
await note("results-panel", await state());
await openMedia();
await note("media-again", await state());

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
console.log("[DEBUG] PANEL REFRESH DONE");
await browser.close();
