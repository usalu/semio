import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, devices, firefox, webkit } from "playwright";
import png from "pngjs";

const { shellReady, selector, surfaceTimeoutMs, interactionTimeoutMs, devices: matrix } = JSON.parse(readFileSync(new URL("./🧪️devices.json", import.meta.url), "utf8"));
const url = process.argv.slice(2).find(argument => !argument.startsWith("--")) ?? "http://127.0.0.1:6040/";
const browserFilter = process.argv.find(argument => argument.startsWith("--browser="))?.split("=")[1];
const withoutWebGpu = process.argv.includes("--without-webgpu");
const withoutAdapter = process.argv.includes("--without-adapter");
const forceGpu = process.argv.includes("--force-gpu");
const expectedRenderer = process.argv.find(argument => argument.startsWith("--expect-renderer="))?.split("=")[1];
assert(browserFilter === undefined || ["chromium", "firefox", "webkit"].includes(browserFilter));
assert(expectedRenderer === undefined || expectedRenderer === "canvas2d" || expectedRenderer === "webgpu");
assert(!(withoutWebGpu && (forceGpu || withoutAdapter)));
const tilePath = /^\/(osm|vt)\//;
const tileMediaType = /^(image\/png|application\/x-protobuf)\b/;
const delay = ms => new Promise(resolve => setTimeout(resolve, ms));
const dragSteps = Array.from({ length: 13 }, (_, index) => index / 12);

function pixelSignature(bytes) {
  const { data, width, height } = png.PNG.sync.read(bytes);
  const colors = [];
  for (let y = 1; y < 12; y++) {
    for (let x = 1; x < 12; x++) {
      const i = (Math.floor(y * height / 12) * width + Math.floor(x * width / 12)) * 4;
      colors.push(`${data[i] >> 3},${data[i + 1] >> 3},${data[i + 2] >> 3}`);
    }
  }
  return { colors, distinct: new Set(colors).size };
}

async function waitForFrame(canvas, accept, timeout, settled = () => true) {
  const started = performance.now();
  let frame;
  while (performance.now() - started < timeout) {
    const remaining = Math.max(1, Math.ceil(timeout - (performance.now() - started)));
    frame = pixelSignature(await canvas.screenshot({ timeout: remaining, scale: "css", animations: "disabled" }));
    if (accept(frame) && settled(frame)) return frame;
    await delay(80);
  }
  assert.fail(`Map canvas did not reach the expected state. Last distinct colors: ${frame?.distinct}`);
}

async function touchGesture(cdp, positions) {
  for (const [index, points] of positions.entries()) {
    await cdp.send("Input.dispatchTouchEvent", {
      type: index ? "touchMove" : "touchStart",
      touchPoints: points.map(([x, y], id) => ({ x, y, id })),
    });
    await delay(18);
  }
  await cdp.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
}

function hasChanged(before, after) {
  return after.distinct > 2 && after.colors.filter((color, index) => color !== before.colors[index]).length > 12;
}

function sameFrame(before, after) {
  return before?.colors.every((color, index) => color === after.colors[index]);
}

function trackTiles(page) {
  const pending = new Set();
  const delivered = new Set();
  let lastCompletion = 0;
  const path = request => new URL(request.url()).pathname;
  const complete = request => {
    if (pending.delete(request)) lastCompletion = performance.now();
  };
  page.on("request", request => { if (tilePath.test(path(request))) pending.add(request); });
  page.on("requestfinished", async request => {
    try {
      if (!pending.has(request)) return;
      const response = await request.response();
      if (response?.ok() && tileMediaType.test(response.headers()["content-type"] ?? "")) delivered.add(path(request));
    } finally {
      complete(request);
    }
  });
  page.on("requestfailed", complete);
  return {
    delivered,
    settled() {
      let previous;
      return frame => {
        const stable = sameFrame(previous, frame);
        previous = frame;
        return stable && !pending.size && performance.now() - lastCompletion >= 200;
      };
    },
  };
}

function element(id) {
  return `[id="${id}"], [data-element-alias~="${id}"]`;
}

async function editAndUndo(page, canvas) {
  const tab = page.locator('[data-tab-id="framework.panel.inspection"]:visible').first();
  if (await tab.getAttribute("aria-pressed") !== "true") await tab.click();
  const counts = page.locator(element("gis2d-play-inspector.positions-count")).locator('[data-slot="tree-description"]');
  const positionCount = Number(await counts.innerText());
  assert(Number.isSafeInteger(positionCount));
  await page.locator(element("framework.window.gis2dMain.engagement.toggle")).click();
  const actions = page.locator(element("framework.window.gis2dMain")).locator('[data-slot="window-action-pane"]');
  await actions.locator('[id="action.addFeature"] [data-slot="tree-label"]').click();
  await actions.locator('[id="action.addFeature.arg.label"] input').fill("GIS device smoke");
  const count = expected => counts.filter({ hasText: new RegExp(`^${expected}$`) }).waitFor({ timeout: interactionTimeoutMs });
  await page.locator(element("framework.window.gis2dMain.action.addFeature.execute")).click();
  await count(positionCount + 1);
  await canvas.click({ position: { x: 8, y: 8 } });
  await page.keyboard.press("ControlOrMeta+z");
  await count(positionCount);
}

