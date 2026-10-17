/** 🧾️ Loads a Vite builder/vitest config module under bun and prints its `resolve.alias` entries as JSON: `bun alias-dump.mjs <file> [build|serve]`. */
import { repositoryCachePartitionRootDataV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
const repo = "/Users/ueli/Documents/semio";
process.env.SEMIO_BUILD_MODE = "prototype";
process.env.SEMIO_VITE_CACHE_DIRECTORY = repositoryCachePartitionRootDataV1(repo, "products") + "/ass-alias-probe";
const [file, command = "build"] = process.argv.slice(2);
const mod = await import(file);
let cfg = mod.default;
if (typeof cfg === "function") cfg = await cfg({ command, mode: "prototype", isSsrBuild: false, isPreview: false });
let alias = cfg?.resolve?.alias ?? [];
if (!Array.isArray(alias)) alias = Object.entries(alias).map(([find, replacement]) => ({ find, replacement }));
const rows = alias.map((a) => ({ find: a.find instanceof RegExp ? a.find.toString() : a.find, regex: a.find instanceof RegExp, replacement: a.replacement }));
console.log("ALIASJSON " + JSON.stringify(rows));
