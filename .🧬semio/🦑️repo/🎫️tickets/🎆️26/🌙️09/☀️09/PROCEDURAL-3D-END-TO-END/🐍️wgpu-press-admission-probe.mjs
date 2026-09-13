/** 🖱️ wgpu PRESS ADMISSION — does a mouse button ever reach `os_host handle_event` in a given mode?
 *
 * The smallest possible discriminator: boot, stand still, click five fixed points, count what the
 * host actually received. Written because a full node-graph journey in edit mode logged 231
 * `handle_event PointerMove` and ZERO `PointerDown`, while the same technique in `&mode=generate`
 * logged 26 — so the question "are presses admitted at all in this mode" had to be asked on its own.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_URL=… SEMIO_PROBE_OUT=wgpu-node-graph/press-1 bun 🐍️wgpu-press-admission-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&example=hexagonal-mushroom-column";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-node-graph/press");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 4000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
let booted = false;
for (let tick = 0; tick < 480; tick += 1) {
  if (lines.some((line) => line.includes("boot_shell leave"))) {
    booted = true;
    break;
  }
  await page.mouse.move(3 + (tick % 5), 3 + (tick % 5)).catch(() => {});
  await page.waitForTimeout(250);
}
await page.waitForTimeout(3000);

/** 🖱️ What the DOM itself saw, independent of anything the wasm host does with it. */
await page.evaluate(() => {
  globalThis.__probeDom = { down: 0, up: 0, move: 0 };
  const seen = globalThis.__probeDom;
  globalThis.addEventListener("pointerdown", () => (seen.down += 1), true);
  globalThis.addEventListener("pointerup", () => (seen.up += 1), true);
  globalThis.addEventListener("pointermove", () => (seen.move += 1), true);
});

const points = [
  [600, 400],
  [300, 300],
  [700, 600],
  [200, 500],
  [800, 200],
];
const presses = [];
for (const [x, y] of points) {
  await page.mouse.move(x, y);
  await page.waitForTimeout(1500);
  const mark = lines.length;
  await page.mouse.down();
  await page.waitForTimeout(300);
  await page.mouse.up();
  await page.waitForTimeout(1800);
  presses.push({ at: [x, y], host: lines.slice(mark).filter((line) => line.includes("handle_event Pointer") && !line.includes("PointerMove")).slice(0, 3) });
}
const dom = await page.evaluate(() => globalThis.__probeDom);

const counts = Object.fromEntries(["handle_event PointerDown", "handle_event PointerUp", "handle_event PointerMove", "discrete input queue overflow", "wgpu-shell pointer button", "graph button surface", "panicked"].map((needle) => [needle, lines.filter((line) => line.includes(needle)).length]));
const verdict = { url, booted, dom, presses, counts, seconds: Math.round(at() / 1000), consoleLines: lines.length };
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log(JSON.stringify(verdict, null, 2).slice(0, 6000));
await browser.close();
