import { writeFileSync, readFileSync } from "node:fs";
import { resolve, join } from "node:path";
const root = resolve(import.meta.dir, "../../../../../../../..");
const S = join(root, "🎓️teaching/🏛️architecture/🧍️staffage");
const { solveRig, apply } = await import(join(S, "🔨️modules/🦴️rig/🟦️.ts"));
const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const id = process.argv[2]!;
const file = join(S, "🗂️catalog/🐕️animals", id, "🔣️.json");
const asset = JSON.parse(readFileSync(file, "utf8")).asset;
const rest = solveRig(asset.rig, {}, { x: 0, y: 0, angleDeg: 0 });
const out: any = { rest: {}, pose: {}, bounds: {} };
for (const b of asset.rig.bones) {
  const o = apply(rest[b.id], 0, 0), t = apply(rest[b.id], b.lengthM, 0);
  out.rest[b.id] = [o.x, o.y, t.x, t.y].map((v: number) => Math.round(v * 1e4) / 1e4);
}
const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: asset.presentation.defaultScale } });
for (const [k, b] of Object.entries<any>(st.bones)) out.pose[k] = [b.origin.x, b.origin.y, b.tip.x, b.tip.y].map((v: number) => Math.round(v * 1e4) / 1e4);
out.bounds = st.bounds;
writeFileSync(join(import.meta.dir, "../🗑️generated/A1", `${id}-probe.json`), JSON.stringify(out));
console.log(JSON.stringify(out.bounds), "hoof rest/pose", JSON.stringify(Object.entries(out.rest).filter(([k]) => /hoof|paw|foot/.test(k)).map(([k, v]: any) => [k, v[1], out.pose[k][1]])));
