/** 🅿️ Solves the root angle of the parked poses of the city bicycle: the angle at which the front tyre patch and the stand feet are level, so that `ground: auto` puts both on the ground; prints the angle, the lift of the rear wheel and the positions of the contacts.
 *
 * Usage: `bun r3-city-bicycle-park.ts <asset document json>`
 * @see ./r3-tools.md */
import { loadAsset, type Json } from "./volumes-lib.ts";
import { rigAt } from "./r3-lib.ts";

const asset: Json = loadAsset(process.argv[2]!);
const pose = asset.poses.find((p: Json) => p.id === "parked-on-stand");
const at = (id: string): number[] => {
  const a = asset.anchors.find((q: Json) => q.id === id);
  const f = rigAt(asset, 100, { kind: "pose", pose: "parked-on-stand" }).frames[a.bone]!;
  return [0, 1, 2].map((k) => f.o[k]! + a.x * f.u[k]! + a.y * f.v[k]!);
};
let lo = -6;
let hi = 0;
for (let i = 0; i < 50; i++) {
  pose.root.angleDeg = (lo + hi) / 2;
  if (at("contact-f")[1]! - at("stand-foot")[1]! > 0) hi = pose.root.angleDeg;
  else lo = pose.root.angleDeg;
}
const f = rigAt(asset, 100, { kind: "pose", pose: "parked-on-stand" }).frames;
process.stdout.write(`root.angleDeg ${pose.root.angleDeg.toFixed(3)}  contact-f ${at("contact-f").map((v) => v.toFixed(4))}  stand-foot ${at("stand-foot").map((v) => v.toFixed(4))}  contact-r ${at("contact-r").map((v) => v.toFixed(4))}  rear axle y ${f["wheel-r"]!.o[1]!.toFixed(4)}\n`);
