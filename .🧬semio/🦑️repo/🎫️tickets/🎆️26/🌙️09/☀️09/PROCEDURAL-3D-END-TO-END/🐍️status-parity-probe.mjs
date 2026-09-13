/** 📈️ Reads the SERVED `World3dComputeStatusV1` contract off all three generation3d preview windows
 * — edit `procedural-preview`, generate `generation3d-generate-preview`, viewer `procedural-view-preview`
 * — plus what `No example` actually leaves on screen in each of them.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=viewer-status/parity bun 🐍️status-parity-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "viewer-status/parity");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1200)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1200)}`));

const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const len = (el, attr) => { const v = parse(el.getAttribute(attr) ?? "[]"); return Array.isArray(v) ? v.length : 0; };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => ({
    surfaceId: el.getAttribute("data-surface-id"),
    meshes: len(el, "data-meshes-json"),
    instances: len(el, "data-instances-json"),
    selection: parse(el.getAttribute("data-selection-json")),
    status: parse(el.getAttribute("data-status-json")),
  }));
  const combo = document.querySelector('[role="combobox"]');
  const pressed = (id) => document.getElementById(id)?.getAttribute("aria-pressed");
  return {
    hosts,
    example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null,
    windows: [...document.querySelectorAll("[data-window-instance-id]")].map((e) => e.getAttribute("data-window-instance-id")),
    modes: { edit: pressed("playground.navbar.modes.edit"), generate: pressed("playground.navbar.modes.generate") },
    roles: { editor: pressed("playground.navbar.roles.editor"), viewer: pressed("playground.navbar.roles.viewer") },
  };
});

const results = [];
const settle = async (label, seconds) => {
  let last = null;
  for (let i = 0; i < seconds; i++) { await page.waitForTimeout(1000); last = await snap(); }
  results.push({ label, t: Date.now() - t0, ...last });
  console.log(`[DEBUG] ${label} windows=${JSON.stringify(last.windows)} hosts=${JSON.stringify(last.hosts.map((h) => [h.surfaceId, h.meshes, h.instances, h.status?.phase, h.status?.progress?.ratio, h.status?.computing ?? null]))}`);
  for (const h of last.hosts) console.log(`[DEBUG]   ${label} ${h.surfaceId} statusKeys=${JSON.stringify(Object.keys(h.status ?? {}))} status=${JSON.stringify(h.status)?.slice(0, 900)}`);
  await page.screenshot({ path: join(outDir, `${results.length}-${label.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  return last;
};
const pick = async (text) => {
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 4000 }); await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: text }).first().click({ timeout: 4000 });
  console.log(`[DEBUG] picked ${text}`);
};

await page.goto(url, { waitUntil: "domcontentloaded" });
await settle("boot", 20);
await pick("Sphere Cut With Torus");
await settle("edit:sphere-cut", 25);
await pick("No example");
await settle("edit:no-example", 20);
await pick("Box Fillet Preview");
await settle("edit:box-fillet", 20);
await page.keyboard.press("Meta+Alt+ArrowRight");
await settle("generate-mode", 8);
{
  const add = page.locator(':text-is("Add Generation"), :text-is("Generation hinzufügen"), [data-action-id="addGeneration"]').first();
  if (await add.count()) { await add.click({ timeout: 4000 }); console.log("[DEBUG] clicked Add Generation"); } else console.log("[DEBUG] Add Generation row not found");
}
await settle("generate-added", 25);
await page.keyboard.press("Meta+Alt+ArrowLeft");
await settle("back-to-edit", 5);
await page.keyboard.press("Meta+Alt+V");
await settle("viewer-role", 20);
await pick("Sphere Box Fuse");
await settle("view:sphere-box-fuse", 25);
await pick("No example");
await settle("view:no-example", 25);

writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE steps", results.length);
await browser.close();
