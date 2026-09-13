/** 🧲️ wgpu NODE-GRAPH SURFACE RETENTION — is the graph reachable in frames its window did not repaint?
 *
 * Drives the coordinator's wgpu serve in edit mode, waits for `boot_shell leave`, then walks the
 * whole node-graph journey the retention fix unblocks and reports, per hop, what the runtime logged:
 *
 *   0. retention — `wgpu-shell engine surfaces syncs=… drains-with-graph=… live-graphs=…` plus the
 *      retained rect the pointer path reports (`wgpu-shell graph pointer move … graphs=[…]`)
 *   1. hover     — `action=interactionHover args={… "targets":"[…]"}` with a NON-EMPTY target list:
 *      the bounded pointer path's own hover witness. (`dag hover changed` is NOT one — it is drained
 *      by `process_engine_events`, which only the screen-pointer path calls.)
 *   2. select    — a click on a hovered node and the `interactionSelect` it dispatches
 *   3. node drag — press/move/release on a node body
 *   4. wire      — port-to-port drag, `node-graph screen gesture … edits=[Connect …]` + `nodeGraphEdit`
 *   5. minimap   — a click in the minimap panel and the camera it moves (`nodeGraphViewport`)
 *   6. fit       — the `Fit graph` chrome control (`shell node-graph fit`)
 *
 * 🩸️ DISCRETE EVENTS NEED A QUIET FRAME. The wgpu host supersedes an in-flight frame build on every
 * pointer move (`frame build superseded: session generation …`), and a run that never stops moving
 * never drains a button: measured `os_host drain events … discrete=0` for a whole 48 s run with 231
 * moves and zero `PointerDown`. Every press below is therefore preceded by `settle()` — a stretch
 * with NO pointer motion at all — and the sweep moves slowly enough to let builds complete.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_OUT=wgpu-node-graph/run-1 bun 🐍️wgpu-node-graph-retention-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6118/?plugin=generation3d&example=hexagonal-mushroom-column";
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wgpu-node-graph/run");
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
/** 🤫 No pointer motion at all, so the in-flight frame build can finish and drain the discrete queue. */
const settle = async (ms = 1600) => page.waitForTimeout(ms);
const waitFor = async (needle, ticks) => {
  for (let tick = 0; tick < ticks; tick += 1) {
    if (lines.some((line) => line.includes(needle))) return true;
    await page.mouse.move(3 + (tick % 5), 3 + (tick % 5)).catch(() => {});
    await page.waitForTimeout(250);
  }
  return false;
};
const since = (mark) => lines.slice(mark);
const hits = (slice, needle, keep = 4) => slice.filter((line) => line.includes(needle)).slice(0, keep);
/** 🖱️ One press, always from a standstill. */
const click = async (x, y) => {
  await page.mouse.move(x, y);
  await settle();
  const mark = lines.length;
  await page.mouse.down();
  await page.waitForTimeout(220);
  await page.mouse.up();
  await settle(1200);
  await nudge(4);
  return mark;
};

await page.goto(url, { waitUntil: "domcontentloaded" }).catch((error) => lines.push(`${at()} gotoerror ${String(error).slice(0, 500)}`));
const booted = await waitFor("boot_shell leave", 480);
await nudge(8);
await settle(2500);

const dock = () => {
  const line = [...lines].reverse().find((entry) => entry.includes("wgpu-shell dock plan"));
  if (!line) return {};
  return Object.fromEntries([...line.matchAll(/([\w.-]+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(\d+(?:\.\d+)?),(\d+(?:\.\d+)?)/gu)].map((match) => [match[1], { w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) }]));
};
const plan = dock();
const window0 = plan["procedural-main"] ?? Object.values(plan)[0];

