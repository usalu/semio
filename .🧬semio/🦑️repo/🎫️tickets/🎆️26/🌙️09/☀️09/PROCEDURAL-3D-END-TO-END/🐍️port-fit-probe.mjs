/** 🔌️📷️ Runtime proof for the two claims of the `port-declaration-fit-camera` lane (ticket 26/09/09).
 *
 *   • PORT SIDES — the `extrusion-axis` node (`math.vector`) used to declare `vector`/`x`/`y`/`z` as
 *     BOTH an input and an output, so `"{nodeId}@{portId}"` named two handles and a press on the
 *     output rect resolved to the input. The proof is the renderer's own geometry census: the handle
 *     ids it publishes for that node must be disjoint per side (`…@x` in, `…@xOut` out), and a press
 *     aimed at the OUTPUT rect must be answered by the renderer with the OUTPUT handle.
 *   • FIT CAMERA — `Fit graph` must publish the camera it COMPUTED. The proof is: pan/zoom the graph
 *     out of view, press `Fit graph`, and read both the `[DEBUG] shell node-graph fit` line (wgpu)
 *     and the census that follows — every node the graph holds must be on screen afterwards, and the
 *     published camera must differ from the pre-fit one.
 *
 * Both surfaces are driven by the same script: wgpu (`:6118`) reads the renderer's diagnostics, React
 * (`:6018`) reads the DOM the React node graph paints. Nothing here is an impression: every claim is
 * a recorded console line or a measured rect.
 *
 * Usage: cd <ticket> && SEMIO_PROBE_TARGET=wgpu|react SEMIO_PROBE_OUT=port-fit/<name> bun 🐍️port-fit-probe.mjs
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const target = process.env.SEMIO_PROBE_TARGET ?? "wgpu";
const wgpu = target === "wgpu";
const url = process.env.SEMIO_PROBE_URL ?? (wgpu ? "http://127.0.0.1:6118/?plugin=generation3d&example=hexagonal-mushroom-column" : "http://127.0.0.1:6018/?plugin=generation3d&example=hexagonal-mushroom-column");
const outDir = join(import.meta.dir, "🗑️generated", process.env.SEMIO_PROBE_OUT ?? `port-fit/${target}`);
mkdirSync(outDir, { recursive: true });

const lines = [];
const t0 = Date.now();
const at = () => Date.now() - t0;
const findings = [];
const record = (id, ok, detail) => {
  findings.push({ id, ok, detail });
  console.log(`${ok ? "✓" : "✗"} ${id} — ${detail}`);
};

const browser = await chromium.launch({ headless: true, args: wgpu ? ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] : [] });
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
// 📏️ 32 000, not 4 000: the node-graph geometry census is ONE console line carrying every node and
// handle rect, and `hexagonal-mushroom-column` publishes ~7 kB of it. A 4 000-char slice truncated the
// JSON mid-object, `censusRows()`'s `JSON.parse` threw, and every handle assertion below answered `[]`
// against a census the same run had actually received (measured 2026-09-14: 14 handle rows in the
// truncated line alone).
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 32000)}`));
page.on("pageerror", (e) => lines.push(`${at()} pageerror ${String(e).slice(0, 600)}`));
await page.goto(url, { waitUntil: "domcontentloaded", timeout: 120000 });

const settle = (ms) => page.waitForTimeout(ms);
const waitFor = async (needle, ticks = 300) => {
  for (let i = 0; i < ticks; i += 1) {
    if (lines.some((line) => line.includes(needle))) return true;
    await settle(1000);
  }
  return false;
};
const nudge = async (ticks) => {
  for (let i = 0; i < ticks; i += 1) {
    await page.mouse.move(700 + (i % 5) * 9, 460 + (i % 3) * 7);
    await settle(220);
  }
};

//#region 🧊️Wgpu
if (wgpu) {
  const booted = await waitFor("boot_shell leave", 420);
  record("wgpu.boot", booted, booted ? "boot_shell left" : "the shell never left boot");
  // ⏳️ `boot_shell leave` is not a walked graph. Boot now lands in ~7 s where this probe was written
  // against a ~28 s one, so 5 s of settling read a shell whose node-graph geometry census had not been
  // published yet and every handle assertion answered `[]` — measured 2026-09-14 against a census that
  // the same build publishes with 19 handles (`🗑️generated/wgpu-verify/node-gestures/verdict.json`)
  // once settled. The tick is input-driven, so the floor NUDGES rather than sleeps.
  await nudge(Math.max(10, Math.round((Number(process.env.SEMIO_PROBE_SETTLE ?? 45) * 1000) / 220)));

  const dock = () => {
    const line = [...lines].reverse().find((entry) => entry.includes("wgpu-shell dock plan"));
    if (!line) return {};
    return Object.fromEntries([...line.matchAll(/([\w.-]+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(\d+(?:\.\d+)?),(\d+(?:\.\d+)?)/gu)].map((m) => [m[1], { w: Number(m[2]), h: Number(m[3]), x: Number(m[4]), y: Number(m[5]) }]));
  };
  const retainedRect = () => {
    const line = [...lines].reverse().find((entry) => entry.includes("wgpu-shell engine surfaces") && entry.includes("live=["));
    const m = line?.match(/live=\["([\w.-]+)@(\d+(?:\.\d+)?)x(\d+(?:\.\d+)?)\+(-?\d+(?:\.\d+)?),(-?\d+(?:\.\d+)?)"/u);
    return m ? { surfaceId: m[1], w: Number(m[2]), h: Number(m[3]), x: Number(m[4]), y: Number(m[5]) } : null;
  };
  const plan = dock();
  const graph = retainedRect() ?? plan["procedural-main"] ?? Object.values(plan)[0];
  record("wgpu.surface", Boolean(graph), graph ? `graph pane ${graph.w}x${graph.h}+${graph.x},${graph.y}` : "no node-graph pane was published");
  if (!graph) {
    writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
    await browser.close();
    process.exit(1);
  }

  const censusLine = () => [...lines].reverse().find((e) => e.includes("wgpu node-graph geometry surface=") && e.includes("entities=["));
  const censusRows = () => {
    const line = censusLine();
    if (!line) return [];
    try {
      return JSON.parse(line.slice(line.indexOf("entities=") + "entities=".length));
    } catch {
      return [];
    }
  };
  const local = (p) => [Math.round(graph.x + p[0]), Math.round(graph.y + p[1])];
  const onSurface = (x, y) => x >= 12 && y >= 12 && x <= graph.w - 12 && y <= graph.h - 12;
  const read = () => {
    const ports = new Map();
    const directions = new Map();
    const nodes = [];
    for (const row of censusRows()) {
      if (row.kind === "node" && row.geometry?.rect) nodes.push({ id: row.id, rect: row.geometry.rect });
      if (row.kind === "handle" && row.geometry?.rect) {
        if (!ports.has(row.id)) ports.set(row.id, row.geometry.rect);
        if (row.direction) directions.set(row.id, row.direction);
      }
    }
    return { ports, directions, nodes };
  };
  const fresh = async (previous, maximumMs = 40000) => {
    const deadline = Date.now() + maximumMs;
    while (Date.now() < deadline) {
      await settle(800);
      const line = censusLine();
      if (line && line !== previous) return read();
    }
    return read();
  };

  await nudge(6);
  let census = read();

  //#region 🔌️PortSides
  const axisPorts = [...census.ports.keys()].filter((id) => id.startsWith("extrusion-axis@"));
  const axisIn = axisPorts.filter((id) => census.directions.get(id) === "in");
  const axisOut = axisPorts.filter((id) => census.directions.get(id) === "out");
  const overlap = axisIn.filter((id) => axisOut.includes(id));
  record("wgpu.ports.disjoint", axisPorts.length > 0 && overlap.length === 0, `extrusion-axis handles in=${JSON.stringify(axisIn)} out=${JSON.stringify(axisOut)} overlap=${JSON.stringify(overlap)}`);
  record("wgpu.ports.suffixed", axisOut.some((id) => id.endsWith("Out")) || axisOut.length === 0, `outputs the census carries: ${JSON.stringify(axisOut)}`);

  const outputTarget = axisOut.map((id) => ({ id, rect: census.ports.get(id) })).find(({ rect }) => onSurface(rect[0] + rect[2] / 2, rect[1] + rect[3] / 2));
  if (outputTarget) {
    const before = lines.length;
    const [px, py] = local([outputTarget.rect[0] + outputTarget.rect[2] / 2, outputTarget.rect[1] + outputTarget.rect[3] / 2]);
    await page.mouse.move(px, py);
    await settle(400);
    await page.mouse.down();
    await settle(500);
    await page.mouse.move(px + 40, py + 4, { steps: 6 });
    await settle(400);
    await page.mouse.up();
    await settle(1500);
    const pressLines = lines.slice(before).filter((l) => l.includes("dag port press"));
    const named = pressLines.find((l) => l.includes(`"${outputTarget.id}"`));
    record("wgpu.press.resolvesOutput", Boolean(named), named ? named.slice(0, 320) : `no press line named ${outputTarget.id}; saw ${JSON.stringify(pressLines.slice(0, 2))}`);
  } else {
    record("wgpu.press.resolvesOutput", false, `no extrusion-axis OUTPUT handle is on screen (census outputs ${JSON.stringify(axisOut)})`);
  }
  //#endregion 🔌️PortSides

  //#region 📷️Fit
  const cameraLines = () => lines.filter((l) => l.includes("shell node-graph fit"));
  await page.mouse.move(graph.x + graph.w / 2, graph.y + graph.h / 2);
  for (let i = 0; i < 6; i += 1) {
    await page.mouse.wheel(0, 120);
    await settle(260);
  }
  await page.mouse.move(graph.x + graph.w * 0.3, graph.y + graph.h * 0.3);
  await page.mouse.down();
  await page.mouse.move(graph.x + graph.w * 0.85, graph.y + graph.h * 0.8, { steps: 10 });
  await page.mouse.up();
  await settle(2000);
  const panned = await fresh(null, 12000);
  const offBefore = panned.nodes.filter(({ rect }) => !onSurface(rect[0] + rect[2] / 2, rect[1] + rect[3] / 2)).length;

  const fitBefore = cameraLines().length;
  const censusBefore = censusLine();
  await page.mouse.move(graph.x + graph.w / 2, graph.y + graph.h / 2);
  await settle(300);
  await page.keyboard.press("f");
  await settle(2500);
  let fit = cameraLines().slice(fitBefore);
  if (fit.length === 0) {
    await page.keyboard.press("KeyF");
    await settle(2500);
    fit = cameraLines().slice(fitBefore);
  }
  record("wgpu.fit.dispatched", fit.length > 0, fit.length > 0 ? fit[fit.length - 1].slice(0, 260) : "no `shell node-graph fit` line after F");

  const after = await fresh(censusBefore, 25000);
  const offAfter = after.nodes.filter(({ rect }) => !onSurface(rect[0] + rect[2] / 2, rect[1] + rect[3] / 2)).length;
  record("wgpu.fit.framesEveryNode", after.nodes.length > 0 && offAfter === 0, `nodes off screen before the fit ${offBefore}/${panned.nodes.length}, after ${offAfter}/${after.nodes.length}`);

  const fitPayload = fit.length > 0 ? JSON.parse(fit[fit.length - 1].slice(fit[fit.length - 1].indexOf("{"))) : null;
  const sceneCamera = { x: 94.75581571737445, y: -97.50833134679668, zoom: 1.7844325616011099 };
  const stale = fitPayload !== null && Math.abs(fitPayload.x - sceneCamera.x) < 1e-6 && Math.abs(fitPayload.y - sceneCamera.y) < 1e-6 && Math.abs(fitPayload.zoom - sceneCamera.zoom) < 1e-6;
  record("wgpu.fit.notTheSceneCamera", fitPayload !== null && !stale, fitPayload ? `published ${JSON.stringify(fitPayload)} vs the scene's own ${JSON.stringify(sceneCamera)}` : "no fit payload");
  //#endregion 📷️Fit
}
//#endregion 🧊️Wgpu

//#region ⚛️React
if (!wgpu) {
  // 🪟️ React paints its node graph on ONE canvas, so the ports are not in the DOM. The host publishes
  // its own geometry through `window.__semioFlowGraphProbe[surfaceId]` — `entity(domain, id)`,
  // `hostSnapshotJson()`, `rect()` — the same door the wire-drag lane aimed through
  // (`📓️node-graph-wire-drag-2026-09-13.md` §4). Entity warm-up is asynchronous, so every read polls.
  await waitFor("converged", 90).catch(() => false);
  await settle(12000);
  const surfaces = await page.evaluate(() => Object.keys(globalThis.__semioFlowGraphProbe ?? {}));
  record("react.probeDoor", surfaces.length > 0, `__semioFlowGraphProbe surfaces: ${JSON.stringify(surfaces)}`);
  const surfaceId = surfaces.find((id) => id.includes("main")) ?? surfaces[0];

  const fixture = async () =>
    page.evaluate((id) => {
      try {
        return JSON.parse(globalThis.__semioFlowGraphProbe?.[id]?.hostSnapshotJson() ?? "null");
      } catch {
        return null;
      }
    }, surfaceId);
  const paneRect = async () => page.evaluate((id) => globalThis.__semioFlowGraphProbe?.[id]?.rect() ?? null, surfaceId);
  // 🔬️ `entity` answers `{ point, rect: { x, y, width, height }, visible }` in VIEWPORT pixels — the
  // resolver`s own units, already absolute — and warms up asynchronously, so every read polls.
  const entity = async (domain, entityId, tries = 14) => {
    for (let i = 0; i < tries; i += 1) {
      const value = await page.evaluate(([id, d, e]) => globalThis.__semioFlowGraphProbe?.[id]?.entity(d, e) ?? null, [surfaceId, domain, entityId]);
      if (value && value.rect && typeof value.rect.width === "number") return value;
      await settle(500);
    }
    return null;
  };

  const doc = await fixture();
  const widgets = Array.isArray(doc?.widgets) ? doc.widgets.map((w) => w.id ?? Object.values(w)[0]?.id).filter(Boolean) : [];
  record("react.graph", widgets.length > 0, `widgets ${JSON.stringify(widgets)}`);
  const wires = Array.isArray(doc?.synapses) ? doc.synapses.map((s) => `${s.from}@${s.fromPort}->${s.to}@${s.toPort}`) : [];
  const axisWire = wires.find((w) => w.startsWith("extrusion-axis@"));
  record("react.graph.wiredFromTheOutput", Boolean(axisWire) && axisWire.startsWith("extrusion-axis@vectorOut"), `the published document's extrusion-axis wire: ${axisWire ?? "none"}`);
  writeFileSync(join(outDir, "react-fixture.json"), JSON.stringify({ widgets, wires, camera: doc?.camera }, null, 2));

  const pane = await paneRect();
  record("react.canvas", Boolean(pane), pane ? `${Math.round(pane.width)}x${Math.round(pane.height)}+${Math.round(pane.x)},${Math.round(pane.y)}` : "no pane rect");

  //#region 🔌️PortSides
  const candidates = ["extrusion-axis@vectorOut", "extrusion-axis@vector", "extrusion-axis@xOut", "extrusion-axis@x", "extrusion-axis@z"];
  const geometries = {};
  for (const handle of candidates) geometries[handle] = await entity("handle", handle, 4);
  writeFileSync(join(outDir, "react-handles.json"), JSON.stringify(geometries, null, 2));
  const known = Object.entries(geometries).filter(([, g]) => g && g.rect && typeof g.rect.width === "number");
  record("react.handles.published", known.length > 0, `handles the host publishes a rect for: ${JSON.stringify(known.map(([k, g]) => `${k}=${Math.round(g.rect.x)},${Math.round(g.rect.y)} ${Math.round(g.rect.width)}x${Math.round(g.rect.height)} visible=${g.visible}`))}`);

  const outHandle = known.find(([k]) => k.endsWith("Out"));
  const inHandle = known.find(([k]) => k === "extrusion-axis@z" || k === "extrusion-axis@x");
  record(
    "react.handles.sidesDiffer",
    Boolean(outHandle && inHandle) && Math.round(outHandle[1].rect.x) !== Math.round(inHandle[1].rect.x),
    outHandle && inHandle ? `${outHandle[0]} x=${Math.round(outHandle[1].rect.x)} vs ${inHandle[0]} x=${Math.round(inHandle[1].rect.x)} — one id per side, one rect per id` : "one of the two sides has no rect",
  );
  const stale = geometries["extrusion-axis@vector"];
  const staleInput = stale && inHandle && Math.round(stale.rect?.x ?? -1) === Math.round(inHandle[1].rect.x);
  record("react.handles.staleIdIsTheInput", Boolean(stale) === Boolean(staleInput), stale ? `the pre-fix id extrusion-axis@vector now resolves to the INPUT column (x=${Math.round(stale.rect.x)}), which is exactly what it is: an input id and nothing else` : "extrusion-axis@vector resolves to nothing");

  if (outHandle && pane) {
    const rect = outHandle[1].rect;
    const px = Math.round(rect.x + rect.width / 2);
    const py = Math.round(rect.y + rect.height / 2);
    const before = lines.length;
    await page.mouse.move(px, py);
    await settle(400);
    await page.mouse.down();
    await settle(500);
    await page.mouse.move(px + 70, py + 34, { steps: 8 });
    await settle(500);
    await page.mouse.up();
    await settle(2500);
    const pressLines = lines.slice(before).filter((l) => l.includes("dag port press"));
    const named = pressLines.find((l) => l.includes(`"${outHandle[0]}"`));
    record("react.press.resolvesOutput", Boolean(named), named ? named.slice(0, 320) : `press lines: ${JSON.stringify(pressLines.slice(0, 3).map((l) => l.slice(0, 200)))}`);
  } else {
    record("react.press.resolvesOutput", false, "no OUTPUT handle rect to press");
  }
  //#endregion 🔌️PortSides

  //#region 📷️Fit
  if (pane) {
    const cameraOf = async () => (await fixture())?.camera ?? null;
    const opening = await cameraOf();
    await page.mouse.move(pane.x + pane.width / 2, pane.y + pane.height / 2);
    for (let i = 0; i < 6; i += 1) {
      await page.mouse.wheel(0, 130);
      await settle(300);
    }
    await settle(2000);
    const zoomed = await cameraOf();
    // 📐️ `hostSnapshotJson().camera` is the DOCUMENT camera — the guest republishes it, so it does not move
    // under a live wheel. What moves is where the host places the nodes, so the fit is measured on the
    // NODE RECTS the host resolves, before and after.
    const rectOf = async (widget) => (await entity("node", widget, 6))?.rect ?? null;
    const zoomedRect = await rectOf("extrude");
    const mark = lines.length;
    const fitControl = page.getByLabel("Fit graph").first();
    await fitControl.click({ timeout: 15000 }).catch(async () => {
      await page.keyboard.press("f");
    });
    await settle(3500);
    const fitted = await cameraOf();
    const viewportLines = lines.slice(mark).filter((l) => l.includes("nodeGraphViewport"));
    record("react.fit.dispatched", viewportLines.length > 0, viewportLines.length > 0 ? viewportLines[viewportLines.length - 1].slice(0, 240) : "no nodeGraphViewport after F");
    const fittedRect = await rectOf("extrude");
    // 📷️ The LIVE camera, as the host itself journals it: every `nodeGraphViewport` the surface
    // publishes lands in the history as `camera camera { x=… y=… zoom=… }`. The wheel's last entry is
    // the camera the fit had to correct; the entry after the `Fit graph` press is what it published.
    const journalled = lines.filter((l) => l.includes("camera camera {")).map((l) => l.slice(l.indexOf("camera camera {")));
    const zoomedCamera = journalled.slice(0, journalled.length - (lines.slice(mark).filter((l) => l.includes("camera camera {")).length || 1)).pop() ?? null;
    const fittedCamera = journalled[journalled.length - 1] ?? null;
    record("react.fit.publishedANewCamera", Boolean(fittedCamera && zoomedCamera) && fittedCamera !== zoomedCamera, `the wheel left ${zoomedCamera}; Fit graph published ${fittedCamera}`);
    // 🪧️ NOT a camera reading: `entity()` answers from the resolver's warmed cache, which is why both
    // rects below are identical across a wheel that the journal above shows moved the camera. The live
    // camera is `react.fit.publishedANewCamera`; this row only records that the read-back resolves.
    record(
      "react.fit.nodeReadBackResolves",
      Boolean(zoomedRect && fittedRect),
      `extrude rect (resolver cache, not a camera) zoomed=${zoomedRect ? `${Math.round(zoomedRect.x)},${Math.round(zoomedRect.y)}` : "none"} fitted=${fittedRect ? `${Math.round(fittedRect.x)},${Math.round(fittedRect.y)}` : "none"}; the DOCUMENT camera the guest republishes is opening=${JSON.stringify(opening)} after=${JSON.stringify(fitted)}`,
    );

    const framed = [];
    for (const widget of widgets) {
      const geometry = await entity("node", widget, 6);
      framed.push({ id: widget, visible: geometry?.visible ?? null, rect: geometry?.rect ?? null });
    }
    writeFileSync(join(outDir, "react-framed.json"), JSON.stringify(framed, null, 2));
    const measured = framed.filter((row) => row.rect && typeof row.rect.width === "number");
    const off = measured.filter((row) => row.rect.x < pane.x || row.rect.y < pane.y || row.rect.x + row.rect.width > pane.x + pane.width || row.rect.y + row.rect.height > pane.y + pane.height);
    record("react.fit.framesEveryNode", measured.length > 0 && off.length === 0, `${measured.length}/${widgets.length} nodes measured, ${off.length} outside the ${Math.round(pane.width)}x${Math.round(pane.height)} pane: ${JSON.stringify(off.map((r) => r.id))}`);
  }
  //#endregion 📷️Fit
}
//#endregion ⚛️React

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "findings.json"), JSON.stringify(findings, null, 2));
console.log(`PROBE DONE ${findings.filter((f) => f.ok).length}/${findings.length} — ${outDir}`);
await browser.close();
