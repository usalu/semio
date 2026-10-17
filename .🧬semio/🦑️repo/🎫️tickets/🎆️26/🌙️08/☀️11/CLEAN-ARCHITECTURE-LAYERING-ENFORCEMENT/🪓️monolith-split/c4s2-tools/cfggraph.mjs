import { build } from "/Users/ueli/Documents/semio/node_modules/esbuild/lib/main.js";
import { relative, resolve } from "node:path";
const repoRoot = "/Users/ueli/Documents/semio";
const entry = resolve(repoRoot, process.argv[2]);
const externalizeDeps = { name: "externalize-deps", setup(b) { b.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true })); } };
const result = await build({ entryPoints: [entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: repoRoot, plugins: [externalizeDeps] });
const mods = Object.keys(result.metafile.inputs).map((p) => relative(repoRoot, resolve(repoRoot, p)).replaceAll("\\", "/")).sort();
console.log(JSON.stringify({ count: mods.length, bytes: Object.values(result.metafile.inputs).reduce((s, i) => s + i.bytes, 0), vite: mods.filter((m) => m.includes("vite-plugins")).map((m) => m.split("vite-plugins/")[1]) }));
