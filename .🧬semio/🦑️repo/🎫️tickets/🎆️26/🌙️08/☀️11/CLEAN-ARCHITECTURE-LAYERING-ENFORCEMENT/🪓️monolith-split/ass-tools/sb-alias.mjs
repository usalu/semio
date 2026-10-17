/** 📖️ Runs the real `.storybook/main.ts` `viteFinal` under bun and dumps its final `resolve.alias` as JSON rows: `bun sb-alias.mjs <out.json>`. */
import { writeFileSync } from "node:fs";
import { repositoryCachePartitionRootDataV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
const repo = "/Users/ueli/Documents/semio";
process.env.SEMIO_BUILD_MODE = "prototype";
process.env.SEMIO_VITE_CACHE_DIRECTORY = repositoryCachePartitionRootDataV1(repo, "products") + "/ass-alias-probe";
const mod = await import(repo + "/.storybook/main.ts");
const cfg = await mod.default.viteFinal({ resolve: { alias: [{ find: /^storybook$/u, replacement: "/sb" }] }, optimizeDeps: {}, plugins: [], server: {}, build: {} }, { configType: "DEVELOPMENT" });
let alias = cfg.resolve.alias;
if (!Array.isArray(alias)) alias = Object.entries(alias).map(([find, replacement]) => ({ find, replacement }));
const rows = alias.map((a) => ({ find: a.find instanceof RegExp ? { source: a.find.source, flags: a.find.flags } : a.find, replacement: a.replacement }));
writeFileSync(process.argv[2], JSON.stringify(rows, null, 1));
console.log("rows", rows.length, "semio", rows.filter((r) => JSON.stringify(r.find).includes("semio-tech")).length);
