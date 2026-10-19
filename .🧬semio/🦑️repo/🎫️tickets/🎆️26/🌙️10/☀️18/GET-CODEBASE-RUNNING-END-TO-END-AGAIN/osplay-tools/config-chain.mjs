import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { build } = require("esbuild");
const root = process.cwd();
const contract = JSON.parse(readFileSync("🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/⚙️config-graph.json", "utf8"));
const externalize = { name: "x", setup(b) { b.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true })); } };
const r = await build({ entryPoints: [root + "/" + contract.entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: root, plugins: [externalize] });
const inputs = r.metafile.inputs;
const rel = (p) => p;
console.log("modules", Object.keys(inputs).length);
const kinds = new Map(); for (const [k, v] of Object.entries(inputs)) for (const i of v.imports) kinds.set(i.kind, (kinds.get(i.kind) ?? 0) + 1);
console.log("edge kinds", JSON.stringify([...kinds]));
const staticSeen = new Set([contract.entry]); const sq = [contract.entry];
while (sq.length) { const c = sq.shift(); for (const i of inputs[c]?.imports ?? []) { if (i.external || i.kind === "dynamic-import" || staticSeen.has(i.path)) continue; staticSeen.add(i.path); sq.push(i.path); } }
console.log("static-only modules", staticSeen.size, "bytes", [...staticSeen].reduce((t, p) => t + inputs[p].bytes, 0), "denied", contract.deny.filter((d) => staticSeen.has(d)).length, "required missing", contract.require.filter((d) => !staticSeen.has(d)).length, "bounds", contract.maxModules, contract.maxSourceBytes);
const parent = new Map([[contract.entry, null]]);
const queue = [contract.entry];
while (queue.length) { const cur = queue.shift(); for (const imp of inputs[cur]?.imports ?? []) { if (imp.external || parent.has(imp.path)) continue; parent.set(imp.path, cur); queue.push(imp.path); } }
const show = (t) => { const chain = []; for (let c = t; c; c = parent.get(c)) chain.push(c.split("/").slice(-4).join("/")); return chain.reverse().join("\n  -> "); };
const denied = contract.deny.filter((d) => parent.has(d));
console.log("denied reachable:", denied.length);
if (denied[0]) console.log(show(denied[0]));
if (process.argv[2]) console.log(show(process.argv[2]));

const sparent = new Map([[contract.entry, null]]); const q2 = [contract.entry];
while (q2.length) { const c = q2.shift(); for (const i of inputs[c]?.imports ?? []) { if (i.external || i.kind === "dynamic-import" || sparent.has(i.path)) continue; sparent.set(i.path, c); q2.push(i.path); } }
const sshow = (t) => { const chain = []; for (let c = t; c; c = sparent.get(c)) chain.push(c.split("/").slice(-4).join("/")); return chain.reverse().join("\n  -> "); };
for (const d of contract.deny.filter((d) => sparent.has(d))) console.log("STATIC DENIED:\n" + sshow(d));
const direct = (inputs[contract.entry].imports).filter((i) => !i.external).map((i) => i.kind[0] + " " + i.path.split("/").slice(-4).join("/"));
console.log("entry direct imports:\n" + direct.join("\n"));
const subtree = (m) => { const seen = new Set([m]); const qq = [m]; while (qq.length) { const c = qq.shift(); for (const i of inputs[c]?.imports ?? []) { if (i.external || i.kind === "dynamic-import" || seen.has(i.path)) continue; seen.add(i.path); qq.push(i.path); } } return seen.size; };
console.log("subtree sizes:\n" + (inputs[contract.entry].imports).filter((i) => !i.external && i.kind !== "dynamic-import").map((i) => subtree(i.path) + " " + i.path.split("/").slice(-4).join("/")).sort((a, b) => parseInt(b) - parseInt(a)).slice(0, 12).join("\n"));
