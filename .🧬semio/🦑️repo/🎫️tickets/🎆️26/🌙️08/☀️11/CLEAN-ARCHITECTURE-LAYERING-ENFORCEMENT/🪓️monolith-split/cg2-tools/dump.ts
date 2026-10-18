// 🔭️ Dry plan dump of one artifact package with explicit target/hard budgets: per crate name, dir, size and the lib/test source paths, plus problems, blockers and notes, as JSON.
import { writeFileSync } from "node:fs";
import { planArtifactCrates, parseCrateBudget } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, out, target, hard] = process.argv.slice(2);
const started = Date.now();
const lift = "standards::v1::subsets::any::schema::*,standards::v1::subsets::any::io::*,editor::*,render::*".split(",");
const plan: any = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, { mutationFamilies: "on", lift, target: target ? parseCrateBudget(target) : undefined, hard: hard ? parseCrateBudget(hard) : undefined, tests: { fixtures: true, aggregate: true, hoist: true, detach: true } } as any);
const rows = plan.crates.map((c: any) => ({ name: c.name, dir: c.packageDir, lib: c.lib && [c.lib.files, c.lib.bytes, c.lib.lines], test: c.test && [c.test.files, c.test.bytes, c.test.lines], libPaths: c.lib?.sourcePaths ?? [], testPaths: c.test?.sourcePaths ?? [], deps: c.dependsOn.map((id: string) => plan.crates.find((e: any) => e.id === id)?.name) }));
writeFileSync(out!, JSON.stringify({ ms: Date.now() - started, crates: rows, problems: plan.problems, blockers: plan.blockers, notes: plan.overlay.notes }, null, 1));
console.log("done", Date.now() - started, "ms", rows.length, "crates", plan.problems.length, "problems");
