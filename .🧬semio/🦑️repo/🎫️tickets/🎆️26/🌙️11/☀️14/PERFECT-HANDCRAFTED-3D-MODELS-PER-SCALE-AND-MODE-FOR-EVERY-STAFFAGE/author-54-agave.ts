import { loftOf, openCtx, saveLists, selectStations, superellipseSection, featureM, type Json, type Section, type V3 } from "./kit-lib.ts";
import { bladeLoft, profile, spanAt, type Pt } from "./author-54-lib.ts";

const scale = Number(process.argv[2]);
const c = openCtx("vegetation/agave", scale);
const geo = c.raw.representations[String(scale)].geometry;
const F = featureM(scale);
const LEAF = "agave-leaf";
const CAUDEX = "agave-caudex";
const shape = (view: string, id: string): Json | undefined => geo[view].find((s: Json) => s.id === id);
const bones: string[] = c.raw.rig.bones.map((b: Json) => b.id).filter((id: string) => /^leaf-[lr]/.test(id));
const body: Json[] = [];

const caudexE = shape("elevation", "caudex")!.points as Pt[];
const caudexF = shape("front", "caudex")!.points as Pt[];
{
  const secs: Section[] = [];
  const n = scale === 50 ? 5 : 3;
  for (let i = 0; i < n; i++) {
    const u = (0.42 * i) / (n - 1);
    const e = spanAt(caudexE, 0, Math.min(0.4199, Math.max(1e-4, u)))!;
    const f = spanAt(caudexF, 1, Math.min(0.4199, Math.max(1e-4, u)))!;
    secs.push(superellipseSection([u, 0, 0], Math.max((f[1] - f[0]) / 2 * (i === n - 1 ? 0.92 : 1), F), Math.max((e[1] - e[0]) / 2 * (i === n - 1 ? 0.92 : 1), F), 2.4));
  }
  body.push(loftOf({ id: "caudex", bone: "caudex", material: CAUDEX, tone: 0.3 }, secs));
}

const spearE = shape("elevation", "leaf-s")!.points as Pt[];
const spearF = shape("front", "leaf-s")!.points as Pt[];
{
  const pr = profile(spearE, 0, scale === 50 ? 14 : 9, scale === 50 ? 0.09 : 0.14);
  const raw = pr.map((p) => {
    const f = spanAt(spearF, 1, 0.3 + Math.min(1.1499, Math.max(0.0001, p.s)));
    const rx = f === null ? F : Math.max((f[1] - f[0]) / 2, F * 0.55);
    return { s: p.s, c1: (p.lo + p.hi) / 2, c2: 0, rx, ry: Math.max((p.hi - p.lo) / 2, F * 0.55), e: 2.1 };
  });
  const st = selectStations(raw, F * 0.5, F * 0.55);
  body.push(loftOf({ id: "leaf-s", bone: "leaf-s", material: LEAF, tone: 0.35 }, st.map((q) => superellipseSection([q.s, q.c1 ?? 0, 0] as V3, q.rx, q.ry, q.e ?? 2))));
}

for (const id of bones) {
  const sh = shape("elevation", `leaf-${id.slice(5)}`) ?? shape("elevation", id)!;
  const hint = sh.solid as { z0: number; z1: number; zTip: [number, number] };
  const isB = /-b$/.test(id);
  const base = hint.z1;
  const tip = Math.max(hint.zTip[1], scale === 200 ? 0.0155 : 0.003);
  body.push(
    bladeLoft(id, id, { material: LEAF, tone: sh.tone }, sh.points as Pt[], {
      n: 64,
      clip: isB || !(scale === 50 && process.env.CLIP) ? undefined : 0.9,
      win: scale === 50 ? 0.02 : scale === 100 ? 0.06 : 0.1,
      tol: F * 0.3,
      fmin: F * 0.55,
      tmin: scale === 200 ? 0.0155 : 0.003,
      thick: (t) => base + (tip - base) * Math.pow(t, 1.3),
      exp: () => (scale === 200 ? 5 : scale === 100 ? 3.4 : 3.4),
    }),
  );
}
const out = { ids: body.map((s) => s.id) };
console.log(JSON.stringify(out));
saveLists(c, { body });
