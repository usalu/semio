// 📋️ Prints the source/test closure paths (with sizes) of one planned crate.
import {statSync, readFileSync} from "node:fs";
import {planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";
const [pkg, suffix, mode] = process.argv.slice(2);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, {mutationFamilies: (mode as "on" | "off" | "auto") ?? "auto", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
const crate = plan.crates.find((c) => c.name.endsWith(suffix!))!;
const libSet = new Set(crate.lib?.paths ?? []);
for (const path of crate.test?.paths ?? []) {
  let size = 0, lines = 0; try { size = statSync("/Users/ueli/Documents/semio/" + path).size; lines = readFileSync("/Users/ueli/Documents/semio/" + path, "utf8").split("\n").length; } catch {}
  console.log(`${libSet.has(path) ? "L" : "T"}\t${size}\t${lines}\t${path.replace(/^.*✳️any\//u, "")}`);
}
console.log("measure lib", JSON.stringify({f: crate.lib?.files, b: crate.lib?.bytes, l: crate.lib?.lines}), "test", JSON.stringify({f: crate.test?.files, b: crate.test?.bytes, l: crate.test?.lines}));
console.log("slices", crate.slices.map((s: any) => s.id ?? s).join(" | ").slice(0, 1500));
