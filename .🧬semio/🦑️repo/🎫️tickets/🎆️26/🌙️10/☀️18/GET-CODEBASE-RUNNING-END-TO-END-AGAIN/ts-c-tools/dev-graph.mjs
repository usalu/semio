import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { build } = require("esbuild");
const repo = "/Users/ueli/Documents/semio/";
const entry = repo + "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts";
const r = await build({ entryPoints: [entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: repo, plugins: [{ name: "x", setup(b) { b.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true })); } }] });
const mods = Object.keys(r.metafile.inputs);
console.log(mods.length);
// who imports whom: show the first importer chain for modules by directory prefix
const groups = {};
for (const m of mods) { const k = m.split("/").slice(0, 5).join("/"); groups[k] = (groups[k] ?? 0) + 1; }
for (const [k, v] of Object.entries(groups).sort((a, b) => b[1] - a[1]).slice(0, 8)) console.log(v, k);
const imp = {}; for (const [m, i] of Object.entries(r.metafile.inputs)) for (const x of i.imports) (imp[x.path] ??= []).push(m);
const roots = mods.filter((m) => m.includes("builder/") || true).slice(0, 0);
const entryImports = r.metafile.inputs[Object.keys(r.metafile.inputs).find((m) => m.endsWith("🌐️vite/🟦️.ts"))].imports.map((x) => x.path);
for (const p of entryImports) console.log("entry imports", p.slice(-90));
