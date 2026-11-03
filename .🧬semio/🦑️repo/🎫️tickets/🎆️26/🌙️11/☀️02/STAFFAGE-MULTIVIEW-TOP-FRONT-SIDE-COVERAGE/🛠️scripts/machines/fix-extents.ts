import { load, save, rep, setShape } from "./lib.ts";
{
  const d = load("concrete-mixer-truck");
  for (const k of [50, 100]) for (const v of ["front", "plan"]) for (const s of rep(d, k).geometry[v]) if (s.id.startsWith("mirror-arm-")) s.points = s.points.map(([a, b]: number[]) => v === "front" ? [Math.sign(a) * Math.min(Math.abs(a), 1.275), b] : [a, Math.sign(b) * Math.min(Math.abs(b), 1.275)]);
  save("concrete-mixer-truck", d);
}
for (const [id, targets] of [["delivery-van", [200, 500]], ["city-car", [200, 500]]] as const) {
  const d = load(id);
  const src = rep(d, 100);
  for (const k of targets) for (const v of ["front", "plan"]) for (const s of src.geometry[v]) if (/^mirror-[lr]$/.test(s.id)) setShape(rep(d, k).geometry[v], structuredClone(s));
  save(id, d);
}
