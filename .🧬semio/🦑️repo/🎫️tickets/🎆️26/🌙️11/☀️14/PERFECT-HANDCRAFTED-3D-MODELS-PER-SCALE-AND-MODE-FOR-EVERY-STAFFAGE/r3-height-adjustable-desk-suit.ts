/** 🖨️ Round 3, furniture/height-adjustable-desk: prints the print-suitability violations of every authored scale of a candidate or catalog document. Usage: bun r3-height-adjustable-desk-suit.ts <asset document json> */
import { resolve } from "node:path";
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { loadAsset } from "./volumes-lib.ts";

const asset = loadAsset(resolve(process.argv[2]!)) as never;
for (const d of [50, 100, 200, 500]) {
  const r = resolveRepresentation(asset, { denominator: d } as never, DEFAULT_PRINT_PROFILE) as { kind: string; suitability?: { violations: unknown[]; minFeatureMm: number; minGapMm: number } };
  console.log(d, r.kind, JSON.stringify(r.suitability?.violations), r.suitability?.minFeatureMm, r.suitability?.minGapMm);
}
