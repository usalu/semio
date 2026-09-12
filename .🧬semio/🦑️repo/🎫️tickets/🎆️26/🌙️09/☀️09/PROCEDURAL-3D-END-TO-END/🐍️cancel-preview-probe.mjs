/** 🛑️ Preview-cancellation runtime probe: boots the procedural playground, switches to the slowest
 * example (`Sphere Cut With Torus`), polls the preview window's published `data-status-json` until
 * work is in flight, then clicks the surface's OWN cancel affordance and captures the status before
 * and after plus every host `[DEBUG] extension …` line.
 *
 * 🪪️ It also records whether the cancel affordance EXISTS at all (`[data-slot="world-compute-cancel"]`)
 * — a run where it never appears is itself the finding, not a probe failure.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=cancel-1 bun 🐍️cancel-preview-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", "cancellation", process.env.SEMIO_PROBE_OUT ?? "cancel");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 180);
const pick = process.env.SEMIO_PROBE_PICK ?? "Sphere Cut With Torus";
const armWait = Number(process.env.SEMIO_PROBE_ARM_WAIT ?? 90);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 3000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));

/** 📈️ Everything the cancel lane is written over, read straight off the DOM contract. */
const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-status-json]")].map((el) => {
    let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, status: parse(el.getAttribute("data-status-json")) };
  });
  const pane = document.querySelector('[data-slot="world-compute-status"]');
  const cancel = document.querySelector('[data-slot="world-compute-cancel"]');
  return {
    hosts,
    pane: pane ? { phase: pane.getAttribute("data-compute-phase"), cancellable: pane.hasAttribute("data-compute-cancellable"), ratio: pane.getAttribute("data-compute-ratio"), text: pane.innerText.replace(/\s+/g, " ").trim() } : null,
    cancelButton: cancel ? { action: cancel.getAttribute("data-cancel-action"), label: cancel.textContent, tag: cancel.tagName } : null,
  };
});

const preview = (s) => s.hosts.find((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));
const results = [];
const record = async (label) => {
  const s = await snap();
  results.push({ label, at: Date.now() - t0, preview: preview(s)?.status ?? null, meshes: preview(s)?.meshes ?? 0, pane: s.pane, cancelButton: s.cancelButton });
  console.log(`[DEBUG] ${label} ${JSON.stringify(results[results.length - 1])}`);
  await page.screenshot({ path: join(outDir, `${results.length}-${label.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  return s;
};

await page.goto(url, { waitUntil: "domcontentloaded" });

/** 📈️ Every DISTINCT status the preview publishes while booting — the cancel affordance is a frame,
 * not a steady state, so a sampler that only reports the end misses it entirely. */
const seen = new Map();
let armed = null;
const observe = async () => {
  const s = await snap();
  const p = preview(s);
  const key = `${p?.status?.phase}|${p?.status?.cancellable}|${p?.status?.progress?.inFlight}|${p?.status?.computing}|${Boolean(s.pane)}|${Boolean(s.cancelButton)}`;
  if (!seen.has(key)) seen.set(key, { at: Date.now() - t0, status: p?.status ?? null, pane: s.pane, cancelButton: s.cancelButton });
  if (!armed && s.cancelButton) armed = s;
  return s;
};
for (let i = 0; i < bootWait * 20; i += 1) {
  await page.waitForTimeout(50);
  const s = await observe();
  if (armed) break;
  if (preview(s)?.status?.phase === "idle" && preview(s)?.status?.progress?.ratio === 1 && preview(s)?.status?.computing !== true) break;
}
await record("boot");

if (!armed) {
  try {
    const combo = page.locator('[role="combobox"]').first();
    await combo.click({ timeout: 10000 });
    await page.waitForTimeout(400);
    await page.locator('[role="option"]').filter({ hasText: pick }).first().click({ timeout: 10000 });
    for (let i = 0; i < armWait * 20; i += 1) {
      await page.waitForTimeout(50);
      await observe();
      if (armed) break;
    }
  } catch (error) {
    console.log(`[DEBUG] example switch unavailable: ${String(error).slice(0, 200)}`);
  }
}
writeFileSync(join(outDir, "status-frames.json"), JSON.stringify([...seen.values()], null, 2));
const beforeMark = lines.length;
await record(armed ? "armed" : "never-armed");

if (armed?.cancelButton) {
  await page.locator('[data-slot="world-compute-cancel"]').first().click({ timeout: 5000 });
  await page.waitForTimeout(1500);
  await record("after-cancel");
  await page.waitForTimeout(4000);
  await record("after-cancel-settled");
} else {
  console.log("[DEBUG] no cancel affordance was ever offered — nothing to click");
}

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "console-around-cancel.txt"), lines.slice(Math.max(0, beforeMark - 40)).join("\n"));
console.log("[DEBUG] DONE", results.length);
await browser.close();
