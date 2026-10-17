/** 🔍️ Resolves every export key this batch added through a real Vite plugin container and loads each asset facade through Vite's SSR loader, comparing the exported value to the asset. Usage: bun vite-check.mjs <batch-snapshot.json> */
import { createServer } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
const root = "/Users/ueli/Documents/semio";
const snap = JSON.parse(readFileSync(process.argv[2], "utf8"));
const rows = [];
for (const [rel, before] of Object.entries(snap)) {
  if (!rel.endsWith("/package.json")) continue;
  const now = JSON.parse(readFileSync(root + "/" + rel, "utf8"));
  const was = before === null ? {} : JSON.parse(before).exports ?? {};
  for (const [key, target] of Object.entries(now.exports ?? {})) if (!(key in was)) rows.push({ spec: now.name + (key === "." ? "" : key.slice(1)), file: rel.slice(0, -"/package.json".length) + "/" + String(target).slice(2) });
}
const importer = root + "/🌎️hub/🧪️tests/🎛️run/🟦️.ts";
const server = await createServer({ root, configFile: false, logLevel: "silent", appType: "custom", server: { middlewareMode: true, fs: { allow: [root] }, watch: null }, resolve: { alias: [] }, optimizeDeps: { noDiscovery: true, include: [] } });
let bad = 0, loaded = 0;
const skipLoad = /testing\/(host|draw-list|schema-oracle)$/u;
for (const r of rows) {
  const res = await server.pluginContainer.resolveId(r.spec, importer, { ssr: false });
  const id = String(res?.id ?? "UNRESOLVED").normalize("NFC");
  const want = (root + "/" + r.file).normalize("NFC");
  if (id !== want) { bad++; console.log("FAIL resolve", r.spec, "->", id.replace(root + "/", "")); continue; }
  if (skipLoad.test(r.spec)) { console.log("ok resolve", r.spec); continue; }
  try {
    const mod = await server.ssrLoadModule(r.spec);
    const src = readFileSync(want, "utf8");
    const names = [...src.matchAll(/export \{ (?:default as )?(\w+) \}/gu)].map(m => m[1]);
    const missing = names.filter(n => !(n in mod));
    const q = src.match(/from "([^"]+)"/u)?.[1] ?? "";
    let detail = "";
    const value = names.length ? mod[names[0]] : undefined;
    if (q.endsWith("?raw")) { const actual = readFileSync(resolve(dirname(want), q.slice(0, -4)), "utf8"); detail = value === actual ? " raw==file" : " RAW-MISMATCH"; if (value !== actual) bad++; }
    else if (q.endsWith("?inline")) { detail = typeof value === "string" && value.startsWith("data:") ? " inline data-url" : " INLINE-BAD"; if (!(typeof value === "string" && value.startsWith("data:"))) bad++; }
    else if (q.endsWith("?url")) { detail = typeof value === "string" && value.length > 0 ? ` url ${value.slice(-60)}` : " URL-BAD"; if (!(typeof value === "string" && value.length)) bad++; }
    if (missing.length) { bad++; console.log("FAIL names", r.spec, missing.join(",")); } else { loaded++; console.log("ok load", r.spec, names.join(",") + detail); }
  } catch (e) { bad++; console.log("FAIL load", r.spec, String(e).split("\n")[0].slice(0, 200)); }
}
await server.close();
console.log({ total: rows.length, loaded, bad });
process.exit(bad ? 1 : 0);
