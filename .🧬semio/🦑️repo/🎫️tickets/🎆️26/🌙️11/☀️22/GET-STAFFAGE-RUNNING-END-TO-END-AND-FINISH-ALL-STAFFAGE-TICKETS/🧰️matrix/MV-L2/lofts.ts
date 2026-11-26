/** 🧵️ Sets the foliage lofts that a search with `../MV-L1/loft.ts --save` wrote (absolute values), idempotent: `bun lofts.ts <id|file.json> --ops loft/<id>.json [--scales 100,200] [--skip 100:plan:fol-top,…] [--out file] [--write]`.
 *
 * For every scale of the ops (or of `--scales`) and every view and bone listed there: the depth plates `mvc-*` of the bone in that view go, the saved lofts stand where the body outline with the id of the first one stood (a flat outline with or without the suffix `-mvs`, or a loft), and every other loft of the bone in that view goes. It is `applyOps` of the search tool, kept here so that the chain of the asset does not depend on a tool another round is still changing; a second run changes nothing. A mass of `--skip` (scale:view:bone) keeps its flat outline (it gets a body from `../MV-3/body.ts` instead).
 * @see ../MV-L1/loft.ts — the search and the meaning of the lofts
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const ops = JSON.parse(readFileSync(flag("ops")!, "utf8"));
const scales = flag("scales")?.split(",") ?? Object.keys(ops);
const skip = new Set(flag("skip")?.split(",") ?? []);
const baseId = (id: string): string => (id.endsWith("-mvs") ? id.slice(0, -4) : id);
let set = 0;
for (const key of scales) for (const view of Object.keys(ops[key] ?? {})) {
  const geometry = next.asset.representations[key].geometry;
  const wanted: any[] = ops[key][view];
  for (const bone of new Set<string>(wanted.map((shape) => shape.bone))) {
    if (skip.has(`${key}:${view}:${bone}`)) continue;
    const shapes = wanted.filter((shape) => shape.bone === bone);
    const list: any[] = geometry[view].filter((shape: any) => !(shape.bone === bone && String(shape.id).startsWith("mvc-")));
    const at = list.findIndex((shape) => shape.bone === bone && baseId(String(shape.id)) === shapes[0].id);
    if (at < 0) throw new Error(`${next.asset.id} 1:${key} ${view}: no shape "${shapes[0].id}" on bone "${bone}" to set`);
    const others = (part: any[]): any[] => part.filter((shape) => !(shape.bone === bone && shape.kind === "loft"));
    geometry[view] = [...others(list.slice(0, at)), ...shapes.map((shape) => structuredClone(shape)), ...others(list.slice(at + 1))];
    set += shapes.length;
  }
}
const written = rewrite(text, previous, next);
console.log(`${next.asset.id}: scales ${scales.join(", ")}: ${set} lofts set; ${written === text ? "unchanged" : "changed"}`);
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  if (written !== text || out !== undefined) writeFileSync(out ?? file, written);
}
