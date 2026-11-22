import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, realpathSync } from "node:fs";
import { isAbsolute, join, relative, sep } from "node:path";
import { fileURLToPath } from "node:url";
import { chromium, devices, firefox, webkit } from "playwright";
import png from "pngjs";

const { shellReady, selector, surfaceTimeoutMs, interactionTimeoutMs, devices: matrix } = JSON.parse(readFileSync(new URL("./🧪️devices.json", import.meta.url), "utf8"));
const url = process.argv.slice(2).find(argument => !argument.startsWith("--")) ?? "http://127.0.0.1:6040/";
const browserFilter = process.argv.find(argument => argument.startsWith("--browser="))?.split("=")[1];
const deviceFilter = process.argv.find(argument => argument.startsWith("--device="))?.slice("--device=".length);
const withoutWebGpu = process.argv.includes("--without-webgpu");
const withoutAdapter = process.argv.includes("--without-adapter");
const forceGpu = process.argv.includes("--force-gpu");
const traceFrames = process.argv.includes("--trace-frames");
const surfacePackagePath = process.argv.find(argument => argument.startsWith("--surface-package="))?.slice("--surface-package=".length);
const expectedRenderer = process.argv.find(argument => argument.startsWith("--expect-renderer="))?.split("=")[1];
assert(browserFilter === undefined || ["chromium", "firefox", "webkit"].includes(browserFilter));
assert(deviceFilter === undefined || matrix.some(profile => profile.name === deviceFilter &&
  (browserFilter === undefined || devices[profile.name].defaultBrowserType === browserFilter)));
const selectedProfiles = matrix.filter(profile => deviceFilter === undefined || profile.name === deviceFilter);
assert(expectedRenderer === undefined || expectedRenderer === "canvas2d" || expectedRenderer === "webgpu");
assert(!(withoutWebGpu && (forceGpu || withoutAdapter)));
const tilePath = /^\/(osm|vt)\//;
const tileMediaType = /^(image\/png|application\/x-protobuf)\b/;
const delay = ms => new Promise(resolve => setTimeout(resolve, ms));
const dragSteps = Array.from({ length: 13 }, (_, index) => index / 12);

function readDiagnosticSurfacePackage(path) {
  const generated = realpathSync(fileURLToPath(new URL("./🗑️generated/", import.meta.url)));
  const packageRoot = realpathSync(path);
  const requested = relative(generated, packageRoot);
  assert(requested && !isAbsolute(requested) && requested.split(sep)[0] !== "..");
  const files = new Map([
    ["framework_surface.js", "text/javascript; charset=utf-8"],
    ["framework_surface_bg.wasm", "application/wasm"],
  ]);
  const rows = [...files].map(([name, contentType]) => {
    const file = join(packageRoot, name);
    assert(lstatSync(file).isFile());
    const body = readFileSync(file);
    return [name, { body, bytes: body.length, contentType, sha256: createHash("sha256").update(body).digest("hex") }];
  });
  const packageFiles = new Map(rows);
  assert(WebAssembly.validate(packageFiles.get("framework_surface_bg.wasm").body));
  const glue = packageFiles.get("framework_surface.js").body.toString("utf8");
  assert(glue.includes("framework_surface_bg.wasm") && !/^\s*import\s+/mu.test(glue));
  console.log(JSON.stringify({ surfaceDiagnosticPackage: packageRoot, files: rows.map(([name, { bytes, sha256 }]) => ({ name, bytes, sha256 })) }));
  return packageFiles;
}

const surfacePackage = surfacePackagePath === undefined ? undefined : readDiagnosticSurfacePackage(surfacePackagePath);

