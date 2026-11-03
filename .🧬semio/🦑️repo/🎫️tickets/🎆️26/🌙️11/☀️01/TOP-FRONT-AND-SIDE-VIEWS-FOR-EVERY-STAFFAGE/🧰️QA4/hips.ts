import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const base = JSON.parse(readFileSync("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/jogger/🔣️.json", "utf8")).asset;
const pose0 = base.poses.find((p: any) => p.id === "stretch-stand");
const out: string[] = [];
const run = (ua: number, fa: number, ha: number, uab: number, fab: number) => {
  const a: any = structuredClone(base);
  const p = a.poses.find((q: any) => q.id === "stretch-stand");
  for (const s of ["l", "r"]) {
    const sg = s === "l" ? 1 : -1;
    p.bones[`upper-arm-${s}`] = { angleDeg: ua, abductionDeg: sg * uab };
    p.bones[`forearm-${s}`] = { angleDeg: fa, abductionDeg: -sg * fab };
    p.bones[`hand-${s}`] = { angleDeg: ha };
  }
  const st: any = evaluate(a, { kind: "pose", pose: "stretch-stand" } as any, { kind: "scale", scale: { denominator: 50 } } as any);
  const b = st.bones;
  const hand = st.shapes.plan.find((s: any) => s.shape.id === "hand-l")!.rings.flat();
  const lat = hand.reduce((m: number, q: any) => m + q.y, 0) / hand.length;
  const elb = st.shapes.plan.find((s: any) => s.shape.id === "forearm-l")!.rings.flat();
  const elat = Math.max(...elb.map((q: any) => q.y));
  return { w: b["hand-l"].origin, t: b["hand-l"].tip, e: b["forearm-l"].origin, pel: b["pelvis"].origin, sh: b["upper-arm-l"].origin, lat, elat };
};
const r0 = run(pose0.bones["upper-arm-l"].angleDeg, 105, 10, 28, 22);
out.push("ref " + JSON.stringify(r0));
const res: any[] = [];
for (let ua = -30; ua <= 30; ua += 10) for (let fa = 20; fa <= 130; fa += 10) for (let ha = -40; ha <= 40; ha += 20) for (let uab = 20; uab <= 50; uab += 10) for (let fab = 0; fab <= 70; fab += 10) {
  const r = run(ua, fa, ha, uab, fab);
  const xt = r.pel.x + 0.02, yt = r.pel.y + 0.11;
  const cost = (r.w.x - xt) ** 2 + (r.w.y - yt) ** 2 + (r.lat - 0.2) ** 2 + 4 * Math.max(0, 0.27 - r.elat) ** 2;
  res.push({ ua, fa, ha, uab, fab, cost, w: [r.w.x, r.w.y], lat: r.lat, elat: r.elat, e: [r.e.x, r.e.y], t: [r.t.x, r.t.y] });
}
res.sort((x, y) => x.cost - y.cost);
for (const r of res.slice(0, 12)) out.push(JSON.stringify(r));
writeFileSync(process.env.QA4_LOG!, out.join("\n"));
