/** 🔭️ Quick candidate of the round 3 sleeping person for looking while the body is authored: the rooted document with the hand-stated 1:50 body (other lists as in the catalog; an optional regex keeps only the matching solids, to look closely), written to `🗑️generated/r3-sleeping-person/look.json` for `r3-sleeping-person-sheet.ts --file`.
 * @see ./r3-sleeping-person-s50.ts */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { Kit, ASSET_ROOTED } from "./r3-sleeping-person-kit.ts";
import { body50 } from "./r3-sleeping-person-s50.ts";
import { rooted } from "./r3-sleeping-person-roots.ts";
const document = rooted([]);
const k = new Kit();
body50(k);
const focus = process.argv[2];
document.asset.volumes["50"].body = focus === undefined ? k.solids : k.solids.filter((x) => new RegExp(focus).test(x.id));
writeFileSync(join(import.meta.dir, "🗑️generated", "r3-sleeping-person", "look.json"), JSON.stringify(document));
console.log("body", k.solids.length, ASSET_ROOTED.id);
