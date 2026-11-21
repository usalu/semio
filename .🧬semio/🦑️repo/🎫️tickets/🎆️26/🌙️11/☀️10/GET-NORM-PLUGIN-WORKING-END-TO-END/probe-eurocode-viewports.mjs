import { strict as assert } from "node:assert";
import { mkdir, readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { basename, join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const ticket = fileURLToPath(new URL(".", import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const output = join(ticket, "🗑️generated", process.env.PLAY_VIEWPORT_OUTPUT ?? "eurocode-viewports");
const require = createRequire(join(root, "🏢️semio-tech", "🎡️play", "package.json"));
const { chromium } = require("playwright");
const { PNG } = require("pngjs");
const cases = new Map([
  ["en1990", { diagrams: true }],
  ["en1991", { palette: 35 }],
  ["en1992", { diagrams: true }],
  ["en1993", { palette: 36 }],
  ["en1994", { diagrams: true }],
]);
const ids = process.argv.length > 2 ? process.argv.slice(2) : [...cases.keys()];
assert(ids.every(id => cases.has(id)));
const timeout = Number(process.env.PLAY_VIEWPORT_TIMEOUT_MS ?? 180_000);
const startupTimeout = Number(process.env.PLAY_VIEWPORT_STARTUP_TIMEOUT_MS ?? timeout);
const exampleTimeout = Number(process.env.PLAY_VIEWPORT_EXAMPLE_TIMEOUT_MS ?? timeout);
const origin = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6034/";
const preview = process.env.PLAY_TEST_GUEST;
const freshPath = preview && "/@fs" + pathToFileURL(resolve(preview)).pathname;
const previewBindings = process.env.PLAY_TEST_GUEST_JS;
const bindingsSource = previewBindings && await (async () => {
  const imports = join(root, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin", "🌐️browser-bundle", "🕸️imports", "🟦️.ts");
  const { PREVIEW2_VENDOR_RELATIVE, rewritePreview2ShimImportSource } = await import(pathToFileURL(imports));
  return rewritePreview2ShimImportSource(await readFile(previewBindings, "utf8"), `../${PREVIEW2_VENDOR_RELATIVE}/`);
})();
const currentBridge = process.env.PLAY_TEST_CURRENT_JOBS === "1" && await (async () => {
  assert(preview && previewBindings, "the current preview requires paired core and bindings");
  const plugin = join(root, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🔌️plugin");
  const generator = join(plugin, "🌐️browser-bundle", "🏗️materialization", "🌉️component-bridge-source", "🟦️.ts");
  const { pluginComponentBridgeSource } = await import(pathToFileURL(generator));
  const component = basename(previewBindings, ".js");
  const module = component.match(/^semio_hub_(norm(?:_\w+)*)_component$/u)?.[1].replaceAll("_", "-");
  assert(module, "the paired preview must be a Norm component");
  return { module, source: pluginComponentBridgeSource(component, basename(preview)) };
})();
await mkdir(output, { recursive: true });

async function reveal(scope, name, witness) {
  if (await witness.count() && await witness.isVisible()) return;
  const tab = scope.getByRole("tab", { name, exact: true });
  if (await tab.count()) await tab.first().click({ timeout });
  await witness.waitFor({ state: "visible", timeout });
}

async function assertModel(model) {
  await model.locator("canvas[data-world-geometry-drawn]").first().waitFor({ state: "visible", timeout });
  const drawn = await model.evaluate(element => {
    const decode = name => JSON.parse(element.getAttribute(name) ?? "[]");
    const canvas = element.querySelector("canvas[data-world-geometry-drawn]");
    return {
      meshes: Object.keys(decode("data-meshes-json")).length,
      instances: Object.keys(decode("data-instances-json")).length,
      revision: element.dataset.worldGeometryRevision,
      drawnRevision: canvas.dataset.worldGeometryRevision,
      vertices: Number(canvas.dataset.worldGeometryVertices),
      instance: canvas.dataset.worldGeometryDrawn,
      camera: element.dataset.viewportCameraJson,
    };
  });
  assert(drawn.meshes > 0 && drawn.instances > 0 && drawn.vertices > 0, JSON.stringify(drawn));
  assert.equal(drawn.revision, drawn.drawnRevision, "the current retained geometry must be drawn");
  return drawn;
}

async function assertModelControls(page, scope, model, selector, camera) {
  const next = scope.getByText("Next model page", { exact: true });
  const before = await model.getAttribute("data-world-geometry-revision");
  if (await next.count()) {
    await next.click({ timeout });
    await page.waitForFunction(({ selector, before }) => {
      const host = document.querySelector(selector);
      const revision = host?.getAttribute("data-world-geometry-revision");
      const draw = host?.querySelector("canvas[data-world-geometry-drawn]");
      return revision && revision !== before && draw?.getAttribute("data-world-geometry-revision") === revision;
    }, { selector, before }, { timeout });
    await scope.getByText(/^Model page 2 of /).waitFor({ state: "visible", timeout });
  }
  const pageRevision = await model.getAttribute("data-world-geometry-revision");
  const help = scope.getByText("Keyboard shortcuts", { exact: true });
  await help.click({ timeout });
  const list = scope.locator('[data-tree-window-key="norm-viewport-help.list"]');
  await list.waitFor({ state: "visible", timeout });
  await page.waitForFunction(() => Number(document.querySelector('[data-tree-window-key="norm-viewport-help.list"]')?.getAttribute("data-tree-window-length")) > 0, undefined, { timeout });
  assert.equal(await model.getAttribute("data-world-geometry-revision"), pageRevision);
  assert.equal(await model.getAttribute("data-viewport-camera-json"), camera);
  return { pageChanged: pageRevision !== before, helpRows: Number(await list.getAttribute("data-tree-window-length")) };
}

function interiorColors(data) {
  const image = PNG.sync.read(data);
  const inset = Math.max(1, Math.floor(Math.min(image.width, image.height) / 5));
  const colors = new Set();
  for (let y = inset; y < image.height - inset; y += 2) {
    for (let x = inset; x < image.width - inset; x += 2) {
      const color = (y * image.width + x) * 4;
      colors.add(image.data.readUInt32BE(color));
    }
  }
  return colors.size;
}

async function assertPalette(scope, total) {
  const menu = scope.locator('[data-tree-window-key="norm-viewport-insert"][data-tree-window-path]').first();
  await menu.waitFor({ state: "visible", timeout });
  assert.equal(Number(await menu.getAttribute("data-tree-window-total")), total);
  const outliner = scope.locator('[data-tree-window-key="norm-outliner-root"][data-tree-window-path]').first();
  await outliner.waitFor({ state: "attached", timeout });
  assert(Number(await outliner.getAttribute("data-tree-window-total")) >= 35);
  const last = menu.locator(`[data-tree-window-row="${total - 1}"]`).first();
  const requestedLastPage = await last.count() === 0;
  if (requestedLastPage) {
    await menu.locator('[data-tree-window-spacer="trailing"]').evaluate(element => element.scrollIntoView({ block: "end" }), undefined, { timeout });
  }
  await last.waitFor({ state: "visible", timeout });
  await last.scrollIntoViewIfNeeded({ timeout });
  return { total, offset: Number(await menu.getAttribute("data-tree-window-offset")), requestedLastPage, last: (await last.innerText()).trim() };
}

const browser = await chromium.launch({ headless: process.env.PLAY_HEADED !== "1", args: ["--in-process-gpu", "--enable-unsafe-swiftshader", "--disable-renderer-backgrounding"] });
try {
  for (const id of ids) {
    const result = { id, faults: [], failedRequests: [] };
    let stage = "shell";
    const started = performance.now();
    const context = await browser.newContext({ locale: "en-US", viewport: { width: 1280, height: 900 } });
    const page = await context.newPage();
    const trace = message => { if (process.env.PLAY_TRACE_ROUTE) console.error(`[DEBUG] ${id} ${message}`); };
    try {
      page.on("response", response => {
        const path = new URL(response.url()).pathname;
        if (/norm.*\.(?:core\.wasm|component\.js)$/.test(path)) trace(`${response.status()} ${path}`);
        if (response.status() >= 400) result.failedRequests.push(`${response.status()} ${path}`);
      });
      page.on("requestfailed", request => result.failedRequests.push(`${request.failure()?.errorText} ${new URL(request.url()).pathname}`));
      page.on("console", message => {
        const text = message.text();
        if ((previewBindings && message.type() === "error") || /eurocode settle|\barchive\b|semantic failure|source-not-ready|setActiveExample|ui\.surface-render/.test(text)) trace(text.split("\n")[0]);
        if (/semantic failure|scene-surface\.encode|ui\.fixed-capacity|ui\.surface-render|input #\d+ .*refused|load-failed|role switch to viewer failed/.test(text)) result.faults.push(text.split("\n")[0]);
      });
      page.on("pageerror", error => result.faults.push(error.message));
      if (process.env.PLAY_TRACE_ROUTE) {
        await page.route(url => decodeURIComponent(url.pathname).includes("/🔁️turn-settle/"), async route => {
          const response = await route.fetch();
          const source = await response.text();
          const original = "const results = [initial];";
          const instrumented = source.replace(original, `${original}
    if (actorId.startsWith("norm-eurocode")) console.info("[DEBUG] eurocode settle initial", JSON.stringify({actorId, status:wireTurnStatusTag(initial.status), operation:call?.label ?? null, required:[...(requiredSurfaceIds ?? [])], requested:[...(requestedSurfaceIds ?? [])], patches:initial.uiPatches.map(wirePatchSurfaceId), effects:initial.effects.map(effect => effect?.tag ?? Object.keys(effect ?? {})), deferred:initial.deferredEvents}));`)
            .replace("const missing = [...requiredSurfaceIds].filter((surface) => !published.has(surface));", `const missing = [...requiredSurfaceIds].filter((surface) => !published.has(surface));
      if (actorId.startsWith("norm-eurocode")) console.info("[DEBUG] eurocode settle missing", JSON.stringify({actorId, operation:call?.label ?? null, missing, published:[...published], results:results.length, status:wireTurnStatusTag(results.at(-1)?.status)}));`);
          trace(`turn-settle observer attached=${instrumented !== source}`);
          await route.fulfill({response, body: instrumented});
        });
      }
      if (currentBridge) {
        await page.context().route(url => {
          const parts = decodeURIComponent(url.pathname).split("/");
          return parts.at(-3) === "🔌️plugin-modules" && parts.at(-2)?.endsWith(currentBridge.module) && parts.at(-1) === "🌉️bridge.js";
        }, async route => {
          result.guestBridge = currentBridge.module;
          await route.fulfill({contentType: "text/javascript", body: currentBridge.source});
        });
      }
      if (bindingsSource) {
        await page.context().route(url => url.pathname.endsWith("/" + basename(previewBindings)), async route => {
          result.guestBindings = previewBindings;
          const response = await route.fetch();
          await route.fulfill({ response, body: bindingsSource });
        });
      }
      if (freshPath) {
        await page.route(url => url.pathname.endsWith("/" + basename(preview)), async route => {
          const url = new URL(route.request().url());
          url.pathname = freshPath;
          result.guest = preview;
          await route.continue({ url: url.href });
        });
      }
      await page.goto(new URL("#" + id, origin).href, { waitUntil: "commit" });
      const scope = page.locator(`[data-layered-pane="${id}"]`);
      const shell = scope.locator(`[data-shell-id="${id}"]`);
      const terminal = shell.locator("xpath=self::*[@data-shell-ready or @data-shell-error or @data-shell-not-found]");
      await terminal.waitFor({ state: "attached", timeout: startupTimeout });
      assert(await shell.getAttribute("data-shell-ready") !== null, JSON.stringify(await shell.evaluate(element => ({ state: { ...element.dataset }, detail: element.innerText.slice(0, 2000) }))));
      result.shellMs = Math.round(performance.now() - started);
      stage = "initial model";
      if (freshPath) assert.equal(result.guest, preview, "the fresh component core must be requested");
      if (previewBindings) assert.equal(result.guestBindings, previewBindings, "the paired generated bindings must be requested");
      if (currentBridge) assert.equal(result.guestBridge, currentBridge.module, "the matching current bridge must be requested");
      const skip = shell.locator('[id="ui.introduction.skip"]');
      if (await skip.count()) await skip.click({ timeout });

      const modelSelector = `[data-ui-node-key="norm.${id}.play.viewport"] .semio-world-3d-host`;
      const model = scope.locator(modelSelector).first();
      await reveal(scope, "Model", model);
      result.initial = await assertModel(model);
      result.firstModelMs = Math.round(performance.now() - started);
      result.initialColors = interiorColors(await model.locator("canvas[data-world-geometry-drawn]").first().screenshot({ timeout }));
      assert(result.initialColors > 1, "the initial model canvas should contain varying interior pixels");
      trace(`initial model drawn at ${result.initial.revision}`);

      if (process.env.PLAY_VIEWPORT_INITIAL_ONLY === "1") {
        assert.deepEqual(result.faults, []);
        result.status = "initial-paint-passed";
        continue;
      }

      const picker = scope.locator('[id="playground.navbar.fixture"]');
      stage = "example picker";
      await picker.waitFor({ state: "visible", timeout });
      const original = (await picker.innerText()).trim();
      assert(original, "the picker must show the curated example");
      await picker.click({ timeout });
      const listboxId = await picker.getAttribute("aria-controls");
      assert(listboxId, "the example picker must control a listbox");
      const options = page.getByRole("listbox").and(page.locator(`[id=${JSON.stringify(listboxId)}]`)).getByRole("option");
      const curated = options.and(page.getByRole("option", { name: original, exact: true }));
      await curated.waitFor({ state: "visible", timeout });
      assert.equal(await curated.getAttribute("aria-selected"), "true");
      if (await options.count() === 1) {
        assert.deepEqual(await options.allTextContents(), [original]);
        await page.keyboard.press("Escape");
        await options.first().waitFor({ state: "hidden", timeout });
      } else if (process.env.PLAY_VIEWPORT_SKIP_EXAMPLE_SWITCH !== "1") {
        stage = "alternate example";
        const blank = options.and(page.locator('[data-value="__none__"]'));
        const hasBlank = await blank.count() > 0;
        const alternative = hasBlank ? blank : options.and(page.locator('[aria-selected="false"]:not([aria-disabled="true"])')).first();
        const alternativeLabel = (await alternative.innerText({ timeout })).trim();
        assert.notEqual(alternativeLabel, original);
        await alternative.click({ timeout });
        await page.waitForFunction(({ id, label, selector, revision, blank }) => {
          if (document.querySelector(`[data-layered-pane="${id}"] [id="playground.navbar.fixture"]`)?.textContent?.trim() !== label) return false;
          const viewport = document.querySelector(selector);
          const next = viewport?.getAttribute("data-world-geometry-revision");
          const data = viewport?.getAttribute("data-instances-json");
          const instances = data ? Object.keys(JSON.parse(data)).length : 0;
          const draw = viewport?.querySelector("canvas[data-world-geometry-drawn]");
          if (blank) return instances === 0;
          return next && next !== revision && instances > 0 && draw?.dataset.worldGeometryRevision === next;
        }, { id, label: alternativeLabel, selector: modelSelector, revision: result.initial.revision, blank: hasBlank }, { timeout: exampleTimeout });
        const alternativeRevision = await model.count() ? await model.getAttribute("data-world-geometry-revision") : null;
        result.alternative = { label: alternativeLabel, blank: hasBlank, revision: alternativeRevision, elapsedMs: Math.round(performance.now() - started) };
        stage = "restore example";
        await picker.click({ timeout });
        await curated.click({ timeout });
        await page.waitForFunction(({ id, label, selector, previous }) => {
          if (document.querySelector(`[data-layered-pane="${id}"] [id="playground.navbar.fixture"]`)?.textContent?.trim() !== label) return false;
          const viewport = document.querySelector(selector);
          const revision = viewport?.getAttribute("data-world-geometry-revision");
          if (!revision || revision === previous) return false;
          const drawn = viewport.querySelector("canvas[data-world-geometry-drawn]");
          return Number(drawn?.dataset.worldGeometryVertices) > 0 && drawn?.dataset.worldGeometryRevision === revision;
        }, { id, label: original, selector: modelSelector, previous: alternativeRevision }, { timeout: exampleTimeout });
        result.restored = await assertModel(model);
        result.restoreMs = Math.round(performance.now() - started);
        trace(`restored ${original} at ${result.restored.revision}`);
      } else {
        await page.keyboard.press("Escape");
        await options.first().waitFor({ state: "hidden", timeout });
      }
      const currentModel = result.restored ?? result.initial;
      stage = "camera zoom";
      result.modelColors = interiorColors(await model.locator("canvas[data-world-geometry-drawn]").first().screenshot({ path: join(output, id + "-model.png"), timeout }));
      assert(result.modelColors > 1, "the model canvas should contain varying interior pixels");
      assert(currentModel.camera, "the viewport exports its live camera");
      await model.hover({ timeout });
      await page.mouse.wheel(0, -125);
      await page.waitForFunction(({ selector, previous }) => {
        const camera = document.querySelector(selector)?.getAttribute("data-viewport-camera-json");
        return camera && camera !== previous;
      }, { selector: modelSelector, previous: currentModel.camera }, { timeout });
      result.cameraAfterZoom = await model.getAttribute("data-viewport-camera-json");

      const scenario = cases.get(id);
      if (scenario.palette) {
        stage = "insert palette";
        result.palette = await assertPalette(scope, scenario.palette);
      }
      if (process.env.PLAY_TEST_VIEWER_TRANSFER) result.controls = await assertModelControls(page, scope, model, modelSelector, result.cameraAfterZoom);
      if (scenario.diagrams) {
        stage = "diagram pixels";
        const diagram = scope.locator(`[data-ui-node-key^="norm.${id}.play.diagrams."] .semio-canvas-2d-host`).first();
        await reveal(scope, "Diagrams", diagram);
        const canvas = diagram.locator("canvas").first();
        await canvas.waitFor({ state: "visible", timeout });
        const diagramDeadline = Date.now() + timeout;
        const image = join(output, id + "-diagrams.png");
        do {
          result.diagramColors = interiorColors(await canvas.screenshot({ path: image, timeout }));
          if (result.diagramColors > 4) break;
          await page.waitForTimeout(250);
        } while (Date.now() < diagramDeadline);
        assert(result.diagramColors > 4, "the mounted diagram canvas should paint its interior");
        result.diagram = await diagram.getAttribute("data-surface-id");
      }
      if (process.env.PLAY_TEST_VIEWER_TRANSFER) {
        const button = shell.locator('[id="playground.navbar.roles.viewer"]');
        await button.click({ timeout });
        await shell.locator('[data-slot="surface-role-chip"][data-role="viewer"]').waitFor({ state: "visible", timeout });
        const viewer = scope.locator(`[data-ui-node-key="norm.${id}.view.viewport"] .semio-world-3d-host`).first();
        await reveal(scope, "Model", viewer);
        result.viewer = await assertModel(viewer);
        const viewport = scope.locator(`[data-ui-node-key="norm.${id}.view.viewport"]`).first();
        assert.equal(await viewport.locator('[data-ui-node-key="norm-viewport-insert"]').count(), 0);
        trace(`viewer model drawn at ${result.viewer.revision}`);
      }
      assert.deepEqual(result.faults, []);
      result.status = "passed";
    } catch (error) {
      result.status = "failed";
      result.stage = stage;
      result.error = String(error);
      result.surfaces = await page.locator("[data-surface-id]").evaluateAll(rows => rows.map(row => row.getAttribute("data-surface-id"))).catch(() => []);
      result.exampleAtFailure = await page.locator(`[data-layered-pane="${id}"] [id="playground.navbar.fixture"]`).first().textContent({ timeout: 1000 }).catch(() => null);
      result.pageTextAtFailure = (await page.locator("body").innerText({ timeout: 1000 }).catch(() => "")).slice(0, 1500);
      await page.screenshot({ path: join(output, id + "-failure.png"), timeout: 5000 }).catch(() => {});
      process.exitCode = 1;
    } finally {
      result.elapsedMs = Math.round(performance.now() - started);
      console.log(JSON.stringify(result));
      await context.close();
    }
  }
} finally {
  await browser.close();
}
