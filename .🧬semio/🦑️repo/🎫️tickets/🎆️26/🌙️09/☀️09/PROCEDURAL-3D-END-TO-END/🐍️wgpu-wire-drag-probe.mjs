/** 🔗️ wgpu NODE-GRAPH WIRE DRAG — does a port-to-port drag on the Flow canvas reach the guest?
 *
 * Drives the coordinator's wgpu serve in the mode whose dock carries `procedural-main` (the node
 * graph), waits for `boot_shell leave`, then drags between candidate port points inside the graph
 * surface and reports every `wgpu node-graph screen gesture` the canvas logged plus every
 * `nodeGraphEdit` it dispatched.
 *
 * The wgpu host ticks on input, so every phase nudges the pointer (see `🐍️wgpu-boot-witness-probe.mjs`).
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-controls/wire-1 bun 🐍️wgpu-wire-drag-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-controls/wire");
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const browser = await chromium.launch({ headless: true, args: ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 6000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 2000)}`));

const nudge = async (ticks) => {
  for (let tick = 0; tick < ticks; tick += 1) {
    await page.mouse.move(3 + (tick % 5), 3 + (tick % 5)).catch(() => {});
    await page.waitForTimeout(200);
  }
};
const waitFor = async (needle, ticks) => {
  for (let tick = 0; tick < ticks; tick += 1) {
    if (lines.some((line) => line.includes(needle))) return true;
    await page.mouse.move(3 + (tick % 5), 3 + (tick % 5)).catch(() => {});
    await page.waitForTimeout(250);
  }
  return false;
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
const booted = await waitFor("boot_shell leave", 480);
await nudge(10);

const dock = () => {
  const line = [...lines].reverse().find((entry) => entry.includes("wgpu-shell dock plan"));
  if (!line) return {};
  return Object.fromEntries([...line.matchAll(/([\w-]+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(\d+(?:\.\d+)?),(\d+(?:\.\d+)?)/gu)].map((match) => [match[1], { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) }]));
};
const plan = dock();
const graph = plan["procedural-main"] ?? Object.values(plan)[0];

/** 🔌️ Where the graph reports a port under the pointer — its own pick channel, `"{nodeId}@{portId}"`. */
const hoveredHandle = () => {
  const line = [...lines].reverse().find((entry) => entry.includes('"granularity": "handle"') || entry.includes('granularity\\":\\"handle'));
  return line ? line.slice(0, 300) : null;
};

// 🔍️ The graph's own hover tells the probe where the ports are: sweep the surface, and record every
// point at which the canvas reported a `handle` pick target.
const ports = [];
if (graph) {
  for (let gy = 0; gy < 14; gy += 1) {
    for (let gx = 0; gx < 20; gx += 1) {
      const x = graph.x + 20 + (gx * (graph.w - 40)) / 19;
      const y = graph.y + 20 + (gy * (graph.h - 40)) / 13;
      await page.mouse.move(x, y);
      await page.waitForTimeout(35);
      const line = [...lines].reverse().slice(0, 6).find((entry) => entry.includes("dag hover changed") && !entry.includes("—"));
      if (line) ports.push({ point: [Math.round(x), Math.round(y)], line: line.slice(0, 200) });
    }
  }
}

// 🔗️ Drag between the two most distant hover points found — the likeliest output/input pair.
const drags = [];
if (ports.length >= 2) {
  const first = ports[0];
  const last = ports[ports.length - 1];
  for (const [from, to] of [
    [first, last],
    [last, first],
  ]) {
    const before = lines.length;
    await page.mouse.move(from.point[0], from.point[1]);
    await page.waitForTimeout(180);
    await page.mouse.down();
    await page.waitForTimeout(150);
    await page.mouse.move((from.point[0] + to.point[0]) / 2, (from.point[1] + to.point[1]) / 2);
    await page.waitForTimeout(150);
    await page.mouse.move(to.point[0], to.point[1]);
    await page.waitForTimeout(180);
    await page.mouse.up();
    await page.waitForTimeout(600);
    await nudge(6);
    const slice = lines.slice(before);
    drags.push({ from: from.point, to: to.point, screenGesture: slice.filter((line) => line.includes("node-graph screen gesture")).slice(0, 4), edits: slice.filter((line) => line.includes("nodeGraphEdit")).slice(0, 4), drawEdge: slice.filter((line) => line.includes("dag edge connected") || line.includes("dag edge removed")).slice(0, 4) });
  }
}

await page.screenshot({ path: join(outDir, "shot.png"), type: "png" }).catch(() => {});
const counts = Object.fromEntries(["boot_shell leave", "node-graph screen gesture", "nodeGraphEdit", "dag edge connected", "dag edge removed", "minimap widget pointer down", "dag hover changed", "panicked"].map((needle) => [needle, lines.filter((line) => line.includes(needle)).length]));
const verdict = { url, booted, plan, graph, portsFound: ports.length, portSamples: ports.slice(0, 6), drags, counts, hoveredHandle: hoveredHandle(), seconds: Math.round(at() / 1000), consoleLines: lines.length };
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log(JSON.stringify(verdict, null, 2).slice(0, 6000));
await browser.close();
