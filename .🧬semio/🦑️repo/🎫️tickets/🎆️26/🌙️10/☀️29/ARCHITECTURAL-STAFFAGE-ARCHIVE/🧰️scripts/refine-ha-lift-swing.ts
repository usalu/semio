import { readFileSync, writeFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
// Inserts smooth override keys that flex the carpus/hock of a swinging limb until no geometry sinks below the ground line.
const [file, out, spec, limitArg] = process.argv.slice(2);
const doc = JSON.parse(readFileSync(file!, "utf8"));
const targets: [string, string][] = JSON.parse(spec!);
const LIMIT = Number(limitArg ?? -0.004);
const res: any = {};
const suffixOf = (id: string) => (id.startsWith("near-fore") ? "fn" : id.startsWith("far-fore") ? "ff" : id.startsWith("near-hind") ? "hn" : id.startsWith("far-hind") ? "hf" : null);
const lowest = (st: any) => { let lo = 1e9, id = ""; for (const e of st.shapes.elevation) { if (e.shape.role === "ground") continue; for (const r of e.rings) for (const p of r) if (p.y < lo) { lo = p.y; id = e.shape.id; } } return { lo, id }; };
for (const [aid, cid] of targets) {
  const a = doc.assets.find((x: any) => x.id === aid);
  const c = a.clips.find((x: any) => x.id === cid);
  c.keys = c.keys.filter((k: any) => !(k.bones && Object.keys(k.bones).length > 12));
  const snapshot = (t: number) => {
    const st = evaluate(a, { kind: "clip", clip: cid, timeS: t });
    const bones: any = {};
    for (const b of a.rig.bones) { if (!st.bones[b.id]) continue; const o = st.bones[b.id].offset; if (b.joint.type === "revolute") bones[b.id] = { angleDeg: Math.round(o.angleDeg * 1e4) / 1e4 }; else if (b.joint.type === "prismatic") bones[b.id] = { dx: o.dx, dy: o.dy }; }
    return bones;
  };
  const withKey = (t: number, bones: any, fn: () => any) => {
    const prev = c.keys.filter((k: any) => k.t <= t).sort((x: any, y: any) => y.t - x.t)[0];
    const key = { t, pose: prev.pose, ease: "smooth", bones };
    c.keys.push(key); c.keys.sort((x: any, y: any) => x.t - y.t);
    const r = fn();
    return { r, key };
  };
  const ts = [...new Set([80, 40].flatMap((n) => Array.from({ length: n }, (_, i) => Math.round((c.durationS * i / n) * 10000) / 10000)))].sort((x, y) => x - y);
  for (let iter = 0; iter < 4; iter++) {
    let changed = false;
    for (const t of ts) {
      if (c.keys.some((k: any) => Math.abs(k.t - t) < 1e-6 && !(k.bones && Object.keys(k.bones).length > 12))) continue;
      const cur = lowest(evaluate(a, { kind: "clip", clip: cid, timeS: t }));
      if (cur.lo >= LIMIT) continue;
      const suf = suffixOf(cur.id); const bid = suf && `cannon-${suf}`;
      const bone = a.rig.bones.find((b: any) => b.id === bid);
      if (!bone) continue;
      const existing = c.keys.find((k: any) => Math.abs(k.t - t) < 1e-6);
      if (existing) c.keys.splice(c.keys.indexOf(existing), 1);
      const base = snapshot(t);
      let chosen: any = null;
      for (let d = 1; d <= 60 && !chosen; d++) for (const sg of [1, -1]) {
        const ang = base[bid].angleDeg + sg * d;
        if (ang < bone.joint.minDeg || ang > bone.joint.maxDeg) continue;
        const trial = JSON.parse(JSON.stringify(base)); trial[bid].angleDeg = ang;
        const { r, key } = withKey(t, trial, () => lowest(evaluate(a, { kind: "clip", clip: cid, timeS: t })));
        c.keys.splice(c.keys.indexOf(key), 1);
        if (r.lo >= LIMIT) { chosen = trial; break; }
      }
      if (chosen) { withKey(t, chosen, () => 0); changed = true; } else if (existing) { c.keys.push(existing); c.keys.sort((x: any, y: any) => x.t - y.t); }
    }
    if (!changed) break;
  }
  res[aid] = res[aid] ?? {};
  res[aid][cid] = c.keys.filter((k: any) => k.bones && Object.keys(k.bones).length > 12);
}
writeFileSync(out!, JSON.stringify(res));
console.log(JSON.stringify(Object.fromEntries(Object.entries(res).map(([k, v]: any) => [k, Object.fromEntries(Object.entries(v).map(([c, l]: any) => [c, l.map((x: any) => x.t)]))]))));
