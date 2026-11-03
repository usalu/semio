import { load, save } from "./lib.ts";
const id = process.argv[2]!;
const x = load(id); const a = x.asset;
const span = (bone: string): { z0: number; z1: number; zTip?: [number, number] } => {
  const h = (t: number): [number, number] => [-t / 2, t / 2];
  const r = (v: number) => Math.round(v * 1e5) / 1e5;
  if (id === "agave") {
    const [b0, b1] = /-a$/.test(bone) ? [0.04, 0.022] : /-b$/.test(bone) ? [0.022, 0.004] : [0.04, 0.004];
    const [z0, z1] = h(b0), tip = h(b1);
    return { z0: r(z0), z1: r(z1), zTip: [r(tip[0]), r(tip[1])] };
  }
  const t = id === "date-palm" ? 0.002 : 0.0015;
  const [z0, z1] = h(t);
  return { z0: r(z0), z1: r(z1) };
};
let n = 0;
for (const rep of Object.values<any>(a.representations)) {
  if (rep.status !== "authored") continue;
  for (const v of ["elevation", "front", "plan"]) for (const s of rep.geometry[v]) {
    if (s.solid?.kind !== "extruded") continue;
    s.solid = { kind: "extruded", sweep: "v", ...span(s.bone) };
    n++;
  }
}
console.log(id, "extruded set to sweep v:", n);
if (process.argv[3] === "write") save(x);
