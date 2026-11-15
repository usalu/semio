import { buildAll, boxAlong, pathSections, treeDecisions, type TreeCfg } from "./author-53-tree.ts";
import { ROUND, add, bestOverlap, boxOf, intervalsAt, lerp, loftSolid, mul, openCtx, reduce, ringsOf, saveLists, sectionAlong, sectionLevel, spanAt, stripe, sub, toLocal, type Ctx, type Json, type Path, type Ring, type Sec, type V3 } from "./author-53-lib.ts";

const GAIN = Number(process.env.GAIN ?? 1);
const GAINV = Number(process.env.GAINV ?? 1.3);
const FR = ["l1", "l2", "l3", "l4", "r1", "r2", "r3", "r4"];
const STEMS = ["root-plate", "stem-1", "stem-2", "stem-3", "crown"];

const thick = (scale: number): number => ({ 50: 0.03, 100: 0.04, 200: 0.05, 500: 0.06 } as Record<number, number>)[scale]!;

export const cfg: TreeCfg = {
  slug: "date-palm",
  stems: STEMS,
  stemFront: STEMS,
  woody: [],
  pairs: [],
  singles: [],
  crown500: "crown",
  bark: { stem: "stem-bark", limb: "stem-bark", stemMat: "palm-stem-bark", limbMat: "palm-stem-bark" },
  barkOf: () => "stem-bark",
  skipShape: (id) => /^(skirt|bunch|fr-|rachis|crown-bud|crown-heart)/.test(id),
  leaf: { sunRegion: "fronds", shadeRegion: "fronds", shade: () => false },
  tone: () => 0.32,
  mat: () => "palm-stem-bark",
  override: { 500: { stems: ["stem-1"], stemFront: ["stem-1"] } },
  scale: { 50: { tolLobe: 0.03, tolLimb: 0.006, stations: 7, bias: 0 }, 100: { tolLobe: 0.04, tolLimb: 0.012, stations: 5, bias: 0 }, 200: { tolLobe: 0.06, tolLimb: 0.024, stations: 4, bias: 0 }, 500: { tolLobe: 0.1, tolLimb: 0.075, stations: 3, bias: 0 } },
  extra: (c: Ctx, built, woodMat: string) => {
    const body: Json[] = [];
    const thermal: Json[] = [];
    const structure: Json[] = [];
    const f = c.feature;
    const rmin = 0.55 * f;
    const rig = c.raw.rig.bones as Json[];
    const n = c.scale === 50 ? 8 : c.scale === 100 ? 7 : 6;
    const ry = thick(c.scale);
    const frondPath = (k: string): { P: (s: number) => V3; T: (s: number) => V3; segs: { id: string; s0: number; s1: number }[] } => {
      const a = c.state.frames[`fr-${k}a`]!;
      const b = c.state.frames[`fr-${k}b`]!;
      const Lb = rig.find((x) => x.id === `fr-${k}b`)!.lengthM as number;
      const P0 = a.o;
      const P1 = b.o;
      const P2 = add(b.o, mul(b.u, Lb));
      const C = sub(mul(P1, 2), mul(add(P0, P2), 0.5));
      const P = (s: number): V3 => add(add(mul(P0, (1 - s) ** 2), mul(C, 2 * s * (1 - s))), mul(P2, s * s));
      const T = (s: number): V3 => add(mul(sub(C, P0), 2 * (1 - s)), mul(sub(P2, C), 2 * s));
      return { P, T, segs: [{ id: `fr-${k}a`, s0: 0, s1: 0.5 }, { id: `fr-${k}b`, s0: 0.5, s1: 1 }] };
    };
    const VIEWS3 = [["elevation", (P: V3): [number, number] => [P[0], P[1]]], ["front", (P: V3): [number, number] => [P[2], P[1]]], ["plan", (P: V3): [number, number] => [P[0], P[2]]]] as const;
    const refined = (segId: string, P: (s: number) => V3, s0: number, s1: number, count: number): Path => {
      const fr = c.state.frames[segId]!;
      const sts = VIEWS3.map(([view, proj]) => {
        const rings = ringsOf(c, view, (q) => q.id === `${segId}-blade`);
        const st = rings.length === 0 ? null : stripe(rings, proj(fr.o), 24);
        if (st === null) return null;
        const mh = st.h.reduce((x, y) => x + y, 0) / st.h.length;
        const len = st.a[st.a.length - 1]! - st.a[0]!;
        return { st, mh, asp: Math.min(1, mh / Math.max(len / 2, 1e-6)) };
      });
      const weight = (i: number): number => (sts[i] === null ? 0 : Math.max(0, 0.85 - sts[i]!.asp));
      const face = sts.map((q, i) => ({ q, i })).filter((x) => x.q !== null).sort((x, y) => y.q!.mh - x.q!.mh)[0];
      const edge = sts.map((q, i) => ({ q, i })).filter((x) => x.q !== null).sort((x, y) => x.q!.mh - y.q!.mh)[0];
      const out: Path = [];
      for (let i = 0; i < count; i++) {
        const t = i / (count - 1);
        const P0 = P(lerp(s0, s1, t));
        const at = sts.map((q) => (q === null ? null : q.st.at(Math.min(0.97, Math.max(0.03, t))).p));
        const mix = (cands: [number, number | undefined][], fallback: number): number => {
          let sw = 0;
          let sv = 0;
          for (const [w0, v] of cands) if (v !== undefined && w0 > 0) { sw += w0; sv += w0 * v; }
          return sw > 0.05 ? sv / sw : fallback;
        };
        const x = mix([[weight(0), at[0]?.[0]], [weight(2), at[2]?.[0]]], P0[0]);
        const y = mix([[weight(0), at[0]?.[1]], [weight(1), at[1]?.[1]]], P0[1]);
        const z = mix([[weight(1), at[1]?.[0]], [weight(2), at[2]?.[1]]], P0[2]);
        const hLat = face === undefined ? 0.45 : sts[face.i]!.st.at(Math.min(0.97, Math.max(0.03, t))).h;
        const hVert = edge === undefined ? 0.03 : sts[edge.i]!.mh;
        out.push({ P: [x, y, z], rLat: Math.max(hLat * GAIN, rmin), rVert: Math.max(Math.min(hVert * GAINV, hLat), rmin) });
      }
      return out;
    };
    const merged = c.scale === 500;
    if (!merged) {
      for (const k of FR) {
        const fp = frondPath(k);
        for (const seg of fp.segs) {
          const path = refined(seg.id, fp.P, seg.s0, seg.s1, n);
          const secs: Sec[] = path.map((q, i) => {
            const a = path[Math.max(0, i - 1)]!.P;
            const b = path[Math.min(path.length - 1, i + 1)]!.P;
            return sectionAlong(c, seg.id, q.P, sub(b, a), q.rLat, q.rVert, 1.6);
          });
          body.push(loftSolid(seg.id, seg.id, "palm-frond", 0.55, secs));
          thermal.push({ id: `th-${seg.id}`, of: seg.id, region: "fronds" });
          const msecs = pathSections(c, seg.id, path, 0.35 * f, (q) => [0.6 * q.rVert, 0.6 * q.rVert], rmin);
          structure.push(loftSolid(`member-${seg.id}`, seg.id, woodMat, 0.4, msecs, { member: seg.id, part: "member" }));
        }
      }
    } else {
      for (const k of FR) {
        const fp = frondPath(k);
        const P0 = fp.P(0);
        const P2 = fp.P(1);
        const armOf = (view: "elevation" | "front" | "plan", proj: (P: V3) => [number, number]): ((t: number) => number) | null => {
          const rings = ringsOf(c, view, (q) => q.id === "crown");
          if (rings.length === 0) return null;
          const o = proj(P0);
          const e = proj(P2);
          const L = Math.hypot(e[0] - o[0], e[1] - o[1]);
          if (L < 0.5) return null;
          const D: [number, number] = [(e[0] - o[0]) / L, (e[1] - o[1]) / L];
          const rot: Ring[] = rings.map((r) => r.map((p) => ({ x: (p.x - o[0]) * D[0] + (p.y - o[1]) * D[1], y: -(p.x - o[0]) * D[1] + (p.y - o[1]) * D[0] })));
          return (t) => {
            const iv = intervalsAt(rot, "x", Math.max(0.02, t * L));
            const hit = iv.find((q) => q[0] <= 0.03 && q[1] >= -0.03) ?? bestOverlap(iv, -0.15, 0.15);
            return hit === null || hit === undefined ? 0 : (hit[1] - hit[0]) / 2;
          };
        };
        const wPlan = armOf("plan", (P) => [P[0], P[2]]);
        const wElev = armOf("elevation", (P) => [P[0], P[1]]);
        const wFront = armOf("front", (P) => [P[2], P[1]]);
        const path: Path = [];
        const m = 8;
        const latCap = wPlan === null ? 0.3 : 1.3 * wPlan(0.3);
        const vertCap = 1.3 * Math.max(wElev === null ? 0 : wElev(0.3), wFront === null ? 0 : wFront(0.3), 0.1);
        for (let i = 0; i < m; i++) {
          const s = (i / (m - 1)) * 0.97;
          const lat = Math.min(wPlan === null ? 0.2 : wPlan(s), latCap);
          const vert = Math.min(Math.max(wElev === null ? 0 : wElev(s), wFront === null ? 0 : wFront(s), 0.1), vertCap);
          path.push({ P: fp.P(s), rLat: Math.max(lat, rmin), rVert: Math.max(vert, rmin) });
        }
        const secs = path.map((q, i) => sectionAlong(c, "crown", q.P, fp.T((i / (m - 1)) * 0.97), q.rLat, q.rVert, 1.6));
        body.push(loftSolid(`frond-${k}`, "crown", "palm-frond", 0.55, secs));
        thermal.push({ id: `th-frond-${k}`, of: `frond-${k}`, region: "fronds" });
        for (const seg of fp.segs) {
          const a = fp.P(seg.s0 + 0.04);
          const b = fp.P(seg.s1 - 0.04);
          structure.push(boxAlong(c, `member-${seg.id}`, seg.id, woodMat, a, b, Math.max(0.5 * ry, rmin), { member: seg.id, part: "member" }));
        }
      }
    }
    const E = (id: string): Ring[] => ringsOf(c, "elevation", (s) => s.id === id);
    const Fr = (id: string): Ring[] => ringsOf(c, "front", (s) => s.id === id);
    for (const side of ["l", "r"]) {
      const id = `bunch-${side}`;
      const e = E(id);
      const fr = Fr(id);
      if (e.length === 0 || fr.length === 0) continue;
      const be = boxOf(e);
      const bf = boxOf(fr);
      const raw: number[][] = [];
      for (let j = 0; j < 30; j++) {
        const tau = (j + 0.5) / 30;
        const xs = spanAt(e, "y", lerp(be.minY, be.maxY, tau));
        const zs = spanAt(fr, "y", lerp(bf.minY, bf.maxY, tau));
        if (xs === null || zs === null) continue;
        raw.push([(lerp(be.minY, be.maxY, tau) + lerp(bf.minY, bf.maxY, tau)) / 2, (xs[0] + xs[1]) / 2, (zs[0] + zs[1]) / 2, Math.max((xs[1] - xs[0]) / 2, rmin), Math.max((zs[1] - zs[0]) / 2, rmin)]);
      }
      if (raw.length < 3) continue;
      const rows = reduce(raw, c.scale === 50 ? 0.03 : 0.06).map((i) => raw[i]!);
      const y0 = (be.minY + bf.minY) / 2;
      const y1 = (be.maxY + bf.maxY) / 2;
      const apex = (r: number[], y: number): number[] => [y, r[1]!, r[2]!, Math.max(r[3]! * 0.7, rmin), Math.max(r[4]! * 0.7, rmin)];
      const all = [apex(rows[0]!, y0), ...rows, apex(rows[rows.length - 1]!, y1)].filter((r, i, a) => i === 0 || r[0]! > a[i - 1]![0]! + 1e-3);
      const secs = all.map((r) => sectionLevel(c, id, [r[1]!, r[0]!, r[2]!], r[3]!, r[4]!, 2.2));
      body.push(loftSolid(id, id, "date-fruit", 0.6, secs));
      thermal.push({ id: `th-${id}`, of: id, region: "fruit" });
      const top = rows[Math.floor(rows.length * 0.25)]!;
      const mid = rows[Math.floor(rows.length * 0.6)]!;
      const rr = Math.max(0.025 * Math.sqrt(c.scale / 50), rmin);
      structure.push({ id: `member-${id}`, kind: "capsule", bone: id, material: woodMat, role: "body", tone: 0.4, p0: toLocal(c, id, [top[1]!, top[0]!, top[2]!]).map(ROUND), p1: toLocal(c, id, [mid[1]!, mid[0]!, mid[2]!]).map(ROUND), r0: rr, r1: Math.max(rr * 0.6, rmin), member: id, part: "member" });
    }
    if (c.scale <= 200) {
      for (const sh of (c.state.elevation as Json[]).filter((s) => /^skirt-/.test(s.shape.id) && s.rings.length > 0)) {
        const st = stripe(sh.rings, [sh.rings[0]![0]!.x, sh.rings[0]![0]!.y], 12);
        if (st === null) continue;
        const raw: number[][] = [];
        for (let i = 0; i < 6; i++) {
          const t = (i + 0.5) / 6;
          const q = st.at(t);
          raw.push([q.p[0], q.p[1], Math.max(q.h, rmin)]);
        }
        const path: Path = raw.map((r) => ({ P: [r[0]!, r[1]!, 0], rLat: r[2]!, rVert: r[2]! }));
        const secs = pathSections(c, sh.shape.bone, path, 0.35 * f, (q) => [q.rLat, q.rVert], rmin);
        body.push(loftSolid(sh.shape.id, sh.shape.bone, "palm-stem-bark", 0.3, secs));
        thermal.push({ id: `th-${sh.shape.id}`, of: sh.shape.id, region: "stem-bark" });
      }
    }
    if (c.scale <= 100) {
      const r = 0.4;
      body.push({ id: "crown-bud", kind: "ellipsoid", bone: "crown", material: "palm-frond", role: "body", tone: 0.5, radii: [r, 0.3, r], transform: { translation: toLocal(c, "crown", [-0.05, 9.05, 0]).map(ROUND) } });
      thermal.push({ id: "th-crown-bud", of: "crown-bud", region: "fronds" });
    }
    return { body, thermal, structure };
  },
};

