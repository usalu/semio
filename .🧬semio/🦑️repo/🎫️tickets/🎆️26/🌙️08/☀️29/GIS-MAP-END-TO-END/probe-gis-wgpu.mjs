import assert from "node:assert/strict";
import { setTimeout as delay } from "node:timers/promises";
import { chromium, devices, webkit } from "playwright";
import png from "pngjs";

const target = process.argv[2] ?? "http://127.0.0.1:6140/?plugin=gis2d";
const profile = process.argv[3] ?? "Desktop Chrome";
const { defaultBrowserType, ...device } = devices[profile];
const desktop = !device.isMobile;
const launcher = defaultBrowserType === "webkit" ? webkit : chromium;
const channel = defaultBrowserType === "chromium" ? process.env.GIS_CHROMIUM_CHANNEL : undefined;
const timeout = 120_000;
const tileRoute = /^\/(osm|vt)\/\d+\/\d+\/\d+\.(png|pbf)$/;
const downloads = new Map();
const pageErrors = [];
const messages = [];
const pending = new Set();
const browser = await launcher.launch({ channel });
let phase = "navigation";
let page;

function changedPixels(a, b) {
  assert.equal(a.width, b.width);
  assert.equal(a.height, b.height);
  let changed = 0;
  for (let index = 0; index < a.data.length; index += 4) {
    if (Math.abs(a.data[index] - b.data[index]) > 16 ||
        Math.abs(a.data[index + 1] - b.data[index + 1]) > 16 ||
        Math.abs(a.data[index + 2] - b.data[index + 2]) > 16) changed++;
  }
  return changed;
}

async function snapshot(clip) {
  return png.PNG.sync.read(await page.screenshot({ clip, scale: "css", timeout: 10_000 }));
}

async function stableFrame(clip, baseline, budget) {
  let prior;
  let firstChange;
  const deadline = performance.now() + budget;
  while (performance.now() < deadline) {
    const frame = await snapshot(clip);
    const changed = baseline ? changedPixels(baseline, frame) : 0;
    const changedEnough = changed >= frame.width * frame.height / 200;
    if (changedEnough) firstChange ??= performance.now();
    if (downloads.size > 0 && !pending.size && prior && changedPixels(prior, frame) < 16 &&
        (!baseline || changedEnough)) return { frame, changed, firstChange };
    prior = frame;
    await delay(140);
  }
  assert.fail(`No stable tile frame during ${phase}`);
}

async function touch(session, count, sample) {
  for (let step = 0; step <= 12; step++) {
    const touchPoints = Array.from({ length: count }, (_, id) => ({ ...sample(step / 12, id), id }));
    await session.send("Input.dispatchTouchEvent", { type: step ? "touchMove" : "touchStart", touchPoints });
    await delay(18);
  }
  await session.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
}

