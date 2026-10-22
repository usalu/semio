// 📏 Builder-side benchmark over the live fact index (read-only): rebuild the index from per-owner row lists exactly as a cold build does, then pack it; time + memory per stage.
import { readFileSync } from "node:fs";
import { brotliDecompressSync } from "node:zlib";
import { join } from "node:path";
const W = "/Users/ueli/Documents/semio", LIB = join(W, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const GEN = process.env.GEN ?? join(W, ".cache/b/g/m/generations");
const gen = await import(join(LIB, "🏗️build/🕸️graph/🕰️generations/🟨️.mjs"));
const mb = n => Math.round(n / 1e6), t0 = performance.now();
const mark = (label, extra = {}) => { const m = process.memoryUsage(); console.log(JSON.stringify({ label, s: +((performance.now() - t0) / 1000).toFixed(1), rssMB: mb(m.rss), heapMB: mb(m.heapUsed), extMB: mb(m.external), ...extra })); };
const cur = JSON.parse(readFileSync(GEN + "/current.json", "utf8")), man = JSON.parse(readFileSync(GEN + "/m/" + cur.manifest + ".json", "utf8"));
const idx = JSON.parse(brotliDecompressSync(readFileSync(GEN + "/x/" + man.factIndex + ".json.br")).toString("utf8"));
mark("loaded", { rows: idx.rows.length, classes: idx.classes.length });
const ownerRows = new Map(); idx.owners.forEach(name => ownerRows.set(name, []));
for (const cls of idx.classes) for (const o of cls.o) { const list = ownerRows.get(idx.owners[o]); for (const r of cls.r) list.push(idx.rows[r]); }
let pairs = 0; for (const l of ownerRows.values()) pairs += l.length;
mark("ownerRows", { pairs });
const built = gen.createFactIndexV1(ownerRows);
mark("createFactIndex(live)", { rows: built.rows.length, classes: built.classes.length });
if (process.env.COMPARE) { const old = await import("/private/tmp/ga1022/gen-old.mjs"); const o = old.createFactIndexV1(ownerRows); mark("createFactIndex(old)", { rows: o.rows.length, classes: o.classes.length }); console.log("IDENTICAL", gen.canonicalJsonV1(o) === gen.canonicalJsonV1(built)); }
if (!process.env.NOPACK) { const bytes = gen.packGenerationDocumentV1(built, 536870912); mark("pack", { packedMB: mb(bytes.length) }); }
