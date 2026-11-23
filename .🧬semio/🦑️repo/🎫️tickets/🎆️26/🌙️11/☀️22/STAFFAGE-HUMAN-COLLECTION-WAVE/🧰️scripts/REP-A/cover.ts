/** 🎯️ The drawn shapes of one view of the default pose that cover a point (closed rings, even-odd), with their kind, fill and bone.  bun cover.ts <asset.json> <scale> <view> <x> <y> */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [path, sc, view, xs, ys] = process.argv.slice(2) as string[];
const asset = (await Bun.file(path!).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } } as never);
const p = { x: Number(xs), y: Number(ys) };
const inside = (ring: any[]) => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i].y > p.y) !== (ring[j].y > p.y) && p.x < ((ring[j].x - ring[i].x) * (p.y - ring[i].y)) / (ring[j].y - ring[i].y) + ring[i].x) c = !c; return c; };
for (const e of state.shapes[view!]) if (e.rings.reduce((n: number, r: any[]) => n + (inside(r) ? 1 : 0), 0) % 2 === 1) console.log(`${e.shape.id} [${e.shape.bone}] kind ${e.shape.kind} role ${e.shape.role} filled ${e.shape.filled} closed ${e.closed} z ${e.shape.z}${e.shape.solid ? " solid " + e.shape.solid.kind : ""}`);
