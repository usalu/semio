/** 🧭️ Vacuum cleaner rig probe: world origin, tip and axes of every bone in a pose or clip frame, for the catalog asset or a candidate document (`--file`).
 *
 * @see ./r3-lib.ts */
import { readFileSync } from "node:fs";

import { cli, load, queryOf, rigAt } from "./r3-lib.ts";

const { spec, flag } = cli(["--pose", "--clip", "--t", "--file", "--scale"]);
const loaded = load(spec ?? "appliances/vacuum-cleaner", "r3-vacuum-cleaner-probe.ts [asset] [--pose P] [--clip C --t S] [--file doc.json]");
const file = flag("--file");
const asset = file === undefined ? loaded.asset : { ...JSON.parse(readFileSync(file, "utf8")).asset, volumes: loaded.asset.volumes };
const rig = rigAt(asset, Number(flag("--scale") ?? 50), queryOf(asset, flag("--pose"), flag("--clip"), flag("--t")));
const f = (v: readonly number[]) => v.map((x) => x.toFixed(4)).join(",");
for (const b of rig.bones) {
  const fr = rig.frames[b.id];
  console.log(`${b.id.padEnd(8)} o=(${f(b.o)}) tip=(${f(b.tip)}) u=(${f(fr.u)}) v=(${f(fr.v)}) w=(${f(fr.w)})`);
}
