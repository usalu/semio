/** 🔬️ Per-solid world extent in a pose: `bun silhouette-solids.ts <slug> [--pose id] [--scale 50]`. */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { extentOf, worldSolids } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
const a = process.argv.slice(2);
const opt = (k: string) => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : undefined; };
const asset = loadAsset(catalogEntries().find((x) => x.slug === a[0])!.path);
const state = evaluate(asset, { kind: "pose", pose: opt("--pose") ?? asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: Number(opt("--scale") ?? 50) } } as never) as any;
for (const s of worldSolids(asset, state.bones, Number(opt("--scale") ?? 50))) { const e = extentOf([s]); console.log(s.id.padEnd(22), [0, 1, 2].map((k) => `${e.min[k]!.toFixed(3)}..${e.max[k]!.toFixed(3)}`).join("  ")); }
