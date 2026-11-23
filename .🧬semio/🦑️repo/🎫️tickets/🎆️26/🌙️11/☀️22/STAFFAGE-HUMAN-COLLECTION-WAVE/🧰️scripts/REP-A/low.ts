/** ⬇️ The drawn shapes of one view that reach below a height in a pose (the ground is y = 0), with their lowest point.  bun low.ts <asset.json> <scale> <view> <pose|-> [below=-0.03] */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [path, sc, view, pose, below] = process.argv.slice(2) as string[];
const asset = (await Bun.file(path!).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: pose && pose !== "-" ? pose : asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } } as never);
for (const e of state.shapes[view!]) { const y = Math.min(...e.rings.flat().map((p: any) => p.y)); if (y < Number(below ?? -0.03) && e.shape.role !== "ground") console.log(`${e.shape.id} [${e.shape.bone}] kind ${e.shape.kind}${e.shape.solid ? " solid " + e.shape.solid.kind + " " + JSON.stringify(e.shape.solid) : ""} lowest ${y.toFixed(3)} m`); }
