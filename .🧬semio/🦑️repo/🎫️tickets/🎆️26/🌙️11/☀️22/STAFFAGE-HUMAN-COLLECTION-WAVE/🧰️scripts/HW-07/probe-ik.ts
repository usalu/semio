/** 🧪️ Probe: leg IK of the scaled jogger for a crouched stance, printing the residual per goal. */
import { at, framesOf, loadHuman, scaleAsset, solvePose, type Goal } from "./lib.ts";
const a = (await loadHuman("jogger")).asset;
scaleAsset(a, 1.74 / 1.82);
const pt = (id: string): [number, number, number] => { const x = a.anchors.find((y: any) => y.id === id); return [x.x, x.y, 0]; };
const root = { x: 0, y: Number(process.argv[2] ?? 0.8), angleDeg: -14 };
const goals: Goal[] = [
  { kind: "point", bone: "foot-l", p: pt("heel-l"), target: [-0.05, 0, 0.26], weight: 4 },
  { kind: "point", bone: "foot-l", p: pt("contact-l"), target: [0.11, 0, 0.27], weight: 4 },
  { kind: "axis", bone: "foot-l", axis: "v", dir: [0, 1, 0], weight: 2 },
];
for (const vars of [[["thigh-l", "angleDeg"], ["shank-l", "angleDeg"], ["foot-l", "angleDeg"]], [["thigh-l", "angleDeg"], ["thigh-l", "abductionDeg"], ["thigh-l", "twistDeg"], ["shank-l", "angleDeg"], ["foot-l", "angleDeg"], ["foot-l", "abductionDeg"], ["foot-l", "twistDeg"]]] as any) {
  const res = solvePose(a, { "thigh-l": { angleDeg: 30 }, "shank-l": { angleDeg: -55 }, "foot-l": { angleDeg: 25 } }, vars, goals, { root, iterations: 300, reg: 1e-6 });
  const { frames } = framesOf(a, res.offsets, root);
  console.log(res.error.toFixed(4), JSON.stringify(res.offsets), at(frames["foot-l"]!, pt("heel-l")).map((x) => x.toFixed(3)), at(frames["foot-l"]!, pt("contact-l")).map((x) => x.toFixed(3)), frames["foot-l"]!.v.map((x) => x.toFixed(2)));
}
