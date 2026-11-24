/** 🪡️ Why a slender solid gets its compared width in a view: the drawn shapes of its bone and material with the strip the guard fits to each (length, width) and the mean distance of the solid's box centre line to it: bun follow.ts <id|file.json> <scale> <view> <solid id> */
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { project, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./people.ts";
const [name, sc, view, sid] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } });
const w = (worldSolids(asset, state.bones as never, Number(sc)) as any[]).find((x) => x.id === sid);
const pts: { x: number; y: number }[] = [];
for (let i = 0; i < w.positions.length; i += 3) pts.push(project(view as never, w.positions[i], w.positions[i + 1], w.positions[i + 2]));
const strip = (points: { x: number; y: number }[]) => {
  const [mx, my] = [points.reduce((t, p) => t + p.x, 0) / points.length, points.reduce((t, p) => t + p.y, 0) / points.length];
  let [sxx, sxy, syy] = [0, 0, 0];
  for (const p of points) [sxx, sxy, syy] = [sxx + (p.x - mx) ** 2, sxy + (p.x - mx) * (p.y - my), syy + (p.y - my) ** 2];
  const angle = 0.5 * Math.atan2(2 * sxy, sxx - syy);
  const [ux, uy] = [Math.cos(angle), Math.sin(angle)];
  let [lo, hi, left, right] = [Infinity, -Infinity, Infinity, -Infinity];
  for (const p of points) { const [t, n] = [(p.x - mx) * ux + (p.y - my) * uy, -(p.x - mx) * uy + (p.y - my) * ux]; [lo, hi, left, right] = [Math.min(lo, t), Math.max(hi, t), Math.min(left, n), Math.max(right, n)]; }
  return { length: hi - lo, width: right - left, angle: (angle * 180) / Math.PI, mx, my };
};
const own = strip(pts);
console.log(`${sid}[${w.bone}] material ${w.material} mesh strip length ${own.length.toFixed(4)} width ${(own.width * 1000).toFixed(2)} mm angle ${own.angle.toFixed(3)}`);
for (const s of state.shapes[view!]) {
  if (s.shape.bone !== w.bone || !["body", "detail", "glazing"].includes(s.shape.role) || (s.shape.material !== undefined && w.material !== undefined && s.shape.material !== w.material)) continue;
  const st = strip(s.rings.flat());
  if (Math.hypot(st.mx - own.mx, st.my - own.my) > 0.3) continue;
  console.log(`   ${s.shape.id} ${s.shape.kind} closed=${s.closed} filled=${s.shape.filled} points ${s.rings.flat().length} strip length ${st.length.toFixed(4)} width ${(st.width * 1000).toFixed(2)} mm angle ${st.angle.toFixed(3)} centre (${st.mx.toFixed(4)}, ${st.my.toFixed(4)})`);
}
