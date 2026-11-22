import { framesOf, solvePose, worldOf, dist, dot } from "./ik.ts";
const a = (await Bun.file("../../🗑️generated/HW-03/doctor.json").json()).asset;
const base = structuredClone(a.poses.find((p: any) => p.id === "standing").bones); Object.assign(base, {"arm-l": {angleDeg: 30, abductionDeg: -10, twistDeg: 0}, "forearm-l": {angleDeg: 90}, "hand-l": {angleDeg: 0}, "clavicle-l": {angleDeg: 0}});
Object.assign(base, { "arm-r": { angleDeg: -13.2 }, "forearm-r": { angleDeg: 117 }, "hand-r": { angleDeg: 22 } });
const f0 = framesOf(a, base);
const n = f0["hand-r"].frame.v;
const P = worldOf(f0, "hand-r", [0.22, 0.02, Number(process.argv[2] ?? 0.17)]);
console.log("target", P.map((x) => x.toFixed(3)), "normal", n.map((x: number) => x.toFixed(3)));
const fields = ["arm-l.angleDeg", "arm-l.abductionDeg", "arm-l.twistDeg", "forearm-l.angleDeg", "hand-l.angleDeg", "clavicle-l.angleDeg"];
const sol = solvePose(a, base, fields, (f) => {
  const tip = worldOf(f, "hand-l", [0.13, 0.0, 0]);
  const e = f["forearm-l"].frame.o;
  const E = [Number(process.argv[3] ?? 0.12), Number(process.argv[4] ?? 1.17), 0.17] as any; const pen = dist(e as any, E) ** 2;
  const fu = f["hand-l"].frame.u;
  return dist(tip, P) ** 2 * 100 + pen * 10 + 0.002 * (1 - Math.abs(dot(fu as any, n as any)));
}, 10);
const f = framesOf(a, sol);
for (const k of ["clavicle-l", "arm-l", "forearm-l", "hand-l"]) console.log(k, JSON.stringify(sol[k]), "o", f[k].frame.o.map((x: number) => x.toFixed(3)));
console.log("tip", worldOf(f, "hand-l", [0.13, 0, 0]).map((x) => x.toFixed(3)));
