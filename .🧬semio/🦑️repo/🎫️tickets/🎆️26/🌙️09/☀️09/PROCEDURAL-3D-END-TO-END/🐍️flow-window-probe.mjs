/** 🔍 Flow-window render-path audit probe — three modes (headless default, headless+webgpu flags,
 * viewport resize) against the generation3d Flow window (`window:procedural-main`, NodeGraph surface).
 * Captures a screenshot plus the DOM geometry (bounding rect, computed style) of the outline (tree)
 * panel vs. the NodeGraph host container and its two canvases, so overlap/z-index/background can be
 * read off directly instead of eyeballed from a screenshot.
 * Mode `wire` additionally drives the ONE gesture only this surface offers and that no report has ever
 * proven: a port-to-port drag. It aims through `window.__semioFlowGraphProbe[surfaceId].entity("handle", …)`
 * — the graph canvas paints itself, so there is no per-port DOM to click — and asserts a NEW synapse in
 * the fixture the host republishes, then cuts it again (ticket 26/09/09/PROCEDURAL-3D-END-TO-END gap #3).
 * Usage: cd <ticket> && SEMIO_PROBE_MODE=default|webgpu|resize|wire|reorganize SEMIO_PROBE_OUT=<dir> bun 🐍️flow-window-probe.mjs
 * (launch pattern copied from 🐍️journey-probe.mjs / 🐍️console-dump-probe.mjs in this same folder — read-only audit, no edits to those files)
 */
import { chromium } from "playwright";
import { writeFileSync, mkdirSync } from "node:fs";
import { join } from "node:path";

const url = process.env.SEMIO_PROBE_URL ?? "http://127.0.0.1:6018/?plugin=generation3d";
const mode = process.env.SEMIO_PROBE_MODE ?? "default"; // default | webgpu | resize
const seconds = Number(process.env.SEMIO_PROBE_SECONDS ?? 40);
const outDir = join(import.meta.dir, "🗑️generated", "flow-window-audit", process.env.SEMIO_PROBE_OUT ?? mode);
mkdirSync(outDir, { recursive: true });
const lines = [];
const t0 = Date.now();

const useWebgpuFlags = mode === "webgpu";
const browser = await chromium.launch({
  headless: true,
  args: useWebgpuFlags ? ["--enable-unsafe-webgpu", "--ignore-gpu-blocklist", "--use-angle=metal"] : [],
});
const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
page.on("console", (m) => lines.push(`${Date.now() - t0} ${m.type()} ${m.text().slice(0, 1500)}`));
page.on("pageerror", (e) => lines.push(`${Date.now() - t0} pageerror ${String(e).slice(0, 1500)}`));

await page.goto(url, { waitUntil: "domcontentloaded" });
await page.waitForTimeout(seconds * 1000);

const geometry = async (label) => {
  const info = await page.evaluate(() => {
    const describe = (el) => {
      if (!el) return null;
      const r = el.getBoundingClientRect();
      const cs = getComputedStyle(el);
      return {
        tag: el.tagName, cls: el.className?.toString().slice(0, 160),
        rect: { x: r.x, y: r.y, w: r.width, h: r.height },
        position: cs.position, zIndex: cs.zIndex, background: cs.backgroundColor, opacity: cs.opacity, display: cs.display,
      };
    };
    const flowHost = document.querySelector('[data-surface-id="window:procedural-main"]') ?? document.querySelector(".semio-node-graph-host");
    const canvases = flowHost ? [...flowHost.querySelectorAll("canvas")].map((c) => ({ ...describe(c), width: c.width, height: c.height })) : [];
    // outline/tree panel: sibling of the NodeGraph host inside the flow window body row
    const outlineCandidates = [...document.querySelectorAll('[id*="procedural-play-graph"], [data-tree-id*="procedural-play-graph"]')];
    const outlineRoot = outlineCandidates[0]?.closest('[class*="tree"], [role="tree"]') ?? outlineCandidates[0];
    const flowWindowBody = flowHost?.closest('[id*="procedural-play-main.body"]') ?? flowHost?.parentElement?.parentElement;
    return {
      flowHost: describe(flowHost),
      canvases,
      outlineRoot: describe(outlineRoot),
      flowWindowBody: describe(flowWindowBody),
      bodyChildren: flowWindowBody ? [...flowWindowBody.children].map(describe) : [],
    };
  });
  writeFileSync(join(outDir, `${label}-geometry.json`), JSON.stringify(info, null, 2));
  console.log(`[DEBUG] ${label} geometry`, JSON.stringify(info).slice(0, 2000));
  return info;
};

