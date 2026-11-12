/** 🔬️ Reproduces the agave polygon-union failure and dumps the failing input.  Usage: bun catalog-agave-union.ts */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import * as poly from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🔺️polygon/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, loadAsset, resolveAsset } from "./volume-lib.ts";
const asset = loadAsset(resolveAsset("agave").path);
for (const d of [500, 200, 100, 50]) for (const p of asset.poses) {
  try { evaluate(asset, { kind: "pose", pose: p.id }, { kind: "scale", scale: { denominator: d } }); }
  catch (e) { console.log(d, p.id, String(e).slice(0, 200)); if ((e as { input?: unknown }).input) writeFileSync(join(GENERATED, "catalog-fix", `agave-union-${d}-${p.id}.json`), JSON.stringify((e as { input: unknown }).input)); }
}
for (const c of asset.clips) for (const t of [0, 0.5 * c.durationS]) for (const d of [50]) {
  try { evaluate(asset, { kind: "clip", clip: c.id, timeS: t }, { kind: "scale", scale: { denominator: d } }); }
  catch (e) { console.log(d, c.id, t, String(e).slice(0, 200)); }
}
console.log("done");
