/** 🧪 Prints the print-suitability violations of every scale of a candidate asset document (stair flight, round 3). */
import { loadAsset } from "./volumes-lib.ts";
import * as R from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const asset = loadAsset(process.argv[2]!) as never;
const P = R as unknown as Record<string, unknown>;
const profile = (P.DEFAULT_PRINT_PROFILE ?? P.defaultPrintProfile) as never;
for (const scale of [50, 100, 200, 500]) {
  const res = R.resolveRepresentation(asset, { denominator: scale } as never, profile) as unknown as { kind: string; suitability?: { violations: unknown[] }; violations?: unknown[] };
  console.log("[DEBUG]", scale, res.kind, JSON.stringify(res.suitability?.violations ?? res.violations ?? []).slice(0, 1500));
}
