// 🔭️ Writes the full plan report (crates, problems, blockers with causes/edges/rings, notes) of one artifact package as JSON; dry, writes only to the given output file.
import { writeFileSync } from "node:fs";
import { planArtifactCrates } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, out, mode = "auto", lift = ""] = process.argv.slice(2);
const started = Date.now();
const plan: any = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, { mutationFamilies: mode as "on" | "off" | "auto", lift: lift ? lift.split(",") : undefined, tests: { fixtures: true, aggregate: true, hoist: true, detach: true } } as any);
const rows = plan.crates.map((c: any) => ({ name: c.name, dir: c.packageDir, lib: c.lib && [c.lib.files, c.lib.bytes, c.lib.lines], test: c.test && [c.test.files, c.test.bytes, c.test.lines], deps: c.dependsOn.map((id: string) => plan.crates.find((e: any) => e.id === id)?.name), attach: c.attach.map((d: any) => d.eff.join("::")) }));
writeFileSync(out!, JSON.stringify({ ms: Date.now() - started, crates: rows, problems: plan.problems, blockers: plan.blockers, notes: plan.overlay.notes, packing: plan.packing }, null, 1));
console.log("done", Date.now() - started, "ms", rows.length, "crates", plan.problems.length, "problems");
