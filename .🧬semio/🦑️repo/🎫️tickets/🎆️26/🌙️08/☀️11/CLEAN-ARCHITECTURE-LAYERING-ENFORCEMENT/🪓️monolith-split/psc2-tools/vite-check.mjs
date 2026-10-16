/** 🔍️ Resolves every planned specifier through a real Vite plugin container using the repo's workspace resolve preset. Usage: bun vite-check.mjs <plan.json> */
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const plan = JSON.parse(readFileSync(process.argv[2], "utf8"));
const importer = root + "/🌎️hub/🧪️tests/🎛️run/🟦️.ts";
const server = await createServer({ root, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [root] }, watch: null }, resolve: { alias: [] }, optimizeDeps: { noDiscovery: true, include: [] } });
let bad = 0, total = 0;
const rows = [...plan.facades.map(f => ({ spec: f.pkg + (f.subpath === "." ? "" : f.subpath.slice(1)), file: f.facade })), ...plan.directExports.map(d => ({ spec: d.pkg + (d.subpath === "." ? "" : d.subpath.slice(1)), file: d.target }))];
for (const f of rows) {
  total++;
  const r = await server.pluginContainer.resolveId(f.spec, importer, { ssr: false });
  const id = String(r?.id ?? "UNRESOLVED").normalize("NFC");
  const ok = id === root + "/" + f.file.normalize("NFC");
  if (!ok) { bad++; console.log("FAIL", f.spec.padEnd(64), id.replace(root + "/", "")); }
}
await server.close();
console.log({ total, bad });
process.exit(bad ? 1 : 0);
