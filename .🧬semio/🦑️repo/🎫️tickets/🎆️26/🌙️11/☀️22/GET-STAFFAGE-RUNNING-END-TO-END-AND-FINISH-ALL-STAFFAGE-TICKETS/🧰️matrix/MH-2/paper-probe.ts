/** 📰️ The paper box of the default pose of one view at the edges of a band, as the matrix measures it: `bun paper-probe.ts <id|file.json> <representation> <view>` prints for the representation's own denominator and both ends of its valid range the paper findings, the box in metres against the dimensions and the three shapes that reach furthest at each end of the first axis (each measured alone with `paperBox`). Run it from `T`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { resolveViews } = await import(join(S, "🔨️modules", "🔭️multiview", "🟦️.ts"));
const [arg, key, view] = process.argv.slice(2);
const asset = readDoc(arg!).doc.asset;
const range = asset.representations[key!].validRange;
for (const denominator of [...new Set([Number(key), range.minDenominator, range.maxDenominator])]) {
  const multi = resolveViews(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", denominator }, "light-line", engine.drawOptions("light-line", denominator));
  const drawn = multi.views[view!];
  if (drawn === undefined) {
    console.log(`${asset.id} 1:${denominator} ${view}: not available`);
    continue;
  }
  const shapes = drawn.state.shapes[view!];
  const k = drawn.scene.metadata.mmPerMetre;
  const box = engine.paperBox(drawn.scene, shapes);
  console.log(`${asset.id} 1:${denominator} ${view}: box ${((box.maxX - box.minX) / k).toFixed(4)} x ${((box.maxY - box.minY) / k).toFixed(4)} m of ${JSON.stringify(asset.dimensions)}; findings ${JSON.stringify(engine.paperFindings(asset, view, drawn.scene, shapes).map((finding: any) => finding.message))}`);
  const rows = shapes.map((evaluated: any) => ({ id: evaluated.shape.id, bone: evaluated.shape.bone, box: engine.paperBox(drawn.scene, [evaluated]) })).filter((row: any) => row.box !== undefined);
  const line = (row: any) => `${row.bone}/${row.id} [${((row.box.minX - box.minX) / k).toFixed(4)}, ${((row.box.maxX - box.minX) / k).toFixed(4)}]`;
  console.log(`  ${rows.length} of ${shapes.length} shapes drawn; low end: ${[...rows].sort((a: any, b: any) => a.box.minX - b.box.minX).slice(0, 3).map(line).join("; ")}`);
  console.log(`  high end: ${[...rows].sort((a: any, b: any) => b.box.maxX - a.box.maxX).slice(0, 3).map(line).join("; ")}`);
}
