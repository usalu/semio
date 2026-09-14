/** 🚫️ PORT-TYPE GUARD — the one gesture a user must never be able to complete.
 *
 * A drag from `extrusion-axis@vectorOut` (a `math.vector` output) released on `extrude@wire` (the
 * `geometry` input of `brep.solid.extrude`) used to be ACCEPTED: the graph disconnected the valid
 * `profile@wire->extrude@wire` and re-solved the model with a vector where a geometry handle belongs
 * (`📓️wgpu-generate-add-port-fit-2026-09-14.md` §3.2/§6.2).
 *
 * This probe drives BOTH halves of the pair on the React serve and asserts, per half:
 *   incompatible → refused, the wire list byte-identical, no `nodeGraphEdit` connect dispatched,
 *                  no re-solve, and the refusal hint visible WHILE the pointer is still held;
 *   compatible   → cut, then reconnected, and the wire list back to what it was.
 *
 * Ports are aimed through `window.__semioFlowGraphProbe[surfaceId].entity("handle", "node@port")` —
 * the graph canvas paints itself, so there is no per-port DOM to click — reading each rect twice a
 * beat apart, because that resolver answers from a cache and refreshes in the background
 * (`🐍️flow-window-probe.mjs`'s own trap).
 *
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane `node-graph-wire-type-guard`.
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6024/?plugin=generation3d";
const bootSeconds = Number(process.env.SEMIO_PROBE_BOOT_SECONDS ?? 45);
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? "wire-guard/probe");
mkdirSync(outDir, { recursive: true });

const lines = [];
const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1600, height: 1000 } });
const t0 = Date.now();
page.on("console", (msg) => lines.push(`${Date.now() - t0} ${msg.type()} ${msg.text().slice(0, 1200)}`));
page.on("pageerror", (err) => lines.push(`${Date.now() - t0} pageerror ${String(err).slice(0, 1200)}`));
await page.goto(url, { waitUntil: "domcontentloaded" });

const surfaces = async () => page.evaluate(() => Object.keys(window.__semioFlowGraphProbe ?? {}));
const fixtureOn = async (surface) => page.evaluate((id) => {
  const text = window.__semioFlowGraphProbe?.[id]?.fixtureJson?.() ?? null;
  try { return text ? JSON.parse(text) : null; } catch { return null; }
}, surface);
const wiresOf = (value) => (value?.synapses ?? []).map((s) => `${s.from}@${s.fromPort ?? s.from_port} -> ${s.to}@${s.toPort ?? s.to_port}`).sort();

/** ⏳️ Waits for a graph surface that actually carries the example's synapses. */
let surface = null;
for (let attempt = 0; attempt < bootSeconds * 2; attempt++) {
  for (const candidate of await surfaces()) {
    const wires = wiresOf(await fixtureOn(candidate));
    if (wires.length >= 2) { surface = candidate; break; }
  }
  if (surface) break;
  await page.waitForTimeout(500);
}
if (!surface) {
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  console.log("[DEBUG] BLOCKED no flow graph surface published a wired fixture within the boot window; surfaces=", JSON.stringify(await surfaces()));
  await browser.close();
  process.exit(1);
}
const fixture = () => fixtureOn(surface);

/** 🔌️ The rect the HOST publishes for a port, read twice a beat apart so the answer is the one the
 * host holds NOW rather than the pre-camera-change cache. */
const publishedPort = async (portId) => {
  let previous = null;
  for (let attempt = 0; attempt < 24; attempt++) {
    const geometry = await page.evaluate(([id, port]) => {
      const resolved = window.__semioFlowGraphProbe?.[id]?.entity?.("handle", port) ?? null;
      return resolved?.visible ? { point: resolved.point, rect: resolved.rect ?? null } : null;
    }, [surface, portId]);
    const key = geometry ? JSON.stringify(geometry) : null;
    if (key && key === previous) return geometry;
    previous = key;
    await page.waitForTimeout(400);
  }
  return null;
};
const centreOf = (geometry) => (geometry?.rect ? { x: geometry.rect.x + geometry.rect.width / 2, y: geometry.rect.y + geometry.rect.height / 2 } : geometry?.point ?? null);

