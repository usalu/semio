/** 🔬️ Bone origins and tips in a pose: `bun silhouette-bones.ts <slug> [--pose id] [--only a,b]`. */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalogEntries, loadAsset } from "./volume-lib.ts";
const a = process.argv.slice(2);
const opt = (k: string) => { const i = a.indexOf(k); return i >= 0 ? a[i + 1] : undefined; };
const asset = loadAsset(catalogEntries().find((x) => x.slug === a[0])!.path);
const state = evaluate(asset, { kind: "pose", pose: opt("--pose") ?? asset.presentation.defaultPose } as never, { kind: "scale", scale: { denominator: 50 } } as never) as any;
const only = opt("--only")?.split(",");
const f = (v: any) => `(${v.x.toFixed(3)},${v.y.toFixed(3)},${v.z.toFixed(3)})`;
for (const [id, b] of Object.entries<any>(state.bones)) if (!only || only.includes(id)) console.log(id.padEnd(16), "o", f(b.frame.o), "u", f(b.frame.u), "v", f(b.frame.v), "w", f(b.frame.w), "tip", b.tip ? f(b.tip) : "");
console.log("ground", JSON.stringify(state.ground ?? null), "root", JSON.stringify(state.root ?? null));
