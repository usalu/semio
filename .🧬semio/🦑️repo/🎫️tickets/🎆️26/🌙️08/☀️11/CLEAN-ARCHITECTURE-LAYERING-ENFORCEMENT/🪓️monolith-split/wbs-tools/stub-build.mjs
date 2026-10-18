import { build } from "/Users/ueli/Documents/semio/node_modules/vite/dist/node/index.js";
const [name, root, cfg, mode = "prototype"] = process.argv.slice(2);
const DROP = new Set(["semio-canonical-wasm-build", "semio-managed-wasm-packages"]);
const STUB = /^(@semio-tech\/(framework-surface-rs|framework-editor-rs|framework-os-node-graph-rs|puzzle-wasm|flow-core)(\/.*)?|[^?]*\.wasm(\?.*)?)$/u;
const stub = { name: "wbs-wasm-stub", enforce: "pre", resolveId(id) { if (STUB.test(id)) return { id: "\0wbs-stub:" + id, syntheticNamedExports: true }; }, load(id) { if (id.startsWith("\0wbs-stub:")) return "export default {};"; } };
const hook = { name: "wbs-plugin-surgery", configResolved(c) { for (let i = c.plugins.length - 1; i >= 0; i--) if (DROP.has(c.plugins[i].name)) c.plugins.splice(i, 1); c.plugins.unshift(stub); } };
const out = `/Users/ueli/.cache/wbs/out/${name}`;
const result = await build({ configFile: cfg, configLoader: process.env.WBS_LOADER ?? "runner", mode, plugins: [hook], build: { outDir: out, emptyOutDir: true, minify: false, sourcemap: false }, logLevel: "info" });
console.log("STUB-BUILD-OK", name, (Array.isArray(result) ? result : [result]).flatMap((r) => r.output?.length ?? 0).join(","));
