/** 🧲️ Turns the result of a trial tool into ops: `bun adopt.ts <id> <trial.json> [--base file] [--ops ops.json]` compares the trial copy of an asset with the catalog file (or `--base`) and writes, into `ops/<id>.json`, an `add` op for every front, plan or elevation shape that is new or changed (the whole shape, before the shape that follows it in the trial), a `remove` op for every shape that is gone and an `anchor` op for every anchor that moved. Ops of the same shape or anchor already in the file are replaced, others stay. A difference anywhere else in the document is refused. This lets `../MR/fit.ts`, which writes a whole document, feed the byte-keeping `apply.ts`.
 * @see ./apply.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { fileOf } from "../MR/lib.ts";

const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const id = args[0]!;
const base = JSON.parse(readFileSync(flag("base") ?? fileOf(id), "utf8")).asset;
const trial = JSON.parse(readFileSync(args[1]!, "utf8")).asset;
const same = (a: unknown, b: unknown): boolean => JSON.stringify(a) === JSON.stringify(b);
const ops: any[] = [];
const key = (shape: any): string => `${shape.id} ${shape.bone}`;
for (const scale of Object.keys(trial.representations)) {
  const [was, now] = [base.representations[scale]?.geometry, trial.representations[scale].geometry];
  if (now === undefined) continue;
  for (const view of ["elevation", "front", "plan"]) {
    const [before, after]: [any[], any[]] = [was[view], now[view]];
    const known = new Map(before.map((shape) => [key(shape), shape]));
    if (known.size !== before.length || new Set(after.map(key)).size !== after.length) throw new Error(`${id} 1:${scale} ${view}: shapes share an id and a bone`);
    after.forEach((shape, i) => {
      if (same(known.get(key(shape)), shape)) return;
      ops.push({ op: "add", scale, view, ...(known.has(key(shape)) || after[i + 1] === undefined ? {} : { before: after[i + 1].id }), shape });
    });
    const kept = new Set(after.map(key));
    for (const shape of before) if (!kept.has(key(shape))) ops.push({ op: "remove", scale, view, id: shape.id, bone: shape.bone });
  }
}
for (const anchor of trial.anchors) {
  const was = base.anchors.find((candidate: any) => candidate.id === anchor.id);
  if (!same(was, anchor)) ops.push({ op: "anchor", id: anchor.id, set: { bone: anchor.bone, x: anchor.x, y: anchor.y, z: anchor.z ?? 0 } });
}
const strip = (asset: any): any => ({ ...asset, anchors: undefined, representations: Object.fromEntries(Object.entries(asset.representations).map(([scale, r]: [string, any]) => [scale, { ...r, geometry: r.geometry === undefined ? undefined : { ...r.geometry, elevation: undefined, front: undefined, plan: undefined } }])) });
if (!same(strip(base), strip(trial))) throw new Error(`${id}: the trial differs outside the shapes and anchors`);
const file = flag("ops") ?? join(import.meta.dir, "ops", `${id}.json`);
const mine = (op: any): string => (op.op === "anchor" ? `anchor ${op.id}` : `${op.scale} ${op.view} ${op.op === "add" ? key(op.shape) : `${op.id} ${op.bone}`}`);
const taken = new Set(ops.map(mine));
const previous: any[] = existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : [];
const next = [...previous.filter((op) => !taken.has(mine(op))), ...ops];
writeFileSync(file, `[\n${next.map((op) => `  ${JSON.stringify(op)}`).join(",\n")}\n]\n`);
console.log(`${id}: ${ops.length} ops adopted (${ops.filter((op) => op.op === "add").length} shapes, ${ops.filter((op) => op.op === "remove").length} removed, ${ops.filter((op) => op.op === "anchor").length} anchors), ${next.length} ops in ${file}`);
