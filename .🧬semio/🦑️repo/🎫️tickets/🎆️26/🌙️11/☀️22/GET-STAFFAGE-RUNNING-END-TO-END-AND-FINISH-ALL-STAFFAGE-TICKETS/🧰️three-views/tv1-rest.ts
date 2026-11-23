/** 🦴️ Prints the rest placement of bones in asset coordinates (the frame front and plan shapes are drawn in): `bun tv1-rest.ts <id>|--file <json> [bone…]` gives origin and tip (ahead x, up y, left z) and the axes u, v of every named bone (all without names). */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const staffage = join(resolve(import.meta.dir, "../../../../../../../.."), "🎓️teaching/🏛️architecture/🧍️staffage");
const { catalog } = await import(join(staffage, "🧪️tests/🧊️three-views/🧰️.ts"));
const { restFrames } = await import(join(staffage, "🔨️modules/🦴️rig/🟦️.ts"));
const args = process.argv.slice(2);
const asset = args[0] === "--file" ? JSON.parse(readFileSync(resolve(args[1]!), "utf8")).asset : catalog().find((candidate: { id: string }) => candidate.id === args[0]);
const names = args.slice(args[0] === "--file" ? 2 : 1);
const rest = restFrames(asset.rig) as Record<string, { frame: { o: number[]; u: number[]; v: number[]; w: number[] } }>;
const f = (v: number[]): string => v.map((n) => n.toFixed(4)).join(", ");
for (const bone of asset.rig.bones as { id: string; lengthM: number; parent: string | null }[]) {
  if (names.length > 0 && !names.includes(bone.id)) continue;
  const { o, u, v, w } = rest[bone.id]!.frame;
  console.log(`${bone.id} (parent ${bone.parent}, ${bone.lengthM} m): o [${f(o)}] tip [${f(o.map((c, i) => c + u[i]! * bone.lengthM))}] z ${o[2]!.toFixed(8)} u [${f(u)}] v [${f(v)}] w [${f(w)}]`);
}
