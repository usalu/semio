/** 🎨 Example-switch runtime probe: boots the procedural playground with the shell's own runtime
 * diagnostics armed, picks ONE example through the navbar combobox and captures the whole
 * completion → refresh → render chain plus the live document read.
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=switch-1 SEMIO_PROBE_PICK="Box Shell Preview" bun 🐍️example-switch-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "example-switch");
const bootWait = Number(process.env.SEMIO_PROBE_BOOT_WAIT ?? 200);
const pickWait = Number(process.env.SEMIO_PROBE_PICK_WAIT ?? 60);
const picks = (process.env.SEMIO_PROBE_PICK ?? "Box Shell Preview,Rectangle Wire Preview").split(",").map((s) => s.trim()).filter(Boolean);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
await page.addInitScript(() => { try { localStorage.setItem("SEMIO_RUNTIME_DIAGNOSTICS", "1"); } catch {} });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 3000)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));

const snap = () => page.evaluate(() => {
  const parse = (s) => { try { return JSON.parse(s); } catch { return s?.slice(0, 200); } };
  const hosts = [...document.querySelectorAll("[data-status-json], [data-meshes-json]")].map((el) => {
    let meshes = 0; try { const v = JSON.parse(el.getAttribute("data-meshes-json") ?? "[]"); meshes = Array.isArray(v) ? v.length : 0; } catch {}
    const st = parse(el.getAttribute("data-status-json"));
    const nodeStatuses = st && typeof st === "object" && !st.phase ? Object.keys(st) : undefined;
    return { surfaceId: el.getAttribute("data-surface-id"), meshes, phase: st?.phase, ratio: st?.progress?.ratio, fault: st?.fault?.code ?? null, nodeIds: nodeStatuses };
  });
  const combo = document.querySelector('[role="combobox"]');
  return { hosts, meshes: hosts.reduce((n, h) => n + h.meshes, 0), example: combo?.innerText?.replace(/\s+/g, " ").trim() ?? null };
});
const converged = (s) => {
  const main = s.hosts.find((h) => h.surfaceId === "window:procedural-main");
  const preview = s.hosts.find((h) => h.surfaceId && h.surfaceId.endsWith("-preview"));
  return Boolean(preview) && preview.phase === "idle" && preview.ratio === 1 && (main?.nodeIds ?? []).length > 0;
};
const settle = async (label, seconds, fixed = false) => {
  let last = null, stable = 0;
  for (let i = 0; i < seconds; i++) { await page.waitForTimeout(1000); last = await snap(); if (!fixed && converged(last)) { stable += 1; if (stable >= 3) break; } else stable = 0; }
  console.log(`[DEBUG] ${label}: ${JSON.stringify(last)}`);
  results.push({ label, ...last });
  await page.screenshot({ path: join(outDir, `${results.length}-${label.replace(/[^a-z0-9]+/gi, "-")}.png`) });
  return last;
};
const results = [];
await page.goto(url, { waitUntil: "domcontentloaded" });
await settle("boot", bootWait);
for (const text of picks) {
  const mark = lines.length;
  const combo = page.locator('[role="combobox"]').first();
  await combo.click({ timeout: 5000 }); await page.waitForTimeout(400);
  await page.locator('[role="option"]').filter({ hasText: text }).first().click({ timeout: 5000 });
  await settle(`pick:${text}`, pickWait, true);
  writeFileSync(join(outDir, `console-${text.replace(/[^a-z0-9]+/gi, "-")}.txt`), lines.slice(mark).join("\n"));
}
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("[DEBUG] DONE", results.length);
await browser.close();
