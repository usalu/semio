import { build } from "esbuild";
import { relative, resolve } from "node:path";
const root = process.argv[2], entry = resolve(root, process.argv[3]);
const ext = { name: "x", setup(b) { b.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true })); } };
const r = await build({ entryPoints: [entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: root, plugins: [ext] });
const inputs = r.metafile.inputs, rel = (p) => relative(root, resolve(root, p));
const keys = Object.keys(inputs);
console.log("modules", keys.length, "bytes", Object.values(inputs).reduce((t, i) => t + i.bytes, 0));
const entryKey = keys.find((k) => resolve(root, k) === entry);
const parent = new Map([[entryKey, null]]); const q = [entryKey];
while (q.length) { const k = q.shift(); for (const i of inputs[k].imports) if (!i.external && inputs[i.path] && !parent.has(i.path)) { parent.set(i.path, k); q.push(i.path); } }
const need = process.argv.slice(4);
const chain = (k) => { const o = []; for (let c = k; c; c = parent.get(c)) o.push(c); return o.reverse(); };
const depth = Number(process.env.GRAPH_DEPTH ?? 1); const sub = {}; for (const k of keys) { const c = chain(k); const h = c[depth] ?? c[c.length - 1]; sub[h] = (sub[h] || 0) + 1; }
console.log("first-hop subtree sizes:"); for (const [h, n] of Object.entries(sub).sort((a, b) => b[1] - a[1]).slice(0, 12)) console.log(n, h);
for (const n of need) { const k = keys.find((x) => x.includes(n)); console.log("PATH to", n, k ? chain(k).join("\n   -> ") : "NOT REACHED"); }

if (process.env.LISTALL) for (const k of keys) console.log("MOD", chain(k).length - 1, k);

if (process.env.IMPORTERS) for (const k of keys) for (const i of inputs[k].imports) if (!i.external && i.path.includes(process.env.IMPORTERS)) console.log("IMP", k, "->", i.path);
