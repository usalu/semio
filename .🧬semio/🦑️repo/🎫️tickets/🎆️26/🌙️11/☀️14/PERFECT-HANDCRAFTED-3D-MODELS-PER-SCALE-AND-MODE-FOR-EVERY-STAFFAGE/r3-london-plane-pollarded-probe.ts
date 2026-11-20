/** 🔎️ Round 3 london plane: prints the default-pose bone frames (origin, tip, axes) in world metres. Usage: bun r3-london-plane-pollarded-probe.ts [asset.json] */
import { readFileSync } from "node:fs";
import { entryOf, poseState } from "./volumes-lib.ts";
const asset = process.argv[2] ? JSON.parse(readFileSync(process.argv[2], "utf8")).asset : entryOf("vegetation/london-plane-pollarded").asset;
const st = poseState(asset, 50);
const r = (v: number[]) => v.map((x) => x.toFixed(3).padStart(7)).join(" ");
for (const b of asset.rig.bones) {
  const f = st.frames[b.id]!;
  console.log(b.id.padEnd(10), "o", r(f.o), "| tip", r(f.o.map((v: number, i: number) => v + f.u[i]! * b.lengthM)), "| u", r(f.u), "| w", r(f.w));
}
