import { catalog, authoredScales } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
const [, , ...ids] = process.argv;
for (const a of catalog().filter((x) => ids.includes(x.id))) {
  const w: Record<string, { x: number; y: number; ang: number }> = {};
  const rad = (d: number) => (d * Math.PI) / 180;
  for (const b of a.rig.bones) { const p = b.parent ? w[b.parent] : { x: 0, y: 0, ang: 0 }; const c = Math.cos(rad(p.ang)), s = Math.sin(rad(p.ang)); w[b.id] = { x: p.x + c * b.x - s * b.y, y: p.y + s * b.x + c * b.y, ang: p.ang + b.angleDeg }; }
  const len = Object.fromEntries(a.rig.bones.map((b) => [b.id, b.lengthM]));
  const out: string[] = [];
  for (const d of authoredScales(a)) {
    const r: any = (a.representations as any)[d];
    for (const s of r.geometry.plan) if (s.kind === "capsule" && s.followTip) {
      const o = w[s.bone], L = len[s.bone], tx = o.x + L * Math.cos(rad(o.ang));
      const e0 = Math.abs(s.x0 - o.x), e1 = Math.abs(s.x1 - tx);
      if (e0 > 0.15 * L + 0.02 || e1 > 0.15 * L + 0.02) out.push(`1:${d} plan ${s.id}/${s.bone} x0=${s.x0} (origin ${o.x.toFixed(3)}) x1=${s.x1} (tip ${tx.toFixed(3)}) L=${L}`);
    }
    for (const s of r.geometry.front) if ((s.kind === "rect" || s.kind === "poly") && s.followTip) {
      const o = w[s.bone], L = len[s.bone], ty = o.y + L * Math.sin(rad(o.ang));
      const ys = s.kind === "rect" ? [s.y, s.y + s.h] : s.points.map((p: number[]) => p[1]);
      const lo = Math.min(...ys), hi = Math.max(...ys), a0 = Math.min(o.y, ty), a1 = Math.max(o.y, ty);
      if (Math.abs(a1 - a0) < 0.5 * L) out.push(`1:${d} front ${s.id}/${s.bone} followTip but bone y-span ${(a1 - a0).toFixed(3)} < half L=${L} (ang ${o.ang.toFixed(0)})`);
      else if (Math.abs(lo - a0) > 0.2 * L + 0.02 || Math.abs(hi - a1) > 0.2 * L + 0.02) out.push(`1:${d} front ${s.id}/${s.bone} y [${lo.toFixed(3)},${hi.toFixed(3)}] vs bone [${a0.toFixed(3)},${a1.toFixed(3)}]`);
    }
  }
  console.log(`## ${a.id} ${out.length}`); for (const l of out) console.log("  " + l);
}
