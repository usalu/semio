// 🧪️ stale-g: a target's RAW sealed roster (generation fragment tokens + literal inputs, before negations) vs the production closure of one stored recipe on the live tree. usage: bun raw-vs-recipe.mjs <project> <target> <record.json> <recipe-key-prefix> [out.json]
import {readFileSync, writeFileSync, existsSync} from "node:fs";
import {join} from "node:path";
import {brotliDecompressSync} from "node:zlib";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
process.chdir(W);
const [project, target, recordFile, prefix, out] = process.argv.slice(2);
const g = W + "/.cache/b/g/m/generations/", cur = JSON.parse(readFileSync(g + "current.json", "utf8")), man = JSON.parse(readFileSync(g + "m/" + cur.manifest + ".json", "utf8"));
const frag = JSON.parse(brotliDecompressSync(readFileSync(g + "o/" + man.owners[project].fragment + ".json.br")).toString("utf8"));
const names = frag.node.namedInputs ?? {}, seen = new Set(), tokens = new Set(), literal = new Set();
const walk = (list) => { for (const item of list ?? []) { if (typeof item === "string") { if (names[item]) { if (!seen.has(item)) { seen.add(item); walk(names[item]); } } else if (item.startsWith("{workspaceRoot}/") && !/[*?{}\[\]!]/u.test(item.slice(16))) literal.add(item.slice(16)); } else if (item?.env?.startsWith?.("SEMIO_SOURCE_CLOSURE_")) tokens.add(item.env); } };
walk(frag.node.targets[target].inputs);
const sealed = new Set(literal);
for (const token of tokens) { const p = `${W}/.cache/b/g/m/build-input-closures/${token.slice(21)}.json`; if (existsSync(p)) for (const f of JSON.parse(readFileSync(p, "utf8")).files ?? []) sealed.add(f); else console.error("MISSING inventory", token); }
const D = W + "/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089/np/";
const text = brotliDecompressSync(readFileSync(D + recordFile)).toString("utf8"), value = JSON.parse(text.slice(text.indexOf("\n") + 1));
const key = Object.keys(value.closureRecipes).find((k) => k.startsWith(prefix)), recipe = value.closureRecipes[key];
const inference = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const scripts = inference.createScriptInputCache({workspaceRoot: W});
const {inputsDigest, ...rest} = recipe.target ?? {};
const def = inputsDigest ? {...rest, inputs: value.closureInputs[inputsDigest]} : rest;
const result = await inference.cacheInternals.targetScriptSources(def, W, scripts, recipe.projectRoot, recipe.nativeManifest);
const fresh = (result.closure ?? []).filter((x) => typeof x === "string").map((x) => x.replace(/^\{workspaceRoot\}\//u, ""));
const missing = fresh.filter((f) => !sealed.has(f) && existsSync(f)), extra = [...sealed].filter((f) => !fresh.includes(f));
console.log(JSON.stringify({project, target, generation: cur.generation, recipe: recipe.fn, command: def.options?.command?.slice(0, 100), sealedRaw: sealed.size, fresh: fresh.length, freshNotInSealed: missing.length, sealedNotInFresh: extra.length}));
for (const f of missing.slice(0, Number(process.env.SHOW ?? 12))) console.log("  +", f);
if (out) writeFileSync(out, JSON.stringify({sealed: [...sealed].sort(), fresh: fresh.sort(), added: missing.sort()}));
process.exit(0);