const previewState = async () => page.evaluate(() => ({
  status: document.querySelector("[data-status-json]")?.getAttribute("data-status-json")?.slice(0, 200) ?? null,
  meshes: document.querySelector("[data-meshes-json]")?.getAttribute("data-meshes-json")?.length ?? 0,
}));
const refusalHint = async () => page.evaluate(() => {
  const node = document.querySelector("[data-wire-refusal-json]");
  return node ? { json: node.getAttribute("data-wire-refusal-json"), text: node.textContent, role: node.getAttribute("role") } : null;
});
const connectDispatches = () => lines.filter((line) => line.includes("node graph wire edit dispatch") && line.includes("\"connect\"")).length;
const evaluations = () => lines.filter((line) => /flow-extension-\w+ evaluate/u.test(line)).length;

/** 🖱️ Press at `from`, walk to `to`, SAMPLE the hint while the button is still down, then release. */
const dragSampling = async (from, to) => {
  await page.mouse.move(from.x, from.y);
  await page.mouse.down();
  await page.mouse.move((from.x + to.x) / 2, (from.y + to.y) / 2, { steps: 8 });
  await page.mouse.move(to.x, to.y, { steps: 10 });
  await page.waitForTimeout(600);
  const held = await refusalHint();
  await page.mouse.move(to.x + 1, to.y, { steps: 2 });
  await page.waitForTimeout(600);
  const heldAgain = held ?? (await refusalHint());
  await page.mouse.up();
  await page.waitForTimeout(2000);
  return heldAgain;
};

const settledWires = async (attempts = 20) => {
  let previous = null;
  for (let attempt = 0; attempt < attempts; attempt++) {
    const live = JSON.stringify(wiresOf(await fixture()));
    if (live === previous) return JSON.parse(live);
    previous = live;
    await page.waitForTimeout(500);
  }
  return JSON.parse(previous ?? "[]");
};

/** 🔍️ Fits the graph so every port the law names is ON the surface — the resolver answers `visible:
 * false` for anything outside the viewport, and the defect's own source port sits far left of the
 * boot camera. `F` is the graph surface's own fit verb. */
const fitGraph = async () => {
  const rect = await page.evaluate((id) => {
    const resolved = window.__semioFlowGraphProbe?.[id]?.entity?.("surface", id)?.rect ?? null;
    return resolved;
  }, surface);
  const centre = rect ? { x: rect.x + rect.width / 2, y: rect.y + rect.height / 2 } : { x: 800, y: 500 };
  await page.mouse.move(centre.x, centre.y);
  await page.mouse.click(centre.x, centre.y);
  await page.keyboard.press("f");
  await page.waitForTimeout(2500);
  return rect;
};
const report = { surface, url, rows: [] };
const surfaceRect = await fitGraph();
console.log("[DEBUG] fitted graph, surface rect", JSON.stringify(surfaceRect));

const before = await settledWires();
const previewBefore = await previewState();
report.wiresBefore = before;
console.log("[DEBUG] surface", surface, "wires before", JSON.stringify(before));

/** 🚧️ The port types only exist in the guest once the operator catalogue is installed. When the
 * contributions pack is REFUSED, every neuron port falls back to an untyped `IoPortSpec::simple` and
 * the rule correctly declines to refuse anything — a run in that state proves nothing, so it must
 * report BLOCKED rather than a green "no refusal was needed". */
const catalogueBlocker = () => {
  const refused = lines.find((line) => line.includes("raw bytes before decoding"));
  if (refused) return { reason: "contributions-pack-refused", line: refused };
  const sources = [...lines].reverse().find((line) => line.includes("contributions document sources"));
  if (sources && sources.includes('"status":"unresolved"')) return { reason: "no-operator-graph", line: sources };
  return null;
};

