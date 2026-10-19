import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const { build } = require("esbuild");
const root = process.cwd();
const entry = process.argv[2];
const externalize = { name: "x", setup(b) { b.onResolve({ filter: /^[^.#].*/ }, ({ path, kind }) => (kind === "entry-point" || path.startsWith("/") ? null : { external: true })); } };
const r = await build({ entryPoints: [root + "/" + entry], bundle: true, write: false, metafile: true, platform: "node", format: "esm", logLevel: "silent", absWorkingDir: root, plugins: [externalize] });
const needle = process.argv[3];
for (const k of Object.keys(r.metafile.inputs)) { if (!/\.[cm]?ts$/.test(k)) continue; const s = readFileSync(root + "/" + k, "utf8"); for (const [i, line] of s.split("\n").entries()) if (needle && line.includes(needle)) console.log(k.split("/").slice(-4).join("/") + ":" + (i + 1) + ": " + line.slice(0, 200)); }
