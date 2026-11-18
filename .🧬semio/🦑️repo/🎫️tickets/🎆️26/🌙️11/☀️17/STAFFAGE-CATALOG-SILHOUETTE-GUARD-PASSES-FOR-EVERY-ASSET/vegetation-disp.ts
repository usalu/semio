import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
for (const name of process.argv.slice(2)) {
  const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
  const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
  const out: string[] = [];
  for (const scale of [50, 100, 200, 500]) {
    if ((asset.volumes as any)[String(scale)]?.status !== "authored") continue;
    const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
    for (const view of ["front", "plan"]) {
      let same = 0;
      const moved = new Set<string>();
      for (const s of state.shapes[view] as any[]) {
        if (s.shape.kind !== "poly" || s.shape.solid !== undefined) continue;
        const d = Math.max(...s.shape.points.map((p: number[], i: number) => Math.hypot(p[0]! - s.rings[0][i].x, p[1]! - s.rings[0][i].y)));
        if (d < 1e-6) same++; else moved.add(s.shape.bone);
      }
      out.push(`${scale}/${view}: same ${same} moved ${moved.size}`);
    }
  }
  console.log(name, out.join(" | "));
}
