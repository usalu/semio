/** 📊️ Catalog cross-check: breadths and depths of the 28 humans' own 1:50 drawings, per part, against the kit table scaled by each rig's stature. Usage: `bun figure-audit.ts` */
import { readdirSync } from "node:fs";
import { openCtx, type Json } from "./kit-lib.ts";
import { boneInfos, cutAt, drawnGroups, personOf, rangeOf, tplAt, type PartName } from "./figure-lib.ts";
const root = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans";
const stats: Record<string, { w: number[]; d: number[]; len: number[]; tw: number[]; td: number[]; names: string[] }> = {};
const mean = (x: number[]) => x.reduce((a, b) => a + b, 0) / (x.length || 1);
const sd = (x: number[]) => Math.sqrt(mean(x.map((v) => (v - mean(x)) ** 2)));
for (const slug of readdirSync(root).filter((s) => !s.startsWith("🔣"))) {
  let c;
  try { c = openCtx(`humans/${slug}`, 50); } catch { continue; }
  const infos = boneInfos(c);
  const bones = Object.fromEntries((c.m.rig.bones as Json[]).map((b) => [b.id, b]));
  const S = 0.08 + ["shank", "thigh", "pelvis", "torso", "neck", "head"].reduce((a, k) => a + (bones[k]?.lengthM ?? bones[`${k}-l`]?.lengthM ?? 0), 0);
  const person = personOf({ statureM: S }, bones);
  const groups = drawnGroups(c, infos);
  const regions = Object.fromEntries((c.m.thermal.regions as Json[]).map((r) => [r.id, r]));
  for (const [bone, byRegion] of Object.entries(groups)) {
    const info = infos[bone]!;
    const part = info.part as PartName;
    for (const g of Object.values(byRegion)) {
      const reg = regions[g.region];
      const kind = reg?.kind ?? "none";
      if (kind === "hair") continue;
      const body = { shapes: g.shapes.filter((s) => s.role === "body" && !/(button|pocket|lapel|ear|nose|eye|thumb|finger|cuff|collar|hem|hood|brim|strap|lace|sole|toe)/.test(s.id)) };
      if (body.shapes.length === 0) continue;
      const r = rangeOf(info, body);
      if (r === null) continue;
      let W = 0, D = 0;
      for (let k = 0; k <= 20; k++) {
        const s = r[0] + ((r[1] - r[0]) * k) / 20;
        const cut = cutAt(info, body, Math.min(r[1] - 1e-4, Math.max(r[0] + 1e-4, s)));
        if (cut.w) W = Math.max(W, (cut.w[1] - cut.w[0]) / 2);
        if (cut.v) D = Math.max(D, (cut.v[1] - cut.v[0]) / 2);
      }
      const key = `${part}/${kind === "skin" ? "bare" : "clothed"}`;
      let tw = 0, td = 0;
      for (let k = 0; k <= 20; k++) { const t = tplAt(part, k / 20, person); tw = Math.max(tw, t.w); td = Math.max(td, t.d); }
      const st = (stats[key] ??= { w: [], d: [], len: [], tw: [], td: [], names: [] });
      st.w.push(W); st.d.push(D); st.len.push((r[1] - r[0]) / S); st.tw.push(tw); st.td.push(td); st.names.push(slug);
    }
  }
}
console.log("part/class  n  |  drawn max half breadth / table (mean sd) | half depth / table (mean sd) | length/stature");
for (const [k, s] of Object.entries(stats).sort()) {
  const rw = s.w.map((w, i) => w / s.tw[i]!), rd = s.d.map((d, i) => d / s.td[i]!);
  console.log(`${k.padEnd(16)} ${String(s.w.length).padStart(3)} | ${mean(rw).toFixed(2)} ${sd(rw).toFixed(2)} | ${mean(rd).toFixed(2)} ${sd(rd).toFixed(2)} | ${mean(s.len).toFixed(3)} ${sd(s.len).toFixed(3)}   abs w ${(mean(s.w) * 1000).toFixed(0)} d ${(mean(s.d) * 1000).toFixed(0)} mm`);
}

/* DuBois: surface area of the template figure against DuBois (1916) and the catalog's own BodyPhysiology region sums */
import { TABLE } from "./figure-lib.ts";
const perim = (a: number, b: number, e: number) => { let p = 0, n = 96, px = a, py = 0; for (let i = 1; i <= n; i++) { const t = (2 * Math.PI * i) / n; const c = Math.cos(t), s = Math.sin(t); const x = Math.sign(c) * Math.abs(c) ** (2 / e) * a, y = Math.sign(s) * Math.abs(s) ** (2 / e) * b; p += Math.hypot(x - px, y - py); px = x; py = y; } return p; };
const bonesStd = { pelvis: 0.095, torso: 0.442, neck: 0.095, head: 0.175, arm: 0.3135, forearm: 0.247, hand: 0.1805, thigh: 0.41325, shank: 0.40375, foot: 0.1635, toe: 0.0572 } as const;
const partLen: Record<string, number> = { head: 0.23, neck: 0.095, torso: 0.442, pelvis: 0.095 + 0.135, uarm: 0.3135, farm: 0.247, hand: 0.1805, thigh: 0.41325, shank: 0.40375, foot: 0.1635 + 0.065, toe: 0.0572 };
const count: Record<string, number> = { head: 1, neck: 1, torso: 1, pelvis: 1, uarm: 2, farm: 2, hand: 2, thigh: 2, shank: 2, foot: 2, toe: 2 };
let tot = 0;
const per: Record<string, number> = {};
for (const [part, rows] of Object.entries(TABLE)) {
  const L = partLen[part]!;
  let a = 0;
  for (let i = 0; i + 1 < rows.length; i++) { const r0 = rows[i]!, r1 = rows[i + 1]!; const p0 = perim(r0[1], r0[2], r0[4]), p1 = perim(r1[1], r1[2], r1[4]); a += ((p0 + p1) / 2) * (r1[0] - r0[0]) * L; }
  per[part] = a * count[part]!; tot += a * count[part]!;
}
const dub = 0.007184 * 70 ** 0.425 * 175 ** 0.725;
console.log(`\nkit template (male 1.75 m, no head hair, mitten hands, no torso-neck overlap) total ${tot.toFixed(3)} m2 vs DuBois(70 kg, 175 cm) ${dub.toFixed(3)} m2 (${((tot / dub - 1) * 100).toFixed(0)} %)`);
console.log(Object.entries(per).map(([k, v]) => `${k} ${v.toFixed(3)} (${((v / tot) * 100).toFixed(1)} %)`).join(", "));
console.log("Lund-Browder / Wallace rule-of-nines shares for comparison: head+neck 9 %, trunk 36 %, arms 18 %, hands 5 %, legs 36 % (incl. feet)");
