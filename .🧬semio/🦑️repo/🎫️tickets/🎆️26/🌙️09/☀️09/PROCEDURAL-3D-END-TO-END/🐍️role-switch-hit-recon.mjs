/** 🎯️ Recon for the inert-`Viewer`-click report: for a set of viewports, boots the React procedural 3d
 * playground, reports what `elementFromPoint` finds at the centre of each navbar role button, clicks
 * `Viewer` the way a user does (real mouse at those coordinates, not a selector) and says whether the
 * role flipped. Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane role-switch-regression.
 *
 * Usage: cd <ticket> && bun 🐍️role-switch-hit-recon.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6028/?plugin=generation3d&example=sphere-cut-with-torus";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-role/hit-recon");
mkdirSync(outDir, { recursive: true });
const viewports = (process.env.SEMIO_PROBE_VIEWPORTS ?? "1600x1000,1440x900,1280x800,1152x720,1024x768").split(",").map((entry) => {
  const [width, height] = entry.split("x").map(Number);
  return { width, height };
});

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const results = [];
for (const viewport of viewports) {
  const page = await browser.newPage({ viewport });
  const lines = [];
  const t0 = Date.now();
  page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 800)}`));
  page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 800)}`));
  const probe = () => page.evaluate(() => {
    const button = (id) => {
      const el = document.getElementById(id);
      if (!el) return null;
      const box = el.getBoundingClientRect();
      const cx = box.x + box.width / 2;
      const cy = box.y + box.height / 2;
      const top = document.elementFromPoint(cx, cy);
      return { pressed: el.getAttribute("aria-pressed"), disabled: el.hasAttribute("disabled"), cx: Math.round(cx), cy: Math.round(cy), w: Math.round(box.width), h: Math.round(box.height), visible: box.width > 0 && box.height > 0, hit: top === null ? null : (top.closest("button")?.id ?? `${top.tagName}.${(top.className || "").toString().slice(0, 60)}`) };
    };
    const preview = [...document.querySelectorAll("[data-status-json]")].map((el) => { try { return JSON.parse(el.getAttribute("data-status-json")).phase ?? null; } catch { return null; } }).filter(Boolean);
    return { editor: button("playground.navbar.roles.editor"), viewer: button("playground.navbar.roles.viewer"), phases: preview, windows: [...document.querySelectorAll("[data-window-instance-id]")].map((e) => e.getAttribute("data-window-instance-id")) };
  });
  await page.goto(url, { waitUntil: "domcontentloaded" });
  let before = null;
  for (let i = 0; i < 120; i++) {
    await page.waitForTimeout(1000);
    before = await probe();
    if (before.viewer && before.phases.includes("idle")) break;
  }
  let after = null;
  if (before?.viewer?.visible) {
    await page.mouse.click(before.viewer.cx, before.viewer.cy);
    for (let i = 0; i < 25; i++) {
      await page.waitForTimeout(1000);
      after = await probe();
      if (after.viewer?.pressed === "true") break;
    }
  }
  await page.screenshot({ path: join(outDir, `${viewport.width}x${viewport.height}.png`) });
  writeFileSync(join(outDir, `${viewport.width}x${viewport.height}-console.txt`), lines.join("\n"));
  const entry = { viewport, before, after, switched: after?.viewer?.pressed === "true", draining: lines.filter((l) => l.includes("surface-switch draining")).length, pageErrors: lines.filter((l) => l.includes("pageerror")).length };
  results.push(entry);
  console.log(`[DEBUG] ${viewport.width}x${viewport.height} switched=${entry.switched} viewerHit=${before?.viewer?.hit} editorHit=${before?.editor?.hit} viewerBox=${before?.viewer?.w}x${before?.viewer?.h}@${before?.viewer?.cx},${before?.viewer?.cy} draining=${entry.draining} pageErrors=${entry.pageErrors}`);
  await page.close();
}
writeFileSync(join(outDir, "results.json"), JSON.stringify(results, null, 2));
await browser.close();
