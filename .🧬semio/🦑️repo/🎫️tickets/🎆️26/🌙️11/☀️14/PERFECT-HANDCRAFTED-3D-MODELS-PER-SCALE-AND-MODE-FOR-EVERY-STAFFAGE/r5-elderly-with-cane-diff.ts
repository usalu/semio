/** 🔬 Lists, per scale and list, the solids of the build script of `humans/elderly-with-cane` that differ (any number by more than 0.5 mm) from the source file `volumes/humans/elderly-with-cane.json`, and the solids added or removed.
 *
 * Usage: `bun r5-elderly-with-cane-diff.ts` */
import { readFileSync } from "node:fs";
import { volumes } from "./r3-elderly-with-cane-build.ts";
const built = volumes();
const source = JSON.parse(readFileSync("volumes/humans/elderly-with-cane.json", "utf8")).volumes;
const close = (a: unknown, b: unknown): boolean => {
  if (typeof a === "number" && typeof b === "number") return Math.abs(a - b) <= 5e-4;
  if (Array.isArray(a) && Array.isArray(b)) return a.length === b.length && a.every((x, i) => close(x, b[i]));
  if (a && b && typeof a === "object" && typeof b === "object") { const ka = Object.keys(a), kb = Object.keys(b); return ka.length === kb.length && ka.every((k) => close((a as any)[k], (b as any)[k])); }
  return a === b;
};
for (const scale of Object.keys(built)) for (const list of ["body", "thermal", "structure"]) {
  const old = new Map((source[scale][list] as any[]).map((s) => [s.id, s]));
  const changed: string[] = [];
  for (const s of built[scale][list]) { const o = old.get(s.id); if (!o) changed.push(`+${s.id}`); else if (!close(s, o)) changed.push(s.id); old.delete(s.id); }
  for (const id of old.keys()) changed.push(`-${id}`);
  console.log(scale, list, changed.length, changed.join(" "));
}
