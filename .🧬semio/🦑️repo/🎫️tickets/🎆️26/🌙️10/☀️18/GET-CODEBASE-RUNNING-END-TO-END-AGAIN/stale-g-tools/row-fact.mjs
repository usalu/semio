// 🧪️ stale-g: full facts of the rows of one owner whose path contains a needle. usage: bun row-fact.mjs <owner> <needle> [kind]
import {readFileSync} from "node:fs";
const R = "/Users/ueli/Documents/semio/", L = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
process.chdir(R);
const inference = await import(L + "🕸️graph/📥️inference/🟨️.mjs"); await inference.libraryBootstrap;
const G = await import(L + "🕸️graph/🕰️generations/🟨️.mjs");
const dir = R + ".cache/b/g/m/generations", c = JSON.parse(readFileSync(dir + "/current.json", "utf8")), m = JSON.parse(readFileSync(dir + "/m/" + c.manifest + ".json", "utf8"));
const idx = G.unpackFactIndexV1(readFileSync(dir + "/x/" + m.factIndex + ".facts.br"), 1 << 30);
const [name, needle, kind] = process.argv.slice(2), id = idx.owners.indexOf(name);
for (const cl of idx.classes) if (cl.o.includes(id)) for (const r of cl.r) { const row = idx.rows[r]; if (row.path.includes(needle) && (!kind || row.kind === kind)) console.log(row.kind, JSON.stringify(row.fact).slice(0, 700)); }
process.exit(0);
