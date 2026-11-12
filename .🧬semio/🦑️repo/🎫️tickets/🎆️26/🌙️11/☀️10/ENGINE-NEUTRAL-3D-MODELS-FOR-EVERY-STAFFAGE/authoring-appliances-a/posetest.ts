import { STAFFAGE, loadAsset, resolveAsset } from "../volume-lib.ts";
const { evaluate } = await import(`${STAFFAGE}/🔨️modules/🧮️evaluation/🟦️.ts`);
const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const bone = process.argv[3]!;
for (const a of [0, 20, -5, 10]) {
  asset.poses = [...asset.poses, { ...asset.poses[0], id: "t" + a, bones: { [bone]: { angleDeg: a } } }];
  const s = evaluate(asset, { kind: "pose", pose: "t" + a }, { kind: "scale", scale: { denominator: 50 } });
  const f = s.bones[bone].frame;
  console.log(a, JSON.stringify(f.u), JSON.stringify(s.bones[bone].offset), JSON.stringify(s.root));
}
