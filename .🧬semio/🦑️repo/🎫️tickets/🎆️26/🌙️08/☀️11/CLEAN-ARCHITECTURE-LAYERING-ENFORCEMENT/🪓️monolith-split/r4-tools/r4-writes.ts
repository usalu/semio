// Dumps the writes of a carve plan (no repository writes) into a directory under F/🗑️generated/r4-writes-<tag>.
import {mkdirSync, writeFileSync} from "node:fs";
import {dirname, join} from "node:path";
import {carveWrites, planArtifactCrates} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️authoring/📦️crate-tree/🟦️.ts";

const [pkg, tag] = process.argv.slice(2);
const OUT = join("/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated", `r4-writes-${tag}`);
const plan = await planArtifactCrates("/Users/ueli/Documents/semio", pkg!, {mutationFamilies: (process.env.MF as "on" | "off" | "auto") ?? "on", tests: {fixtures: true, aggregate: true, hoist: true, detach: true}});
console.log("problems", plan.problems);
const {writes} = await carveWrites(plan);
for (const write of writes) {
  const to = join(OUT, write.path.replaceAll("/", "__").slice(-180));
  mkdirSync(dirname(to), {recursive: true});
  writeFileSync(to, `// ${write.action} ${write.path}\n${write.content}`);
}
console.log("wrote", writes.length, "to", OUT);
