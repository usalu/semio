import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
const root = "/Users/ueli/Documents/semio";
const barrel = root + "/🧰️framework/📦️packages/🟦️typescript/🟦️.ts";
const importer = root + "/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🚀️plugin-lifecycle/🟦️.ts";
const specs = process.argv.slice(2);
const server = await createServer({ root, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [root] }, watch: null }, resolve: { alias: [{ find: /^@semio-tech\/framework$/u, replacement: barrel }] }, optimizeDeps: { noDiscovery: true, include: [] } });
let bad = 0;
for (const s of specs) {
  const r = await server.pluginContainer.resolveId(s, importer, { ssr: false });
  if (!r?.id) bad++;
  console.log(String(r?.id ?? "UNRESOLVED").replace(root + "/", "").slice(0, 150).padEnd(120), "<-", s);
}
console.log("unresolved:", bad, "of", specs.length);
await server.close();
