import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { build } = require("esbuild");
const root = process.cwd();
const contract = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/⚙️config-graph.json", "utf8"));
const externalize = { name: "x", setup(b) { b.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true })); } };
const r = await build({ entryPoints: [root + "/" + contract.entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: root, plugins: [externalize] });
const inputs = r.metafile.inputs;
const closure = (m) => { const s = new Set([m]); const q = [m]; while (q.length) { const c = q.pop(); for (const i of inputs[c]?.imports ?? []) if (!i.external && !s.has(i.path)) { s.add(i.path); q.push(i.path); } } return s; };
const union = new Set();
for (const m of contract.require) { const c = closure(m); for (const x of c) union.add(x); console.log(c.size, m.split("/").slice(-3).join("/")); }
console.log("union", union.size);
