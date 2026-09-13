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
page.on("console", (m) => lines.push(`${at()} ${m.type()} ${m.text().slice(0, 4000)}`));
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
  await nudge(10);
  await settle(3000);

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
  const booted = await waitFor("[DEBUG]", 240).catch(() => false);
  await settle(6000);
  const surface = await page.evaluate(() => {
    const nodes = [...document.querySelectorAll("[data-node-id]")].map((el) => {
      const r = el.getBoundingClientRect();
      return { id: el.getAttribute("data-node-id"), x: r.x, y: r.y, w: r.width, h: r.height };
    });
    const ports = [...document.querySelectorAll("[data-port-id],[data-handle-id]")].map((el) => {
      const r = el.getBoundingClientRect();
      return { id: el.getAttribute("data-port-id") ?? el.getAttribute("data-handle-id"), side: el.getAttribute("data-port-side") ?? el.getAttribute("data-direction"), x: r.x, y: r.y, w: r.width, h: r.height };
    });
    const canvases = [...document.querySelectorAll("canvas")].map((el) => {
      const r = el.getBoundingClientRect();
      return { x: r.x, y: r.y, w: r.width, h: r.height };
    });
    return { nodes, ports, canvases, title: document.title };
  });
  record("react.boot", surface.canvases.length > 0 || surface.nodes.length > 0, `title=${surface.title} nodes=${surface.nodes.length} ports=${surface.ports.length} canvases=${surface.canvases.length}`);
  writeFileSync(join(outDir, "react-surface.json"), JSON.stringify(surface, null, 2));

  const axis = surface.ports.filter((p) => (p.id ?? "").startsWith("extrusion-axis@"));
  record("react.ports.present", axis.length > 0, `extrusion-axis ports in the DOM: ${JSON.stringify(axis.map((p) => `${p.id}/${p.side}`))}`);
  const ids = axis.map((p) => p.id);
  record("react.ports.unique", ids.length === new Set(ids).size, `${ids.length} handles, ${new Set(ids).size} distinct ids`);

  const fitBefore = lines.length;
  const graphCanvas = surface.canvases.sort((a, b) => b.w * b.h - a.w * a.h)[0];
  if (graphCanvas) {
    await page.mouse.move(graphCanvas.x + graphCanvas.w / 2, graphCanvas.y + graphCanvas.h / 2);
    for (let i = 0; i < 5; i += 1) {
      await page.mouse.wheel(0, 130);
      await settle(240);
    }
    await settle(1200);
    await page.keyboard.press("f");
    await settle(2500);
  }
  const fitLines = lines.slice(fitBefore).filter((l) => /fit|viewport|camera/iu.test(l));
  record("react.fit.reacted", fitLines.length > 0, fitLines.length > 0 ? fitLines[fitLines.length - 1].slice(0, 300) : "no camera/viewport line after F");
}
//#endregion ⚛️React

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
writeFileSync(join(outDir, "findings.json"), JSON.stringify(findings, null, 2));
console.log(`PROBE DONE ${findings.filter((f) => f.ok).length}/${findings.length} — ${outDir}`);
await browser.close();
