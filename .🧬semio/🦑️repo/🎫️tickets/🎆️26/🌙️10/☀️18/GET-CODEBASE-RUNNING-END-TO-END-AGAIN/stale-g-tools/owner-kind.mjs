// 🧪️ stale-g: prints the rows of one kind held by one owner of the sealed fact index. usage: bun owner-kind.mjs <owner> <kind> [limit]
import {readFileSync} from "node:fs";
const R = "/Users/ueli/Documents/semio/", L = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
process.chdir(R);
const inference = await import(L + "🕸️graph/📥️inference/🟨️.mjs"); await inference.libraryBootstrap;
const G = await import(L + "🕸️graph/🕰️generations/🟨️.mjs");
const dir = R + ".cache/b/g/m/generations", c = JSON.parse(readFileSync(dir + "/current.json", "utf8")), m = JSON.parse(readFileSync(dir + "/m/" + c.manifest + ".json", "utf8"));
const idx = G.unpackFactIndexV1(readFileSync(dir + "/x/" + m.factIndex + ".facts.br"), 1 << 30);
const [name, kind, limit = "30"] = process.argv.slice(2);
const id = idx.owners.indexOf(name), out = [];
for (const cl of idx.classes) if (cl.o.includes(id)) for (const r of cl.r) if (idx.rows[r].kind === kind) out.push(idx.rows[r]);
console.log("generation", c.generation, "owner", name, kind, out.length);
for (const r of out.slice(0, Number(limit))) console.log(JSON.stringify({path: String(r.path).replace(R, "").slice(0, 160), fact: typeof r.fact === "string" ? r.fact.slice(0, 80) : JSON.stringify(r.fact).slice(0, 160)}));
process.exit(0);
