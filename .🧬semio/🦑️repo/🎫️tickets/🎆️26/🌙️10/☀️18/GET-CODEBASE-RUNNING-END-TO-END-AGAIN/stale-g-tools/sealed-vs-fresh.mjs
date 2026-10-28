// 🧪️ stale-g: sealed closure of a target (generation inventory tokens + literal inputs) vs the PRODUCTION closure on the live tree (targetScriptSources, graph-free). usage: bun sealed-vs-fresh.mjs <project> <target> [out.json]. RSS guard 1.2 GB.
import {readFileSync, writeFileSync, existsSync} from "node:fs";
import {join} from "node:path";
import {brotliDecompressSync} from "node:zlib";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
process.chdir(W);
const [project, target, out] = process.argv.slice(2);
const g = W + "/.cache/b/g/m/generations/", cur = JSON.parse(readFileSync(g + "current.json", "utf8")), man = JSON.parse(readFileSync(g + "m/" + cur.manifest + ".json", "utf8"));
const row = man.owners[project], frag = JSON.parse(brotliDecompressSync(readFileSync(g + "o/" + row.fragment + ".json.br")).toString("utf8"));
const names = frag.node.namedInputs ?? {}, seen = new Set(), tokens = new Set(), literal = new Set();
const walk = (list) => { for (const item of list ?? []) { if (typeof item === "string") { if (names[item]) { if (!seen.has(item)) { seen.add(item); walk(names[item]); } } else if (item.startsWith("{workspaceRoot}/") && !/[*?{}\[\]!]/u.test(item.slice(16))) literal.add(item.slice(16)); } else if (item?.env?.startsWith?.("SEMIO_SOURCE_CLOSURE_")) tokens.add(item.env); } };
walk(frag.node.targets[target].inputs);
const sealed = new Set(literal);
for (const token of tokens) { const p = `${W}/.cache/b/g/m/build-input-closures/${token.slice(21)}.json`; if (!existsSync(p)) { console.error("MISSING inventory", token); continue; } for (const f of JSON.parse(readFileSync(p, "utf8")).files ?? []) sealed.add(f); }
const inference = await import(join(LIB, "🏗️build/🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
let definition = frag.node.targets[target];
const declared = JSON.parse(readFileSync(join(W, frag.root, "📋️project.json"), "utf8")).targets?.[target];
if (declared) definition = {...definition, ...declared, options: {...(definition.options ?? {}), ...(declared.options ?? {})}, metadata: definition.metadata ? {...definition.metadata, ...(declared.metadata ?? {})} : declared.metadata};
const scripts = inference.createScriptInputCache({workspaceRoot: W});
const result = await inference.cacheInternals.targetScriptSources(definition, W, scripts, frag.root);
const fresh = new Set((result.closure ?? []).filter((x) => typeof x === "string").map((x) => x.replace(/^\{workspaceRoot\}\//u, "")));
const added = [...fresh].filter((f) => !sealed.has(f) && existsSync(join(W, f))).sort(), removed = [...sealed].filter((f) => !fresh.has(f) && existsSync(join(W, f)) && false);
const info = {project, target, generation: cur.generation, fragment: row.fragment, sealed: sealed.size, fresh: fresh.size, freshNotSealed: added.length, rssMb: Math.round(process.memoryUsage().rss / 1048576)};
console.log(JSON.stringify(info));
if (out) writeFileSync(out, JSON.stringify({info, added, sealed: [...sealed].sort(), fresh: [...fresh].sort()}));
for (const f of added.slice(0, Number(process.env.SHOW ?? 15))) console.log("  +", f);
process.exit(0);
