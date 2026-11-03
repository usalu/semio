import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
// writes the default-pose bone cache the A2 builders read: <T>/🗑️generated/A2/<asset>-<scale>.json (pose labels only, no clips)
const CAT = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🐕️animals";
const OUT = "C:/git/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/TOP-FRONT-AND-SIDE-VIEWS-FOR-EVERY-STAFFAGE/🗑️generated/A2";
mkdirSync(OUT, { recursive: true });
const num = (v: number) => Math.round(v * 1e4) / 1e4;
for (const id of ["mallard-duck", "pig", "pigeon", "red-fox", "red-squirrel", "roe-deer", "sheep"]) {
  const asset = JSON.parse(readFileSync(`${CAT}/${id}/🔣️.json`, "utf8")).asset;
  for (const [scale, rep] of Object.entries<any>(asset.representations)) {
    if (!rep.geometry) continue;
    const result: any[] = [];
    for (const pose of asset.poses) {
      const st: any = evaluate(asset, { kind: "pose", pose: pose.id } as any, { kind: "scale", scale: { denominator: Number(scale) } } as any);
      const bones: any = {};
      for (const [b, v] of Object.entries<any>(st.bones)) bones[b] = [num(v.origin.x), num(v.origin.y), num(v.tip.x), num(v.tip.y)];
      result.push({ label: pose.id, bones });
    }
    writeFileSync(`${OUT}/${id}-${scale}.json`, JSON.stringify(result));
  }
}
console.log("evals done");
