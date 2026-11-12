import { STAFFAGE, loadAsset, resolveAsset } from "./volume-lib.ts";
const { evaluate } = await import(`${STAFFAGE}/🔨️modules/🧮️evaluation/🟦️.ts`);
const [slug, poseId] = process.argv.slice(2);
const asset = loadAsset(resolveAsset(slug!).path);
const state = evaluate(asset, { kind: "pose", pose: poseId }, { kind: "scale", scale: { denominator: 50 } });
const arr = (v: any) => (Array.isArray(v) ? v : [v.x, v.y, v.z]);
const r = (v: number[]) => v.map((x) => x.toFixed(3)).join(",");
for (const [b, e] of Object.entries<any>(state.bones)) {
  const f = e.frame, o = arr(f.o), u = arr(f.u), v = arr(f.v), w = arr(f.w);
  console.log(b.padEnd(14), "o", r(o), "u", r(u), "v", r(v), "w", r(w));
}

if (process.argv[4] === "--json") {
  const out: any = {};
  for (const [b, e] of Object.entries<any>(state.bones)) out[b] = { o: arr(e.frame.o), u: arr(e.frame.u), v: arr(e.frame.v), w: arr(e.frame.w) };
  await Bun.write(import.meta.dir + `/🗑️generated/authoring-objects/pose-${slug}-${poseId}.json`, JSON.stringify(out));
}
