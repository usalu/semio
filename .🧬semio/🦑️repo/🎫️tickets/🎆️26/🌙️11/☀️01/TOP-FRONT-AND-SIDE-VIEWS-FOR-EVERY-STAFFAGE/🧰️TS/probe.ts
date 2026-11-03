import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const root = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const asset = JSON.parse(readFileSync(`${root}/🧫️fixtures/👤️sample-figure/🔣️.json`, "utf8")).asset;
const rep = asset.representations["100"];
const shapes = (bone: string, id: string, extra: object) => ({ id, kind: "rect", bone, role: "body", z: 1, filled: true, tone: 0.5, ...extra });
const patched = {
  ...asset,
  representations: {
    ...asset.representations,
    "100": { ...rep, geometry: { elevation: rep.geometry.elevation, front: [shapes("pelvis", "p", { x: -0.2, y: 0.8, w: 0.4, h: 0.2 }), shapes("thigh-l", "t", { x: 0, y: 0.5, w: 0.1, h: 0.45, followTip: true })], plan: [shapes("pelvis", "p", { x: 0, y: -0.2, w: 0.1, h: 0.4 })] } },
  },
};
for (const pose of ["stand", "stride-left-forward"]) {
  const state = evaluate(patched, { kind: "pose", pose }, { kind: "scale", scale: { denominator: 100 } });
  console.log(pose, "pelvis", JSON.stringify(state.bones.pelvis!.origin), "thigh-l", JSON.stringify(state.bones["thigh-l"]!.origin), JSON.stringify(state.bones["thigh-l"]!.tip));
  for (const v of ["front", "plan"] as const) for (const s of state.shapes[v]) console.log(v, s.shape.id, JSON.stringify(s.rings[0]!.map((p) => [Math.round(p.x * 1e5) / 1e5, Math.round(p.y * 1e5) / 1e5])));
}
