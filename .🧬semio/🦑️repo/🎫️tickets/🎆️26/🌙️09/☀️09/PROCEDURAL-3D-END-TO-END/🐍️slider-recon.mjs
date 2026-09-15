/** 🔎 Recon: boot the React editor on one example and inventory every slider-shaped control —
 * the node-graph inline sliders, the Inspection panel's number fields/sliders and the generate-mode
 * Form sliders — plus the surfaces that carry `data-status-json`/`data-meshes-json`.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=slider/recon bun 🐍️slider-recon.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const example = process.env.SEMIO_PROBE_EXAMPLE ?? "hexagonal-mushroom-column";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "slider/recon");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 600)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 600)}`));

const hosts = () => page.evaluate(() => [...document.querySelectorAll("[data-status-json]")].map((el) => {
  let meshes = 0;
  try { meshes = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]").length; } catch {}
  let status = null;
  try { status = JSON.parse(el.getAttribute("data-status-json")); } catch {}
  return { id: el.getAttribute("data-surface-id"), meshes, meshesLen: (el.getAttribute("data-meshes-json") ?? "").length, phase: status?.phase, ratio: status?.progress?.ratio };
}));

const sliders = () => page.evaluate(() => [...document.querySelectorAll('[role="slider"]')].map((el) => {
  const box = el.getBoundingClientRect();
  const owner = el.closest("[id]");
  return {
    id: el.id || null,
    ownerId: owner?.id ?? null,
    label: el.getAttribute("aria-label"),
    now: el.getAttribute("aria-valuenow"),
    min: el.getAttribute("aria-valuemin"),
    max: el.getAttribute("aria-valuemax"),
    rect: { x: Math.round(box.x), y: Math.round(box.y), w: Math.round(box.width), h: Math.round(box.height) },
  };
}));

await page.goto(`${url}&example=${example}`, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 150; i++) {
  await page.waitForTimeout(1000);
  const h = await hosts();
  if (h.some((x) => x.id === "window:procedural-preview" && x.meshes > 0)) break;
}
const boot = await hosts();
console.log("[DEBUG] boot", JSON.stringify(boot));
const graph = await page.evaluate(() => {
  const el = document.querySelector("[data-node-graph-nodes]") ?? document.querySelector("[data-surface-id*='node-graph']");
  return { attrs: el ? Object.fromEntries([...el.attributes].map((a) => [a.name, a.value.slice(0, 200)])) : null };
});
console.log("[DEBUG] graph", JSON.stringify(graph).slice(0, 1200));
console.log("[DEBUG] sliders-boot", JSON.stringify(await sliders()));
await page.screenshot({ path: join(outDir, "1-boot.png") });

// 🖱️ Open the Inspection panel by picking geometry in the preview.
const canvas = page.locator('[data-surface-id="window:procedural-preview"] canvas').first();
if (await canvas.count()) {
  const box = await canvas.boundingBox();
  if (box) { await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2); await page.waitForTimeout(3500); }
}
console.log("[DEBUG] sliders-after-pick", JSON.stringify(await sliders()));
const inputs = await page.evaluate(() => [...document.querySelectorAll("input")].map((el) => ({ id: el.id, type: el.type, value: el.value, ownerId: el.closest("[id]")?.id ?? null })).slice(0, 60));
console.log("[DEBUG] inputs", JSON.stringify(inputs).slice(0, 3000));
await page.screenshot({ path: join(outDir, "2-picked.png") });

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
await browser.close();
console.log("RECON DONE");
