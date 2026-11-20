/** 🖨️ Prints the print-suitability violations of every scale of an asset document, and with `--split` per pose and viewpoint: `bun r3-courier-suit.ts <asset document json> [--split]`. */
import { readFileSync } from "node:fs";
import { DEFAULT_PRINT_PROFILE, printProfile, resolveRepresentation } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const text = readFileSync(process.argv[2]!, "utf8");
const profile = typeof DEFAULT_PRINT_PROFILE === "string" ? printProfile(DEFAULT_PRINT_PROFILE as never) : DEFAULT_PRINT_PROFILE;
const report = (asset: any, d: number, label: string): void => {
  const r = resolveRepresentation(asset, { denominator: d } as never, profile as never) as any;
  console.log(d, label, r.kind, r.suitability?.minStrokeMm, r.suitability?.minGapMm, r.suitability?.minFeatureMm);
  for (const v of r.suitability?.violations ?? []) console.log("  ", JSON.stringify(v).slice(0, 400));
};
for (const d of [50, 100, 200, 500]) {
  report(JSON.parse(text).asset, d, "all");
  if (!process.argv.includes("--split")) continue;
  for (const pose of JSON.parse(text).asset.poses)
    for (const view of ["elevation", "front", "plan"]) {
      const asset = JSON.parse(text).asset;
      asset.poses = asset.poses.filter((p: any) => p.id === pose.id);
      for (const other of ["elevation", "front", "plan"]) if (other !== view) asset.representations[String(d)].geometry[other] = [];
      try { report(asset, d, `${pose.id} ${view}`); } catch (e) { console.log(d, pose.id, view, "threw", String(e).slice(0, 120)); }
    }
}