async function verifyBasemapModes(page, canvas, tiles) {
  const mode = page.locator('[role="combobox"][id$="/gis2d-play-window.render-mode"]:visible').first();
  const published = "data-published-value";
  const select = async (value, label) => {
    if (await mode.getAttribute(published) !== value) {
      await mode.click();
      await page.getByRole("option", { name: label, exact: true }).click();
      await mode.and(page.locator(`[${published}="${value}"]`)).waitFor();
    }
  };
  const hasTile = prefix => [...tiles.delivered].some(path => path.startsWith(prefix));
  await select("vector", "Vector");
  const vector = await waitForFrame(canvas, frame => frame.distinct > 2 && hasTile("/vt/"), surfaceTimeoutMs, tiles.settled());
  await select("image", "Image");
  const image = await waitForFrame(canvas, frame => hasChanged(vector, frame) && hasTile("/osm/"), surfaceTimeoutMs, tiles.settled());
  await select("combined", "Combined");
  await waitForFrame(canvas, frame => hasChanged(image, frame), surfaceTimeoutMs, tiles.settled());
}

async function sampleFrames(page) {
  await page.evaluate(() => {
    const times = [];
    let handle;
    const tick = now => { times.push(now); handle = requestAnimationFrame(tick); };
    handle = requestAnimationFrame(tick);
    window.stopGisFrameProbe = () => { cancelAnimationFrame(handle); return times; };
  });
  return async () => {
    const times = await page.evaluate(() => window.stopGisFrameProbe());
    const gaps = times.slice(1).map((now, index) => now - times[index]).sort((a, b) => a - b);
    return { samples: gaps.length, p95Ms: gaps.length < 7 ? null : Math.round(gaps[Math.ceil(gaps.length * 0.95) - 1]) };
  };
}

