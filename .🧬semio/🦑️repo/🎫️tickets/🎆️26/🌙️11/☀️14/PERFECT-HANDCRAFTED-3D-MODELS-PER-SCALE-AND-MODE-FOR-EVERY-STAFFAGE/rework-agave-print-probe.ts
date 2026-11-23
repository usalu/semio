/** 🔬 Finds the pose and the view in which a shape falls below the printable feature: `bun rework-agave-print-probe.ts <asset document json> <representation> <requested denominator> <shape id>…`. Every pose is measured alone (the others are marked as key poses, which the measurement skips) with one view list at a time. */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [file, scale, requested, ...ids] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(file!, "utf8")).asset;
const representation = asset.representations[scale!];
for (const pose of asset.poses.filter((p: { key?: boolean }) => !p.key)) {
  for (const view of ["elevation", "front", "plan"]) {
    const geometry = { elevation: [], front: [], plan: [], [view]: representation.geometry[view] };
    const drawn = { ...representation, geometry };
    const single = { ...asset, poses: asset.poses.map((p: { id: string }) => (p.id === pose.id ? p : { ...p, key: true })), representations: { ...asset.representations, [scale!]: drawn } };
    const result = measureSuitability(single as never, drawn as never, Number(requested), DEFAULT_PRINT_PROFILE);
    const hits = result.violations.filter((v) => ids.length === 0 || v.shapes.some((s) => ids.includes(s)));
    console.log(`${pose.id} ${view}: min feature ${result.minFeatureMm.toFixed(4)} mm${hits.map((v) => ` | ${v.check} ${v.shapes.join(",")} ${v.measuredMm.toFixed(4)}`).join("")}`);
  }
}
