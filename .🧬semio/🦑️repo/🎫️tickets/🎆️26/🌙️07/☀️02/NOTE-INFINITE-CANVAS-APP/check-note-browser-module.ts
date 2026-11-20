import assert from "node:assert/strict";
import { join } from "node:path";
import { createServer } from "vite";

const workspace = process.cwd();
const outputRoot = join(import.meta.dir, "🗑️generated", "note-catalog");
const specificCatalog = join(workspace, "✏️s/🧑‍💻dev/🧩️catalog");
const variant = "✏️s/🧑‍💻dev/🎭️variants/🗒️note";
const frameworkDev = "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev";
process.env.VITE_SEMIO_PLUGIN = "note";
const vite = await createServer({
  root: workspace,
  configFile: false,
  cacheDir: join(import.meta.dir, "🗑️generated", "note-vite"),
  optimizeDeps: { noDiscovery: true, include: [] },
  server: { middlewareMode: true, watch: null },
  plugins: [{
    name: "isolated-note-catalog",
    enforce: "pre",
    resolveId(path, importer) {
      if (!importer?.startsWith(specificCatalog) || !/^\.\.?\/🤖️generated\/.*\.ts$/u.test(path)) return;
      const entry = path.split("/").slice(-2).join("/");
      return join(outputRoot, entry);
    },
  }],
});
try {
  const entry = await vite.transformRequest(`/${variant}/🚀️entry/🟦️.ts`);
  assert.ok(entry?.code.includes("bootFrameworkOsDev"));
  assert.ok(entry.code.includes('VITE_SEMIO_PLUGIN": "note') || entry.code.includes('VITE_SEMIO_PLUGIN":"note'));
  const catalog = await vite.transformRequest("/✏️s/🧑‍💻dev/🧩️catalog/🟦️.ts");
  assert.ok(catalog?.code.includes("composeSpecificOsCatalogV1"));
  const host = await vite.transformRequest(`/${frameworkDev}/🟦️.ts`);
  assert.ok(host?.code.includes("resolvePlaygroundBoot"));
  console.log("[DEBUG] Vite transformed authentic Note entry, Specific catalog and framework boot");
} finally {
  await vite.close();
}
