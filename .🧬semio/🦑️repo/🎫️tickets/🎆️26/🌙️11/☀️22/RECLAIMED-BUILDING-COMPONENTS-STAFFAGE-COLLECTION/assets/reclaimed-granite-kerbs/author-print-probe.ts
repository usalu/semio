/** 🔬️ Prints the print-profile violations of one representation of the candidate per named pose and per view, to find which pose, view and shape a finding comes from. */
import { readFileSync } from "node:fs";
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";

const [scale, requested] = [process.argv[2] ?? "500", Number(process.argv[3] ?? "500")];
const asset = JSON.parse(readFileSync(new URL("./🔣️.json", import.meta.url), "utf8")).asset;
for (const pose of asset.poses)
  for (const view of ["elevation", "front", "plan"]) {
    const rep = asset.representations[scale];
    const geometry = { elevation: [], front: [], plan: [], [view]: rep.geometry[view] };
    const only = { ...asset, poses: asset.poses.map((p: { id: string }) => (p.id === pose.id ? p : { ...p, key: true })), representations: { ...asset.representations, [scale]: { ...rep, geometry } } };
    const s = measureSuitability(only, only.representations[scale], requested, DEFAULT_PRINT_PROFILE);
    console.log(`[DEBUG] ${pose.id} ${view}`, s.violations.map((v) => `${v.check} ${v.shapes.join("+")} ${v.measuredMm.toFixed(3)}`).join("; ") || "ok");
  }
