import { load, stateAt, three } from "./lib.ts";
const { asset: a } = load("date-palm"); const pose = a.poses.find((q: any) => q.id === "calm");
let best: any[] = [];
for (let al = -46; al <= -30; al += 2) for (let ph = 66; ph <= 90; ph += 2) {
  pose.bones["fr-l1a"].angleDeg = al; pose.bones["fr-r1a"].angleDeg = -al; pose.bones["fr-l1a"].abductionDeg = ph; pose.bones["fr-r1a"].abductionDeg = -ph;
  const st = stateAt(a, 100); const e = three.extentOf(st.shapes.elevation), f = three.extentOf(st.shapes.front), p = three.extentOf(st.shapes.plan);
  const err = Math.abs(e.maxX - e.minX - 6.3) / 6.3 + Math.abs(f.maxX - f.minX - 7) / 7 + Math.abs(p.maxY - p.minY - 7) / 7 + Math.abs(p.maxX - p.minX - 6.3) / 6.3 + Math.abs(e.maxY - e.minY - 12.5) / 12.5;
  best.push([err, al, ph, (e.maxX - e.minX).toFixed(2), (f.maxX - f.minX).toFixed(2), (p.maxX - p.minX).toFixed(2), (e.maxY - e.minY).toFixed(2)]);
}
best.sort((x, y) => x[0] - y[0]); console.log(best.slice(0, 6).map((r) => r.join(" ")).join("\n"));
