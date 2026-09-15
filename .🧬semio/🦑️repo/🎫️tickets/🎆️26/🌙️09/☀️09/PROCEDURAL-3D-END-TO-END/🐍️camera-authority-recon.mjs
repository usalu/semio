/** 🔎 Recon: what happens to the preview camera between a user gesture and the next re-evaluation.
 * Reads BOTH lanes every second — `data-viewport-camera-json` (the rig's live pose) and
 * `data-camera-json` (the pose the GUEST published on the window config lane) — so a snap-back can be
 * attributed to the guest echoing a different pose rather than guessed at.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=... SEMIO_PROBE_OUT=boot-frame/authority bun 🐍️camera-authority-recon.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6028/?plugin=generation3d&example=box-fillet-preview";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "boot-frame/authority");
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 700)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 700)}`));

const read = () =>
  page.evaluate(() => {
    const parse = (raw) => { try { return JSON.parse(raw ?? "null"); } catch { return null; } };
    const el = [...document.querySelectorAll("[data-meshes-json]")].find((node) => (node.getAttribute("data-surface-id") ?? "").endsWith("procedural-preview"));
    return el ? { viewport: parse(el.getAttribute("data-viewport-camera-json")), scene: parse(el.getAttribute("data-camera-json")), status: parse(el.getAttribute("data-status-json"))?.phase ?? null } : null;
  });

await page.goto(url, { waitUntil: "domcontentloaded" });
const trace = [];
const sample = async (tag) => { trace.push({ tag, t: Date.now() - t0, ...(await read()) }); };
for (let i = 0; i < 70; i += 1) {
  await page.waitForTimeout(1000);
  const snapshot = await read();
  if (snapshot?.status === "idle") break;
}
await sample("converged");
const canvas = page.locator('[data-meshes-json] canvas, .semio-world-3d-host canvas').last();
const box = await canvas.boundingBox();
await page.keyboard.down("Alt");
await page.mouse.move(box.x + box.width * 0.6, box.y + box.height * 0.45);
await page.mouse.down({ button: "right" });
await page.mouse.move(box.x + box.width * 0.6 + 120, box.y + box.height * 0.45 + 45, { steps: 14 });
await page.mouse.up({ button: "right" });
await page.keyboard.up("Alt");
await sample("orbit-end");
for (let i = 0; i < 20; i += 1) { await page.waitForTimeout(1000); await sample(`after-orbit+${i + 1}s`); }
await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
await page.mouse.wheel(0, -180);
for (let i = 0; i < 15; i += 1) { await page.waitForTimeout(1000); await sample(`after-wheel+${i + 1}s`); }
writeFileSync(join(outDir, "trace.json"), JSON.stringify(trace, null, 2));
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
for (const row of trace) console.log(`[DEBUG] ${row.tag} viewport=${JSON.stringify(row.viewport?.position)}→${JSON.stringify(row.viewport?.target)} scene=${JSON.stringify(row.scene?.position)}→${JSON.stringify(row.scene?.target)} ${row.status}`);
await browser.close();
