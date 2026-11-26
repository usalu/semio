/** 🌳️ Sets the lofts that `search-rowan.sh` saved (`ops/rowan.json`, absolute values of `../MV-L1/loft.ts`) into `rowan` for chosen scales only, idempotent: `bun apply-rowan.ts --scales 100,200 [--in file] [--out file] [--write]`.
 *
 * `loft.ts --ops` removes the depth plates of every scale. The lofts of the 1:50 band do not pass the silhouette guard of `🧊️catalog-silhouette` in the plan (67.2 mm at rest, 15 mm allowed, `multiview-ML-3.md`), so that band keeps its proven state (the depth plates `mvc-*` and sheets `-mvs` of `../MV-3/carry.ts`). For each chosen scale the run removes the plates `mvc-*`, takes the suffix `-mvs` off the sheets and sets every saved loft in the place of the shape of its bone and id (further lofts of a mass after its first), as `applyOps` of the tool does.
 * @see ../MV-L1/loft.ts
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const scales = (flag("scales") ?? "").split(",").filter((key) => key !== "");
if (scales.length === 0) throw new Error("usage: bun apply-rowan.ts --scales 100,200 [--in file] [--out file] [--write]");
const file = flag("in") ?? fileOf("rowan");
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const ops = JSON.parse(readFileSync(join(dirname(Bun.main), "ops", "rowan.json"), "utf8"));
let [plates, sheets, set] = [0, 0, 0];
for (const key of scales) {
  const geometry = next.asset.representations[key].geometry;
  for (const view of ["front", "plan"] as const) {
    const before = geometry[view].length;
    geometry[view] = geometry[view].filter((shape: any) => !String(shape.id).startsWith("mvc-"));
    plates += before - geometry[view].length;
    for (const shape of geometry[view]) if (String(shape.id).endsWith("-mvs")) { shape.id = shape.id.slice(0, -4); sheets++; }
    for (const shape of ops[key]?.[view] ?? []) {
      const at = geometry[view].findIndex((known: any) => known.bone === shape.bone && known.id === shape.id);
      const after = geometry[view].findLastIndex((known: any) => known.bone === shape.bone && known.kind === "loft");
      if (at >= 0) geometry[view][at] = structuredClone(shape);
      else if (after >= 0) geometry[view].splice(after + 1, 0, structuredClone(shape));
      else throw new Error(`rowan 1:${key} ${view}: no shape "${shape.id}" on bone "${shape.bone}" to set`);
      set++;
    }
  }
}
const written = rewrite(text, previous, next);
console.log(`rowan: scales ${scales.join(", ")}: ${plates} depth plates removed, ${sheets} sheet suffixes taken off, ${set} lofts set; ${written === text ? "unchanged" : "changed"}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || (out !== undefined && !existsSync(out))) writeFileSync(out ?? file, written);
}
