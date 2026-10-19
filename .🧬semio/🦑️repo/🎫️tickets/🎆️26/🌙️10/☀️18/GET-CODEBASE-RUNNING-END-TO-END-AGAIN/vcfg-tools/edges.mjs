import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { build } = require("esbuild");
const root = process.cwd();
const contract = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/⚙️config-graph.json", "utf8"));
const externalize = { name: "x", setup(b) { b.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true })); } };
const r = await build({ entryPoints: [root + "/" + contract.entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: root, plugins: [externalize] });
const inputs = r.metafile.inputs;
const short = (p) => p.replace("🧰️framework/🛍️products/", "").split("/").slice(-3).join("/");
const roots = process.argv.slice(2);
for (const k of Object.keys(inputs)) {
  if (roots.length && !roots.some((x) => k.includes(x))) continue;
  console.log(short(k), inputs[k].bytes);
  for (const i of inputs[k].imports) console.log("   ", i.external ? "E" : i.kind[0], i.external ? i.path : short(i.path));
}
