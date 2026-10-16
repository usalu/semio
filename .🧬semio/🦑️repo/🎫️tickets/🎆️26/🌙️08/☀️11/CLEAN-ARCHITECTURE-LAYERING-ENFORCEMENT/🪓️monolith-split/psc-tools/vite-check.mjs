/** 🔍️ Resolves the new package specifiers through a real Vite plugin container using the repo's workspace resolve preset. Usage: bun vite-check.mjs <plan.json> */
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
import { readFileSync } from "node:fs";
const root = "/Users/ueli/Documents/semio";
const plan = JSON.parse(readFileSync(process.argv[2], "utf8"));
const names = { value: "@semio-tech/value-ts", validator: "@semio-tech/schema-validator-ts", schema: "@semio-tech/schema-ts" };
const importer = root + "/🌎️hub/🧪️tests/🎛️run/🟦️.ts";
const server = await createServer({ root, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [root] }, watch: null }, resolve: { alias: [] }, optimizeDeps: { noDiscovery: true, include: [] } });
let bad = 0;
for (const f of plan.facades) {
  const spec = names[f.owner] + (f.subpath === "." ? "" : f.subpath.slice(1));
  const r = await server.pluginContainer.resolveId(spec, importer, { ssr: false });
  const id = String(r?.id ?? "UNRESOLVED").normalize("NFC");
  const want = f.facade.normalize("NFC");
  const ok = id === root + "/" + want;
  if (!ok) bad++;
  console.log(ok ? "ok  " : "FAIL", spec.padEnd(64), ok ? "" : id.replace(root + "/", ""));
}
await server.close();
console.log({ total: plan.facades.length, bad });
process.exit(bad ? 1 : 0);
