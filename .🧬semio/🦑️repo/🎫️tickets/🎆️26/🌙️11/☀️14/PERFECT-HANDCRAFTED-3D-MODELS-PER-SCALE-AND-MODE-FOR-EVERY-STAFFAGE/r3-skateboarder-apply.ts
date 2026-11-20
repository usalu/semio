/** 📬 Round 3 skateboarder: writes the validated candidate document into the catalog asset (roots, 2D, dimensions; two-space JSON as the catalog keeps it) and the volumes into the ticket source `volumes/humans/skateboarder.json`; refuses when the catalog file is no longer the one the candidate was built from. Run `volumes-insert.py` afterwards so the `volumes` member takes the catalog's compact style.
 *
 * Usage: `bun r3-skateboarder-apply.ts` */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { GENERATED } from "./volumes-lib.ts";

const CATALOG = join(import.meta.dir, "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/skateboarder/🔣️.json");
const out = join(GENERATED, "r3-skateboarder");
const before = readFileSync(join(out, "asset.before.json"), "utf8");
const applied = join(out, "asset.applied.json");
const now = readFileSync(CATALOG, "utf8");
let last = "";
try {
  last = readFileSync(applied, "utf8");
} catch {
  last = "";
}
if (now !== before && now !== last) throw new Error("the catalog asset changed under this candidate: rebuild from the current file");
const doc = JSON.parse(readFileSync(join(out, "🔣️.json"), "utf8"));
writeFileSync(join(import.meta.dir, "volumes/humans/skateboarder.json"), `${JSON.stringify({ volumes: doc.asset.volumes }, null, 1)}\n`);
writeFileSync(CATALOG, `${JSON.stringify(doc, null, 2)}\n`);
process.stdout.write(`written ${CATALOG}\n`);
