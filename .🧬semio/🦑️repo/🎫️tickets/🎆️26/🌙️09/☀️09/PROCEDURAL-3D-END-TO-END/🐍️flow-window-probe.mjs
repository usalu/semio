/** 🔍 Flow-window render-path audit probe — three modes (headless default, headless+webgpu flags,
 * viewport resize) against the generation3d Flow window (`window:procedural-main`, NodeGraph surface).
 * Captures a screenshot plus the DOM geometry (bounding rect, computed style) of the outline (tree)
 * panel vs. the NodeGraph host container and its two canvases, so overlap/z-index/background can be
 * read off directly instead of eyeballed from a screenshot.
 * Mode `wire` additionally drives the ONE gesture only this surface offers and that no report has ever
 * proven: a port-to-port drag. It aims through `window.__semioFlowGraphProbe[surfaceId].entity("handle", …)`
 * — the graph canvas paints itself, so there is no per-port DOM to click — and asserts a NEW synapse in
 * the fixture the host republishes, then cuts it again (ticket 26/09/09/PROCEDURAL-3D-END-TO-END gap #3).
 * Usage: cd <ticket> && SEMIO_PROBE_MODE=default|webgpu|resize|wire SEMIO_PROBE_OUT=<dir> bun 🐍️flow-window-probe.mjs
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
    const text = probe?.fixtureJson?.() ?? document.querySelector(`[data-surface-id="${id}"]`)?.getAttribute("data-fixture-json") ?? null;
    try { return text ? JSON.parse(text) : null; } catch { return null; }
  }, surface);

  /** 🔌️ Screen position of one port handle, polled because the resolver warms asynchronously. */
  const handleAt = async (portId) => {
    for (let attempt = 0; attempt < 20; attempt++) {
      const point = await page.evaluate(([id, port]) => {
        const probe = window.__semioFlowGraphProbe?.[id];
        const geometry = probe?.entity?.("handle", port) ?? null;
        return geometry?.visible ? geometry.point : null;
      }, [surface, portId]);
      if (point) return point;
      await page.waitForTimeout(500);
    }
    return null;
  };

  const before = await fixture();
  const wires = (value) => (value?.synapses ?? []).map((s) => `${s.from}@${s.fromPort ?? s.from_port} -> ${s.to}@${s.toPort ?? s.to_port}`);
  console.log("[DEBUG] wires before", JSON.stringify(wires(before)));

  // 🎯️ A source output port and a target input port on a different widget that no wire already occupies.
  const occupied = new Set((before?.synapses ?? []).map((s) => `${s.to}@${s.toPort ?? s.to_port}`));
  const ports = await page.evaluate((id) => {
    const probe = window.__semioFlowGraphProbe?.[id];
    const text = probe?.fixtureJson?.() ?? null;
    if (!text) return null;
    try { return JSON.parse(text); } catch { return null; }
  }, surface);
  const candidates = [];
  for (const synapse of before?.synapses ?? []) candidates.push({ from: `${synapse.from}@${synapse.fromPort ?? synapse.from_port}`, to: `${synapse.to}@${synapse.toPort ?? synapse.to_port}`, id: synapse.id });
  const target = candidates[0] ?? null;
  writeFileSync(join(outDir, "wire-candidates.json"), JSON.stringify({ wires: wires(before), candidates, ports: ports ? Object.keys(ports) : null }, null, 2));

  if (!target) {
    console.log("[DEBUG] wire: no synapse to work from — cannot aim a port drag");
  } else {
    // 1. Cut the existing wire by dragging its TARGET endpoint away, then 2. redraw it port-to-port.
    const source = await handleAt(target.from);
    const sink = await handleAt(target.to);
    console.log("[DEBUG] wire endpoints", JSON.stringify({ target, source, sink }));
    if (source && sink) {
      const empty = { x: Math.min(source.x, sink.x) - 140, y: Math.max(source.y, sink.y) + 140 };
      await page.mouse.move(sink.x, sink.y);
      await page.mouse.down();
      await page.mouse.move(empty.x, empty.y, { steps: 12 });
      await page.mouse.up();
      await page.waitForTimeout(4000);
      const cut = await fixture();
      console.log("[DEBUG] wires after cut", JSON.stringify(wires(cut)));
      await page.screenshot({ path: join(outDir, "5-wire-cut.png") });

      await page.mouse.move(source.x, source.y);
      await page.mouse.down();
      await page.mouse.move(sink.x, sink.y, { steps: 12 });
      await page.mouse.up();
      await page.waitForTimeout(4000);
      const redrawn = await fixture();
      console.log("[DEBUG] wires after redraw", JSON.stringify(wires(redrawn)));
      await page.screenshot({ path: join(outDir, "6-wire-redrawn.png") });
      writeFileSync(join(outDir, "wire-result.json"), JSON.stringify({ before: wires(before), afterCut: wires(cut), afterRedraw: wires(redrawn), endpoints: { source, sink } }, null, 2));
    } else console.log("[DEBUG] wire: the handle resolver never reported a visible port — see wire-candidates.json");
  }
}
//#endregion 🔌️WireEditing

// zoom crop around the reported black rectangle / stray-label region for a closer look
await page.screenshot({ path: join(outDir, "4-zoom-graph-region.png"), clip: { x: 340, y: 40, width: 640, height: 400 } }).catch(() => {});

writeFileSync(join(outDir, "console.txt"), lines.join("\n"));
console.log("DONE lines", lines.length, "mode", mode);
await browser.close();
