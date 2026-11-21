import assert from "node:assert/strict";
import { createServer } from "vite";
import tailwind from "@tailwindcss/vite";
import { chromium } from "playwright";
import { dirname, resolve } from "node:path";
import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";

const checkout = process.cwd();
const generated = resolve(import.meta.dirname, "🗑️generated/channel-text-browser");
const renderer = resolve(checkout, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const fixtureFile = resolve(import.meta.dirname, "🗑️generated/channel-text/snapshots.json");
const textFixture = JSON.parse(await readFile(resolve(checkout, "✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📊️channel-values/🧪️tests/🔣️.json"), "utf8"));
const keyFixture = JSON.parse(await readFile(resolve(checkout, "✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📊️channel-values/🧪️tests/📛️keys.json"), "utf8"));
await mkdir(generated, { recursive: true });
await Promise.all([
  copyFile(resolve(import.meta.dirname, "channel-text-browser-mount.tsx"), resolve(generated, "mount.tsx")),
  writeFile(resolve(generated, "index.html"), '<!doctype html><html><head><meta charset="utf-8"></head><body><div id="root"></div><script type="module" src="/mount.tsx"></script></body></html>'),
]);
const server = await createServer({
  configFile: false,
  root: generated,
  cacheDir: resolve(generated, "vite-cache"),
  esbuild: { jsx: "automatic" },
  plugins: [tailwind()],
  server: { host: "127.0.0.1", fs: { allow: [checkout, dirname(fixtureFile)] } },
  optimizeDeps: { entries: [resolve(generated, "index.html")], include: ["react", "react-dom", "react-dom/client", "react/jsx-runtime", "react/jsx-dev-runtime", "scheduler"] },
  resolve: { alias: {
    "probe:interpreter": resolve(renderer, "🧱️elements/🗣️Interpreter/🟦️.tsx"),
    "probe:document-store": resolve(renderer, "🧱️elements/📃️UiDocumentStore/🟦️.tsx"),
    "probe:action-bridge": resolve(renderer, "🧱️elements/🛠️ShellHelpers/🌳️tree-windows/🟦️.tsx"),
    "probe:window-chrome": resolve(checkout, "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🧱️window-chrome/🟦️.tsx"),
    "probe:styles": resolve(renderer, "🎨️.css"),
    "probe:fixture": fixtureFile,
    "probe:scene-hosts": resolve(renderer, "🧱️elements/🗣️Interpreter/🗺️element-host-registry/🟦️.ts"),
    "probe:wasm-bindings": resolve(renderer, "🧱️elements/🪪️WasmSessionLoader/🟦️.tsx"),
    "probe:text-editor": resolve(renderer, "🧱️elements/✏️TextEditor/🟦️.tsx"),
    "probe:tree-context": resolve(renderer, "🧱️elements/🗣️Interpreter/🪟️tree-windows/🟦️.ts"),
  } },
});
let browser;
try {
  await server.listen();
  browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 720, height: 650 } });
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  const open = async (label) => {
    const button = page.getByText(label, { exact: true });
    try { await button.waitFor({ timeout: 15000 }); } catch (error) {
      throw new Error(JSON.stringify({ label, errors, state: await page.evaluate(() => window.channelState), text: await page.locator("body").innerText() }), { cause: error });
    }
    await button.click();
  };
  const until = async (label, condition, value) => {
    try { await page.waitForFunction(condition, value, { timeout: 7000 }); } catch (error) {
      const snapshot = await page.evaluate(() => ({ state: window.channelState, intents: window.channelIntents, packet: window.channelEditorPacket }));
      throw new Error(JSON.stringify({ label, errors, snapshot }), { cause: error });
    }
  };
  const goto = (query = "") => page.goto(server.resolvedUrls.local[0] + query, { waitUntil: "domcontentloaded" });
  const textControl = page.locator('[data-ui-surface-shell] textarea[aria-readonly="true"]');
  const awaitText = async () => {
    try { await textControl.waitFor({ state: "attached", timeout: 15000 }); } catch (error) {
      throw new Error(JSON.stringify({ errors, state: await page.evaluate(() => window.channelState), surface: await page.locator("[data-ui-surface-shell]").evaluateAll(nodes => nodes.map(node => node.outerHTML.slice(0, 1500))) }), { cause: error });
    }
  };
  await goto();
  assert.equal(await textControl.count(), 0);
  for (const label of ["source@out", "Ausgang", "value", "3"]) await open(label);
  await until("long page open", () => window.channelState[0] === 4);
  await awaitText();
  assert.equal(await textControl.inputValue(), textFixture.cases[3].pages[2].replaceAll("\r\n", "\n"));
  await until("long packet", (expected) => window.channelEditorPacket?.settings?.readOnly === true && window.channelEditorPacket?.buffer === expected, textFixture.cases[3].pages[2]);
  const hover = page.getByText("3", { exact: true });
  await hover.hover();
  await until("graph hover", () => window.channelIntents.some(a => a.action === "interactionHover" && a.args.targetId === "source@out" && a.args.granularityId === "handle"));
  await open("3");
  await until("long page closed", () => window.channelState[0] === 3);
  await textControl.waitFor({ state: "detached" });
  await goto();
  for (const label of ["sink@in", "Eingang"]) await open(label);
  await until("short input open", () => window.channelState[1] === 2);
  await awaitText();
  assert.equal(await textControl.inputValue(), textFixture.cases[1].text.replaceAll("\r\n", "\n"));
  await until("short packet", (expected) => window.channelEditorPacket?.settings?.readOnly === true && window.channelEditorPacket?.buffer === expected, textFixture.cases[1].text);
  assert.equal(await textControl.evaluate(node => node.closest("[data-tree-window-length]").getAttribute("data-tree-window-length")), "1");
  await goto("?dictionary");
  const firstKey = keyFixture.cases.find(row => row.name === "long identical prefix A").key;
  const title = JSON.stringify(Array.from(firstKey).slice(0, 80).join("") + "…");
  for (const label of ["source@out", "Ausgang", title, "Schlüssel", "3"]) await open(label);
  await until("dictionary key opened", () => window.channelState[0] === 5);
  await awaitText();
  assert.equal(await textControl.inputValue(), firstKey.slice(-2));
  await until("exact dictionary key page", (expected) => window.channelEditorPacket?.settings?.readOnly === true && window.channelEditorPacket?.buffer === expected, firstKey.slice(-2));
  assert.deepEqual(errors, []);
  console.log("Native projection in Chromium: expanded and retired a requested page, preserved text, verified graph hover, input preview, and the requested full dictionary key");
} finally {
  await browser?.close();
  await server.close();
}
