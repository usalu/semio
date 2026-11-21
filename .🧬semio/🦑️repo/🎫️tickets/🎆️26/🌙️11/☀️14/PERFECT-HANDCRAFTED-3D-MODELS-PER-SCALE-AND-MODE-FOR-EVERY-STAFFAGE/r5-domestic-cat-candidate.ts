/** 🧪️ Round 5 domestic cat: writes the working asset (catalog + corrected roots + redrawn representations + source volumes) as a candidate document to 🗑️generated/r5-domestic-cat/🔣️.json for `r3-validate.ts --file` and `r3-2d.ts --file`, and the root members to `members.json` for the catalog write.
 * Usage: bun r5-domestic-cat-candidate.ts
 * @see ./r5-domestic-cat-work.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { entryOf } from "./volumes-lib.ts";
import { OUT, SPEC, working } from "./r5-domestic-cat-work.ts";

const doc = JSON.parse(readFileSync(entryOf(SPEC).entry.path, "utf8"));
const asset = working();
const ordered: Record<string, unknown> = {};
for (const key of Object.keys(doc.asset)) ordered[key] = asset[key];
writeFileSync(join(OUT, "🔣️.json"), JSON.stringify({ ...doc, asset: ordered }));
const members: Record<string, unknown> = {};
for (const key of ["materials", "rig", "anchors", "representations", "poses"]) members[key] = asset[key];
writeFileSync(join(OUT, "members.json"), JSON.stringify(members));
console.log("candidate and members written");
