/** 🖨️ Round 3 mallard: print suitability of the drawings as the product measures it, per scale and print profile (the violations that make a scale `unsuitable`). Usage: bun r3-mallard-duck-print.ts [--catalog] [asset spec] */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";

const spec = process.argv.find((a, i) => i > 1 && !a.startsWith("--")) ?? "animals/mallard-duck";
const asset = process.argv.includes("--catalog") || spec !== "animals/mallard-duck" ? entryOf(spec).asset : JSON.parse(readFileSync(new URL("./🗑️generated/r3-mallard-duck-full/🔣️.json", import.meta.url), "utf8")).asset;
for (const scale of [50, 100, 200]) {
  const r = resolveRepresentation(asset, { denominator: scale } as never, DEFAULT_PRINT_PROFILE as never) as { kind: string; suitability?: { violations: { check: string; shapes: string[]; measuredMm: number; requiredMm: number; view?: string }[] }; violations?: { check: string; shapes: string[]; measuredMm: number; requiredMm: number; view?: string }[] };
  const violations = r.violations ?? r.suitability?.violations ?? [];
  console.log(`1:${scale}: ${r.kind}, ${violations.length} violations${violations.length === 0 && r.kind === "unsuitable" ? ` ${JSON.stringify(r).slice(0, 600)}` : ""}`);
  const seen = new Set<string>();
  for (const v of violations) seen.add(`${v.check} ${v.view ?? ""} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)} < ${v.requiredMm}`);
  for (const k of [...seen].slice(0, 60)) console.log("   ", k);
}
