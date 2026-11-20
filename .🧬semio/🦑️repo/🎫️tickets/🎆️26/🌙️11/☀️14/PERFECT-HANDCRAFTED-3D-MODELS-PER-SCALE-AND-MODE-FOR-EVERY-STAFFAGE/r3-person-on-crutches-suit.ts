/** 🖨️ Prints the print-suitability violations of the candidate document of `humans/person-on-crutches` at every authored scale under the default print profile.
 * Usage: `bun r3-person-on-crutches-suit.ts` */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { CANDIDATE } from "./r3-person-on-crutches-build.ts";
const asset = JSON.parse(readFileSync(CANDIDATE, "utf8")).asset;
console.log("profile", JSON.stringify(DEFAULT_PRINT_PROFILE));
for (const d of [50, 100, 200, 500]) {
  const r = resolveRepresentation(asset, { denominator: d } as never, DEFAULT_PRINT_PROFILE) as never as { kind: string; suitability?: { violations: unknown[] } };
  console.log(d, r.kind, JSON.stringify(r.suitability?.violations ?? []).slice(0, 1500));
}
