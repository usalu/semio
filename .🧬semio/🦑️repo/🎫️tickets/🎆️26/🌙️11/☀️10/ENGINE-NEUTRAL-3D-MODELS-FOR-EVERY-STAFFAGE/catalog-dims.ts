/** 📏️ Drawn extents of the default pose against `dimensions` per view and scale. Usage: bun catalog-dims.ts <slug>... */
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { loadAsset, resolveAsset } from "./volume-lib.ts";
for (const slug of process.argv.slice(2)) {
  const asset = loadAsset(resolveAsset(slug).path);
  const d = asset.dimensions;
  const pose = asset.presentation?.defaultPose ?? asset.poses[0].id;
  const out: string[] = [`${slug} dims w ${d.widthM} h ${d.heightM} d ${d.depthM} | default pose ${pose} (poses: ${asset.poses.map((p: any) => p.id).join(",")})`];
  for (const sc of [50, 100]) {
    try {
      const st = evaluate(asset, { kind: "pose", pose }, { kind: "scale", scale: { denominator: sc } }) as any;
      const ext = (v: string) => {
        const pts = st.shapes[v].filter((s: any) => s.closed && ["body", "detail", "glazing"].includes(s.shape.role)).flatMap((s: any) => s.rings.flat());
        const r = (a: "x" | "y") => (pts.length ? Math.max(...pts.map((p: any) => p[a])) - Math.min(...pts.map((p: any) => p[a])) : 0);
        return [r("x"), r("y")];
      };
      const [ex, ey] = ext("elevation");
      const [fx, fy] = ext("front");
      const [px, py] = ext("plan");
      out.push(`  1:${sc} elevation ${ex.toFixed(3)} x ${ey.toFixed(3)} | front lateral ${fx.toFixed(3)} x ${fy.toFixed(3)} | plan ${px.toFixed(3)} x ${py.toFixed(3)}`);
    } catch (e) { out.push(`  1:${sc} ${String(e).slice(0, 100)}`); }
  }
  console.log(out.join("\n"));
}
