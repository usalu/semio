/** 📐️ Geometry recon for the occluded navbar role group: measures the navbar clusters and the top-anchored
 * chrome panel rails at one viewport, reports every element whose box intersects the role group, and
 * prints the winning stacking context at the role buttons' centres.
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane role-switch-regression.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_VIEWPORT=1280x800 bun 🐍️navbar-rail-overlap-recon.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6028/?plugin=generation3d&example=sphere-cut-with-torus";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "react-role/overlap-recon");
mkdirSync(outDir, { recursive: true });
const [width, height] = (process.env.SEMIO_PROBE_VIEWPORT ?? "1280x800").split("x").map(Number);

const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width, height } });
await page.goto(url, { waitUntil: "domcontentloaded" });
for (let i = 0; i < 120; i++) {
  await page.waitForTimeout(1000);
  const ready = await page.evaluate(() => Boolean(document.getElementById("playground.navbar.roles")));
  if (ready) break;
}
await page.waitForTimeout(3000);

const report = await page.evaluate(() => {
  const describe = (el) => {
    if (!el) return null;
    const box = el.getBoundingClientRect();
    const style = getComputedStyle(el);
    return { id: el.id || null, tag: el.tagName, cls: (el.className || "").toString().slice(0, 160), x: Math.round(box.x), y: Math.round(box.y), w: Math.round(box.width), h: Math.round(box.height), z: style.zIndex, position: style.position, pointerEvents: style.pointerEvents, dataset: { ...el.dataset } };
  };
  const roleGroup = document.getElementById("playground.navbar.roles");
  const roleBox = roleGroup.getBoundingClientRect();
  const intersects = (box) => box.width > 0 && box.height > 0 && box.x < roleBox.right && box.right > roleBox.x && box.y < roleBox.bottom && box.bottom > roleBox.y;
  const overlapping = [...document.querySelectorAll("*")]
    .filter((el) => el !== roleGroup && !roleGroup.contains(el) && !el.contains(roleGroup) && intersects(el.getBoundingClientRect()))
    .map(describe);
  const chain = (x, y) => document.elementsFromPoint(x, y).slice(0, 8).map(describe);
  const rails = [...document.querySelectorAll('[data-panel-anchor], [data-panel-rail], [id^="framework.panel."]')].map(describe);
  const ancestors = (el) => { const out = []; let node = el; while (node && node !== document.body) { out.push(describe(node)); node = node.parentElement; } return out; };
  return {
    viewport: { w: window.innerWidth, h: window.innerHeight },
    roleGroup: describe(roleGroup),
    roleAncestors: ancestors(roleGroup),
    viewerChain: chain(roleBox.x + roleBox.width * 0.75, roleBox.y + roleBox.height / 2),
    editorChain: chain(roleBox.x + roleBox.width * 0.25, roleBox.y + roleBox.height / 2),
    overlapping,
    rails,
  };
});
writeFileSync(join(outDir, `overlap-${width}x${height}.json`), JSON.stringify(report, null, 2));
await page.screenshot({ path: join(outDir, `overlap-${width}x${height}.png`) });
console.log(JSON.stringify({ viewport: report.viewport, roleGroup: report.roleGroup, viewerChain: report.viewerChain.slice(0, 5), overlapping: report.overlapping.slice(0, 12) }, null, 2));
await browser.close();
