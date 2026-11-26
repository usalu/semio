import assert from "node:assert/strict";
import { existsSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import { createServer, type Plugin, type UserConfig } from "vite";
import { bimShellModuleMiddleware, rawGuestIsReady } from "./bim-shell-modules.ts";

const workspace = process.cwd();
const scratch = join(import.meta.dirname, "🗑️generated");
const framework = "🧰️framework";
const os = `${framework}/🛍️products/💻️os/🔨️modules`;
const dev = `${os}/🧑‍💻dev`;
const load = (name: string) => import(pathToFileURL(resolve(workspace, name)).href);
const entry = resolve(workspace, "✏️s/🧑‍💻dev/🎭️variants/🏙️bim/🚀️entry/🟦️.ts");
const { bimStdioFormatClaimsVitePlugin } = await load("✏️s/🧑‍💻dev/🎭️variants/🏙️bim/🏗️builder/🌐️vite/🟦️.ts");
const { uiReactBuildPlugin, uiTailwindBuildPlugins, semioHostHtmlVitePlugin, semioEmojiIndexHtmlVitePlugin, playgroundSceneHostResolveAliases, playgroundSceneHostOptimizeDeps, runtimeAssetDeliveries, semioAssetsVitePlugin, packageAssetDeliveries, staticDirMountVitePlugins, frameworkOsEngineDeliveriesV1, managedWasmPackagesVitePluginV1, flowBrowserDeliveryV1, playgroundFlowWasmDevStubPlugin, repositoryCachePartitionRootDataV1 } = await load(`${dev}/🏗️builder/🌐️vite/🏭️build-owners/🟦️.ts`);
const { semioProductionTestBoundaryVitePlugin } = await load(`${dev}/🔌️vite-plugins/🚧️production-test-boundary/🟦️.ts`);
const { semioBackboneVitePlugin } = await load(`${dev}/🔌️vite-plugins/🧱️backbone-vite-plugin/🟦️.ts`);
const { semioBlobVitePlugin } = await load(`${dev}/🔌️vite-plugins/🫧️blob-vite-plugin/🟦️.ts`);
const { repoPlaywrightBrowsersPath } = await load("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🌿️environment/🟦️.ts");
process.env.PLAYWRIGHT_BROWSERS_PATH = repoPlaywrightBrowsersPath(workspace);
const { browserLaunchOptionsV1, launchBrowserV1 } = await load(`${framework}/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🌐️browser/🟦️.ts`);
const { playgroundSessionViteAlias, playgroundSessionsRootV1 } = await load(`${dev}/♻️activation/🟦️.ts`);
const playDir = resolve(workspace, dev);
const productA = [semioProductionTestBoundaryVitePlugin(), ...uiReactBuildPlugin(), ...uiTailwindBuildPlugins()] as Plugin[];
const products = repositoryCachePartitionRootDataV1(workspace, "products");
const engines = frameworkOsEngineDeliveriesV1(products, { version: 1, mode: "prototype", facet: "renderer" });
const actualEngines = engines.filter((item: { root: string; files: readonly string[] }) => item.files.every(file => existsSync(join(item.root, file))));
type EngineDelivery = { packageName: string; entrypoint: "source" | "generated"; files: readonly string[] };
const absentEngines: EngineDelivery[] = engines.filter((item: unknown) => !actualEngines.includes(item));
const missingEngine: Plugin = {
  name: "ticket-unavailable-optional-engine",
  enforce: "pre",
  resolveId(source) {
    const missing = absentEngines.find(({ packageName, entrypoint, files }) =>
      (entrypoint === "generated" && source === packageName) ||
      (source.startsWith(packageName + "/") && files.includes(source.slice(packageName.length + 1))));
    return missing ? "\0ticket-absent-engine:" + missing.packageName : undefined;
  },
  load(id) {
    const mark = "\0ticket-absent-engine:";
    if (id.startsWith(mark)) return "throw new Error(" + JSON.stringify("Optional engine was actually imported without a publication: " + id.slice(mark.length)) + "); export {};";
  },
};
const modules: Plugin = { name: "ticket-bim-diagnostic-module-delivery", configureServer(server) { server.middlewares.use(bimShellModuleMiddleware); } };
const alias = (name: string, path: string) => ({ find: new RegExp("^" + name + "$", "u"), replacement: resolve(workspace, path) });
const settings: UserConfig = {
  configFile: false,
  root: playDir,
  cacheDir: join(scratch, "bim-react-vite"),
  publicDir: false,
  assetsInclude: ["**/*.wasm"],
  mode: "prototype",
  server: { host: "127.0.0.1", port: 16302, strictPort: true, hmr: false, watch: null, fs: { allow: [workspace] } },
  worker: { format: "es", plugins: () => [semioProductionTestBoundaryVitePlugin()] },
  plugins: [
    modules, missingEngine, bimStdioFormatClaimsVitePlugin(),
    managedWasmPackagesVitePluginV1(actualEngines.map(({ packageName, root, entry, files, entrypoint }: (typeof actualEngines)[number]) => ({ packageName, root, entry, files, entrypoint }))),
    playgroundFlowWasmDevStubPlugin(workspace, flowBrowserDeliveryV1(workspace, products, { version: 1, mode: "prototype", command: "serve" })),
    ...semioHostHtmlVitePlugin(workspace, { title: "semio · os", entry: "/@fs/" + entry.replaceAll("\\", "/") }),
    semioEmojiIndexHtmlVitePlugin(playDir),
    semioBackboneVitePlugin(), semioBlobVitePlugin(), ...semioAssetsVitePlugin(workspace),
    ...staticDirMountVitePlugins(workspace, [...runtimeAssetDeliveries(), ...packageAssetDeliveries()]),
    ...productA,
  ],
  resolve: {
    alias: [
      playgroundSessionViteAlias(playgroundSessionsRootV1(workspace, "prototype"), "bim"),
      ...playgroundSceneHostResolveAliases(workspace),
      alias("@semio-tech/framework", `${framework}/📦️packages/🟦️typescript/🟦️.ts`),
      alias("@semio-tech/framework-os", `${os}/../📦️packages/🟦️typescript/🟦️.ts`),
      alias("@semio-tech/framework-renderer-react", `${os}/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`),
      alias("@semio-tech/ui-react", `${framework}/🔨️modules/🖱️ui/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx`),
    ],
    dedupe: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
  },
  optimizeDeps: { entries: [entry], ...playgroundSceneHostOptimizeDeps({ exclude: absentEngines.map(item => item.packageName) }) },
  define: {
    "import.meta.vitest": "undefined",
    "import.meta.env.VITE_SEMIO_PLUGIN": '"bim"',
    "import.meta.env.VITE_SEMIO_RENDERER": '"react"',
    "import.meta.env.VITE_S_HUB_URL": '""',
    "import.meta.env.VITE_SEMIO_LOCKED_LOCALE": '"en"',
  },
};

const server = await createServer(settings);
try {
  await server.listen();
  const home = "http://127.0.0.1:16302";
  const initial = await fetch(home);
  assert.equal(initial.status, 200);
  assert.match(await initial.text(), /\/@@?fs\//u);
  const entryResponse = await fetch(new URL("/@fs/" + entry, home));
  assert.equal(entryResponse.status, 200, (await entryResponse.clone().text()).slice(0, 300));
  const formats = await fetch(new URL("/@fs/" + resolve(workspace, "✏️s/🧑‍💻dev/🚀️entry/🧭️format-catalog/🟦️.ts"), home));
  assert.equal(formats.status, 200);
  const formatSource = await formats.text();
  const formatModule = formatSource.match(/"(\/@id\/__x00__bim-stdio-format-claims\.json[^"]*)"/u)?.[1];
  assert.ok(formatModule);
  const formatData = await fetch(new URL(formatModule, home));
  assert.equal(formatData.status, 200);
  const claims = await formatData.text();
  assert.match(claims, /s\.stdio\.ifc@4\/\*/u);
  const hostPorts = await fetch(new URL("/@fs/" + resolve(workspace, dev, "🪝️host-ports/🕸️wasm-bindings/🟦️.ts"), home));
  assert.equal(hostPorts.status, 200);
  await hostPorts.body?.cancel();
  const shard = await fetch(new URL("/🔌️plugin-modules/🧵️shard/🟨️shard-worker.js", home));
  assert.equal(shard.status, 200);
  await shard.body?.cancel();
  console.log("[DEBUG] Real React entry, runtime helpers and publisher-owned shard are served at", home);
  if (!process.argv.includes("--preflight")) {
    const cutoff = Date.now() + 3600000;
    while (!rawGuestIsReady()) {
      if (Date.now() > cutoff) throw Error("Raw BIM browser materialization or actual descriptor was unavailable");
      await Bun.sleep(3000);
    }
    const browser = await launchBrowserV1(browserLaunchOptionsV1());
    try {
      const page = await browser.newPage({ viewport: { width: 1440, height: 960 }, acceptDownloads: true });
      page.on("pageerror", error => console.error("[DEBUG] React page fault", error.message));
      page.on("console", message => { if (message.type() === "error") console.error("[DEBUG] React console", message.text()); });
      page.on("response", response => { if (response.status() >= 400) console.error("[DEBUG] React HTTP", response.status(), response.url()); });
      const result = await page.goto(home + "/?example=house", { waitUntil: "domcontentloaded", timeout: 120000 });
      assert.equal(result?.status(), 200);
      const app = page.locator('[data-slot="surface-role-chip"][data-app-id="s.bim.model@1/*#editor"]');
      await app.waitFor({ timeout: 180000 });
      await page.locator("canvas").first().waitFor({ timeout: 120000 });
      const label = await app.getAttribute("data-role");
      assert.equal(label, "editor");
      const report = await page.evaluate(() => ({ title: document.title, canvases: document.querySelectorAll("canvas").length, text: document.body.innerText.slice(0, 1200) }));
      assert.ok(report.canvases > 0);
      console.log("[DEBUG] Real BIM house mounted in the React editor", JSON.stringify(report));
      await page.screenshot({ path: join(scratch, "bim-react-house.png") });
      const ifc4 = page.getByRole("checkbox", { name: "IFC4 (STEP)", exact: true }).first();
      if (!(await ifc4.isVisible())) {
        const exports = page.getByRole("button", { name: "Export", exact: true }).first();
        if (!(await exports.isVisible())) {
          const measures = page.getByRole("button", { name: /(?:window )?measures/iu }).first();
          if (await measures.isVisible()) await measures.click();
        }
        await exports.click({ timeout: 20000 });
      }
      await ifc4.waitFor({ timeout: 30000 });
      const downloaded = page.waitForEvent("download", { timeout: 150000 });
      await ifc4.click();
      const download = await downloaded;
      assert.equal(download.suggestedFilename(), "Family-House.ifc");
      const output = join(scratch, "bim-react-house.ifc");
      await download.saveAs(output);
      const step = await readFile(output, "utf8");
      assert.match(step, /^ISO-10303-21;/u);
      assert.match(step, /FILE_SCHEMA\s*\(\(\s*'IFC4'\s*\)\)/u);
      const walls = [...step.matchAll(/=\s*IFCWALL(?:STANDARDCASE)?\s*\(/gu)].length;
      const slabs = [...step.matchAll(/=\s*IFCSLAB\s*\(/gu)].length;
      assert.equal(walls, 30);
      assert.equal(slabs, 15);
      assert.equal([...step.matchAll(/=\s*IFCSLAB\([^;]+\.FLOOR\.\s*\);/gu)].length, 4);
      console.log("[DEBUG] Real IFC4 download", JSON.stringify({ filename: download.suggestedFilename(), bytes: Buffer.byteLength(step), walls, slabs }));
    } finally {
      await browser.close();
    }
  }
} finally {
  await server.close();
}
