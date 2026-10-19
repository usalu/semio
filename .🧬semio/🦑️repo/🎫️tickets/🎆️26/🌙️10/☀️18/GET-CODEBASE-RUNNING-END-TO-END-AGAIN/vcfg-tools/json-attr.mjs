import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { build } = require("esbuild");
const root = process.cwd();
const externalize = { name: "x", setup(b) { b.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true })); } };
for (const entry of process.argv.slice(2)) {
  const r = await build({ entryPoints: [root + "/" + entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: root, plugins: [externalize] });
  const missing = [];
  for (const [k, v] of Object.entries(r.metafile.inputs)) for (const i of v.imports) if (!i.external && i.path.endsWith(".json") && !(i.with && i.with.type === "json")) missing.push(k.split("/").slice(-4).join("/") + " -> " + i.path.split("/").slice(-3).join("/"));
  console.log(entry.split("/").slice(-3).join("/"), "modules", Object.keys(r.metafile.inputs).length, "json-without-attribute", missing.length);
  for (const m of missing.slice(0, 12)) console.log("  ", m);
}
