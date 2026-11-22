/** 📐️ Per drawn shape the extent along one paper axis of a viewpoint at one scale in the default pose: bun shapex.ts <asset.json> <scale> <viewpoint> [x|y] */
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [file, scale, viewpoint, axis] = process.argv.slice(2);
const asset = (await Bun.file(file!).json()).asset;
const st = evaluate(asset, { kind: "pose", pose: process.env.POSE ?? asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scale) }, viewpoint } as never);
const k = axis === "y" ? "y" : "x";
const rows = (st.shapes as Record<string, { shape: { id: string }; rings: { x: number; y: number }[][] }[]>)[viewpoint!]!.map((e) => {
  const v = e.rings.flat().map((p) => p[k]);
  return [e.shape.id, Math.min(...v), Math.max(...v)] as const;
});
rows.sort((a, b) => a[1] - b[1]);
console.log(rows.slice(0, 3).map((r) => `${r[0]} ${r[1].toFixed(3)}`).join(" | "), " ... ", rows.sort((a, b) => b[2] - a[2]).slice(0, 4).map((r) => `${r[0]} ${r[2].toFixed(3)}`).join(" | "));
