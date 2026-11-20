/** 🖨️ Prints how every scale of the catalog asset `humans/standing-adult` resolves under the default print profile, with every violation.
 * Usage: `bun r3-standing-adult-suit.ts` */
import * as rep from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";

const { asset } = entryOf("humans/standing-adult");
for (const d of [50, 100, 200, 500]) console.log(d, JSON.stringify((rep as any).resolveRepresentation(asset, { denominator: d }, (rep as any).DEFAULT_PRINT_PROFILE), null, 0).slice(0, 3000));
