/** 🔎️ Probe of the round 3 sleeping person: world frame (origin, u, v, w) and tip of every bone in a pose or a clip frame of the catalog asset or of a candidate document.
 * Usage: `bun r3-sleeping-person-probe.ts [pose] [--file <document>] [--clip ID --t S]`
 * @see ./r3-lib.ts */
import { readFileSync } from "node:fs";
import { cli, load, rigAt, queryOf, type Json } from "./r3-lib.ts";
const { flag, spec } = cli(["--file", "--clip", "--t"]);
const file = flag("--file");
const asset: Json = file === undefined ? load("humans/sleeping-person", "probe").asset : JSON.parse(readFileSync(file, "utf8")).asset;
const rig = rigAt(asset, 50, queryOf(asset, spec, flag("--clip"), flag("--t")));
const f3 = (p: readonly number[]): string => p.map((x) => x.toFixed(3).padStart(7)).join(",");
for (const b of rig.bones) {
  const f = rig.frames[b.id]!;
  console.log(b.id.padEnd(12), "o", f3(f.o), "tip", f3(b.tip), "u", f3(f.u), "v", f3(f.v), "w", f3(f.w));
}
