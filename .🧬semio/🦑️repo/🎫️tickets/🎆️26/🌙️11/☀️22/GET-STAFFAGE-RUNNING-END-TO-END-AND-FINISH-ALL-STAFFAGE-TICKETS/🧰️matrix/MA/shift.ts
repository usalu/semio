/** 📏️ What a trial copy draws elsewhere than its base in the default pose: `bun shift.ts <base.json> <trial.json> [--mm 6] [--scales 50,100]` prints every shape (scale, view, bone, id) whose extents moved by more than the limit, with both extents in mm. Run it from `T`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const engine = await import(join(S, "🧪️tests/🔭️multiview-coverage/🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : undefined);
const limit = Number(flag("mm") ?? 6);
const [a, b] = [readDoc(args[0]!).doc.asset, readDoc(args[1]!).doc.asset];
const box = (shape: any): number[] => {
  const p = shape.rings.flat();
  return [Math.min(...p.map((q: any) => q.x)), Math.max(...p.map((q: any) => q.x)), Math.min(...p.map((q: any) => q.y)), Math.max(...p.map((q: any) => q.y))].map((v) => Math.round(v * 1000));
};
for (const key of Object.keys(a.representations).filter((k) => engine.authoredOf(a, k) !== undefined && (flag("scales") === undefined || flag("scales")!.split(",").includes(k)))) {
  const query = { kind: "pose", pose: a.presentation.defaultPose };
  const [sa, sb] = [evaluate(a, query, engine.scaleSelector(Number(key))), evaluate(b, query, engine.scaleSelector(Number(key)))];
  for (const view of ["elevation", "front", "plan"])
    for (const shape of sa.shapes[view]) {
      const other = sb.shapes[view].find((s: any) => s.shape.id === shape.shape.id && s.shape.bone === shape.shape.bone);
      if (other === undefined) console.log(`1:${key} ${view} ${shape.shape.bone}/${shape.shape.id} gone`);
      else {
        const [p, q] = [box(shape), box(other)];
        const worst = Math.max(...p.map((v, i) => Math.abs(v - q[i]!)));
        if (worst > limit) console.log(`1:${key} ${view} ${shape.shape.bone}/${shape.shape.id} ${worst} mm: x [${p[0]}, ${p[1]}] y [${p[2]}, ${p[3]}] -> x [${q[0]}, ${q[1]}] y [${q[2]}, ${q[3]}]`);
      }
    }
}
