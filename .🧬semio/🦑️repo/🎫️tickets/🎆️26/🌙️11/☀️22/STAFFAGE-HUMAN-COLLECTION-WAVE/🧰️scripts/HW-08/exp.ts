import { loadDoc, dropShapes, dropSolids, measureAsset, featureOf } from "./lib.ts";
const a = loadDoc("hiker-with-trekking-poles").asset;
dropShapes(a, (s) => /^pole/.test(s.bone)); dropSolids(a, (s) => /^pole/.test(s.bone ?? "") || /^pole/.test(s.member ?? ""));
const p = a.poses.find((x: any) => x.id === a.presentation.defaultPose);
const base = structuredClone(p.bones);
for (const [hand, fore, abd] of [[8, 62, 4], [8, 62, 6], [8, 70, 6], [8, 55, 6], [4, 62, 6]]) {
  p.bones = structuredClone(base);
  for (const s of ["l", "r"]) { p.bones[`hand-${s}`] = { angleDeg: hand }; p.bones[`forearm-${s}`] = { angleDeg: fore }; p.bones[`arm-${s}`] = { ...p.bones[`arm-${s}`], abductionDeg: s === "r" ? abd : -abd }; }
  const m = measureAsset(a, 50, undefined, undefined, featureOf(50 as never));
  console.log(hand, fore, abd, ["elevation", "front", "plan"].map((v) => `${(m.views as any)[v].iou.toFixed(3)}/${((m.views as any)[v].hausdorffM * 1000).toFixed(0)}`).join(" "));
}