await page.screenshot({ path: join(outDir, "1-boot.png") });
await geometry("1-boot");

if (mode === "resize") {
  await page.setViewportSize({ width: 1000, height: 700 });
  await page.waitForTimeout(3000);
  await page.screenshot({ path: join(outDir, "2-resize-1000x700.png") });
  await geometry("2-resize-1000x700");
  await page.setViewportSize({ width: 1440, height: 900 });
  await page.waitForTimeout(3000);
  await page.screenshot({ path: join(outDir, "3-resize-1440x900.png") });
  await geometry("3-resize-1440x900");
}

//#region 🔌️WireEditing
if (mode === "wire") {
  const surface = "window:procedural-main";
  const fixture = async () => page.evaluate((id) => {
    const probe = window.__semioFlowGraphProbe?.[id];
    const text = probe?.fixtureJson?.() ?? null;
    try { return text ? JSON.parse(text) : null; } catch { return null; }
  }, surface);
  const wires = (value) => (value?.synapses ?? []).map((s) => `${s.from}@${s.fromPort ?? s.from_port} -> ${s.to}@${s.toPort ?? s.to_port}`);

  /** 🔌️ The rect the HOST publishes for a port, read FRESH. `dagIntroductionResolver` answers from a
   * cache and starts the real read in the background, so the first answer after any camera change is
   * the pre-change rect — the trap the 2026-09-13 run fell into when it zoomed and then aimed. Two
   * consecutive equal reads, a beat apart, is the geometry the host holds now. */
  const publishedPort = async (portId) => {
    let previous = null;
    for (let attempt = 0; attempt < 24; attempt++) {
      const geometry = await page.evaluate(([id, port]) => {
        const probe = window.__semioFlowGraphProbe?.[id];
        const resolved = probe?.entity?.("handle", port) ?? null;
        return resolved?.visible ? { point: resolved.point, rect: resolved.rect ?? null } : null;
      }, [surface, portId]);
      const key = geometry ? JSON.stringify(geometry) : null;
      if (key && key === previous) return geometry;
      previous = key;
      await page.waitForTimeout(400);
    }
    return null;
  };
  /** 🎯️ The centre of the published rect — the ONE point a caller that trusts the host can derive. */
  const centreOf = (geometry) => (geometry?.rect ? { x: geometry.rect.x + geometry.rect.width / 2, y: geometry.rect.y + geometry.rect.height / 2 } : geometry?.point ?? null);

  const previewState = async () => page.evaluate(() => {
    const host = document.querySelector("[data-meshes-json]");
    return { status: document.querySelector("[data-status-json]")?.getAttribute("data-status-json")?.slice(0, 240) ?? null, meshes: host?.getAttribute("data-meshes-json")?.length ?? 0 };
  });

  const drag = async (from, to) => {
    await page.mouse.move(from.x, from.y);
    await page.mouse.down();
    await page.mouse.move((from.x + to.x) / 2, (from.y + to.y) / 2, { steps: 8 });
    await page.mouse.move(to.x, to.y, { steps: 8 });
    await page.mouse.up();
    await page.waitForTimeout(3000);
  };

  const before = await fixture();
  console.log("[DEBUG] wires before", JSON.stringify(wires(before)));
  const synapses = before?.synapses ?? [];
  const target = synapses[0] ?? null;
  writeFileSync(join(outDir, "wire-candidates.json"), JSON.stringify({ wires: wires(before), synapses }, null, 2));

  if (!target) {
    console.log("[DEBUG] wire: the graph has no synapse to work from — cannot name a port pair");
  } else {
    const fromPort = `${target.from}@${target.fromPort ?? target.from_port}`;
    const toPort = `${target.to}@${target.toPort ?? target.to_port}`;
    const source = await publishedPort(fromPort);
    const sink = await publishedPort(toPort);
    console.log("[DEBUG] published port geometry", JSON.stringify({ fromPort, toPort, source, sink }));
    const steps = [];

    if (!source || !sink) {
      console.log("[DEBUG] wire: the host published no rect for one of the ports");
    } else {
      // 1️⃣ CUT: drag the wired input off its port onto empty canvas.
      const previewBefore = await previewState();
      // 🗑️ An empty spot INSIDE the graph canvas — where a user drops a wire to cut it. (Dropping it
      // outside is now delivered too, thanks to the canvas's pointer capture, but the gesture under
      // test is the one a user performs.)
      const host = await page.evaluate((id) => { const el = document.querySelector(`[data-surface-id="${id}"]`) ?? document.querySelector(".semio-node-graph-host"); const r = el?.getBoundingClientRect(); return r ? { x: r.x, y: r.y, width: r.width, height: r.height } : null; }, surface);
      const empty = host ? { x: host.x + host.width * 0.5, y: host.y + host.height - 60 } : { x: sink.point.x, y: sink.point.y + 200 };
      await drag(centreOf(sink), empty);
      const cut = await fixture();
      steps.push({ phase: "cut", at: centreOf(sink), wires: wires(cut) });
      console.log("[DEBUG] wires after cut", JSON.stringify(wires(cut)));
      await page.screenshot({ path: join(outDir, "5-wire-cut.png") });

      // 2️⃣ CONNECT: redraw it from the output's published centre to the input's.
      const sourceAgain = await publishedPort(fromPort);
      const sinkAgain = await publishedPort(toPort);
      await drag(centreOf(sourceAgain ?? source), centreOf(sinkAgain ?? sink));
      const redrawn = await fixture();
      steps.push({ phase: "connect", from: centreOf(sourceAgain ?? source), to: centreOf(sinkAgain ?? sink), wires: wires(redrawn) });
      console.log("[DEBUG] wires after redraw", JSON.stringify(wires(redrawn)));
      await page.screenshot({ path: join(outDir, "6-wire-redrawn.png") });
      await page.waitForTimeout(6000);
      const previewAfter = await previewState();

      const dispatched = lines.filter((line) => line.includes("node graph wire edit dispatch"));
      const result = {
        before: wires(before),
        afterCut: wires(cut),
        afterRedraw: wires(redrawn),
        cutRemovedTheWire: wires(cut).length === wires(before).length - 1,
        redrawRestoredTheWire: wires(redrawn).sort().join("|") === wires(before).sort().join("|"),
        dispatched,
        preview: { before: previewBefore, after: previewAfter },
        published: { source, sink },
        steps,
      };
      writeFileSync(join(outDir, "wire-result.json"), JSON.stringify(result, null, 2));
      console.log("[DEBUG] wire result", JSON.stringify({ cut: result.cutRemovedTheWire, redraw: result.redrawRestoredTheWire, dispatched: dispatched.length, preview: result.preview }));
    }
  }
}
//#endregion 🔌️WireEditing

