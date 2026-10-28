// 🧪️ stale-g: owners of the sealed fact index that hold the stamp row `inferrer:script-inputs` (what a bump of that stamp stales) and the age of their newest analysis source. usage: bun stamp-census.mjs
import {readFileSync} from "node:fs";
const R = "/Users/ueli/Documents/semio/", L = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
process.chdir(R);
const inference = await import(L + "🕸️graph/📥️inference/🟨️.mjs"); await inference.libraryBootstrap;
const G = await import(L + "🕸️graph/🕰️generations/🟨️.mjs");
const dir = R + ".cache/b/g/m/generations", c = JSON.parse(readFileSync(dir + "/current.json", "utf8")), m = JSON.parse(readFileSync(dir + "/m/" + c.manifest + ".json", "utf8"));
const idx = G.unpackFactIndexV1(readFileSync(dir + "/x/" + m.factIndex + ".facts.br"), 1 << 30);
const stampRows = idx.rows.map((r, i) => [r, i]).filter(([r]) => r.kind === "stamp" && r.path === "inferrer:script-inputs").map(([, i]) => i);
const owners = new Set(); for (const cl of idx.classes) if (cl.r.some((r) => stampRows.includes(r))) for (const o of cl.o) owners.add(idx.owners[o]);
const real = [...owners].filter((o) => !o.startsWith("@derived:") && !o.startsWith("@"+"env"));
console.log(JSON.stringify({generation: c.generation, ownersInManifest: Object.keys(m.owners).length, ownersHoldingStamp: owners.size, projectsHoldingStamp: real.length}));
process.exit(0);
