/** 5️⃣ The 1:50 lists of `humans/parent-with-stroller`: body (figure, toddler, stroller), thermal (layer shells nested under the garments, the uncovered skin as its own patches, outlets) and structure (skeletons and the chassis as the load path).
 * @see ./r3-stroller-s50-figure.ts
 * @see ./r3-stroller-s50-pram.ts */
import { adult50, child50 } from "./r3-stroller-s50-figure.ts";
import { pram50 } from "./r3-stroller-s50-pram.ts";
import { skeleton50 } from "./r3-stroller-s50-structure.ts";
import { thermal50 } from "./r3-stroller-s50-thermal.ts";
import { shiftW, type Json } from "./r3-stroller-kit.ts";
import { DECISIONS } from "./r3-stroller-decisions.ts";

export const ZC = 0.1925;
export const strip_ = (s: Json): Json => { const { _r, ...rest } = s; return rest; };
export function body50(): Json[] {
  const p = pram50();
  return [...adult50(), ...child50().map((s) => shiftW(s, ZC)), ...[...p.stroller, ...p.hood, ...p.rear, ...p.front].map((s) => shiftW(s, ZC))];
}
export function scale50(): { body: Json[]; thermal: Json[]; structure: Json[]; decisions: Json[] } {
  const tagged = body50();
  return { body: tagged.map(strip_), thermal: thermal50(tagged), structure: skeleton50(tagged), decisions: DECISIONS["50"]! };
}