try {
  const context = await browser.newContext({ ...device, locale: "en-US" });
  page = await context.newPage();
  page.on("pageerror", error => pageErrors.push(error.message));
  page.on("console", message => {
    if (["warning", "error"].includes(message.type())) messages.push(message.text().slice(0, 1200));
  });
  context.on("request", request => {
    if (tileRoute.test(new URL(request.url()).pathname)) pending.add(request);
  });
  context.on("requestfailed", request => pending.delete(request));
  context.on("requestfinished", async request => {
    if (!pending.has(request)) return;
    try {
      const response = await request.response();
      const mediaType = response?.headers()["content-type"] ?? "";
      if (response?.ok() && /^(image\/png|application\/x-protobuf)\b/.test(mediaType)) {
        downloads.set(new URL(request.url()).pathname, mediaType);
      }
    } finally {
      pending.delete(request);
    }
  });
  const start = performance.now();
  const response = await page.goto(target, { waitUntil: "domcontentloaded", timeout });
  assert(response?.ok(), `Launch HTTP status ${response?.status()}`);
  phase = "WGPU shell readiness";
  const html = page.locator("html");
  await page.locator("[data-semio-os-ready], [data-semio-os-error]").first().waitFor({ state: "attached", timeout });
  assert.equal(await html.getAttribute("data-semio-os-ready"), "gis2d", (await page.getByRole("alert").allTextContents()).join("\n"));
  const readyMs = Math.round(performance.now() - start);
  const canvas = page.locator("#semio-wgpu-canvas");
  await canvas.waitFor({ state: "visible", timeout });
  const bounds = await canvas.boundingBox();
  const x = Math.round(bounds.x + bounds.width * 0.5);
  const y = Math.round(bounds.y + bounds.height * 0.5);
  const clipWidth = Math.round(bounds.width * 0.39);
  const clipHeight = Math.round(bounds.height * 0.39);
  const clip = { x: x - Math.floor(clipWidth / 2), y: y - Math.floor(clipHeight / 2), width: clipWidth, height: clipHeight };
  phase = "initial tile paint";
  const initial = await stableFrame(clip, undefined, timeout);
  const structure = await page.evaluate(async () => JSON.parse(await window.semioWgpuIntrospection.dumpStructure()));
  const mapWindow = structure.windowIds.find(id => /gis2d-main/.test(id));
  assert(mapWindow, `GIS2D editor window is absent: ${JSON.stringify(structure.windowIds)}`);
  const distinct = new Set();
  for (let index = 0; index < initial.frame.data.length; index += 4) {
    const pixel = initial.frame.data.subarray(index, index + 3);
    distinct.add(pixel.join(","));
  }
  assert(distinct.size >= 20, `Initial map crop has only ${distinct.size} colors`);
  const paintMs = Math.round(performance.now() - start);
  const initialTiles = downloads.size;
  const result = { profile, mapWindow, readyMs, paintMs, initialTiles, initialColors: distinct.size };
  const session = !desktop && defaultBrowserType === "chromium" ? await context.newCDPSession(page) : undefined;
  if (desktop || session) {
    const dx = Math.min(100, Math.round(bounds.width * 0.2));
    phase = "pan";
    if (session) {
      await touch(session, 1, step => ({ x: x - dx / 2 + dx * step, y }));
    } else {
      await page.mouse.move(x - dx / 2, y);
      await page.mouse.down({ button: "middle" });
      await page.mouse.move(x + dx / 2, y, { steps: 12 });
      await page.mouse.up({ button: "middle" });
    }
    const panReleased = performance.now();
    const panned = await stableFrame(clip, initial.frame, 20_000);
    result.panPaintAfterReleaseMs = Math.round(panned.firstChange - panReleased);
    result.panSettledAfterReleaseMs = Math.round(performance.now() - panReleased);
    result.panChangedPixels = panned.changed;
    phase = "zoom";
    if (session) {
      await touch(session, 2, (step, id) => ({ x: x + (id ? 1 : -1) * (20 + 27 * step), y }));
    } else {
      await page.mouse.move(x, y);
      await page.mouse.wheel(0, -360);
    }
    const zoomReleased = performance.now();
    const zoomed = await stableFrame(clip, panned.frame, 20_000);
    result.zoomPaintAfterReleaseMs = Math.round(zoomed.firstChange - zoomReleased);
    result.zoomSettledAfterReleaseMs = Math.round(performance.now() - zoomReleased);
    result.zoomChangedPixels = zoomed.changed;
    assert(downloads.size > initialTiles, "Map camera interactions must request additional tiles");
  } else {
    phase = "trusted touch tap";
    await canvas.evaluate(element => element.addEventListener("pointerdown", event => {
      element.dataset.gisProbeTouch = `${event.pointerType}:${event.isTrusted}`;
    }, { once: true }));
    await page.touchscreen.tap(x, y);
    assert.equal(await canvas.getAttribute("data-gis-probe-touch"), "touch:true");
    result.trustedTap = true;
  }
  assert.deepEqual(pageErrors, []);
  console.log(JSON.stringify({ ...result, rasterTiles: [...downloads.keys()].filter(path => path.startsWith("/osm/")).length,
    vectorTiles: [...downloads.keys()].filter(path => path.startsWith("/vt/")).length, messages: messages.slice(-8) }));
} catch (error) {
  const status = page ? await page.locator("[role=status], [role=alert]").allTextContents().catch(() => []) : [];
  console.error(JSON.stringify({ profile, phase, status, tileCount: downloads.size, pageErrors, messages: messages.slice(-20) }));
  throw error;
} finally {
  await browser.close();
}
