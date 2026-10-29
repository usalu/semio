import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
// Inserts override keys into clips where interpolated joints sink geometry below the ground: toe bones are bent to stay level, a prismatic hip-lift is raised by the remaining sink.
const [file, out, spec, stepArg] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(file!, "utf8"));
const targets: [string, string][] = JSON.parse(spec!);
const step = Number(stepArg ?? 0.1), LIMIT = -0.0048;
const res: any = {};
const lowest = (st: any) => { let lo = 1e9; for (const e of st.shapes.elevation) { if (e.shape.role === "ground") continue; for (const r of e.rings) for (const p of r) lo = Math.min(lo, p.y); } return lo; };
for (const [aid, cid] of targets) {
  const a = doc.assets.find((x: any) => x.id === aid);
  const c = a.clips.find((x: any) => x.id === cid);
  c.keys = c.keys.filter((k: any) => !(k.bones && Object.keys(k.bones).length > 8));
  const added: any[] = [];
  const snapshot = (t: number) => {
    const st = evaluate(a, { kind: "clip", clip: cid, timeS: t });
    const bones: any = {};
    for (const b of a.rig.bones) {
      if (!st.bones[b.id]) continue;
      const o = st.bones[b.id].offset;
      if (b.joint.type === "revolute") bones[b.id] = { angleDeg: Math.round(o.angleDeg * 1e4) / 1e4 };
      else if (b.joint.type === "prismatic") bones[b.id] = { dx: Math.round(o.dx * 1e4) / 1e4, dy: Math.round(o.dy * 1e4) / 1e4 };
    }
    for (const s of ["l", "r"]) {
      const toe = a.rig.bones.find((b: any) => b.id === `toe-${s}`);
      if (!toe) continue;
      const tb = st.bones[`toe-${s}`], base = bones[`toe-${s}`].angleDeg * Math.PI / 180, alpha = tb.angleRad - base;
      const need = Math.asin(Math.max(-1, Math.min(1, (0.0003 - tb.origin.y) / toe.lengthM))) - alpha;
      const want = Math.max(0, Math.min(toe.joint.maxDeg * Math.PI / 180, need));
      const sinks = tb.origin.y + toe.lengthM * Math.sin(alpha) < 0.0003;
      bones[`toe-${s}`] = { angleDeg: Math.round((sinks ? want : 0) * 180 / Math.PI * 100) / 100 };
    }
    return bones;
  };
  const insert = (t: number, bones: any) => {
    const prev = c.keys.filter((k: any) => k.t <= t).sort((x: any, y: any) => y.t - x.t)[0];
    c.keys.push({ t, pose: prev.pose, ease: "smooth", bones });
    c.keys.sort((x: any, y: any) => x.t - y.t);
  };
  for (let iter = 0; iter < 5; iter++) {
    let changed = false;
    for (let t = 0; t <= c.durationS + 1e-9; t += step) {
      t = Math.round(t * 1000) / 1000;
      const key = c.keys.find((k: any) => Math.abs(k.t - t) < 1e-6);
      const lo = lowest(evaluate(a, { kind: "clip", clip: cid, timeS: t }));
      if (lo >= LIMIT) continue;
      if (key && !(key.bones && Object.keys(key.bones).length > 8)) continue;
      if (key) { key.bones = snapshot(t); const b = key.bones; if (b["hip-lift"]) b["hip-lift"].dy = Math.round((b["hip-lift"].dy - lo) * 1e4) / 1e4; changed = true; continue; }
      const bones = snapshot(t);
      insert(t, bones);
      changed = true;
    }
    for (const k of c.keys) {
      if (!(k.bones && Object.keys(k.bones).length > 8)) continue;
      const lo = lowest(evaluate(a, { kind: "clip", clip: cid, timeS: k.t }));
      if (lo < -0.0005 && k.bones["hip-lift"]) { k.bones["hip-lift"].dy = Math.round((k.bones["hip-lift"].dy - lo) * 1e4) / 1e4; changed = true; }
    }
    if (!changed) break;
  }
  res[aid] = res[aid] ?? {};
  res[aid][cid] = c.keys.filter((k: any) => k.bones && Object.keys(k.bones).length > 8);
}
writeFileSync(out!, JSON.stringify(res));
console.log(JSON.stringify(Object.fromEntries(Object.entries(res).map(([k, v]: any) => [k, Object.fromEntries(Object.entries(v).map(([c, l]: any) => [c, l.map((x: any) => x.t)]))]))));
