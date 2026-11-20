/** ⚡️ Prints `totalPowerW` of the candidate document and of the catalog asset (the manifest row must state it within its tolerance).
 * Usage: `bun r3-runner-power.ts` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { totalPowerW } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";
import { ASSET_FILE, OUT } from "./r3-runner-roots.ts";
for (const file of [join(OUT, "🔣️.json"), ASSET_FILE]) console.log(totalPowerW(JSON.parse(readFileSync(file, "utf8")).asset).toFixed(1));
