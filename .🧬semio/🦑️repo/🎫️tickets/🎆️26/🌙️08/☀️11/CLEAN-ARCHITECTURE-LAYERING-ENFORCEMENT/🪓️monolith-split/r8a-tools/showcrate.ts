// usage: bun showcrate.ts <pkg> <crate-name-suffix> [--no-test-aggregate]: prints the generated lib root (and Cargo.toml) of one planned crate
import { planArtifactCrates, carveWrites } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, suffix, ...rest] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, { mutationFamilies: "on", tests: { fixtures: true, aggregate: !rest.includes("--no-test-aggregate"), hoist: true, detach: true } });
console.log("problems", plan.problems.length);
const { writes } = await carveWrites(plan);
const crate = plan.crates.find((c) => c.name.endsWith(suffix!));
if (!crate) throw new Error("no crate " + suffix);
const pkgDir = crate.packageDir; console.log("crate", crate.name, pkgDir); for (const w of writes) if (w.path === pkgDir + "/Cargo.toml" || w.path === pkgDir + "/🦀️.rs") { console.log("=====", w.action, w.path); console.log(String((w as any).content ?? (w as any).text ?? "").slice(0, 16000)); }
