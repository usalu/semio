// 🔭️ Prints the full blocker report of a crate plan (dry, writes nothing).
import { planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, mode = "auto"] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, { mutationFamilies: mode as "on" | "off" | "auto", tests: { fixtures: true, aggregate: !process.env.R8A_NOAGG, hoist: true, detach: true } });
for (const c of plan.crates) console.log(`crate ${c.name} lib ${c.lib?.files}/${c.lib?.bytes}/${c.lib?.lines} test ${c.test?.files}/${c.test?.bytes}/${c.test?.lines}`);
for (const p of plan.problems) console.log("problem:", p);
for (const b of plan.blockers) console.log(JSON.stringify(b, null, 1));
for (const n of plan.overlay.notes) if (!n.startsWith("made ")) console.log("note:", n);
const only = process.env.PATHS_OF;
if (only) for (const c of plan.crates) if (c.name.endsWith(only)) { console.log("PATHS", c.name, "test"); for (const p of c.test?.paths ?? []) { let sz = 0; try { sz = (await import("node:fs")).statSync(p).size; } catch {} console.log("  ", sz, p.replace(/.*\/✳️any\//, "")); } console.log("deps", c.dependsOn.join(",")); }
for (const [k, v] of [...plan.overlay.edits, ...plan.overlay.creates]) if (k.includes("move-vertex") && k.includes("tests")) console.log("OVERLAY", k.slice(-50), (v.match(/include_str/g) ?? []).length, (v.match(/runtime_fixture_text/g) ?? []).length);
console.log("overlay sizes", plan.overlay.edits.size, plan.overlay.creates.size);
let n = 0;
for (const c of plan.crates) if (c.name.endsWith("mut-fixture-tests")) for (const p of c.test?.sourcePaths ?? []) { const t = plan.overlay.edits.get(p) ?? (await Bun.file(p).text()); const inc = (t.match(/include_str/g) ?? []).length; if (inc && n++ < 3) console.log("INC", p.slice(-70), inc, plan.overlay.edits.has(p)); }
console.log("INC files", n);
for (const c of plan.crates) if (c.name.endsWith("mut-fixture-tests")) { console.log("SRC", c.test?.sourcePaths.length, c.test?.paths.length, c.test?.embedded); const rel = (c.test?.sourcePaths ?? []).slice(0, 2); console.log("SRC0", rel.map((p) => p.slice(-60))); }
