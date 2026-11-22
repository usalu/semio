/** 🔬️ [DEBUG] probe of the arm IK of the winter coat person. */
import { framesOf, readDoc } from "./lib.ts";
const a = (await readDoc("person-in-winter-coat")).asset;
const pt = (f: any, at: number[]) => [0, 1, 2].map((k) => +(f.o[k] + at[0]! * f.u[k] + at[1]! * f.v[k] + at[2]! * f.w[k]).toFixed(3));
for (const pose of a.poses.slice(0, 2)) {
  const F = framesOf(a, pose.bones);
  console.log(pose.id, JSON.stringify(Object.fromEntries(Object.entries(pose.bones).filter(([k]) => /^(arm|forearm|hand)-/.test(k)))));
  console.log(" hand-r palm", pt(F["hand-r"].frame, [0.07, 0, 0]), "mouth", pt(F["head"].frame, [-0.035, -0.08, 0]), "shoulder", pt(F["arm-r"].frame, [0, 0, 0]), "elbow", pt(F["forearm-r"].frame, [0, 0, 0]));
}
for (const ang of [0, 30, 60, 90]) {
  const F = framesOf(a, { "arm-r": { angleDeg: ang } });
  console.log("arm-r angle", ang, "elbow", pt(F["forearm-r"].frame, [0, 0, 0]));
}
for (const ab of [-30, 30]) { const F = framesOf(a, { "arm-r": { abductionDeg: ab } }); console.log("arm-r abd", ab, "elbow", pt(F["forearm-r"].frame, [0, 0, 0])); }
for (const fa of [0, 90]) { const F = framesOf(a, { "forearm-r": { angleDeg: fa } }); console.log("forearm-r", fa, "hand", pt(F["hand-r"].frame, [0, 0, 0])); }
