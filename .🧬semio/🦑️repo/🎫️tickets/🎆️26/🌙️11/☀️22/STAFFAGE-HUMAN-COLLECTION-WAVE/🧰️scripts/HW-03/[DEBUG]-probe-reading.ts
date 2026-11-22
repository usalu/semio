import { framesOf, solvePose, worldOf, dist, dot } from "./ik.ts";
const a = (await Bun.file("../../🗑️generated/HW-03/doctor.json").json()).asset;
const base = structuredClone(a.poses.find((p: any) => p.id === "standing").bones);
const target: [number, number, number] = [Number(process.argv[2] ?? 0.34), Number(process.argv[3] ?? 1.36), -0.121];
const ang = (Number(process.argv[4] ?? 38) * Math.PI) / 180;
const dir: [number, number, number] = [Math.cos(ang), Math.sin(ang), 0];
const sol = solvePose(a, base, ["arm-r.angleDeg", "forearm-r.angleDeg", "hand-r.angleDeg"], (f) => {
  const c = worldOf(f, "hand-r", [0.22, 0, 0.1]);
  const u = f["hand-r"]!.frame.u;
  return dist(c, target) ** 2 + 0.05 * (1 - dot(u as any, dir)) ;
});
const f = framesOf(a, sol);
console.log(JSON.stringify({ arm: sol["arm-r"], fore: sol["forearm-r"], hand: sol["hand-r"] }), worldOf(f, "hand-r", [0.22, 0, 0.1]).map((x) => x.toFixed(3)), f["hand-r"].frame.u.map((x: number) => x.toFixed(3)), "elbow", f["forearm-r"].frame.o.map((x: number) => x.toFixed(3)));
