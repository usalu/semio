import { openCtx, saveLists, stationLoft, featureM, type Json } from "./kit-lib.ts";
import { bladeLoft, pathPoints, type Pt } from "./author-54-lib.ts";

const scale = Number(process.argv[2]);
const c = openCtx("vegetation/hosta-clump", scale);
const geo = c.raw.representations[String(scale)].geometry;
const F = featureM(scale);
const BLADE = "hosta-blade";
const PETIOLE = "hosta-petiole";
const body: Json[] = [];
const THICK = Number(process.env.THICK ?? (scale === 50 ? 0.0045 : 0.008));
const WIN = scale === 50 ? 0.012 : 0.02;

for (const b of c.raw.rig.bones.filter((x: Json) => /^petiole-/.test(x.id))) {
  const sh = geo.elevation.find((s: Json) => s.id === b.id)!;
  const pts = sh.points as Pt[];
  const L = Math.max(...pts.map((p) => p[0]));
  const r0 = Math.max(...pts.filter((p) => p[0] < 1e-6).map((p) => p[1]));
  const r1 = Math.max(...pts.filter((p) => p[0] > L - 1e-6).map((p) => p[1]));
  body.push(
    stationLoft({ id: b.id, bone: b.id, material: PETIOLE, tone: sh.tone }, "u", [
      { s: 0, rx: Math.max(r0 * 1.0, F * 0.55), ry: Math.max(r0, F * 0.55), e: 2 },
      { s: L * 0.5, rx: Math.max((r0 + r1) / 2, F * 0.55), ry: Math.max((r0 + r1) / 2, F * 0.55), e: 2 },
      { s: L, rx: Math.max(r1, F * 0.55), ry: Math.max(r1, F * 0.55), e: 2 },
    ]),
  );
}
for (const b of c.raw.rig.bones.filter((x: Json) => /^blade-/.test(x.id))) {
  const sh = geo.elevation.find((s: Json) => s.id === b.id)!;
  const poly = pathPoints(sh.d, 6);
  body.push(
    bladeLoft(b.id, b.id, { material: BLADE, tone: sh.tone }, poly, {
      n: scale === 50 ? 40 : 28,
      win: WIN,
      tol: F * 0.3,
      fmin: F * 0.55,
      tmin: THICK * 0.6,
      gap: 0.1,
      peak: Number(process.env.PEAK ?? 0.6),
      thick: (t) => THICK * (1 - 0.45 * t),
      exp: () => Number(process.env.EXP ?? 1.5),
    }),
  );
}
console.log(body.length, "solids");
saveLists(c, { body });