//#region 🗺️ReorganizeKeybinding
if (mode === "reorganize") {
  const surface = "window:procedural-main";
  const layout = () => page.evaluate((id) => {
    const probe = window.__semioFlowGraphProbe?.[id];
    const text = probe?.fixtureJson?.() ?? document.querySelector(`[data-surface-id="${id}"]`)?.getAttribute("data-fixture-json") ?? null;
    try { const parsed = text ? JSON.parse(text) : null; return parsed?.layout ?? null; } catch { return null; }
  }, surface);
  const canvas = page.locator(`[data-surface-id="${surface}"] canvas`).first();
  const box = (await canvas.count()) ? await canvas.boundingBox() : null;
  if (box) await page.mouse.click(box.x + 20, box.y + 20);
  await page.waitForTimeout(800);
  const before = await layout();
  await page.keyboard.press("Meta+Alt+KeyL");
  await page.waitForTimeout(5000);
  const after = await layout();
  const moved = before && after ? Object.keys(after).filter((id) => before[id] && (before[id].x !== after[id].x || before[id].y !== after[id].y)) : [];
  console.log("[DEBUG] reorganize movedWidgets", JSON.stringify({ moved, count: moved.length, before, after }).slice(0, 1200));
  writeFileSync(join(outDir, "reorganize-result.json"), JSON.stringify({ moved, before, after }, null, 2));
  await page.screenshot({ path: join(outDir, "7-reorganized.png") });
}
//#endregion 🗺️ReorganizeKeybinding

// zoom crop around the reported black rectangle / stray-label region for a closer look
await page.screenshot({ path: join(outDir, "4-zoom-graph-region.png"), clip: { x: 340, y: 40, width: 640, height: 400 } }).catch(() => {});

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE lines", lines.length, "mode", mode);
await browser.close();