// 0️⃣ RETENTION — a coarse slow pass over the window makes the pointer path print the RETAINED rect,
// which is the surface's own rect inside the body and not the body rect.
if (window0) {
  for (let step = 0; step < 24; step += 1) {
    await page.mouse.move(window0.x + 20 + ((window0.w - 40) * (step % 8)) / 7, window0.y + 20 + ((window0.h - 40) * Math.floor(step / 8)) / 2);
    await page.waitForTimeout(140);
  }
}
await settle();
const retainedRect = () => {
  const line = [...lines].reverse().find((entry) => entry.includes("wgpu-shell engine surfaces") && entry.includes("live=["));
  const match = line?.match(/live=\["([\w.-]+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)"/u);
  return match ? { surfaceId: match[1], w: Number(match[2]), h: Number(match[3]), x: Number(match[4]), y: Number(match[5]) } : null;
};
const graph = retainedRect() ?? window0;
const retention = lines.filter((line) => line.includes("wgpu-shell engine surfaces")).slice(-8);

// 1️⃣ HOVER SWEEP — inside the RETAINED rect. A non-empty `interactionHover` target list says what is
// under the pointer: `granularity: "handle"` is a PORT, anything else is a node body.
//
// 🕰️ The worker's console is BATCHED: on a loaded serve every line of a 12-press journey arrived in
// one burst 45 s after the presses were made, so a per-hop `lines.slice(mark)` window is worthless.
// Every hop below therefore just performs its gesture, and the verdict is assembled at the end by
// segmenting the worker's own (correctly ordered) stream on its `graph button … down=true x=… y=…`
// lines — the press coordinates are the segment key.
const nodePoints = [];
const portPoints = [];
if (graph) {
  // 🐌️ Sweep density is bounded by ADMISSION, not by time: every move over the graph reserves three
  // bounded action credits, and a sweep faster than the frame loop drains them answers
  // `BoundedActionFault::ItemCredits` — after which the session publishes nothing further (measured:
  // a 17x15 sweep at 95 ms faulted at point 200 and the console went dead). 12x10 at 150 ms does not.
  for (let gy = 0; gy < 14; gy += 1) {
    for (let gx = 0; gx < 16; gx += 1) {
      const x = graph.x + 12 + (gx * (graph.w - 24)) / 15;
      const y = graph.y + 12 + (gy * (graph.h - 24)) / 13;
      const mark = lines.length;
      await page.mouse.move(x, y);
      await page.waitForTimeout(150);
      const hover = since(mark).find((line) => line.includes("action=interactionHover") && !line.includes('"targets":"[]"'));
      if (!hover) continue;
      const id = hover.match(/id\\":\\"([^\\"]*)\\"/u)?.[1] ?? "?";
      if (hover.includes("handle")) portPoints.push({ point: [Math.round(x), Math.round(y)], id });
      else nodePoints.push({ point: [Math.round(x), Math.round(y)], id });
    }
  }
}
await settle(3000);

const presses = [];
const press = (label, x, y) => presses.push({ label, key: `x=${x} y=${y}` });

// 2️⃣ SELECT — a node body if the sweep found one, else a port.
const bodyPoint = nodePoints[0]?.point ?? portPoints[0]?.point;
if (bodyPoint) {
  press("select", bodyPoint[0], bodyPoint[1]);
  await click(bodyPoint[0], bodyPoint[1]);
}

// 3️⃣ NODE DRAG
if (bodyPoint) {
  const [x, y] = bodyPoint;
  await page.mouse.move(x, y);
  await settle();
  press("nodeDrag", x, y);
  await page.mouse.down();
  await page.waitForTimeout(200);
  for (let step = 1; step <= 5; step += 1) {
    await page.mouse.move(x + (step * 70) / 5, y + (step * 40) / 5);
    await page.waitForTimeout(120);
  }
  await page.mouse.up();
  await settle(1400);
  await nudge(4);
}

// 4️⃣ WIRE — ordered pairs of distinct ports the sweep found, until one wires. A pair of two outputs
// is a legitimate no-op, so one failed pair proves nothing.
const pairs = [];
for (const from of portPoints) {
  for (const to of portPoints) {
    if (from.id !== to.id) pairs.push([from, to]);
  }
}
for (const [from, to] of pairs.slice(0, 6)) {
  await page.mouse.move(from.point[0], from.point[1]);
  await settle();
  press(`wire ${from.id} -> ${to.id}`, from.point[0], from.point[1]);
  await page.mouse.down();
  await page.waitForTimeout(200);
  for (let step = 1; step <= 5; step += 1) {
    await page.mouse.move(from.point[0] + ((to.point[0] - from.point[0]) * step) / 5, from.point[1] + ((to.point[1] - from.point[1]) * step) / 5);
    await page.waitForTimeout(120);
  }
  await page.mouse.up();
  await settle(1500);
  await nudge(4);
}

// 5️⃣ MINIMAP — bottom-right of the retained rect in both renderers.
if (graph) {
  const x = Math.round(graph.x + graph.w - 55);
  const y = Math.round(graph.y + graph.h - 45);
  press("minimap", x, y);
  await click(x, y);
}

// 6️⃣ FIT — the `Fit graph` chrome control, painted at the retained rect's top-left.
if (graph) {
  const x = Math.round(graph.x + 34);
  const y = Math.round(graph.y + 22);
  press("fit", x, y);
  await click(x, y);
}
await nudge(20);
await settle(4000);

/** 🕰️ One segment of the worker's own ordered log stream per press, keyed on its coordinates. */
const segments = [];
lines.forEach((line, index) => {
  if (line.includes("graph button surface") && line.includes("down=true")) segments.push({ key: line.slice(line.indexOf("x=")).trim(), from: index });
  else if (line.includes("wgpu-shell pointer button") && line.includes("down=true")) segments.push({ key: line.slice(line.indexOf("x=")).trim(), from: index });
});
segments.forEach((segment, index) => {
  segment.slice = lines.slice(segment.from, segments[index + 1]?.from ?? lines.length);
});
const segmentFor = (key) => segments.find((segment) => segment.key.startsWith(`${key} `) || segment.key === key);
const journey = presses.map(({ label, key }) => {
  const segment = segmentFor(key);
  if (!segment) return { label, key, found: false };
  return {
    label,
    key,
    graphButton: hits(segment.slice, "graph button surface", 2),
    screenGesture: hits(segment.slice, "node-graph screen gesture", 2).map((line) => line.slice(-260)),
    nodeGraphEdit: hits(segment.slice, "action=nodeGraphEdit", 2).map((line) => line.slice(-260)),
    edgeConnected: hits(segment.slice, "dag edge connected", 2).map((line) => line.slice(-160)),
    edgeRemoved: hits(segment.slice, "dag edge removed", 2).map((line) => line.slice(-120)),
    interactionSelect: hits(segment.slice, "action=interactionSelect", 1).map((line) => line.slice(-200)),
    viewport: hits(segment.slice, "action=nodeGraphViewport", 1).map((line) => line.slice(-160)),
    chromeHit: hits(segment.slice, "wgpu-shell pointer button", 2).map((line) => line.slice(-150)),
    dagDraw: hits(segment.slice, "dag draw", 2).map((line) => line.slice(-80)),
    fault: hits(segment.slice, "fault=", 2).map((line) => line.slice(-120)),
  };
});

await page.screenshot({ path: join(outDir, "shot.png"), type: "png" }).catch(() => {});
const counts = Object.fromEntries(
  [
    "boot_shell leave",
    "wgpu-shell engine surfaces",
    "graph move into",
    "graph move fault",
    "graph button surface",
    "handle_event PointerDown",
    "dag draw",
    "node-graph screen gesture",
    "action=nodeGraphEdit",
    "action=interactionSelect",
    "action=interactionHover",
    "action=nodeGraphViewport",
    "dag edge connected",
    "dag edge removed",
    "shell node-graph fit",
    "panicked",
    "BoundedActionFault",
  ].map((needle) => [needle, lines.filter((line) => line.includes(needle)).length]),
);
const verdict = { url, booted, plan, window: window0, graph, retention, ports: portPoints, bodies: nodePoints, journey, counts, seconds: Math.round(at() / 1000), consoleLines: lines.length };
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "verdict.json"), JSON.stringify(verdict, null, 2));
console.log(JSON.stringify(verdict, null, 2).slice(0, 16000));
await browser.close();
