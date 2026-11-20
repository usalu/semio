import assert from "node:assert/strict";
import {readFile} from "node:fs/promises";
import {createRequire} from "node:module";
import {join, resolve} from "node:path";
import {fileURLToPath, pathToFileURL} from "node:url";

const ticket = fileURLToPath(new URL(".", import.meta.url));
const root = resolve(ticket, "../../../../../../..");
const play = join(root, "🏢️semio-tech", "🎡️play");
const require = createRequire(join(play, "package.json"));
const {chromium} = require("playwright");
const {PNG} = require("pngjs");
const marker = "[DEBUG] diagram-window ";
const diagrams = (await readFile(process.argv[2], "utf8")).split("\n")
  .filter(line => line.startsWith(marker)).map(line => JSON.parse(line.slice(marker.length)));
assert.deepEqual(diagrams.map(item => item.variant), ["all-diagrams", "last-diagram"]);

const elements = join(root, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "📺️renderer", "🧑‍🎨engine", "🧱️elements");
const vitePath = path => "/@fs" + pathToFileURL(path).pathname;
const urls = [
  vitePath(join(elements, "⚛️react-port", "🟦️.ts")),
  vitePath(join(elements, "📃️UiDocumentStore", "🟦️.tsx")),
  vitePath(join(elements, "🗣️Interpreter", "🟦️.tsx")),
  vitePath(join(elements, "🪢️ElementHosts", "🟦️.ts")),
  vitePath(join(elements, "🗣️Interpreter", "🪟️tree-windows", "🟦️.ts")),
  vitePath(join(play, "🎨️globals.css")),
];
const browser = await chromium.launch({args: ["--in-process-gpu", "--enable-unsafe-swiftshader"]});
try {
  const page = await browser.newPage({locale: "en-US", viewport: {width: 960, height: 680}});
  const faults = [];
  page.on("pageerror", error => faults.push(error.message));
  const url = new URL("/__eurocode_diagram_window_probe__", process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6033/");
  await page.route(url.href, route => route.fulfill({contentType: "text/html", body: `<!doctype html>
<html lang="en"><body><div id="diagram-probe" style="width:640px;height:600px;overflow:auto"></div>
<script type="module">import {injectIntoGlobalHook} from "/@react-refresh"; injectIntoGlobalHook(window);
window.$RefreshReg$=()=>{};window.$RefreshSig$=()=>type=>type;window.__vite_plugin_react_preamble_installed__=true;</script></body></html>`}));
  await page.goto(url.href);
  await page.evaluate(async sources => {
    const [{createElement, createRoot}, {UiDocumentStore}, {interpretUiNode}, {registerElementHostsV1}, {TreeWindowContext}] =
      await Promise.all(sources.map(source => import(source)));
    registerElementHostsV1();
    const renderer = createRoot(document.getElementById("diagram-probe"));
    window.diagramWindowReports = [];
    const reportWindows = requests => window.diagramWindowReports.push(...requests);
    window.renderDiagramWindow = authored => {
      const nodes = [];
      function insert(node) {
        const id = nodes.length;
        const record = {...node, id, children: []};
        nodes.push(record);
        record.children = node.children.map(insert);
        return id;
      }
      insert(authored);
      const store = new UiDocumentStore(authored.key);
      store.loadSnapshot({surface: authored.key, revision: 0, root: 0, nodes, layoutEpoch: 0n});
      const channel = {bodyKey: authored.key.replace(/\.gallery$/, ""), openStates: {}, setOpen() {}, reportWindows};
      renderer.render(createElement(TreeWindowContext.Provider, {value: channel}, interpretUiNode(store, {store, onAction() {}, onIntent() {}})));
    };
  }, urls);
  const seen = [];
  for (const {variant, root: diagram} of diagrams) {
    const key = diagram.children[0].children[0].children[0].children.find(node => node.component.type === "surface").key;
    await page.evaluate(value => window.renderDiagramWindow(value), diagram);
    const canvas = page.locator(`[data-ui-node-key="${key}"] canvas`).first();
    await canvas.waitFor({state: "visible", timeout: 30_000});
    const rect = await canvas.boundingBox();
    assert(rect && rect.width >= 400 && rect.height >= 100, JSON.stringify(rect));
    const image = PNG.sync.read(await canvas.screenshot());
    const colors = new Set();
    for (let y = 10; y < image.height - 10; y += 2) {
      for (let x = 10; x < image.width - 10; x += 2) colors.add(image.data.readUInt32BE((y * image.width + x) * 4));
    }
    assert(colors.size > 4, `Canvas of ${key} did not paint chart content`);
    assert.deepEqual(faults, []);
    seen.push(key);
    console.log("[DEBUG] diagram window", JSON.stringify({variant, key, width: rect.width, height: rect.height, colors: colors.size}));
    if (variant === "all-diagrams") {
      const section = diagram.children[0];
      await page.waitForFunction(key => window.diagramWindowReports.some(row => row.nodeKey === key), section.key);
      await page.locator("#diagram-probe").evaluate(root => { root.scrollTop = root.scrollHeight; });
      const tail = await page.waitForFunction(({nodeKey, total}) => window.diagramWindowReports.find(
        row => row.nodeKey === nodeKey && row.offset > 0 && row.offset + row.rows >= total,
      ), {nodeKey: section.key, total: section.component.window.total}, {timeout: 5000});
      console.log("[DEBUG] diagram scroll", JSON.stringify(await tail.jsonValue()));
    }
  }
  assert.equal(new Set(seen).size, 2);
} finally {
  await browser.close();
}