async function run(profile, browser) {
  const { defaultBrowserType, ...device } = devices[profile.name];
  const context = await browser.newContext({ ...device, locale: "en-US" });
  const pageErrors = [];
  const consoleErrors = [];
  const consoleWarnings = [];
  try {
    if (withoutWebGpu) {
      await context.addInitScript(() => {
        Object.defineProperty(navigator, "gpu", { configurable: true, value: undefined });
      });
    } else if (withoutAdapter) {
      await context.addInitScript(() => {
        if (navigator.gpu) Object.defineProperty(navigator.gpu, "requestAdapter", { configurable: true, value: async () => null });
      });
    }
    const page = await context.newPage();
    page.setDefaultTimeout(interactionTimeoutMs);
    const tiles = trackTiles(page);
    page.on("pageerror", error => pageErrors.push(String(error)));
    page.on("console", message => {
      if (message.type() === "error") consoleErrors.push(message.text());
      if (message.type() === "warning" && consoleWarnings.length < 12) consoleWarnings.push(message.text().slice(0, 1200));
    });
    const start = performance.now();
    const response = await page.goto(url, { timeout: surfaceTimeoutMs, waitUntil: "domcontentloaded" });
    assert(response?.ok(), `Launch HTTP status ${response?.status()}`);
    const webGpuExposed = await page.evaluate(() => Boolean(navigator.gpu));
    if (withoutWebGpu) assert.equal(webGpuExposed, false);
    if (withoutAdapter) {
      assert.equal(webGpuExposed, true, "An exposed GPU object is required to test adapter rejection");
      assert.equal(await page.evaluate(async () => (await navigator.gpu.requestAdapter()) === null), true);
    }
    const shell = page.locator("[data-shell-ready], [data-shell-error], [data-shell-not-found]").first();
    await shell.waitFor({ state: "attached", timeout: surfaceTimeoutMs });
    const error = await shell.getAttribute("data-shell-error");
    const notFound = await shell.getAttribute("data-shell-not-found");
    assert.equal(await shell.getAttribute("data-shell-ready"), shellReady, error !== null
      ? `Shell failed for "${error}"`
      : notFound !== null ? `Shell did not find "${notFound}"` : `Expected the "${shellReady}" shell`);
    const canvas = page.locator(selector).first();
    await canvas.waitFor({ state: "visible", timeout: surfaceTimeoutMs });
    const initial = await waitForFrame(canvas, frame => frame.distinct > 2 && tiles.delivered.size > 0, surfaceTimeoutMs, tiles.settled());
    const mapRenderer = await canvas.getAttribute("data-map-renderer");
    if (expectedRenderer) assert.equal(mapRenderer, expectedRenderer);
    const bootMs = Math.round(performance.now() - start);
    const { x, y, width, height } = await canvas.boundingBox();
    assert(width >= 120 && height >= 120, `Map area is ${width}×${height}`);
    const cx = Math.round(x + width * 0.5);
    const cy = Math.round(y + height * 0.5);
    const result = { browser: defaultBrowserType, device: profile.name, input: profile.input, mapSize: [width, height], bootMs, webGpuExposed, mapRenderer };
    if (profile.input === "tap") {
      await canvas.evaluate(element => element.addEventListener("pointerdown", event => {
        element.dataset.gisProbeTouch = `${event.pointerType}:${event.isTrusted}`;
      }, { once: true }));
      await page.touchscreen.tap(cx, cy);
      assert.equal(await canvas.getAttribute("data-gis-probe-touch"), "touch:true");
      assert.deepEqual(pageErrors, []);
      return { ...result, deliveredTiles: tiles.delivered.size, consoleErrors };
    }
    const dx = Math.round(Math.min(width * 0.3, 160));
    const cdp = profile.input === "touch" ? await context.newCDPSession(page) : undefined;
    const stopPanFrames = await sampleFrames(page);
    const panStarted = performance.now();
    if (cdp) {
      await touchGesture(cdp, dragSteps.map(step => [[cx - 30 + dx * step, cy]]));
    } else {
      await page.mouse.move(cx - 30, cy);
      await page.mouse.down({ button: "middle" });
      for (const step of dragSteps.slice(1)) {
        await page.mouse.move(cx - 30 + dx * step, cy);
        await delay(18);
      }
      await page.mouse.up({ button: "middle" });
    }
    const panReleased = performance.now();
    const panned = await waitForFrame(canvas, frame => hasChanged(initial, frame), interactionTimeoutMs);
    const panShown = performance.now();
    const panFrames = await stopPanFrames();
    const settled = await waitForFrame(canvas, frame => hasChanged(initial, frame), surfaceTimeoutMs, tiles.settled());
    const stopZoomFrames = await sampleFrames(page);
    const zoomStarted = performance.now();
    if (cdp) {
      await touchGesture(cdp, dragSteps.map(step => [[cx - 24 - 28 * step, cy], [cx + 24 + 28 * step, cy]]));
    } else {
      await page.mouse.move(cx, cy);
      await page.mouse.wheel(0, -360);
    }
    const zoomReleased = performance.now();
    await waitForFrame(canvas, frame => hasChanged(settled, frame) && hasChanged(panned, frame), interactionTimeoutMs);
    const zoomShown = performance.now();
    const zoomFrames = await stopZoomFrames();
    const keyboardVerified = profile.input === "mouse";
    if (keyboardVerified) {
      const beforeKeyboard = await waitForFrame(canvas, frame => frame.distinct > 2, surfaceTimeoutMs, tiles.settled());
      await canvas.press("ArrowRight");
      await waitForFrame(canvas, frame => hasChanged(beforeKeyboard, frame), interactionTimeoutMs);
    }
    const editVerified = profile.name === "Desktop Chrome" && process.argv.includes("--edit");
    if (editVerified) await editAndUndo(page, canvas);
    const modesVerified = profile.name === "Desktop Chrome" && process.argv.includes("--modes");
    if (modesVerified) await verifyBasemapModes(page, canvas, tiles);
    assert.deepEqual(pageErrors, []);
    return {
      ...result,
      panResponseMs: Math.round(panShown - panStarted), panPaintAfterReleaseMs: Math.round(panShown - panReleased), panFrames,
      zoomResponseMs: Math.round(zoomShown - zoomStarted), zoomPaintAfterReleaseMs: Math.round(zoomShown - zoomReleased), zoomFrames,
      deliveredTiles: tiles.delivered.size, keyboardVerified, editVerified, modesVerified, consoleErrors,
    };
  } catch (error) {
    console.error(JSON.stringify({ device: profile.name, pageErrors: pageErrors.slice(-12), consoleErrors: consoleErrors.slice(-12), consoleWarnings }));
    throw error;
  } finally {
    await context.close();
  }
}

const launchers = {
  chromium: () => chromium.launch({
    channel: process.env.GIS_CHROMIUM_CHANNEL,
    args: forceGpu ? ["--enable-unsafe-webgpu", "--enable-gpu", "--ignore-gpu-blocklist"] : [],
  }),
  firefox: () => {
    const output = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
    mkdirSync(output, { recursive: true });
    const data = mkdtempSync(join(output, "firefox-"));
    const local = join(data, "local");
    mkdirSync(local);
    return firefox.launch({ env: { ...process.env, MOZ_APP_DATA: data, MOZ_LOCAL_APP_DATA: local } });
  },
  webkit: () => webkit.launch(),
};
for (const family of Object.keys(launchers).filter(browser => browserFilter === undefined || browser === browserFilter)) {
  let browser;
  try {
    browser = await launchers[family]();
  } catch (error) {
    process.exitCode = 1;
    console.error(JSON.stringify({ browser: family, error: String(error) }));
    continue;
  }
  try {
    for (const profile of matrix.filter(profile => devices[profile.name].defaultBrowserType === family)) {
      try {
        console.log(JSON.stringify(await run(profile, browser)));
      } catch (error) {
        process.exitCode = 1;
        console.error(JSON.stringify({ browser: family, device: profile.name, error: String(error) }));
      }
    }
  } finally {
    await browser.close();
  }
}
