// 🧪️ stale-g: recomputes ONE closure recipe of a stored inference project record on the live tree with the production function, and compares it with a sealed inventory roster (token list of the generation target) . usage: bun recipe-closure.mjs <record.json> <recipe-key-prefix> [sealed-list]
import {readFileSync, writeFileSync, existsSync} from "node:fs";
import {join} from "node:path";
import {brotliDecompressSync} from "node:zlib";
const LIVE = "/Users/ueli/Documents/semio", W = process.env.STL_ROOT ?? LIVE, LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
process.chdir(W);
const D = LIVE + "/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const [recordFile, prefix, sealedFile] = process.argv.slice(2);
const text = brotliDecompressSync(readFileSync(D + recordFile)).toString("utf8"), value = JSON.parse(text.slice(text.indexOf("\n") + 1));
const key = Object.keys(value.closureRecipes).find((k) => k.startsWith(prefix)), recipe = value.closureRecipes[key];
const inference = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const scripts = inference.createScriptInputCache({workspaceRoot: W});
const {inputsDigest, ...rest} = recipe.target ?? {};
const target0 = inputsDigest ? {...rest, inputs: value.closureInputs[inputsDigest]} : rest;
const target = W === LIVE ? target0 : {...target0, options: {...target0.options, command: target0.options.command.replaceAll(LIVE, W)}};
const result = await inference.cacheInternals.targetScriptSources(target, W, scripts, recipe.projectRoot);
const fresh = (result.closure ?? []).filter((x) => typeof x === "string").map((x) => x.replace(/^\{workspaceRoot\}\//u, ""));
console.log(JSON.stringify({recipe: recipe.fn, command: target.options?.command?.slice(0, 160), fresh: fresh.length, fallback: result.fallback}));
if (process.env.OUT) writeFileSync(process.env.OUT, fresh.join("\n") + "\n");
if (sealedFile) { const sealed = new Set(readFileSync(sealedFile, "utf8").split("\n").filter(Boolean)); const added = fresh.filter((f) => !sealed.has(f) && existsSync(f)); console.log("fresh not in sealed key:", added.length); for (const f of added.slice(0, 25)) console.log("  +", f); }
process.exit(0);
