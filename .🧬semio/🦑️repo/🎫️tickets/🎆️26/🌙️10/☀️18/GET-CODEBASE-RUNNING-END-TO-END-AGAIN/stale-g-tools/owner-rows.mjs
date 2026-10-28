// 🧪️ stale-g: rows held by one owner (and its derived pseudo owners) in the sealed fact index. usage: bun owner-rows.mjs <owner-name> [pathNeedle]. RSS guard 1.5 GB.
import {readFileSync} from "node:fs";
const R = "/Users/ueli/Documents/semio/", L = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
process.chdir(R);
const inference = await import(L + "🕸️graph/📥️inference/🟨️.mjs"); await inference.libraryBootstrap;
const G = await import(L + "🕸️graph/🕰️generations/🟨️.mjs");
const dir = R + ".cache/b/g/m/generations", c = JSON.parse(readFileSync(dir + "/current.json", "utf8")), m = JSON.parse(readFileSync(dir + "/m/" + c.manifest + ".json", "utf8"));
const idx = G.unpackFactIndexV1(readFileSync(dir + "/x/" + m.factIndex + ".facts.br"), 1 << 30);
const name = process.argv[2], needle = process.argv[3];
const ownerId = idx.owners.indexOf(name); if (ownerId < 0) { console.log("owner not in index"); process.exit(1); }
const rowsOf = (id) => { const out = []; for (const cl of idx.classes) if (cl.o.includes(id)) out.push(...cl.r); return out; };
const mine = rowsOf(ownerId), kinds = {};
for (const r of mine) kinds[idx.rows[r].kind] = (kinds[idx.rows[r].kind] ?? 0) + 1;
console.log(JSON.stringify({generation: c.generation, owner: name, rows: mine.length, kinds, ownersInIndex: idx.owners.length}));
const derived = mine.filter((r) => idx.rows[r].kind === "derived");
for (const r of derived) { const row = idx.rows[r]; let q; try { q = JSON.parse(row.path); } catch { q = {}; } console.log("DERIVED", JSON.stringify(q).slice(0, 260), "fact", String(row.fact).slice(0, 16)); const pseudo = idx.owners.findIndex((o) => o.startsWith("@derived:") && rowsOf(idx.owners.indexOf(o)).includes(r)); if (pseudo >= 0) { const sup = rowsOf(pseudo); const k2 = {}; for (const s of sup) k2[idx.rows[s].kind] = (k2[idx.rows[s].kind] ?? 0) + 1; console.log("   pseudo", idx.owners[pseudo].slice(0, 20), "support rows", sup.length, JSON.stringify(k2)); if (needle) for (const s of sup) if (idx.rows[s].path.includes(needle)) console.log("     ", idx.rows[s].kind, idx.rows[s].path.replace(R, "").slice(0, 150)); } }
if (needle) for (const r of mine) if (idx.rows[r].kind !== "derived" && idx.rows[r].path.includes(needle)) console.log("OWN", idx.rows[r].kind, idx.rows[r].path.replace(R, "").slice(0, 150));
process.exit(0);