let sourcePort = await publishedPort("extrusion-axis@vectorOut");
if (!sourcePort) {
  await fitGraph();
  sourcePort = await publishedPort("extrusion-axis@vectorOut");
}
const blocker = catalogueBlocker();
if (blocker) {
  report.blocked = blocker;
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  console.log("[DEBUG] BLOCKED", blocker.reason, "-", blocker.line);
  console.log("[DEBUG] the guest has no operator catalogue, so no port declares a type and this probe cannot judge the guard");
  await browser.close();
  process.exit(2);
}
const wireInput = await publishedPort("extrude@wire");
const vectorInput = await publishedPort("extrude@vector");
report.geometry = { sourcePort, wireInput, vectorInput };
console.log("[DEBUG] port geometry", JSON.stringify(report.geometry));

if (!sourcePort || !wireInput || !vectorInput) {
  writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
  writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
  console.log("[DEBUG] BLOCKED the host published no rect for one of the three ports");
  await browser.close();
  process.exit(1);
}

// 1️⃣ INCOMPATIBLE — a `vector` output dropped on a `geometry` input.
const connectsBefore = connectDispatches();
const evaluationsBefore = evaluations();
const hint = await dragSampling(centreOf(sourcePort), centreOf(wireInput));
const afterIncompatible = await settledWires();
const previewAfter = await previewState();
await page.screenshot({ path: join(outDir, "incompatible.png"), type: "png" });
report.rows.push({
  row: "incompatible",
  from: "extrusion-axis@vectorOut",
  to: "extrude@wire",
  wiresUnchanged: JSON.stringify(afterIncompatible) === JSON.stringify(before),
  wiresAfter: afterIncompatible,
  connectDispatched: connectDispatches() - connectsBefore,
  evaluationsDuring: evaluations() - evaluationsBefore,
  hintVisible: hint != null,
  hint,
  previewBefore,
  previewAfter,
});
console.log("[DEBUG] incompatible row", JSON.stringify(report.rows[0]));

// 2️⃣ COMPATIBLE — the same output back onto the `vector` input it belongs on, after cutting it.
const emptySpot = surfaceRect ? { x: surfaceRect.x + surfaceRect.width * 0.5, y: surfaceRect.y + surfaceRect.height * 0.9 } : null;
const cutTo = emptySpot ?? { x: centreOf(vectorInput).x, y: centreOf(vectorInput).y + 180 };
await dragSampling(centreOf(vectorInput), cutTo);
const afterCut = await settledWires();
const freshSource = (await publishedPort("extrusion-axis@vectorOut")) ?? sourcePort;
const freshVector = (await publishedPort("extrude@vector")) ?? vectorInput;
const connectsBeforeCompatible = connectDispatches();
const compatibleHint = await dragSampling(centreOf(freshSource), centreOf(freshVector));
const afterCompatible = await settledWires();
await page.screenshot({ path: join(outDir, "compatible.png"), type: "png" });
report.rows.push({
  row: "compatible",
  from: "extrusion-axis@vectorOut",
  to: "extrude@vector",
  wiresAfterCut: afterCut,
  cutRemovedTheWire: !afterCut.includes("extrusion-axis@vectorOut -> extrude@vector"),
  wiresAfter: afterCompatible,
  reconnected: afterCompatible.includes("extrusion-axis@vectorOut -> extrude@vector"),
  connectDispatched: connectDispatches() - connectsBeforeCompatible,
  hintVisible: compatibleHint != null,
});
console.log("[DEBUG] compatible row", JSON.stringify(report.rows[1]));

const incompatible = report.rows[0];
const compatible = report.rows[1];
report.verdict = {
  refusedTheIncompatibleDrop: incompatible.wiresUnchanged && incompatible.connectDispatched === 0,
  keptTheDisplacedWire: incompatible.wiresAfter.includes("profile@wire -> extrude@wire"),
  didNotReSolve: incompatible.previewAfter.meshes === incompatible.previewBefore.meshes,
  showedTheReason: incompatible.hintVisible,
  acceptedTheCompatibleDrop: compatible.reconnected,
  noHintOnTheCompatibleDrop: !compatible.hintVisible,
};
report.ok = Object.values(report.verdict).every(Boolean);
writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "report.json"), JSON.stringify(report, null, 2));
console.log("[DEBUG] VERDICT", JSON.stringify(report.verdict), "ok=", report.ok);
await browser.close();
process.exit(report.ok ? 0 : 1);
