/** 📏️ The drawn extents of asset documents in the default pose, per view and with the three widest shapes of each lateral range: `bun extent.ts <scale> <file.json>…` (run it from `T`). For a `paper-width` or `paper-height` cell: which shape carries the width of the front and the plan. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const engine = await import(join(S, "🧪️tests/🔭️multiview-coverage/🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [scale, ...files] = process.argv.slice(2);
for (const file of files) {
  const asset = readDoc(file!).doc.asset;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(scale)));
  for (const [view, axis] of [["front", "x"], ["plan", "y"]] as const) {
    const rows = state.shapes[view].filter((entry: any) => entry.shape.role !== "ground").map((entry: any) => {
      const values = entry.rings.flat().map((p: any) => p[axis] as number);
      return { id: entry.shape.id as string, low: Math.min(...values), high: Math.max(...values) };
    }).filter((row: any) => Number.isFinite(row.low));
    const [low, high] = [Math.min(...rows.map((r: any) => r.low)), Math.max(...rows.map((r: any) => r.high))];
    const widest = [...rows].sort((a: any, b: any) => b.high - b.low - (a.high - a.low)).slice(0, 3).map((r: any) => `${r.id} [${r.low.toFixed(3)}, ${r.high.toFixed(3)}]`);
    console.log(`${asset.id} 1:${scale} ${view} lateral [${low.toFixed(3)}, ${high.toFixed(3)}] = ${(high - low).toFixed(3)} m of ${asset.dimensions.depthM}; widest: ${widest.join(", ")}`);
  }
}
