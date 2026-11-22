import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { loadHuman } from "./kin.ts";
const [slug, pose, scale, view, re] = process.argv.slice(2);
const { asset } = await loadHuman(slug!);
const s = evaluate(asset, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: Number(scale) } });
for (const e of s.shapes[view as "front"]) { if (!new RegExp(re!).test(e.shape.id + "@" + e.shape.bone)) continue; const p = e.rings.flat(); console.log("[DEBUG]", e.shape.id, e.shape.bone, Math.min(...p.map((q: any) => q.x)).toFixed(3), Math.max(...p.map((q: any) => q.x)).toFixed(3), Math.min(...p.map((q: any) => q.y)).toFixed(3), Math.max(...p.map((q: any) => q.y)).toFixed(3)); }
