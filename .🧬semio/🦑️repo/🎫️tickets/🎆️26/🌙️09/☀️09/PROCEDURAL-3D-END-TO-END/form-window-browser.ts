import assert from "node:assert/strict";
import { createServer } from "vite";
import tailwind from "@tailwindcss/vite";
import { chromium } from "playwright";
import { dirname, resolve } from "node:path";
import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";

const checkout = process.cwd();
const generated = resolve(import.meta.dirname, "🗑️generated/form-window-browser");
const renderer = resolve(checkout, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine");
const fixtureFile = resolve(checkout, "✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🧬️generate/🪟️windows/📝️form/🧪️tests/🧫️fixtures/form-panel.json");
const fixture = JSON.parse(await readFile(fixtureFile, "utf8"));
await mkdir(generated, { recursive: true });
await Promise.all([
  copyFile(resolve(import.meta.dirname, "form-window-browser-mount.tsx"), resolve(generated, "mount.tsx")),
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
    "probe:tree-context": resolve(renderer, "🧱️elements/🗣️Interpreter/🪟️tree-windows/🟦️.ts"),
    "probe:pages": resolve(checkout, "✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📝️form/pages.json"),
    "probe:navigation": resolve(checkout, "🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧫️fixtures/form-page-window.json"),
  } },
});
let browser;
try {
  await server.listen();
  browser = await chromium.launch();
  const page = await browser.newPage({ viewport: { width: 500, height: 400 } });
  const errors = [];
  page.on("pageerror", (error) => errors.push(error.message));
  for (const locale of fixture.locales) {
    const de = locale.id === "de";
    await page.goto(server.resolvedUrls.local[0] + "?locale=" + locale.id, { waitUntil: "domcontentloaded" });
    const form = page.getByRole("form", { name: locale.label });
    const next = form.getByRole("button", { name: de ? "Nächste Seite" : "Next page" });
    const prev = form.getByRole("button", { name: de ? "Vorherige Seite" : "Previous page" });
    const status = form.locator('[aria-live="polite"]');
    const viewport = (index) => form.locator('[data-ui-node-key="generate.form.viewport.' + index + '"]');
    await next.waitFor({ timeout: 30000 });
    assert.equal(await prev.isDisabled(), true);
    assert.match(await status.textContent(), /1 (of|von) 2/);
    const dimensions = await viewport(0).evaluate((node) => ({ client: node.clientHeight, height: node.scrollHeight }));
    assert.ok(dimensions.client < fixture.viewport.height && dimensions.height > dimensions.client, JSON.stringify(dimensions));
    await viewport(0).evaluate((node) => { node.scrollTop = 400; });
    assert.ok(await viewport(0).evaluate((node) => node.scrollTop > 0));
    await next.press("Enter");
    await status.filter({ hasText: /2 (of|von) 2/ }).waitFor();
    assert.ok(await next.isDisabled());
    assert.equal(await viewport(1).evaluate((node) => node.scrollTop), 0);
    const size = de ? "Größe " : "Size ";
    assert.equal(await form.getByRole("spinbutton", { name: size + "0", exact: true }).count(), 0);
    const field = form.getByRole("spinbutton", { name: size + "51", exact: true });
    await field.fill("7");
    await field.press("Tab");
    await page.waitForFunction(() => window.formIntents.length === 1);
    const [edit] = await page.evaluate(() => window.formIntents);
    assert.deepEqual(edit.action.args, { generationId: fixture.generation, questionId: "size-51", value: 7 });
    assert.equal(edit.action.action, "updateGenerationValues");
    await prev.press("Enter");
    await status.filter({ hasText: /1 (of|von) 2/ }).waitFor();
    assert.equal(await page.evaluate(() => window.formIntents.length), 1);
    console.log(locale.id + ": both pages, fixed navigation, viewport reset, exact field commit");
  }
  assert.deepEqual(errors, []);
} finally {
  await browser?.close();
  await server.close();
}
