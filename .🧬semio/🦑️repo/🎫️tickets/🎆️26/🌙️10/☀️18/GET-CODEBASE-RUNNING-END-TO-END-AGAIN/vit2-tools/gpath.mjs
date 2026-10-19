import { createRequire } from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { build } = require("esbuild");
const root = process.cwd();
const externalize = { name: "x", setup(b) { b.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true })); } };
const entry = process.argv[2], needle = process.argv[3];
const r = await build({ entryPoints: [root + "/" + entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: root, plugins: [externalize], loader: { ".json": "json" } });
const g = r.metafile.inputs;
const start = Object.keys(g).find((k) => k === entry) ?? Object.keys(g)[0];
const prev = new Map([[start, null]]); const q = [start]; let hit = null;
while (q.length) { const k = q.shift(); if (k.includes(needle)) { hit = k; break; } for (const i of g[k]?.imports ?? []) if (!prev.has(i.path) && g[i.path]) { prev.set(i.path, k); q.push(i.path); } }
const out = []; for (let k = hit; k; k = prev.get(k)) out.push(k); console.log(out.reverse().join("\n"));
