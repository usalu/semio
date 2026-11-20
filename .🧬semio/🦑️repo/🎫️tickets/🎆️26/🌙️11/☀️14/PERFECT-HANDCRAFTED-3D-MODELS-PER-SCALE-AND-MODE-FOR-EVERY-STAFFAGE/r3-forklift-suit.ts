/** 🧾 Prints the print-suitability violations of the forklift drawings at every authored scale, grouped by check and shapes: `bun r3-scissor-lift-suit.ts [asset document json]`. */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";

const asset = process.argv[2] ? JSON.parse(readFileSync(process.argv[2], "utf8")).asset : entryOf("machinery/forklift").asset;
for (const scale of [50, 100, 200, 500]) {
  const resolved = (resolveRepresentation as any)(asset, { denominator: scale }, DEFAULT_PRINT_PROFILE);
  const violations = resolved.suitability?.violations ?? resolved.violations ?? [];
  const groups = new Map<string, number>();
  for (const violation of violations) {
    const key = `${violation.check} ${violation.shapes.join("+")} ${Number(violation.measuredMm).toFixed(3)}<${violation.requiredMm} ${violation.pose ?? ""} ${violation.view ?? ""}`;
    groups.set(key, (groups.get(key) ?? 0) + 1);
  }
  console.log(`1:${scale} ${resolved.kind} violations ${violations.length}`);
  for (const [key, count] of [...groups].slice(0, 25)) console.log(`  ${count}x ${key}`);
  if (scale === 500) for (const v of violations) console.log("[DEBUG]", JSON.stringify(v).slice(0, 600));
}
