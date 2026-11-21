/** 🔎 Checks a re-solved arm file of `r5-person-with-rollator-arms.ts`: for every pose and for every clip sampled at 20 frames per second, the distance between the old and the new world position of both hand anchors and the new shoulder breadth.
 *
 * Usage: `bun r5-person-with-rollator-arms-check.ts <category/slug> <anchor of the left hand> <roots.json>`
 * @see ./r5-person-with-rollator-arms.ts */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf, type Json } from "./volumes-lib.ts";
const [spec, anchor, file] = process.argv.slice(2) as [string, string, string];
const old = { ...(entryOf(spec).asset as Json), volumes: undefined } as Json;
const neu = { ...old, ...JSON.parse(readFileSync(file, "utf8")) } as Json;
const at = (a: Json, q: Json, id: string): number[] => { const s = evaluate(a, q as never) as Json; const x = s.anchors[id]; return [x.at.x, x.at.y, x.z ?? 0]; };
const ids = [anchor, anchor.replace(/-l$/, "-r")];
let worst = 0, where = "";
const probe = (q: Json, name: string): void => { for (const id of ids) { const a = at(old, q, id), b = at(neu, q, id); const d = Math.hypot(a[0]! - b[0]!, a[1]! - b[1]!, a[2]! - b[2]!); if (d > worst) { worst = d; where = `${name} ${id}`; } } };
for (const p of neu.poses as Json[]) probe({ kind: "pose", pose: p.id }, `pose ${p.id}`);
for (const c of neu.clips as Json[]) for (let t = 0; t < c.durationS; t += 0.05) probe({ kind: "clip", clip: c.id, timeS: t }, `clip ${c.id} t ${t.toFixed(2)}`);
console.log(`worst hand anchor move ${(worst * 1000).toFixed(2)} mm at ${where}`);
