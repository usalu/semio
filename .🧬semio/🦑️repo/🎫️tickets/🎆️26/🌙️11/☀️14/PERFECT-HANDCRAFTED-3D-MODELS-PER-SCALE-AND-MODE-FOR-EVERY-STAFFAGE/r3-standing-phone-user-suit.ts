/** 🖨️ Prints how every scale of `humans/standing-phone-user` resolves under the default print profile, with every violation: the candidate document `🗑️generated/r3-standing-phone-user/🔣️.json` by default, the catalog asset with `--catalog`.
 * Usage: `bun r3-standing-phone-user-suit.ts [--catalog]` */
import { readFileSync } from "node:fs";
import { join } from "node:path";

import * as rep from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { GENERATED, entryOf } from "./volumes-lib.ts";

const asset = process.argv.includes("--catalog") ? entryOf("humans/standing-phone-user").asset : JSON.parse(readFileSync(join(GENERATED, "r3-standing-phone-user", "🔣️.json"), "utf8")).asset;
for (const d of [50, 100, 200, 500]) {
  const r = (rep as any).resolveRepresentation(asset, { denominator: d }, (rep as any).DEFAULT_PRINT_PROFILE);
  console.log(d, r.status ?? r.kind ?? "", JSON.stringify(r.violations ?? r.suitability?.violations ?? r, null, 0).slice(0, 1800));
}
