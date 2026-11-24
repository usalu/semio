/** 🧷️ The multiview repair of a plant of MV-2, the last step of its chain (after its generator and after `🧰️silhouette/SV-2/apply.ts` or `🧰️silhouette/R4-P/make/<id>.sh`): `bun apply.ts <id> [--in file] [--out file] [--write]` applies `ops/<id>.json` in order and writes only the values that differ into the text of the file (`rewrite` keeps every other byte); the file is read right before it is written and nothing is written when it changed meanwhile. Every op sets absolute values, so a second run changes nothing.
 *
 * Ops:
 * - `{ "op": "anchor", id, "set": { bone, x, y, z } }`: binds an anchor to the bone that draws its place.
 * - `{ "op": "set", scale, view, id, bone?, "set": {…} }`: sets fields of one shape (`null` removes a field).
 * - `{ "op": "remove", scale, view, id, bone? }`: removes a shape.
 * - `{ "op": "add", scale, view, "before"?: id, "shape": {…} }`: adds a shape (replaces the one of the same id and bone) before the named shape, else at the end.
 * @see ./anchors.ts
 * @see ../../🧰️silhouette/R4-P/preserve.ts */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const id = args[0]!;
const file = flag("in") ?? fileOf(id);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const ops = JSON.parse(readFileSync(flag("ops") ?? join(import.meta.dir, "ops", `${id}.json`), "utf8")) as any[];
let refused = 0;
for (const op of ops) {
  const list: any[] | undefined = op.scale === undefined ? undefined : asset.representations[op.scale]?.geometry?.[op.view];
  const index = (shapeId: string, bone?: string): number => list!.findIndex((shape) => shape.id === shapeId && (bone === undefined || shape.bone === bone));
  const refuse = (why: string): void => {
    refused++;
    console.log(`REFUSED ${JSON.stringify(op).slice(0, 160)}: ${why}`);
  };
  if (op.op === "anchor") {
    const anchor = asset.anchors.find((candidate: any) => candidate.id === op.id);
    if (anchor === undefined) refuse("no such anchor");
    else Object.assign(anchor, op.set);
  } else if (list === undefined) refuse("no such view list");
  else if (op.op === "set") {
    const i = index(op.id, op.bone);
    if (i < 0) refuse("no such shape");
    else for (const [key, value] of Object.entries(op.set)) value === null ? delete list[i][key] : (list[i][key] = value);
  } else if (op.op === "remove") {
    const i = index(op.id, op.bone);
    if (i >= 0) list.splice(i, 1);
  } else if (op.op === "add") {
    const i = index(op.shape.id, op.shape.bone);
    if (i >= 0) list[i] = op.shape;
    else {
      const before = op.before === undefined ? -1 : index(op.before);
      list.splice(before < 0 ? list.length : before, 0, op.shape);
    }
  } else refuse("unknown op");
}
const written = rewrite(text, previous, next);
const out = flag("out");
console.log(`${id}: ${ops.length} ops, ${refused} refused, ${written === text ? "nothing changes" : `${text.length} -> ${written.length} characters`}`);
const canonical = (value: unknown): string => JSON.stringify(value, (_, v) => (v !== null && typeof v === "object" && !Array.isArray(v) ? Object.fromEntries(Object.entries(v).sort(([p], [q]) => (p < q ? -1 : 1))) : v));
if (canonical(JSON.parse(written)) !== canonical(next)) throw new Error(`${id}: the rewritten text is not the changed document`);
if (out !== undefined) writeFileSync(out, written);
else if (args.includes("--write") && written !== text) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  writeFileSync(file, written);
  console.log(`${id}: written`);
}
process.exit(refused === 0 ? 0 : 1);
