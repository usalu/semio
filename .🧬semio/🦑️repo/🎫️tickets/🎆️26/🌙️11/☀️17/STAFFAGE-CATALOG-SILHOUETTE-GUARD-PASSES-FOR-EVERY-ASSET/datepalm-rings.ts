/** 🔬 Evaluated rings of the shapes of one bone: bun datepalm-rings.ts <scale> <view> <shapeId> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [sc, view, id] = process.argv.slice(2) as string[];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === "date-palm")!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } }) as any;
for (const s of state.shapes[view] as any[]) if (s.shape.id === id) { console.log(id, "closed", s.closed); for (const r of s.rings) console.log(r.map((p: any) => `${p.x.toFixed(3)},${p.y.toFixed(3)}`).join(" ")); }
const f = state.bones[process.argv[5] ?? ""]?.frame; if (f) console.log("frame", JSON.stringify(f));
