import assert from "node:assert/strict";
import sharp from "sharp";
import { JSDOM } from "jsdom";
import { changedPaintPixels } from "./browser-physical-interactions.mjs";
import { waitForProjection } from "./browser-projection.mjs";

const numberToken = /[-+]?(?:\d+\.?\d*|\.\d+)(?:e[-+]?\d+)?/gi;

function rectBounds(path) {
  const commands = path.getAttribute("d") ?? "";
  if (commands.replace(numberToken, "").replace(/[\s,]/g, "").toUpperCase() !== "MLLLZ") return null;
  const coordinates = (commands.match(numberToken) ?? []).map(Number);
  if (coordinates.length !== 8) return null;
  const transform = path.getAttribute("transform") ?? "matrix(1 0 0 1 0 0)";
  if (!transform.startsWith("matrix(")) return null;
  const matrix = (transform.match(numberToken) ?? []).map(Number);
  if (matrix.length !== 6) return null;
  const [a, b, c, d, e, f] = matrix;
  const points = Array.from({ length: 4 }, (_, i) => {
    const [x, y] = coordinates.slice(2 * i, 2 * i + 2);
    return [a * x + c * y + e, b * x + d * y + f];
  });
  const xs = [...new Set(points.map(point => point[0]))].sort((left, right) => left - right);
  const ys = [...new Set(points.map(point => point[1]))].sort((left, right) => left - right);
  if (xs.length !== 2 || ys.length !== 2 || new Set(points.map(String)).size !== 4) return null;
  return [xs[0], ys[0], xs[1] - xs[0], ys[1] - ys[0]];
}

export function verifyLargeSheetSvg(bytes, fixture) {
  const page = fixture.document.pages[0];
  const window = new JSDOM(bytes.toString("utf8"), { contentType: "image/svg+xml" }).window;
  try {
    const { document } = window;
    const svg = document.documentElement;
    assert.equal(svg.localName, "svg");
    assert.equal(Number(svg.getAttribute("width")), page.width);
    assert.equal(Number(svg.getAttribute("height")), page.height);
    assert.deepEqual(svg.getAttribute("viewBox")?.split(/\s+/).map(Number), [0, 0, page.width, page.height]);

    const rects = new Map();
    for (const path of document.querySelectorAll("path[fill]:not([fill='none'])")) {
      const bounds = rectBounds(path);
      if (bounds === null) continue;
      const key = String(bounds);
      rects.set(key, (rects.get(key) ?? 0) + 1);
    }
    for (const frame of page.frames) {
      const { x, y, w, h } = frame.bounds;
      const key = String([x, y, w, h]);
      const count = rects.get(key) ?? 0;
      assert.ok(count > 0, "SVG draws the fixture rectangle at " + key);
      rects.set(key, count - 1);
    }
    return { valid: true, width: page.width, height: page.height, rectangleCount: page.frames.length };
  } finally { window.close(); }
}

export function verifyLargeSheetUnchanged(before, after) {
  const baseline = new JSDOM(before.toString("utf8"), { contentType: "image/svg+xml" }).window;
  let current;
  try {
    current = new JSDOM(after.toString("utf8"), { contentType: "image/svg+xml" }).window;
    assert.ok(baseline.document.documentElement.isEqualNode(current.document.documentElement),
      "viewport motion keeps the full SVG document");
    return true;
  } finally {
    current?.close();
    baseline.close();
  }
}

export async function verifyLargeViewport({ page, owner }) {
  const structure = await page.evaluate(async id => JSON.parse(await window.semioWgpuIntrospection.dumpStructure(id)), owner);
  assert.equal(structure.windowId, owner);
  const scenes = structure.nodes.filter(node => node.kind === "componentScene" && node.visible && node.path.includes("blueprint"));
  assert.equal(scenes.length, 1, "one visible Blueprint");
  const rect = scenes[0].rect;
  const [left, top, width, height] = rect;
  assert.ok(width >= 240 && height >= 180, "Blueprint has room for a pan gesture");
  const from = [left + Math.floor(width * 0.65), top + Math.floor(height * 0.55)];
  const to = [from[0] - 120, from[1] - 65];
  const readChrome = () => page.evaluate(async () => JSON.parse(await window.semioWgpuIntrospection.dumpChrome()));
  const capture = async () => sharp(await page.screenshot()).removeAlpha().raw().toBuffer({ resolveWithObject: true });
  const pause = milliseconds => page.waitForTimeout(milliseconds);
  const settled = async (name, gesture, previousCamera) => {
    const before = await capture();
    const cursor = (await readChrome()).actions.at(-1)?.seq ?? 0;
    const started = performance.now();
    await gesture();
    const inputMs = performance.now() - started;
    let cameraMs;
    let previous;
    let consecutive = 0;
    const latestCamera = async () => {
      for (const row of (await readChrome()).actions.toReversed()) {
        if (row.seq <= cursor || row.action !== "setCamera" || typeof row.args !== "string") continue;
        const args = JSON.parse(row.args);
        if (typeof args.surfaceId === "string" && args.surfaceId.includes("blueprint")) {
          const { x, y, zoom } = args.camera;
          if ([x, y, zoom].every(Number.isFinite)) return { seq: row.seq, surfaceId: args.surfaceId, x, y, zoom };
        }
      }
      return null;
    };
    const { camera, changed } = await waitForProjection(name + " camera and rendered pixels", async () => {
      const camera = await latestCamera();
      if (camera === null) return null;
      if (previous?.camera.seq !== camera.seq) cameraMs = performance.now() - started;
      const paint = await capture();
      if ((await latestCamera())?.seq !== camera.seq) {
        previous = undefined;
        consecutive = 0;
        return null;
      }
      const changed = changedPaintPixels(before, paint, rect, 24);
      consecutive = previous?.camera.seq === camera.seq &&
        changedPaintPixels(previous.paint, paint, rect, 1) === 0 ? consecutive + 1 : 1;
      previous = { camera, paint };
      return { camera, changed, consecutive };
    }, current => current?.changed > 0 && current.consecutive >= 3, { pause, interval: 100 });
    const paintMs = performance.now() - started;
    if (previousCamera) {
      assert.equal(camera.surfaceId, previousCamera.surfaceId, "wheel and pan target the same Blueprint");
      assert.equal(camera.zoom, previousCamera.zoom, "middle-button drag keeps the wheel zoom");
      assert.notDeepEqual([camera.x, camera.y], [previousCamera.x, previousCamera.y], "middle-button drag moves the camera");
    }
    const frame = await page.evaluate(async id => {
      const report = JSON.parse(await window.semioWgpuIntrospection.dumpFrameStats(id));
      return {
        uiTurn: document.getElementById("semio-wgpu-canvas")?.dataset.uiTurn ?? null,
        timing: Object.fromEntries(Object.entries(report).filter(([name, value]) => /turn|time|ms|frame/i.test(name) && typeof value === "number")),
      };
    }, owner);
    return { camera, changed, inputMs, cameraMs, paintMs, frame };
  };

  await page.mouse.move(...from);
  const wheel = await settled("wheel", () => page.mouse.wheel(0, -120));
  assert.ok(wheel.camera.zoom > 1, "wheel zoom grows from the fresh Demo camera");
  const pan = await settled("middle-button pan", async () => {
    await page.mouse.down({ button: "middle" });
    await page.mouse.move(...to, { steps: 8 });
    await page.mouse.up({ button: "middle" });
  }, wheel.camera);
  return { valid: true, wheel, pan };
}
