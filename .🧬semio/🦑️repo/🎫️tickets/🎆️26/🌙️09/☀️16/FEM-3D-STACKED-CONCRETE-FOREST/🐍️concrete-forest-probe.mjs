/** 🌲️ Concrete-forest probe: boots the fem3d react playground, switches the navbar example combobox to
 * "Concrete Forest", then records the guest `[DEBUG]` lines (setActiveExample + results solves), every
 * rendered window host with its mesh/instance counts, the World3d scene instances of the results
 * window (deformed node positions) and screenshots.
 * Usage: cd <ticket> && SEMIO_PROBE_URL=http://127.0.0.1:6087/?plugin=fem3d SEMIO_PROBE_OUT=concrete-forest-1 bun 🐍️concrete-forest-probe.mjs
 */
import { chromium } from "playwright";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6087/?plugin=fem3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "concrete-forest");
const wanted = process.env.SEMIO_PROBE_EXAMPLE ?? "Concrete Forest";
const settleSeconds = Number(process.env.SEMIO_PROBE_SETTLE ?? 20);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await (await browser.newContext({ viewport: { width: 1600, height: 1000 } })).newPage();
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, m.type() === "error" ? 6000 : 800)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 2000)}`));
const report = { url, steps: [] };
const guestLines = (from) => lines.slice(from).filter((l) => /\[DEBUG\] fem|trapped|panicked|action failed|shell fault|faults=/.test(l)).map((l) => l.slice(0, 400));
const note = async (step, detail, from) => {
  report.steps.push({ step, t: Date.now() - t0, detail, guest: guestLines(from) });
  console.log(`[DEBUG] ${step} ${JSON.stringify(detail).slice(0, 1500)}`);
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  await page.screenshot({ path: join(outDir, `${report.steps.length}-${step.replace(/[^a-z0-9]+/gi, "-")}.png`) }).catch(() => {});
};
const state = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  const hosts = [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const meshes = parse(el.getAttribute("data-meshes-json"));
    const instances = parse(el.getAttribute("data-instances-json"));
    const attrs = [...el.attributes].filter((a) => a.name.startsWith("data-")).map((a) => `${a.name}:${a.value.length}`);
    return { id: el.getAttribute("data-surface-id"), canvases: el.querySelectorAll("canvas").length, meshes: Array.isArray(meshes) ? meshes.length : null, instances: Array.isArray(instances) ? instances.length : null, attrs, text: (el.innerText ?? "").replace(/\s+/g, " ").slice(0, 120) };
  });
  return { ready: document.documentElement.getAttribute("data-semio-os-ready"), error: document.documentElement.getAttribute("data-semio-os-error"), hosts, combobox: document.querySelector('[role="combobox"]')?.innerText?.replace(/\s+/g, " ").trim() ?? null };
});
const sceneDump = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return null; } };
  return [...document.querySelectorAll("[data-surface-id]")].map((el) => {
    const out = { id: el.getAttribute("data-surface-id") };
    for (const a of el.attributes) if (a.name.startsWith("data-") && a.name.endsWith("-json")) out[a.name] = a.value.length > 400000 ? a.value.slice(0, 400000) : parse(a.value) ?? a.value;
    return out;
  });
});

await page.goto(url, { waitUntil: "domcontentloaded" });
let s = null;
for (let i = 0; i < 150; i++) { await page.waitForTimeout(1000); s = await state(); if (s.ready && s.hosts.length >= 2 && i > 8) break; if (s.error) break; }
await note("boot", s, 0);
writeFileSync(join(outDir, "scene-boot.json"), JSON.stringify(await sceneDump(), null, 1));

{
  const from = lines.length;
  const combo = page.locator('[role="combobox"]').first();
  let picked = "no-combobox";
  if (await combo.count()) {
    await combo.click({ timeout: 5000 }).catch((e) => (picked = String(e).slice(0, 100)));
    await page.waitForTimeout(500);
    const options = await page.locator('[role="option"]').allInnerTexts().catch(() => []);
    const option = page.locator('[role="option"]').filter({ hasText: wanted }).first();
    if (await option.count()) picked = await option.click({ timeout: 5000 }).then(() => `ok:${wanted} of ${options.join("|")}`).catch((e) => String(e).slice(0, 100));
    else { picked = `no-option:${options.join("|")}`; await page.keyboard.press("Escape"); }
  }
  await page.waitForTimeout(settleSeconds * 1000);
  await note("example-switch", { picked, ...(await state()) }, from);
  writeFileSync(join(outDir, "scene-concrete-forest.json"), JSON.stringify(await sceneDump(), null, 1));
}
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] DONE lines", lines.length);
await browser.close();
