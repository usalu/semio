/** 🧭️ Round 3 goat: prints the bone frames (origin and tip, world metres) of a pose of the catalog asset, optionally with trial lateral offsets `bone=z`. Usage: bun r3-goat-frames.ts [pose] [bone=z ...] */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf, type Json } from "./volumes-lib.ts";
const asset: Json = JSON.parse(JSON.stringify(entryOf("animals/goat").asset));
const pose = process.argv[2] ?? asset.presentation.defaultPose;
for (const arg of process.argv.slice(3)) { const [id, z] = arg.split("="); asset.rig.bones.find((b: Json) => b.id === id).z = Number(z); }
const state: Json = evaluate(asset as never, { kind: "pose", pose } as never, { kind: "scale", scale: { denominator: 50 } } as never);
for (const b of asset.rig.bones as Json[]) {
  const f = state.bones[b.id].frame;
  console.log(b.id.padEnd(15), [f.o.x, f.o.y, f.o.z].map((v: number) => v.toFixed(4)).join(" "), "->", [f.o.x + f.u.x * b.lengthM, f.o.y + f.u.y * b.lengthM, f.o.z + f.u.z * b.lengthM].map((v: number) => v.toFixed(4)).join(" "));
}
