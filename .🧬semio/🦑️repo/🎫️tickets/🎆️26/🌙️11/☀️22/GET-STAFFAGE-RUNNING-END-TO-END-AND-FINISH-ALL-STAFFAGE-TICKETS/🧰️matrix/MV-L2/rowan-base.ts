/** 🌳️ Sets the 1:100 and 1:200 front and plan of `rowan` back to one flat outline per part, the step of its chain before the lofts, idempotent: `bun rowan-base.ts [file.json] [--out file] [--write]`.
 *
 * The two bands carried the lofts round ML-3 set with the first version of `../MV-L1/loft.ts` (contour of an ellipsoid across Aᵀn instead of A⁻¹n: up to 61 and 73 mm short of the ellipsoids in the front in `gale`, measured with `short.ts`; several lofts per mass). Their front and plan lists are set to the lists of `loft/rowan-base.json` (1:100 as authored before the depth plates of MV-3, 1:200 as authored before the depth-slab redraw of MS, the list of `../ML-3/ops/rowan-200-base.json`); they differ from the lists in the catalog only in the foliage masses (compared shape by shape). The next step (`lofts.ts`) sets the lofts of the corrected tool. The 1:50 band is not touched here (its plates become bodies, `../MV-3/body.ts`).
 * @see ./lofts.ts — the step after this one
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0] !== undefined && !args[0].startsWith("--") ? args[0] : "rowan");
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const base = JSON.parse(readFileSync(join(dirname(Bun.main), "loft", "rowan-base.json"), "utf8"));
for (const view of ["front", "plan"] as const) for (const key of ["100", "200"]) next.asset.representations[key].geometry[view] = structuredClone(base[key][view]);
const written = rewrite(text, previous, next);
console.log(`rowan: flat front and plan at 1:100 and 1:200; ${written === text ? "unchanged" : "changed"}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || out !== undefined) writeFileSync(out ?? file, written);
}
