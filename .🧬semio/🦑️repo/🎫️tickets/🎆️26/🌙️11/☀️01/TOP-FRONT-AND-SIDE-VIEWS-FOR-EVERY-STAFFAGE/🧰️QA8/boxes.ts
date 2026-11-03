import { catalog } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
// usage: boxes.ts <asset> <scale> <shape id>   bounding box (mm on paper) of that shape per pose and viewpoint
const [id, scale, sid] = process.argv.slice(2);
const asset = catalog().find((a: any) => a.id === id)!;
for (const pose of asset.poses.filter((p: any) => !p.key))
  for (const vp of ["elevation", "front", "plan"]) {
    const st: any = evaluate(asset, { kind: "pose", pose: pose.id } as any, { kind: "scale", scale: { denominator: Number(scale) } } as any);
    for (const s of st.shapes[vp].filter((s: any) => s.shape.id === sid)) {
      const xs = s.rings.flat().map((p: any) => p.x), ys = s.rings.flat().map((p: any) => p.y);
      console.log(pose.id, vp, "filled", s.shape.filled, "w", ((Math.max(...xs) - Math.min(...xs)) * 1000 / Number(scale)).toFixed(3), "h", ((Math.max(...ys) - Math.min(...ys)) * 1000 / Number(scale)).toFixed(3));
    }
  }