if (import.meta.main) {
  const scale = Number(process.argv[2]);
  const c = openCtx("vegetation/date-palm", scale);
  const r = buildAll(c, cfg, { exp: 2.0, cross: false, joints: scale === 50, core: 0.75, category: "vegetation", split: false });
  console.log(`body ${r.body.length} thermal ${r.thermal.length} structure ${r.structure.length}`);
  const words = {
    name: { en: "Date palm", de: "Dattelpalme" },
    coarse: {
      50: { en: "The stem is four tapering lofts and a root bulb on the measured centre lines, 16 fronds are curved lofted blades (a quadratic curve through the frame origins of both frond bones, flat superelliptic sections whose half widths are read from the plan outline of each blade), the two fruit bunches are hull lobes read from elevation and front, the four dead frond bases hang from the stem top and the crown bud closes the shaft.", de: "Der Stamm besteht aus vier verjüngten Lofts und einer Wurzelknolle auf den gemessenen Mittellinien, 16 Wedel sind gekrümmte Blatt-Lofts (eine quadratische Kurve durch die Knochenursprünge beider Wedelknochen, flache superelliptische Schnitte mit den Halbbreiten aus der Grundrisskontur jedes Blattes), die beiden Fruchtstände sind Hüllenlappen aus Aufriss und Front, die vier toten Wedelbasen hängen am Stammkopf und die Krone wird von der Knospe geschlossen." },
      100: { en: "The drawn outlines are simpler, so the stem lofts use a larger tolerance, fronds have five sections per segment and slightly thicker blades, bunches and skirts are kept.", de: "Die gezeichneten Konturen sind einfacher, die Stamm-Lofts haben daher eine größere Toleranz, die Wedel fünf Schnitte je Segment und etwas dickere Blätter, Fruchtstände und Wedelbasen bleiben." },
      200: { en: "Fronds keep four sections per segment, the crown bud is merged into the frond bases, skirts stay as stubs.", de: "Die Wedel behalten vier Schnitte je Segment, die Knospe geht in den Wedelbasen auf, die Wedelbasen bleiben als Stümpfe." },
      500: { en: "The 2D draws one crown polygon, so the sixteen frond segments are merged into eight whole-frond lofts of a fixed 0.5 m half width on the crown bone, the stem is one loft, the two bunches stay; dead bases and bud are merged away.", de: "Die 2D zeichnet nur ein Kronenpolygon, die sechzehn Wedelsegmente sind daher zu acht ganzen Wedel-Lofts mit fester Halbbreite 0,5 m am Kronenknochen verschmolzen, der Stamm ist ein Loft, die beiden Fruchtstände bleiben; tote Basen und Knospe sind weggelassen." },
    } as Record<number, { en: string; de: string }>,
    hard: { en: "", de: "" },
    full: true,
  };
  const decisions = treeDecisions(c, r, words);
  saveLists(c, { body: r.body, thermal: r.thermal, structure: r.structure, decisions });
}
