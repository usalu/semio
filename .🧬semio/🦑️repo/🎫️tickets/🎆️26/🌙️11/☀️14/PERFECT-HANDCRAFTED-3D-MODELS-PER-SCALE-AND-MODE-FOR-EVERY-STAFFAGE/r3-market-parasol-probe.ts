/** 🔭️ Prints the world frame, joint, tip and plan azimuth of every bone of the market parasol for a pose or a clip frame, from the catalog asset (with the volumes source) or from a candidate document (`--file`).
 *
 * @see ./r3-objects-market-parasol.md */
import { readFileSync } from "node:fs";

import { cli, load, queryOf, rigAt } from "./r3-lib.ts";

const { spec, flag } = cli(["--pose", "--clip", "--t", "--file"]);
const file = flag("--file");
const asset = file === undefined ? load(spec ?? "objects/market-parasol", "r3-market-parasol-probe.ts [asset] [--pose ID] [--file DOC]").asset : JSON.parse(readFileSync(file, "utf8")).asset;
const rig = rigAt(asset, 50, queryOf(asset, flag("--pose"), flag("--clip"), flag("--t")));
const f = (v: readonly number[]) => v.map((x) => x.toFixed(4)).join(",");
for (const b of rig.bones) {
  const fr = rig.frames[b.id]!;
  const d = [b.tip[0] - b.o[0], b.tip[1] - b.o[1], b.tip[2] - b.o[2]];
  const az = (Math.atan2(-d[2]!, d[0]!) * 180) / Math.PI;
  const el = (Math.atan2(d[1]!, Math.hypot(d[0]!, d[2]!)) * 180) / Math.PI;
  console.log(`${b.id.padEnd(14)} o=${f(b.o)} tip=${f(b.tip)} az=${az.toFixed(2)} el=${el.toFixed(2)} u=${f(fr.u)} v=${f(fr.v)} w=${f(fr.w)}`);
}
