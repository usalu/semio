/** ⚡️ Prints the total power of `humans/running-child` as the catalog validator sums it, for the `powerW` of the manifest row.
 * Usage: `bun r3-running-child-power.ts` */
import { totalPowerW } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/⚡️power/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";
console.log(totalPowerW(entryOf("humans/running-child").asset as never).toFixed(1));
