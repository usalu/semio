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

  /** 🔌️ The rect the HOST publishes for a port, read FRESH. `dagIntroductionResolver`
   * (`🕸️NodeGraph/🟦️.tsx:1638`) answers from an UNDATED cache and starts the real read in the
   * background, so an answer can be any number of frames old and says nothing about its own age.
   *
   * 🩸️ TWO equal reads 400 ms apart is not enough: both can be the same stale cache entry. Measured —
   * the 18:28 run aimed at a `height@number` rect 6.4 px above where the graph then was (and a
   * `extrusion-axis@z` rect 19.2 px wide against the 11.9 px the same port published a run later, i.e.
   * a different port-row draw state entirely); the press landed on empty canvas, panned the camera, and
   * the row read as "redraw does not restore the wire" when no wire draw had ever STARTED. THREE equal
   * reads spanning 1.2 s is the geometry the host holds now (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
   * `📓️generate-add-flow-wire-quiet-tick-2026-09-14.md`). */
  const publishedPort = async (portId) => {
    let previous = null;
    let repeats = 0;
    for (let attempt = 0; attempt < 30; attempt++) {
      const geometry = await page.evaluate(([id, port]) => {
        const probe = window.__semioFlowGraphProbe?.[id];
        const resolved = probe?.entity?.("handle", port) ?? null;
        return resolved?.visible ? { point: resolved.point, rect: resolved.rect ?? null } : null;
      }, [surface, portId]);
      const key = geometry ? JSON.stringify(geometry) : null;
      repeats = key && key === previous ? repeats + 1 : 0;
      if (key && repeats >= 2) return geometry;
      previous = key;
      await page.waitForTimeout(400);
    }
    return null;
  };

  /** 🎯️ Whether the host's own press log says the gesture that just ran ENTERED a wire draw at `portId`.
   * A press that never reached a port cannot be evidence about wiring, and used to be indistinguishable
   * from a refusal to wire — the row simply read red. */
  const pressEnteredWireDraw = (portId, since) => lines.slice(since).some((line) => line.includes("dag port press") && line.includes(`"${portId}"`) && line.includes("interaction=draw-edge"));
  /** 🎯️ The centre of the published rect — the ONE point a caller that trusts the host can derive. */
  const centreOf = (geometry) => (geometry?.rect ? { x: geometry.rect.x + geometry.rect.width / 2, y: geometry.rect.y + geometry.rect.height / 2 } : geometry?.point ?? null);

  const previewState = async () => page.evaluate(() => {
    const host = document.querySelector("[data-meshes-json]");
    return { status: document.querySelector("[data-status-json]")?.getAttribute("data-status-json")?.slice(0, 240) ?? null, meshes: host?.getAttribute("data-meshes-json")?.length ?? 0 };
  });

  /** 🖱️ One pointer gesture, then WAIT for the guest to republish: the fixture this probe reads is the
   * guest's own scene, which lands a few frames after the dispatch settles — reading it too early
   * reports the graph as it was and hides a change that did happen. */
  const drag = async (from, to, expectWires) => {
    await page.mouse.move(from.x, from.y);
    await page.mouse.down();
    await page.mouse.move((from.x + to.x) / 2, (from.y + to.y) / 2, { steps: 8 });
    await page.mouse.move(to.x, to.y, { steps: 8 });
    await page.mouse.up();
    for (let attempt = 0; attempt < 30; attempt++) {
      await page.waitForTimeout(500);
      const live = wires(await fixture());
      if (expectWires === undefined || live.length === expectWires) return live;
    }
    return wires(await fixture());
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

    /** 🔍️ Wheels the graph camera until its zoom crosses `target`, then reports the zoom reached. The
     * LOD band a zoom lands in used to decide whether ports were hit-testable at all, so a wire proof
     * that only ever ran at the boot zoom proved one band out of six. */
    const liveZoom = () => {
      for (let index = lines.length - 1; index >= 0; index--) {
        const match = /dag draw lod=(\w+) zoom=([0-9.]+)/u.exec(lines[index]);
        if (match) return { lod: match[1], zoom: Number(match[2]) };
      }
      return null;
    };
    /** 🔍️ Wheels out one notch at a time until the host reports one of `bands` — the LOD tier is what
     * port hit-picking used to be gated on, so the proof has to name the tier it ran in, not a zoom
     * number. `minimap` is deliberately excluded: that tier is a silhouette and withholds ports. */
    const zoomToBand = async (bands) => {
      const anchor = centreOf(sink);
      for (let step = 0; step < 40; step++) {
        const live = liveZoom();
        if (live && bands.includes(live.lod)) return live;
        if (live && live.lod === "minimap") return live;
        await page.mouse.move(anchor.x, anchor.y);
        await page.mouse.wheel(0, 40);
        await page.waitForTimeout(350);
      }
      return liveZoom();
    };

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
      const cutWires = await drag(centreOf(sink), empty, wires(before).length - 1);
      const cut = await fixture();
      await page.waitForTimeout(5000);
      const previewCut = await previewState();
      steps.push({ phase: "cut", at: centreOf(sink), wires: cutWires });
      console.log("[DEBUG] wires after cut", JSON.stringify(cutWires), "preview", JSON.stringify(previewCut));
      await page.screenshot({ path: join(outDir, "5-wire-cut.png") });

      // 2️⃣ CONNECT: redraw it from the output's published centre to the input's.
      // 🎯️ A press that never entered a wire draw is the HOST's published rect missing the port it
      // names, not a refusal to wire — so it is re-aimed at freshly published geometry once and the
      // miss is REPORTED (`staleAim`), never silently graded as "the redraw does not restore the wire".
      const staleAim = [];
      let redrawn = null;
      let aimedFrom = centreOf(source);
      let aimedTo = centreOf(sink);
      let redrawPressLanded = false;
      for (let aim = 0; aim < 2; aim++) {
        const sourceAgain = await publishedPort(fromPort);
        const sinkAgain = await publishedPort(toPort);
        aimedFrom = centreOf(sourceAgain ?? source);
        aimedTo = centreOf(sinkAgain ?? sink);
        const mark = lines.length;
        await drag(aimedFrom, aimedTo, wires(before).length);
        redrawn = await fixture();
        redrawPressLanded = pressEnteredWireDraw(fromPort, mark);
        if (redrawPressLanded || wires(redrawn).length === wires(before).length) break;
        staleAim.push({ aim, aimedAt: aimedFrom, publishedSource: sourceAgain, publishedSink: sinkAgain });
        console.log("[DEBUG] wire: the press did not enter a wire draw — the published rect is stale, re-aiming", JSON.stringify(staleAim.at(-1)));
      }
      steps.push({ phase: "connect", from: aimedFrom, to: aimedTo, wires: wires(redrawn) });
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
        redrawPressLanded,
        staleAim,
        dispatched,
        preview: { before: previewBefore, afterCut: previewCut, afterRedraw: previewAfter },
        published: { source, sink },
        steps,
      };
      // 3️⃣ The same gesture in a LOWER zoom band — where port hit-picking used to be switched off.
      const zoomReached = await zoomToBand(["compact", "overview"]);
      const zoomedSink = await publishedPort(toPort);
      const zoomedSource = await publishedPort(fromPort);
      let zoomedCut = null;
      let zoomedRedraw = null;
      if (zoomedSink && zoomedSource) {
        zoomedCut = await drag(centreOf(zoomedSink), { x: host ? host.x + host.width * 0.5 : centreOf(zoomedSink).x, y: host ? host.y + host.height - 60 : centreOf(zoomedSink).y + 200 }, wires(before).length - 1);
        const sinkAfter = await publishedPort(toPort);
        const sourceAfter = await publishedPort(fromPort);
        zoomedRedraw = await drag(centreOf(sourceAfter ?? zoomedSource), centreOf(sinkAfter ?? zoomedSink), wires(before).length);
      }
      // 📶️ The wheel crosses LOD bands faster than it can be stopped in one (the host pins the tier for
      // the duration of a wheel gesture), so this phase reports the band it actually reached. The
      // `minimap` tier withholds ports BY DESIGN — a whole-graph silhouette whose nodes are a few
      // pixels wide — so a no-op there is the rule holding, not the gesture failing. Zoom coverage of
      // the port-bearing bands (0.5 / 1 / 2) is carried deterministically by the Rust law over the
      // real `DagHost` (`🕸️dag/🧪️tests/🔗️wire-edit/🦀️.rs`).
      const withholdsPorts = zoomReached?.lod === "minimap";
      result.lowZoom = { zoomReached, bandWithholdsPortsByDesign: withholdsPorts, publishedSink: zoomedSink, cut: zoomedCut, redraw: zoomedRedraw, cutRemovedTheWire: zoomedCut ? zoomedCut.length === wires(before).length - 1 : null };
      result.dispatched = lines.filter((line) => line.includes("node graph wire edit dispatch"));
      console.log("[DEBUG] low zoom wire result", JSON.stringify(result.lowZoom));
      await page.screenshot({ path: join(outDir, "7-wire-low-zoom.png") });
      writeFileSync(join(outDir, "wire-result.json"), JSON.stringify(result, null, 2));
      console.log("[DEBUG] wire result", JSON.stringify({ cut: result.cutRemovedTheWire, redraw: result.redrawRestoredTheWire, dispatched, preview: result.preview }));
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