function pixelSignature(bytes) {
  const { data, width, height } = png.PNG.sync.read(bytes);
  const colors = [];
  for (let y = 1; y < 12; y++) {
    for (let x = 1; x < 12; x++) {
      const i = (Math.floor(y * height / 12) * width + Math.floor(x * width / 12)) * 4;
      colors.push(`${data[i] >> 3},${data[i + 1] >> 3},${data[i + 2] >> 3}`);
    }
  }
  return { data, width, height, colors, distinct: new Set(colors).size };
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
  if (after.distinct <= 2 || before.width !== after.width || before.height !== after.height) return false;
  let pixelsNeeded = Math.max(64, Math.ceil(after.width * after.height / 500));
  for (let offset = 0; offset < after.data.length; offset += 4) {
    if (Math.abs(before.data[offset] - after.data[offset]) < 16 &&
        Math.abs(before.data[offset + 1] - after.data[offset + 1]) < 16 &&
        Math.abs(before.data[offset + 2] - after.data[offset + 2]) < 16) continue;
    pixelsNeeded--;
    if (pixelsNeeded === 0) return true;
  }
  return false;
}

function sameFrame(before, after) {
  return before?.colors.every((color, index) => color === after.colors[index]);
}

function trackTiles(page) {
  const pending = new Set();
  const delivered = new Set();
  let lastCompletion = 0;
  let firstDelivery;
  const path = request => new URL(request.url()).pathname;
  const complete = request => {
    if (pending.delete(request)) lastCompletion = performance.now();
  };
  page.on("request", request => { if (tilePath.test(path(request))) pending.add(request); });
  page.on("requestfinished", async request => {
    try {
      if (!pending.has(request)) return;
      const response = await request.response();
      if (response?.ok() && tileMediaType.test(response.headers()["content-type"] ?? "")) {
        firstDelivery ??= performance.now();
        delivered.add(path(request));
      }
    } finally {
      complete(request);
    }
  });
  page.on("requestfailed", complete);
  return {
    delivered,
    get firstDelivery() { return firstDelivery; },
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
  const counts = page.locator('[id="panel:framework.panel.inspection/gis2d-play-inspector.positions-count"] [data-slot="tree-label"]');
  const countLabel = await counts.innerText();
  const positionCount = Number(/^Positions\s+(\d+)$/.exec(countLabel.trim())?.[1]);
  assert(Number.isSafeInteger(positionCount));
  await page.locator(element("framework.window.gis2dMain.engagement.toggle")).click();
  const actions = page.locator('[data-slot="window-action-pane"]:visible');
  await actions.locator('[id="action.addFeature"] [data-slot="tree-label"]').click();
  await actions.locator('[id="action.addFeature.arg.label"] input').fill("GIS device smoke");
  const count = expected => counts.filter({ hasText: new RegExp(`^Positions\\s+${expected}$`) }).waitFor({ timeout: surfaceTimeoutMs });
  const addStarted = performance.now();
  await page.locator(element("framework.window.gis2dMain.action.addFeature.execute")).click();
  await count(positionCount + 1);
  const added = performance.now();
  await canvas.press("ControlOrMeta+z");
  await count(positionCount);
  return { addFeatureMs: Math.round(added - addStarted), undoMs: Math.round(performance.now() - added) };
}

async function verifyBasemapModes(page, canvas, tiles) {
  const pane = page.locator(element("framework.window.gis2dMain.measures"));
  const mode = pane.getByRole("combobox", { name: "Render Mode", exact: true });
  if (!(await mode.isVisible())) await pane.locator(element("framework.window.gis2dMain.measures.unfold")).click();
  const published = "data-published-value";
  const settleMeasure = (control, value) => control.and(page.locator(`[${published}="${value}"]`)).waitFor({ timeout: surfaceTimeoutMs });
  const land = pane.locator(element("gis2d-main/gis2d-play-window.layer.land"));
  await settleMeasure(pane.getByRole("combobox", { name: "Vector Style", exact: true }), "colored");
  await settleMeasure(land, "true");
  const hideLand = process.argv.includes("--hide-land-during-modes");
  if (hideLand) {
    await land.click();
    await settleMeasure(land, "false");
  }
  const select = async (value, label) => {
    if (await mode.getAttribute(published) === value) return 0;
    await mode.click();
    const start = performance.now();
    await page.getByRole("option", { name: label, exact: true }).click();
    await settleMeasure(mode, value);
    return Math.round(performance.now() - start);
  };
  const tileCount = prefix => [...tiles.delivered].filter(path => path.startsWith(prefix)).length;
  const verify = async (value, label, acceptFrame) => {
    const publicationMs = await select(value, label);
    try {
      const frame = await waitForFrame(canvas, acceptFrame, surfaceTimeoutMs, tiles.settled());
      console.log(JSON.stringify({ basemap: value, publicationMs, vectorTiles: tileCount("/vt/"), rasterTiles: tileCount("/osm/") }));
      return frame;
    } catch (error) {
      throw new Error(`Basemap ${value} was published in ${publicationMs} ms but its frame did not settle: ${error}`, { cause: error });
    }
  };
  const vector = await verify("vector", "Vector", frame => frame.distinct > 2 && tileCount("/vt/") > 0);
  const image = await verify("image", "Image", frame => hasChanged(vector, frame) && tileCount("/osm/") > 0);
  await verify("combined", "Combined", frame => hasChanged(image, frame) && hasChanged(vector, frame));
  if (hideLand) await land.click();
  await settleMeasure(land, "true");
}

async function sampleFrames(page) {
  await page.evaluate(trace => {
    const times = [];
    const mapFrames = [];
    const slowFrames = [];
    const observer = trace && PerformanceObserver.supportedEntryTypes.includes("long-animation-frame")
      ? new PerformanceObserver(entries => slowFrames.push(...entries.getEntries()))
      : undefined;
    if (trace) window.__gisMapFrameCosts = mapFrames;
    observer?.observe({ type: "long-animation-frame" });
    let handle;
    const tick = now => { times.push(now); handle = requestAnimationFrame(tick); };
    handle = requestAnimationFrame(tick);
    window.stopGisFrameProbe = () => {
      cancelAnimationFrame(handle);
      delete window.__gisMapFrameCosts;
      if (!observer) return { times, ...(trace ? { mapFrames } : {}) };
      slowFrames.push(...observer.takeRecords());
      observer.disconnect();
      return { times, mapFrames, longFrames: slowFrames.sort((a, b) => b.duration - a.duration).slice(0, 5).map(frame => ({
        durationMs: Math.round(frame.duration), blockingMs: Math.round(frame.blockingDuration),
        renderMs: frame.renderStart ? Math.round(frame.startTime + frame.duration - frame.renderStart) : null,
        scripts: [...frame.scripts].sort((a, b) => b.duration - a.duration).slice(0, 3).map(script => ({
          durationMs: Math.round(script.duration), invoker: script.invoker, functionName: script.sourceFunctionName,
          source: script.sourceURL && decodeURI(new URL(script.sourceURL, location.href).pathname),
        })),
      })) };
    };
  }, traceFrames);
  return async () => {
    const { times, longFrames, mapFrames } = await page.evaluate(() => window.stopGisFrameProbe());
    const gaps = times.slice(1).map((now, index) => now - times[index]).sort((a, b) => a - b);
    const map = mapFrames?.length ? {
      draws: mapFrames.length,
      visibilityMs: Math.round(mapFrames.reduce((total, [visibility]) => total + visibility, 0)),
      renderMs: Math.round(mapFrames.reduce((total, [, render]) => total + render, 0)),
      maxRenderMs: Math.round(Math.max(...mapFrames.map(([, render]) => render))),
    } : undefined;
    return { samples: gaps.length, p95Ms: gaps.length < 7 ? null : Math.round(gaps[Math.ceil(gaps.length * 0.95) - 1]), map, longFrames };
  };
}

async function run(profile, browser) {
  const { defaultBrowserType, ...device } = devices[profile.name];
  const context = await browser.newContext({ ...device, locale: "en-US" });
  const pageErrors = [];
  const consoleErrors = [];
  const consoleWarnings = [];
  const boot = {};
  const continuationSources = [];
  const surfaceFilesServed = new Set();
  const editVerified = profile.name === "Desktop Chrome" && process.argv.includes("--edit");
  const gisCoreResponses = [];
  let frameProbeInstalled = false;
  let skippedDevUpdates = 0;
  let stage = "navigation";
  try {
    if (surfacePackage) {
      const origin = new URL(url).origin;
      const surfaceAsset = /\/wasm-packages\/[a-f\d]{24}\/(framework_surface\.js|framework_surface_bg\.wasm)$/u;
      await context.route(candidate => candidate.origin === origin && surfaceAsset.test(candidate.pathname), async route => {
        const request = new URL(route.request().url());
        const name = surfaceAsset.exec(request.pathname)[1];
        const { body, contentType } = surfacePackage.get(name);
        await route.fulfill({ body, contentType, headers: { "cache-control": "no-store" } });
        surfaceFilesServed.add(name);
      });
    }
    if (traceFrames) {
      const origin = new URL(url).origin;
      await context.route(candidate => candidate.origin === origin && decodeURIComponent(candidate.pathname).endsWith("/🧭️TiledMapHost/🖼️map-renderer/🟦️.ts"), async route => {
        const response = await route.fetch();
        const paint = /this\.pollVisibleTilesForRefresh\(\);\s*this\.session\.renderFrame\(\);/u;
        const source = await response.text();
        frameProbeInstalled = paint.test(source);
        await route.fulfill({ response, body: source.replace(paint, `
          const costs = globalThis.__gisMapFrameCosts;
          const beforeVisibility = costs && performance.now();
          this.pollVisibleTilesForRefresh();
          const beforeRender = costs && performance.now();
          this.session.renderFrame();
          if (costs) costs.push([beforeRender - beforeVisibility, performance.now() - beforeRender]);
        `) });
      });
    }
    if (process.argv.includes("--trace-continuation")) {
      const devOrigin = new URL(url).origin;
      await context.route(candidate => candidate.origin === devOrigin && decodeURIComponent(candidate.pathname).endsWith("/🚚️handle-turn-drive/🟦️.ts"), async route => {
        const response = await route.fetch();
        const source = await response.text();
        const before = "const quietMs = performance.now() - lastObservableMs;";
        const after = "const continued = await submitTurn(actorId, acknowledgements, { activation });";
        const matched = source.split(before).length === 2 && source.split(after).length === 2;
        continuationSources.push({ status: response.status(), characters: source.length, matched });
        if (!matched) {
          await route.fulfill({ response });
          return;
        }
        const instrumented = source.replace(before, `${before}
          const gisWaitStart = performance.now();
          const gisBackoff = lastTurnStatus !== "more-work" || quietMs >= COMMAND_CONTINUATION_BURST_MS;
        `).replace(after, `
          const gisWaits = globalThis.__gisContinuationWaits ??= { burstCount: 0, burstMs: 0, backoffCount: 0, backoffMs: 0 };
          gisWaits[gisBackoff ? "backoffCount" : "burstCount"]++;
          gisWaits[gisBackoff ? "backoffMs" : "burstMs"] += performance.now() - gisWaitStart;
          ${after}
        `);
        await route.fulfill({ response, body: instrumented });
      });
    }
    if (process.argv.includes("--freeze-vite-updates")) {
      const devHost = new URL(url).host;
      await context.routeWebSocket(candidate => candidate.host === devHost && candidate.pathname === "/" && candidate.searchParams.has("token"), socket => {
        socket.connectToServer().onMessage(message => {
          const type = typeof message === "string" ? JSON.parse(message).type : undefined;
          if (type === "full-reload" || type === "update" || type === "prune") skippedDevUpdates++;
          else socket.send(message);
        });
      });
    }
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
    if (editVerified) page.on("response", response => {
      const source = new URL(response.url());
      if (!source.pathname.endsWith("/semio_hub_gis_component.core.wasm")) return;
      const headers = response.headers();
      gisCoreResponses.push({ path: source.pathname, httpStatus: response.status(), bytes: Number(headers["content-length"]), etag: headers.etag });
    });
    page.setDefaultTimeout(interactionTimeoutMs);
    const tiles = trackTiles(page);
    page.on("pageerror", error => pageErrors.push(String(error)));
    page.on("console", message => {
      if (message.type() === "error") consoleErrors.push(message.text());
      if (message.type() === "warning" && consoleWarnings.length < 12) consoleWarnings.push(message.text().slice(0, 1200));
    });
    const start = performance.now();
    const elapsed = () => Math.round(performance.now() - start);
    const startup = process.argv.includes("--trace-startup") ? { workers: [], wasm: [] } : undefined;
    if (startup) {
      page.on("worker", worker => startup.workers.push({ file: new URL(worker.url(), url).pathname.split("/").at(-1), ms: elapsed() }));
      page.on("requestfinished", request => {
        const path = new URL(request.url()).pathname;
        if (path.endsWith(".wasm")) startup.wasm.push({ file: path.split("/").at(-1), completeMs: elapsed(), requestMs: Math.round(request.timing().responseEnd) });
      });
    }
    const response = await page.goto(url, { timeout: surfaceTimeoutMs, waitUntil: "domcontentloaded" });
    boot.domContentLoadedMs = elapsed();
    assert(response?.ok(), `Launch HTTP status ${response?.status()}`);
    const webGpuExposed = await page.evaluate(() => Boolean(navigator.gpu));
    if (withoutWebGpu) assert.equal(webGpuExposed, false);
    if (withoutAdapter) {
      assert.equal(webGpuExposed, true, "An exposed GPU object is required to test adapter rejection");
      assert.equal(await page.evaluate(async () => (await navigator.gpu.requestAdapter()) === null), true);
    }
    stage = "shell readiness";
    const shell = page.locator("[data-shell-ready], [data-shell-error], [data-shell-not-found]").first();
    await shell.waitFor({ state: "attached", timeout: surfaceTimeoutMs });
    const error = await shell.getAttribute("data-shell-error");
    const notFound = await shell.getAttribute("data-shell-not-found");
    assert.equal(await shell.getAttribute("data-shell-ready"), shellReady, error !== null
      ? `Shell failed for "${error}"`
      : notFound !== null ? `Shell did not find "${notFound}"` : `Expected the "${shellReady}" shell`);
    boot.shellReadyMs = elapsed();
    stage = "visible map";
    const canvas = page.locator(selector).first();
    await canvas.waitFor({ state: "visible", timeout: surfaceTimeoutMs });
    boot.canvasVisibleMs = elapsed();
    stage = "initial painted tiles";
    const initial = await waitForFrame(canvas, frame => frame.distinct > 2 && tiles.delivered.size > 0, surfaceTimeoutMs, tiles.settled());
    const mapRenderer = await canvas.getAttribute("data-map-renderer");
    if (expectedRenderer) assert.equal(mapRenderer, expectedRenderer);
    if (traceFrames) assert(frameProbeInstalled, "Map frame probe did not match the served renderer source");
    if (surfacePackage) assert.deepEqual(surfaceFilesServed, new Set(surfacePackage.keys()));
    const bootMs = elapsed();
    boot.firstTileMs = Math.round(tiles.firstDelivery - start);
    if (startup) {
      startup.crossings = await page.evaluate(() => window.__semioCrossingCensus);
      startup.continuation = await page.evaluate(() => window.__gisContinuationWaits);
      if (continuationSources.length > 0) startup.continuationSources = continuationSources;
      startup.workerSpans = await page.evaluate(() => {
        const totals = {};
        const driveStops = {};
        for (const entry of performance.getEntriesByType("measure")) {
          const prefix = "semio.hop.worker.";
          if (!entry.name.startsWith(prefix)) continue;
          const stage = entry.name.slice(prefix.length);
          const actor = entry.detail?.actorId ?? "unknown";
          const key = `${actor}/${stage}`;
          const row = totals[key] ??= { count: 0, ms: 0 };
          row.count++;
          row.ms += entry.duration;
          if (stage === "guest" && entry.detail?.driveStopped) {
            const stop = `${actor}/${entry.detail.driveStopped}`;
            const summary = driveStops[stop] ??= { count: 0, extraPolls: 0 };
            summary.count++;
            summary.extraPolls += entry.detail.drivePolls ?? 0;
          }
        }
        for (const row of Object.values(totals)) row.ms = Math.round(row.ms);
        return { totals, driveStops };
      });
      console.log(JSON.stringify({ device: profile.name, boot, bootMs, startup }));
    }
    const { x, y, width, height } = await canvas.boundingBox();
    assert(width >= 120 && height >= 120, `Map area is ${width}×${height}`);
    const cx = Math.round(x + width * 0.5);
    const cy = Math.round(y + height * 0.5);
    const result = { browser: defaultBrowserType, device: profile.name, input: profile.input, mapSize: [width, height], bootMs, boot, webGpuExposed, mapRenderer };
    if (profile.input === "tap") {
      stage = "touch tap";
      await canvas.evaluate(element => element.addEventListener("pointerdown", event => {
        element.dataset.gisProbeTouch = `${event.pointerType}:${event.isTrusted}`;
      }, { once: true }));
      await page.touchscreen.tap(cx, cy);
      assert.equal(await canvas.getAttribute("data-gis-probe-touch"), "touch:true");
      const keyboardVerified = process.argv.includes("--mobile-keyboard");
      if (keyboardVerified) {
        for (const key of ["ArrowRight", "="]) {
          stage = `emulated hardware keyboard ${key}`;
          const beforeKey = await waitForFrame(canvas, frame => frame.distinct > 2, surfaceTimeoutMs, tiles.settled());
          await canvas.evaluate(element => element.addEventListener("keydown", event => {
            element.dataset.gisProbeKey = `${event.key}:${event.isTrusted}`;
          }, { once: true }));
          await canvas.press(key);
          assert.equal(await canvas.getAttribute("data-gis-probe-key"), `${key}:true`);
          await waitForFrame(canvas, frame => hasChanged(beforeKey, frame), interactionTimeoutMs);
        }
      }
      assert.deepEqual(pageErrors, []);
      return { ...result, deliveredTiles: tiles.delivered.size, keyboardVerified, skippedDevUpdates, consoleErrors };
    }
    const dx = Math.round(Math.min(width * 0.3, 160));
    const cdp = profile.input === "touch" ? await context.newCDPSession(page) : undefined;
    stage = "pan";
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
    stage = "pan settlement";
    const settled = await waitForFrame(canvas, frame => hasChanged(initial, frame), surfaceTimeoutMs, tiles.settled());
    stage = "zoom";
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
      stage = "keyboard pan";
      const beforeKeyboard = await waitForFrame(canvas, frame => frame.distinct > 2, surfaceTimeoutMs, tiles.settled());
      await canvas.press("ArrowRight");
      await waitForFrame(canvas, frame => hasChanged(beforeKeyboard, frame), interactionTimeoutMs);
    }
    stage = "edit and undo";
    if (editVerified) console.log(JSON.stringify({ gisCoreResponses }));
    const editTiming = editVerified ? await editAndUndo(page, canvas) : undefined;
    const modesVerified = profile.name === "Desktop Chrome" && process.argv.includes("--modes");
    stage = "basemap modes";
    if (modesVerified) await verifyBasemapModes(page, canvas, tiles);
    assert.deepEqual(pageErrors, []);
    return {
      ...result,
      panResponseMs: Math.round(panShown - panStarted), panPaintAfterReleaseMs: Math.round(panShown - panReleased), panFrames,
      zoomResponseMs: Math.round(zoomShown - zoomStarted), zoomPaintAfterReleaseMs: Math.round(zoomShown - zoomReleased), zoomFrames,
      deliveredTiles: tiles.delivered.size, keyboardVerified, editVerified, editTiming, modesVerified, skippedDevUpdates, consoleErrors,
    };
  } catch (error) {
    console.error(JSON.stringify({ device: profile.name, stage, boot, skippedDevUpdates, pageErrors: pageErrors.slice(-12), consoleErrors: consoleErrors.slice(-12), consoleWarnings }));
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
  const profiles = selectedProfiles.filter(profile => devices[profile.name].defaultBrowserType === family);
  if (profiles.length === 0) continue;
  let browser;
  try {
    browser = await launchers[family]();
  } catch (error) {
    process.exitCode = 1;
    console.error(JSON.stringify({ browser: family, error: String(error) }));
    continue;
  }
  try {
    for (const profile of profiles) {
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
