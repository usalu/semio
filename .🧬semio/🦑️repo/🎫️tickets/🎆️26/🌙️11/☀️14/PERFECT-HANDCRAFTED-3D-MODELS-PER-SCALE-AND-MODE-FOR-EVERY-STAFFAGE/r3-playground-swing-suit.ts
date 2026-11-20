/** 🖨️ Prints the print-profile suitability of every authored scale of a candidate asset document: `bun r3-playground-swing-suit.ts <asset document json>`. */
import { readFileSync } from "node:fs";

import { resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { DEFAULT_PRINT_PROFILE } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🧾️profiles/🟦️.ts";

const asset = JSON.parse(readFileSync(process.argv[2]!, "utf8")).asset;
for (const denominator of [50, 100, 200, 500]) {
  const r = resolveRepresentation(asset, { denominator }, DEFAULT_PRINT_PROFILE) as { kind: string; suitability?: { violations: unknown[] } };
  console.log(denominator, r.kind, JSON.stringify(r.suitability?.violations ?? []).slice(0, 1500));
}
