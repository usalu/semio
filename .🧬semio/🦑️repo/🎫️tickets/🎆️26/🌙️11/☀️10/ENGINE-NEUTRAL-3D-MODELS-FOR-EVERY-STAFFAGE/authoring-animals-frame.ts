import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { loadAsset, resolveAsset } from "./volume-lib.ts";
const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const r = (p: any) => `[${[p.x, p.y, p.z].map((x: number) => Math.round(x * 1000) / 1000)}]`;
const bones = process.argv.slice(4);
const state: any = evaluate(asset, { kind: "pose", pose: process.argv[3]! } as any, { kind: "scale", scale: { denominator: 50 } } as any);
for (const b of bones) { const f = state.bones[b].frame; console.log(b, "o", r(f.o), "u", r(f.u), "v", r(f.v), "w", r(f.w)); }
