/** 🧭️ Prints the rest frames of the japanese-maple rig (origin, along-bone and across-bone axes in world metres) for authoring by hand.
 * @see ./r3-vegetation-japanese-maple.md */
import { readFileSync } from "node:fs";
import { entryOf, frames, type Json } from "./volumes-lib.ts";

const file = process.argv[2];
const asset: Json = file ? JSON.parse(readFileSync(file, "utf8")).asset : entryOf("vegetation/japanese-maple").asset;
const F = frames(asset);
const f3 = (v: number[]): string => v.map((x) => x.toFixed(3).padStart(7)).join(" ");
for (const b of asset.rig.bones) {
  const f = F[b.id]!;
  const tip = [0, 1, 2].map((i) => f.o[i]! + f.u[i]! * b.lengthM);
  console.log(`${b.id.padEnd(14)} o ${f3(f.o)}  tip ${f3(tip)}  u ${f3(f.u)}  w ${f3(f.w)}  len ${b.lengthM} z ${b.z ?? 0}`);
}
