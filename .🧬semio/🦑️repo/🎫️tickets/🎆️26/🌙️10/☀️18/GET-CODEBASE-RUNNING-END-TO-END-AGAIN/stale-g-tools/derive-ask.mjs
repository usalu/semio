// 🧪️ stale-g: asks every derived question of one owner again against the live tree (the early-cutoff step of plan()) and compares the answer digest with the stored fact. usage: bun derive-ask.mjs <owner>. RSS guard 1.5 GB.
import {readFileSync} from "node:fs";
const R = "/Users/ueli/Documents/semio/", L = R + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
process.chdir(R);
const inference = await import(L + "🕸️graph/📥️inference/🟨️.mjs"); await inference.libraryBootstrap;
const obs = await import(L + "👁️observation/🟨️.mjs"), si = await import(L + "🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const G = await import(L + "🕸️graph/🕰️generations/🟨️.mjs");
const admission = await import(L + "🕸️graph/🚪️admission/🟦️.ts");
const registry = admission.readBuildOwnerRegistryV1(JSON.parse(readFileSync(R + "📋️build-owners.json", "utf8")));
inference.prepareSourceAnalysesV1();
const deriver = await inference.openGenerationDeriverV1({workspaceRoot: R.slice(0, -1), registry});
const dir = R + ".cache/b/g/m/generations", c = JSON.parse(readFileSync(dir + "/current.json", "utf8")), m = JSON.parse(readFileSync(dir + "/m/" + c.manifest + ".json", "utf8"));
const idx = G.unpackFactIndexV1(readFileSync(dir + "/x/" + m.factIndex + ".facts.br"), 1 << 30);
const id = idx.owners.indexOf(process.argv[2]); const rows = [];
for (const cl of idx.classes) if (cl.o.includes(id)) for (const r of cl.r) if (idx.rows[r].kind === "derived") rows.push(idx.rows[r]);
console.log("generation", c.generation, "owner", process.argv[2], "derived rows", rows.length);
for (const row of rows) {
  const q = JSON.parse(row.path);
  const asked = await deriver(row).catch((e) => ({error: String(e.message).slice(0, 100)}));
  console.log(q.program ? "program" : "walk", "roots", JSON.stringify((q.roots ?? q.projects ?? []).map((x) => String(x).replace(R, "").slice(-70))), "stored", String(row.fact).slice(0, 10), "now", asked.error ?? String(asked.fact).slice(0, 10), asked.error ? "ERROR" : asked.fact === row.fact ? "SAME" : "CHANGED");
}
process.exit(0);
