// keys3-g: replays what @tailwindcss/vite 4.3.3 does for one CSS entry (compile -> sources -> oxide Scanner -> build) so the scan region and the served CSS can be compared offline.
// usage: bun tw-scan.mjs <entry.css> <vite root> <outDir> [--remap from=to ...]   (remap: an @import resolved path is replaced by another file, to test a patched copy without touching the tree)
import {readFileSync, writeFileSync, mkdirSync} from "node:fs";
import {dirname, resolve} from "node:path";
import {createRequire} from "node:module";
const require = createRequire("/Users/ueli/Documents/semio/package.json");
const {compile} = await import(require.resolve("@tailwindcss/node"));
const {Scanner} = await import(require.resolve("@tailwindcss/oxide"));
const [entry, root, outDir, ...rest] = process.argv.slice(2);
const remap = new Map();
for (let at = 0; at < rest.length; at++) if (rest[at] === "--remap") { const [from, to] = rest[++at].split("="); remap.set(resolve(from), resolve(to)); }
mkdirSync(outDir, {recursive: true});
const css = readFileSync(process.env.TW_CSS_TEXT ?? entry, "utf8");
const compiler = await compile(css, {base: dirname(resolve(entry)), onDependency() {}, shouldRewriteUrls: true,
  customCssResolver: remap.size ? async (id, base) => { const path = resolve(base, id); return remap.has(path) ? remap.get(path) : undefined; } : undefined});
const sources = (compiler.root === "none" ? [] : compiler.root === null ? [{base: resolve(root), pattern: "**/*", negated: false}] : [{...compiler.root, negated: false}]).concat(compiler.sources);
const scanner = new Scanner({sources});
const candidates = scanner.scan();
writeFileSync(outDir + "/sources.json", JSON.stringify(sources, null, 1));
writeFileSync(outDir + "/files.txt", [...scanner.files].sort().join("\n") + "\n");
writeFileSync(outDir + "/candidates.txt", [...new Set(candidates)].sort().join("\n") + "\n");
writeFileSync(outDir + "/out.css", compiler.build(candidates));
console.error(`sources ${sources.length}, files ${scanner.files.length}, candidates ${new Set(candidates).size}, css ${compiler.build(candidates).length} bytes, rss ${Math.round(process.memoryUsage().rss / 1048576)} MB`);
