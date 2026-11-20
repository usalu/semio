import { strict as assert } from "node:assert";
import { mkdir, readFile } from "node:fs/promises";
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const ticket = fileURLToPath(new URL(".", import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const output = join(ticket, "🗑️generated", "eurocode-retained-scenes");
const require = createRequire(join(root, "🏢️semio-tech", "🎡️play", "package.json"));
const { chromium } = require("playwright");
const { PNG } = require("pngjs");
const lines = (await readFile(process.argv[2], "utf8")).split("\n");
const messages = tag => lines.filter(line => line.startsWith(`[DEBUG] ${tag} `)).map(line => JSON.parse(line.slice(tag.length + 9)));
const retained = messages("retained");
const galleries = messages("gallery");
const explanation = messages("explanation")[0];
const viewport = {
  initial: messages("viewport-initial")[0], next: messages("viewport-next")[0], nextHelp: messages("viewport-next-help")[0],
  nextInsertTail: messages("viewport-next-insert-tail")[0],
  help: messages("viewport-help")[0], helpTail: messages("viewport-help-tail")[0],
  viewer: messages("viewport-viewer")[0], viewerNext: messages("viewport-viewer-next")[0],
};
const expected = process.argv.slice(3);
assert.deepEqual(retained.map(scene => scene.key).sort(), expected.sort());
assert(retained.length > 0 || viewport.initial || explanation);
await mkdir(output, { recursive: true });
const elements = join(root, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📺️renderer", "🧑‍🎨engine", "🧱️elements");
const sourceUrl = path => "/@fs" + pathToFileURL(path).pathname;
const source = {
  react: sourceUrl(join(elements, "⚛️react-port", "🟦️.ts")),
  store: sourceUrl(join(elements, "📃️UiDocumentStore", "🟦️.tsx")),
  interpreter: sourceUrl(join(elements, "🗣️Interpreter", "🟦️.tsx")),
  windows: sourceUrl(join(elements, "🗣️Interpreter", "🪟️tree-windows", "🟦️.ts")),
  hosts: sourceUrl(join(elements, "🪢️ElementHosts", "🟦️.ts")),
  transfer: sourceUrl(join(elements, "🏛️ShellHost", "🔀️surface-switch", "📄️document", "🟦️.ts")),
  css: sourceUrl(join(root, "🏢️semio-tech", "🎡️play", "🎨️globals.css")),
};

const browser = await chromium.launch({ headless: process.env.PLAY_HEADED !== "1", executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE, args: ["--in-process-gpu", "--enable-unsafe-swiftshader"] });
const page = await browser.newPage({ locale: "en-US", viewport: { width: 960, height: 660 } });
if (process.env.PLAY_REFRESH_GALLERY_MODULES) {
  await page.route(url => /\/(?:🧱️node-tree|🪢️views)\/🟦️\.tsx$/u.test(decodeURIComponent(url.pathname)) && !url.searchParams.has("gallery"),
    route => {
      const url = new URL(route.request().url());
      url.searchParams.set("gallery", process.env.PLAY_REFRESH_GALLERY_MODULES);
      return route.continue({ url: url.href });
    });
}
if (explanation && process.env.PLAY_TEST_TEXT_DISCLOSURE_SOURCE) {
  const nodeTree = join(elements, "🗣️Interpreter", "🧱️node-tree", "🟦️.tsx");
  const current = await readFile(nodeTree, "utf8");
  const replacement = current.match(/    const contained = child\.component[^]*?    if \(content\) contentRecords\.push\(child\);/)?.[0];
  assert(replacement);
  await page.route(url => decodeURIComponent(url.pathname).endsWith("/🧱️node-tree/🟦️.tsx"), async route => {
    const response = await route.fetch();
    const source = await response.text();
    const prior = /    const content = child\.component\.type === "surface"[^]*?    if \(content\) contentRecords\.push\(child\);/;
    assert(source.includes("const prose = ") || prior.test(source));
    await route.fulfill({response, body: source.includes("const prose = ") ? source : source.replace(prior, replacement)});
  });
}
const failures = [];
page.on("pageerror", error => failures.push(error.message));
page.on("console", message => {
  if (message.type() === "error" && /\[Interpreter\]|\[ShellFaultBoundary\]|canvas session/.test(message.text())) failures.push(message.text());
});
const url = new URL("/__eurocode_retained_scene_probe", process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6034/");
await page.route(url.href, route => route.fulfill({
  contentType: "text/html",
  body: `<!doctype html><html lang="en"><body><div id="scene" style="position:relative;width:900px;height:600px"></div>
<script type="module">import {injectIntoGlobalHook} from "/@react-refresh"; injectIntoGlobalHook(window);
window.$RefreshReg$=()=>{};window.$RefreshSig$=()=>type=>type;window.__vite_plugin_react_preamble_installed__=true;</script>
</body></html>`,
}));

try {
  await page.goto(url.href);
  await page.evaluate(async source => {
    const [{ createElement, createRoot }, { UiDocumentStore }, { InterpretedUiNode }, { TreeWindowContext }, { registerElementHostsV1 }, { prepareDocumentSurfaceV1 }] = await Promise.all([
      import(source.react), import(source.store), import(source.interpreter), import(source.windows), import(source.hosts), import(source.transfer), import(source.css),
    ]);
    registerElementHostsV1();
    const renderer = createRoot(document.getElementById("scene"));
    const onAction = () => {};
    const stores = new Map();
    const revisions = new Map();
    window.viewportIntents = [];
    window.viewportOpenRequests = [];
    window.mountRetainedScene = (authored, body, openStates = {}) => {
      const nodes = [];
      function recordOf(node) {
        const id = nodes.length;
        const record = { ...node, id, children: [] };
        nodes.push(record);
        record.children = (node.children ?? []).map(recordOf);
        return id;
      }
      recordOf(authored);
      const surface = body ?? authored.key;
      const store = stores.get(surface) ?? new UiDocumentStore(surface);
      const revision = (revisions.get(surface) ?? -1) + 1;
      stores.set(surface, store);
      revisions.set(surface, revision);
      store.loadSnapshot({ surface, revision, root: 0, nodes, layoutEpoch: 0n });
      const onIntent = intent => window.viewportIntents.push({ nodeKey: intent.nodeKey, args: intent.args, action: intent.action, surface: intent.surface });
      const tree = createElement(InterpretedUiNode, { key: surface, store, onAction, onIntent });
      if (body) {
        window.reportedDiagramWindows = [];
        const value = { bodyKey: body, openStates, setOpen: (key, open) => window.viewportOpenRequests.push({key, open}), reportWindows: requests => window.reportedDiagramWindows.push(...requests) };
        renderer.render(createElement("div", { style: { height: "100%", overflowY: "auto" }, id: "diagram-scroll" },
          createElement(TreeWindowContext.Provider, { value }, tree)));
      } else {
        renderer.render(tree);
      }
    };
    window.transferViewportViewer = async ({ editorBody, viewerBody, viewer }) => {
      const receipt = [];
      const successor = await prepareDocumentSurfaceV1({
        current: () => stores.has(editorBody),
        capture: async () => { receipt.push("capture editor document"); return editorBody; },
        create: async () => { receipt.push("create viewer"); return viewerBody; },
        restore: async () => { receipt.push("restore document"); },
        attach: async () => { receipt.push("attach viewer"); },
        release: async () => { receipt.push("release unpublished viewer"); },
      });
      window.mountRetainedScene(viewer, successor);
      return receipt;
    };
  }, source);
  for (const authored of retained) {
    await page.evaluate(scene => window.mountRetainedScene(scene), authored);
    const host = page.locator(`[data-ui-node-key="${authored.key}"]`);
    const model = authored.key.endsWith(".model");
    const canvas = host.locator(model ? "canvas[data-world-geometry-drawn]" : "canvas").first();
    await canvas.waitFor({ state: "visible", timeout: 70_000 });
    const result = { scene: authored.key, fixed: authored.component.doc.bytes.length };
    if (model) {
      Object.assign(result, await canvas.evaluate(element => {
        const host = element.closest(".semio-world-3d-host");
        return {
          meshes: Object.keys(JSON.parse(host.dataset.meshesJson)).length,
          instances: Object.keys(JSON.parse(host.dataset.instancesJson)).length,
          vertices: Number(element.dataset.worldGeometryVertices),
          current: element.dataset.worldGeometryRevision === host.dataset.worldGeometryRevision,
        };
      }));
      assert(result.meshes > 0 && result.instances > 0 && result.vertices > 0 && result.current, JSON.stringify(result));
    }
    const deadline = Date.now() + 35_000;
    do {
      const image = PNG.sync.read(await canvas.screenshot({ path: join(output, authored.key + ".png") }));
      const colors = new Set();
      for (let y = Math.floor(image.height * 0.2); y < image.height * 0.8; y += 2) {
        for (let x = Math.floor(image.width * 0.2); x < image.width * 0.8; x += 2) {
          colors.add(image.data.readUInt32BE((y * image.width + x) * 4));
        }
      }
      result.interiorColors = colors.size;
      if (result.interiorColors > 4) break;
      await page.waitForTimeout(300);
    } while (Date.now() < deadline);
    assert(result.interiorColors > 4, JSON.stringify(result));
    if (model) {
      const surface = host.locator(".semio-world-3d-host");
      const before = await surface.getAttribute("data-viewport-camera-json");
      assert(before);
      await surface.hover();
      await page.mouse.wheel(0, -120);
      await page.waitForFunction(({ key, before }) => {
        const camera = document.querySelector(`[data-ui-node-key="${key}"] .semio-world-3d-host`)?.getAttribute("data-viewport-camera-json");
        return camera && camera !== before;
      }, { key: authored.key, before }, { timeout: 8000 });
      result.zoomed = true;
    }
    assert.deepEqual(failures, []);
    console.log(JSON.stringify(result));
  }
  if (explanation) {
    const body = "framework.panel.inspection";
    await page.evaluate(({panel, body}) => window.mountRetainedScene(panel, body), {panel: explanation.panel, body});
    const trigger = page.getByText("Explanation", {exact: true});
    await trigger.waitFor({state: "visible"});
    const disclosure = trigger.locator("xpath=ancestor::*[@aria-expanded][1]");
    assert.equal(await disclosure.getAttribute("aria-expanded"), "true");
    const chunks = page.locator('[data-slot="tree-item-content"] [data-ui-node-key^="norm-text-chunk-"]');
    assert((await chunks.count()) > 1);
    assert.equal(await chunks.evaluateAll(nodes => nodes.map(node => node.textContent).join("")), explanation.text);
    const paragraph = await chunks.first().boundingBox();
    const bodyBox = await page.locator("#diagram-scroll").boundingBox();
    assert(paragraph.width > bodyBox.width * 0.7, JSON.stringify(paragraph));
    const key = "norm-inspection.check.explanation";
    const mountDisclosure = open => page.evaluate(({panel, body, key, open}) => window.mountRetainedScene(panel, body, {[key]: open}), {panel: explanation.panel, body, key, open});
    await trigger.click();
    await page.waitForFunction(key => window.viewportOpenRequests.some(event => event.key === key && !event.open), key);
    await mountDisclosure(false);
    await page.waitForFunction(() => document.querySelector('p[data-ui-node-key^="norm-text-chunk-"]') === null);
    assert.equal(await disclosure.getAttribute("aria-expanded"), "false");
    await disclosure.press("Enter");
    await page.waitForFunction(key => window.viewportOpenRequests.some(event => event.key === key && event.open), key);
    await mountDisclosure(true);
    await chunks.first().waitFor({state: "visible"});
    assert.equal(await chunks.evaluateAll(nodes => nodes.map(node => node.textContent).join("")), explanation.text);
    console.log(JSON.stringify({explanationBytes: Buffer.byteLength(explanation.text), chunks: await chunks.count(), width: paragraph.width, keyboardDisclosure: true}));
  }
  if (viewport.initial) {
    assert(viewport.next && viewport.help && viewport.helpTail);
    const body = viewport.initial.children.at(-1).key;
    const mount = (root, openStates, key = body) => page.evaluate(({root, key, openStates}) => window.mountRetainedScene(root, key, openStates), {root, key, openStates});
    const host = page.locator(`[data-ui-node-key="${body}"] .semio-world-3d-host`);
    const draw = host.locator("canvas[data-world-geometry-drawn]");
    const shape = () => host.evaluate(element => ({
      meshes: Object.keys(JSON.parse(element.dataset.meshesJson)),
      instances: JSON.parse(element.dataset.instancesJson),
      camera: element.dataset.viewportCameraJson,
    }));
    const helpKey = "norm-viewport-help.list";
    const help = page.locator(`[data-tree-window-key="${helpKey}"]`);
    await mount(viewport.initial);
    await draw.waitFor({state: "attached", timeout: 30000});
    const helpTrigger = page.getByText("Keyboard shortcuts", {exact: true});
    assert.equal(await helpTrigger.evaluate(label => label.closest('[aria-expanded]')?.getAttribute('aria-expanded')), "false");
    const first = await shape();
    assert(first.meshes.length && first.instances.length);
    if (viewport.viewer) {
      await host.hover();
      await page.mouse.wheel(0, -125);
      await page.waitForFunction(before => {
        const camera = document.querySelector(".semio-world-3d-host")?.getAttribute("data-viewport-camera-json");
        return camera && camera !== before;
      }, first.camera);
      first.zoomed = (await shape()).camera;
    }
    await page.getByText("Next model page", {exact: true}).click();
    const intent = await page.waitForFunction(() => window.viewportIntents.find(i => i.nodeKey === "norm-viewport-scene-page.next"));
    assert.deepEqual((await intent.jsonValue()).args, {action: "viewportSetScenePage", number: 1});
    await mount(viewport.next);
    await page.getByText(/^Model page 2 of /).waitFor();
    await page.waitForFunction(() => {
      const host = document.querySelector(".semio-world-3d-host");
      const canvas = host?.querySelector("canvas[data-world-geometry-drawn]");
      return canvas?.getAttribute("data-world-geometry-revision") === host?.getAttribute("data-world-geometry-revision");
    });
    const second = await shape();
    assert(second.meshes.length && second.instances.length);
    assert.notDeepEqual(first.instances, second.instances);
    assert.equal(first.zoomed ?? first.camera, second.camera);
    if (viewport.nextInsertTail) {
      const insertKey = "norm-viewport-insert";
      const insert = page.locator(`[data-tree-window-key="${insertKey}"]`);
      const last = Number(await insert.getAttribute("data-tree-window-total")) - 1;
      await insert.locator('[data-tree-window-spacer="trailing"]').last().evaluate(element => element.scrollIntoView({block: "end"}));
      await page.waitForFunction(({insertKey, last}) => window.reportedDiagramWindows.some(request => request.nodeKey === insertKey && request.offset <= last && request.offset + request.rows > last), {insertKey, last});
      await mount(viewport.nextInsertTail);
      await page.waitForFunction(({insertKey, last}) => document.querySelector(`[data-tree-window-key="${insertKey}"]`)?.getAttribute("data-tree-window-offset") === String(last), {insertKey, last});
      const toolbarScrolled = await shape();
      assert.deepEqual(toolbarScrolled.instances, second.instances);
      assert.equal(toolbarScrolled.camera, second.camera);
    }
    await helpTrigger.click();
    await page.waitForFunction(key => window.viewportOpenRequests.some(r => r.key === key && r.open), helpKey);
    await mount(viewport.nextHelp ?? viewport.help, {[helpKey]: true});
    await page.getByText(/ · Everywhere$/).first().waitFor();
    if (viewport.nextHelp) {
      await page.getByText(/^Model page 2 of /).waitFor();
      const expanded = await shape();
      assert.deepEqual(expanded.instances, second.instances);
      assert.equal(expanded.camera, second.camera);
    }
    assert.equal(await help.getAttribute("data-tree-window-length"), "16");
    const last = Number(await help.getAttribute("data-tree-window-total")) - 1;
    await help.locator('[data-tree-window-spacer="trailing"]').last().evaluate(element => element.scrollIntoView({block: "end"}));
    await page.waitForFunction(({helpKey, last}) => window.reportedDiagramWindows.some(request => request.nodeKey === helpKey && request.offset <= last && request.offset + request.rows > last), {helpKey, last});
    await mount(viewport.helpTail, {[helpKey]: true});
    await page.waitForFunction(({helpKey, last}) => document.querySelector(`[data-tree-window-key="${helpKey}"]`)?.getAttribute("data-tree-window-offset") === String(last), {helpKey, last});
    if (viewport.viewer) {
      await page.getByText(/^Model page 2 of /).waitFor();
      const afterScroll = await shape();
      assert.deepEqual(afterScroll.instances, second.instances);
      assert.equal(afterScroll.camera, second.camera);
      assert(viewport.viewerNext);
      const viewerBody = viewport.viewer.children.at(-1).key;
      const receipt = await page.evaluate(({ editorBody, viewerBody, viewer }) => window.transferViewportViewer({ editorBody, viewerBody, viewer }), { editorBody: body, viewerBody, viewer: viewport.viewer });
      assert.deepEqual(receipt, ["capture editor document", "create viewer", "restore document", "attach viewer"]);
      const readonly = page.locator(`[data-ui-node-key="${viewerBody}"] .semio-world-3d-host`);
      await readonly.locator("canvas[data-world-geometry-drawn]").waitFor({state: "attached"});
      assert.equal(await page.getByText(/^Model page 1 of /).count(), 1);
      assert.equal(await page.getByText("Keyboard shortcuts", {exact: true}).evaluate(label => label.closest('[aria-expanded]')?.getAttribute('aria-expanded')), "false");
      assert.equal(await page.getByText("Insert", {exact: true}).count(), 0);
      assert.equal(await readonly.getAttribute("data-viewport-camera-json"), first.camera);
      await page.getByText("Next model page", {exact: true}).click();
      const viewerIntent = await page.waitForFunction(key => window.viewportIntents.find(i => i.nodeKey === "norm-viewport-scene-page.next" && i.surface === key), viewerBody);
      const action = await viewerIntent.jsonValue();
      assert.deepEqual(action.args, {action: "viewportSetScenePage", number: 1});
      assert.match(JSON.stringify(action.action), /#viewer/);
      await mount(viewport.viewerNext, undefined, viewerBody);
      await page.getByText(/^Model page 2 of /).waitFor();
      console.log(JSON.stringify({ viewer: viewerBody, archivePreparation: receipt, readonlyPager: true, editorZoomReset: true, closedHelp: true }));
    }
    assert.deepEqual(failures, []);
    console.log(JSON.stringify({viewport: body, firstInstances: first.instances.length, nextInstances: second.instances.length, sameCamera: true, pageAction: true, insertWindowKeptPage: Boolean(viewport.nextInsertTail), shortcuts: last + 1, lastShortcut: true}));
  }
  for (const { body, head, tail } of galleries) {
    const [first] = head.children;
    const [last] = tail.children;
    const section = page.locator(`[data-tree-window-key="${first.key}"]`);
    const canvas = section.locator(".semio-canvas-2d-host canvas").first();
    await page.evaluate(({ head, body }) => window.mountRetainedScene(head, body), { head, body });
    await canvas.waitFor({ state: "visible", timeout: 30_000 });
    assert.equal(await section.getAttribute("data-tree-window-total"), String(first.component.window.total));
    assert.equal(await section.getAttribute("data-tree-window-length"), String(first.children.length));
    const bounds = await canvas.boundingBox();
    const scroller = await page.locator("#diagram-scroll").boundingBox();
    assert(bounds.width / scroller.width > 0.6 && bounds.height > 150, JSON.stringify(bounds));
    const target = first.component.window.total - 1;
    await section.locator(':scope [data-tree-window-spacer="trailing"]').last().evaluate(element => element.scrollIntoView({ block: "end" }));
    await page.waitForFunction(({ key, target }) => window.reportedDiagramWindows.some(window => window.nodeKey === key && window.offset <= target && window.offset + window.rows > target),
      { key: first.key, target }, { timeout: 8000 });
    await page.evaluate(({ tail, body }) => window.mountRetainedScene(tail, body), { tail, body });
    await page.waitForFunction(({ key, target }) => document.querySelector(`[data-tree-window-key="${key}"]`)?.getAttribute("data-tree-window-offset") === String(target),
      { key: last.key, target }, { timeout: 8000 });
    await canvas.scrollIntoViewIfNeeded();
    await canvas.waitFor({ state: "visible", timeout: 10_000 });
    assert.equal(await section.getAttribute("data-tree-window-length"), "1");
    await canvas.screenshot({ path: join(output, body + "-last.png"), timeout: 10_000 });
    console.log(JSON.stringify({ gallery: body, initial: first.children.length, total: first.component.window.total, last: target, canvasWidth: bounds.width, canvasHeight: bounds.height }));
  }
} catch (error) {
  console.log(JSON.stringify({ error: String(error), mounted: await page.locator("#scene").evaluate(element => ({
    text: element.innerText,
    counts: [...element.querySelectorAll(".semio-world-3d-host,.semio-canvas-2d-host,canvas")].map(node => ({
      kind: node.className,
      width: node.clientWidth,
      height: node.clientHeight,
      meshes: node.getAttribute("data-meshes-json")?.length,
      instances: node.getAttribute("data-instances-json")?.length,
    })),
  })).catch(() => undefined) }));
  throw error;
} finally {
  if (failures.length) console.log(JSON.stringify({ failures }));
  await browser.close();
}
