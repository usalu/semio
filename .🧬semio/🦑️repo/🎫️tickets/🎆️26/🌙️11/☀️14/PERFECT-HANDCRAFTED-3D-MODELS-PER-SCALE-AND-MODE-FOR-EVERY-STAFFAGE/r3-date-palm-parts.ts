/** 🌴️ The date palm stated by hand in world space (pose `calm`), per scale: root collar and pencil roots, stem with the spiral of old leaf bases, the fibrous boot under the crown with two whorls of cut petiole stubs, the spear leaves, sixteen arching feather fronds (sheath, spiny petiole, rachis, the two leaflet vanes of the induplicate V, single leaflets as texture), two fruit bunches (stalk, strands, dates) and the transpiration outlets.
 * Real measures: Zaid & de Wet (2002) FAO Plant Production and Protection Paper 156, ch. 3 (botanical and systematic description of the date palm); Barreveld (1993) FAO Agricultural Services Bulletin 101; see the reference table in `r3-vegetation-date-palm.md`.
 * @see ./r3-date-palm-kit.ts
 * @see ./r3-date-palm-rig-fix.ts */
import type { Frame, Json, P2, V3 } from "./volumes-lib.ts";
import { D2R, add, cross, dot, len, mul, project, sub, sweep, table, unit, type Blob, type Section, type Tube } from "./r3-date-palm-kit.ts";
import type { Frond } from "./r3-date-palm-rig-fix.ts";

export type Scale = 50 | 100 | 200 | 500;
export type Rod = { id: string; bone: string; p0: V3; p1: V3; r0: number; r1: number; fields: Json };
export type Plate = { id: string; bone: string; frond: string; ring: P2[]; z0: number; z1: number; zTip: [number, number]; centre: V3 };
export type Stroke = { id: string; bone: string; lines: P2[][]; z0: number; z1: number; zTip: [number, number]; role: "texture" | "detail"; centre: V3 };
export type Repeat = { solid: Json };
export type Parts = {
  tubes: Tube[]; blobs: Blob[]; rods: Rod[]; repeats: Json[];
  plates: Plate[]; strokes: Stroke[];
  regions: Record<string, string>; outlines: Record<string, { bone: string; region?: string; role: string; tone: number; material?: string; parts: string[] }>;
  emitters: { id: string; anchor: string; bone: string; tube: Tube; anchorX: number; anchorY: number }[];
  worldTexture: { id: string; bone: string; kind: "stub" | "date" | "strand"; polys: (view: "elevation" | "front" | "plan") => { lines: P2[][]; depth: number[] } }[];
};

const BARK = { material: "palm-stem-bark", role: "body", tone: 0.32 };
const FROND = { material: "palm-frond", role: "body", tone: 0.52 };
const FRUIT = { material: "date-fruit", role: "body", tone: 0.36 };

/** 📏️ Stem radius (with the persistent leaf bases) at a world height. */
const stemR = (y: number): number => table([[0, 0.36], [0.06, 0.34], [0.14, 0.31], [0.24, 0.285], [0.4, 0.268], [0.8, 0.262], [3.3, 0.25], [6, 0.238], [7.7, 0.23]], y);

/** 🌿️ One frond in world space: the arch of its rachis over its two bones and every derived measure. */
export class FrondGeom {
  readonly L: number;
  readonly half: number;
  constructor(readonly f: Frond, readonly A: Frame, readonly B: Frame) {
    this.L = f.La + f.Lb;
    this.half = ((f.elA - f.elB) / 2) * D2R;
  }
  private seg(S: number): { F: Frame; x: number; Lb: number } {
    return S <= this.f.La ? { F: this.A, x: S, Lb: this.f.La } : { F: this.B, x: S - this.f.La, Lb: this.f.Lb };
  }
  /** 📍️ Centre, tangent, upper normal and lateral axis of the rachis at the distance S from the frond base (negative: the sheath, straight inside the boot). */
  at(S: number, bone?: "a" | "b"): { p: V3; t: V3; n: V3; w: V3; y: number } {
    const h = bone ?? (S <= this.f.La ? "a" : "b");
    const [F, Lb, x] = h === "a" ? [this.A, this.f.La, S] : [this.B, this.f.Lb, S - this.f.La];
    const sag = (Lb / 2) * Math.tan(this.half / 2);
    const xx = h === "a" ? Math.max(0, x) : x;
    const y = (4 * sag * xx * (Lb - xx)) / (Lb * Lb);
    const dy = (4 * sag * (Lb - 2 * xx)) / (Lb * Lb);
    const t = unit(add(F.u, mul(F.v, dy)));
    const n = unit(sub(F.v, mul(F.u, dy)));
    const base = add(add(F.o, mul(F.u, xx)), mul(F.v, y));
    return { p: add(base, mul(t, x - xx)), t, n, w: F.w, y };
  }
  sigma = (S: number): number => S / this.L;
  rw = (S: number): number => table([[-0.06, 0.1], [0, 0.085], [0.04, 0.05], [0.2, 0.03], [0.6, 0.016], [1, 0.005]], this.sigma(S));
  rd = (S: number): number => table([[-0.06, 0.05], [0, 0.04], [0.05, 0.03], [0.2, 0.022], [0.6, 0.013], [1, 0.004]], this.sigma(S));
  leaflet = (S: number): number => (this.L / 3.6) * table([[0.19, 0], [0.2, 0.14], [0.3, 0.3], [0.45, 0.45], [0.6, 0.5], [0.75, 0.44], [0.88, 0.3], [1, 0.08]], this.sigma(S));
  alpha = (S: number): number => table([[0.2, 40], [0.6, 36], [1, 26]], this.sigma(S)) * D2R;
  get beta(): number {
    return ({ A: 16, B: 15, C: 14, D: 12 } as const)[this.f.tier] * D2R;
  }
  /** 🪶️ The outward direction of the vane of one side (+1 left, -1 right) at S. */
  e(S: number, side: number): V3 {
    const g = this.at(S);
    return unit(add(mul(g.w, side * Math.cos(this.beta)), mul(g.n, Math.sin(this.beta))));
  }
  vaneReach = (S: number, share: number): number => this.rw(S) + share * this.leaflet(S) * Math.sin(this.alpha(S));
}

const steps = (from: number, to: number, step: number): number[] => {
  const n = Math.max(1, Math.ceil((to - from) / step));
  return Array.from({ length: n + 1 }, (_, i) => from + ((to - from) * i) / n);
};

/** 🎛️ What each scale shows of the palm. */
const LOD = {
  50: { stemStep: 0.3, frondStep: 0.14, vaneShare: 0.84, vaneThick: 0.0045, leafletStep: 0.04, spines: true, roots: 14, stubStep: 0.032, karab: 9, spears: 2, strands: 11, dates: true, plateSegs: 8, platesStroke: true, merged: false },
  100: { stemStep: 0.6, frondStep: 0.26, vaneShare: 0.84, vaneThick: 0.008, leafletStep: 0.09, spines: false, roots: 8, stubStep: 0.07, karab: 6, spears: 1, strands: 7, dates: false, plateSegs: 7, platesStroke: true, merged: false },
  200: { stemStep: 1.0, frondStep: 0.4, vaneShare: 0.84, vaneThick: 0.016, leafletStep: 0, spines: false, roots: 0, stubStep: 0, karab: 6, spears: 1, strands: 0, dates: false, plateSegs: 6, platesStroke: false, merged: true },
  500: { stemStep: 3.0, frondStep: 0.65, vaneShare: 0.84, vaneThick: 0.04, leafletStep: 0, spines: false, roots: 0, stubStep: 0, karab: 0, spears: 0, strands: 0, dates: false, plateSegs: 5, platesStroke: false, merged: true },
} as const;

const toLocal2 = (F: Frame, p: V3): V3 => {
  const d = sub(p, F.o);
  return [dot(d, F.u), dot(d, F.v), dot(d, F.w)];
};

/** 🌴️ Every part of the palm at one scale. */
export function build(scale: Scale, F: Record<string, Frame>, fronds: Frond[], anchorsOn: string[]): Parts {
  const lod = LOD[scale];
  const feature = (0.15 * scale) / 1000;
  const P: Parts = { tubes: [], blobs: [], rods: [], repeats: [], plates: [], strokes: [], regions: {}, outlines: {}, emitters: [], worldTexture: [] };
  const region = (id: string, r: string) => (P.regions[id] = r);
  const group = (id: string, bone: string, tone: number, role: string, parts: string[], regionId?: string, material?: string) => (P.outlines[id] = { bone, tone, role, parts, ...(regionId ? { region: regionId } : {}), ...(material ? { material } : {}) });

  const root = F["root-plate"]!, s1 = F["stem-1"]!, s2 = F["stem-2"]!, s3 = F["stem-3"]!, crown = F["crown"]!;
  const axisAt = (y: number): V3 => {
    const chain: [Frame, number, number][] = [[root, 0, 0.3], [s1, 0.3, 3.3], [s2, 3.3, 6.0], [s3, 6.0, 8.4], [crown, 8.4, 9.2]];
    const [fr, y0] = chain.find(([, a, b]) => y <= b) ?? chain[chain.length - 1]!;
    return add(fr.o, mul(fr.u, (y - y0) / Math.max(1e-6, fr.u[1])));
  };
  const minMass = Math.max(lod.stemStep, 0.1);

  const stemTube = (id: string, bone: string, fr: Frame, from: number, to: number, ys: number[]): Tube => {
    const pts = ys.map((u) => {
      const p = add(fr.o, mul(fr.u, u));
      return { p, a: stemR(p[1]) * 1.012, b: stemR(p[1]) * 0.988 };
    });
    return { id, bone, sections: sweep(pts, fr.v), fields: { ...BARK } };
  };
  const collarTop = scale === 500 ? -0.3 : 0.08;
  if (scale !== 500) {
    const ys = scale === 200 ? [0, 0.12, 0.3, 0.38] : [0, 0.04, 0.09, 0.15, 0.22, 0.3, 0.38];
    P.tubes.push(stemTube("root-collar", "root-plate", root, 0, 0.38, ys));
    region("root-collar", "stem-bark");
  }
  P.tubes.push(stemTube("stem-1", "stem-1", s1, collarTop, 3.06, steps(scale === 500 ? -0.3 : -0.08, 3.06, lod.stemStep)));
  P.tubes.push(stemTube("stem-2", "stem-2", s2, -0.06, 2.76, steps(-0.06, 2.76, lod.stemStep)));
  P.tubes.push(stemTube("stem-3", "stem-3", s3, -0.06, 1.75, steps(-0.06, 1.75, lod.stemStep / 2)));
  for (const id of ["stem-1", "stem-2", "stem-3"]) region(id, "stem-bark");
  group("root-bulb", "root-plate", 0.3, "body", scale === 500 ? [] : ["root-collar"], "stem-bark", "palm-stem-bark");
  group("stem-1", "stem-1", 0.32, "body", ["stem-1"], "stem-bark", "palm-stem-bark");
  group("stem-2", "stem-2", 0.32, "body", ["stem-2"], "stem-bark", "palm-stem-bark");
  group("stem-3", "stem-3", 0.32, "body", ["stem-3"], "stem-bark", "palm-stem-bark");

  if (lod.roots > 0) {
    const ids: string[] = [];
    for (let i = 0; i < lod.roots; i++) {
      const az = (i * 360) / lod.roots + 9 + (i % 3) * 7;
      const rad: V3 = [Math.cos(az * D2R), 0, -Math.sin(az * D2R)];
      const [y0, reach] = [0.05 + (i % 4) * 0.025, 0.42 + (i % 3) * 0.06];
      const r = scale === 50 ? 0.019 : 0.026;
      const p0 = add(mul(rad, 0.27), [0, y0, 0]);
      const p1 = add(mul(rad, reach), [0, r * 0.75, 0]);
      const id = `root-${i + 1}`;
      P.rods.push({ id, bone: "root-plate", p0, p1, r0: r * 1.25, r1: r * 0.85, fields: { material: "palm-stem-bark", role: "detail", tone: 0.28 } });
      region(id, "stem-bark");
      ids.push(id);
    }
    P.outlines["root-bulb"]!.parts.push(...ids);
  }

  if (lod.stubStep > 0) {
    const [h, depth, wide] = scale === 50 ? [0.12, 0.055, 0.2] : [0.15, 0.065, 0.22];
    for (const [bone, fr, from, to] of [["stem-1", s1, 0.55, 3.0], ["stem-2", s2, 0.0, 2.7], ["stem-3", s3, 0.0, 1.5]] as [string, Frame, number, number][]) {
      const count = Math.floor((to - from) / lod.stubStep);
      const r = stemR(fr.o[1] + from) - 0.03;
      const tilt = 34 * D2R;
      const rot = [0, 0, -Math.sin(tilt / 2), Math.cos(tilt / 2)];
      P.repeats.push({ id: `${bone}-leaf-bases`, kind: "box", bone, material: "palm-stem-bark", role: "texture", tone: 0.26, size: [h, depth, wide], cornerRadius: Math.min(0.015, depth / 2.5), texture: true, transform: { translation: [from, r + depth / 2, 0], rotation: rot.map((x) => Math.round(x * 1e5) / 1e5) }, repeat: { count, rotationAxis: [1, 0, 0], rotationRad: 2.39996, translation: [Math.round(((to - from) / count) * 1e5) / 1e5, 0, 0] } });
      P.worldTexture.push({
        id: `${bone}-leaf-bases`, bone, kind: "stub",
        polys: (view) => {
          const lines: P2[][] = [];
          const depthOut: number[] = [];
          const step = (to - from) / count;
          for (let k = 0; k < count; k++) {
            const ang = k * 2.39996;
            const u = from + k * step;
            const radial = add(mul(fr.v, Math.cos(ang)), mul(fr.w, Math.sin(ang)));
            const tang = cross(fr.u, radial);
            const c = add(add(fr.o, mul(fr.u, u)), mul(radial, r + depth * 0.6));
            const toward = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] }[view] as V3;
            if (dot(radial, toward) < 0.25) continue;
            const pts = [[-1, -1], [-0.2, -1.15], [0.2, -1.15], [1, -1], [1, 1], [-1, 1]].map(([a, b]) => add(add(c, mul(tang, (a! * wide) / 2.3)), mul(fr.u, (b! * h) / 2 + Math.abs(a!) * h * 0.25)));
            const proj = pts.map((p) => project(view, p));
            lines.push([...proj, proj[0]!]);
            depthOut.push(dot(c, toward));
          }
          return { lines, depth: depthOut };
        },
      });
    }
  }

  const bootRows: [number, number][] = scale === 500 ? [[-0.95, 0.25], [-0.4, 0.3], [0.1, 0.34], [0.45, 0.3], [0.75, 0.16], [0.95, 0.06]] : [[-0.85, 0.238], [-0.6, 0.256], [-0.3, 0.29], [0.0, 0.318], [0.25, 0.332], [0.45, 0.312], [0.6, 0.25], [0.7, 0.15], [0.76, 0.05]];
  const boot: Tube = { id: "crown-boot", bone: "crown", sections: sweep(bootRows.map(([u, r]) => ({ p: add(crown.o, mul(crown.u, u)), a: r, b: r * 0.97, n: 2.2 })), crown.v), fields: { material: "palm-stem-bark", role: "body", tone: 0.27 } };
  P.tubes.push(boot);
  region("crown-boot", "stem-bark");
  group("crown-shaft", "crown", 0.27, "body", ["crown-boot"], "stem-bark", "palm-stem-bark");

  if (lod.karab > 0) {
    const whorls = scale === 200 ? [[-0.45, 0]] : scale === 50 ? [[-0.68, 0], [-0.47, 180 / lod.karab], [-0.26, 90 / lod.karab]] : [[-0.6, 0], [-0.33, 180 / lod.karab]];
    whorls.forEach(([u0, phase], wi) => {
      const az = phase!;
      const rad: V3 = [Math.cos(az * D2R), 0, -Math.sin(az * D2R)];
      const radL = toLocal2(crown, add(crown.o, rad));
      const el = (46 + wi * 6) * D2R;
      const dir = add(mul(rad, Math.cos(el)), mul([0, 1, 0], Math.sin(el)));
      const start = add(add(crown.o, mul(crown.u, u0!)), mul(rad, 0.17));
      const lengthM = scale === 50 ? 0.27 : 0.3;
      const [a, b] = scale === 200 ? [0.03, 0.075] : [0.026, 0.075];
      const pts = [0, 0.5, 0.97, 1].map((s, k) => ({ p: add(start, mul(dir, lengthM * s)), a: k === 3 ? a * 0.6 : a * (1.2 - 0.25 * s), b: k === 3 ? b * 0.75 : b * (1.12 - 0.2 * s), n: 3 }));
      const id = `karab-${wi + 1}`;
      const tube: Tube = { id, bone: "crown", sections: sweep(pts, cross(dir, rad)), fields: { material: "palm-stem-bark", role: "detail", tone: 0.24, repeat: { count: lod.karab, rotationAxis: [1, 0, 0], rotationRad: Math.round(((2 * Math.PI) / lod.karab) * 1e5) / 1e5, origin: [0, 0, 0] } } };
      void radL;
      P.tubes.push(tube);
      region(id, "stem-bark");
      P.outlines["crown-shaft"]!.parts.push(id);
    });
  }

  for (let i = 0; i < lod.spears; i++) {
    const az = 40 + i * 150;
    const lean = (4 + i * 3) * D2R;
    const dir: V3 = [Math.sin(lean) * Math.cos(az * D2R), Math.cos(lean), -Math.sin(lean) * Math.sin(az * D2R)];
    const start = add(crown.o, add(mul(crown.u, 0.5), [0.03 * Math.cos(az * D2R), 0, -0.03 * Math.sin(az * D2R)]));
    const L = 1.25 + i * 0.3;
    const pts = steps(0, 1, scale === 50 ? 0.125 : 0.34).map((s) => ({ p: add(start, mul(dir, L * s)), a: Math.max(feature * 0.55, 0.034 * (1 - s) + 0.004), b: Math.max(feature * 0.6, 0.05 * (1 - 0.85 * s)) }));
    const id = `spear-${i + 1}`;
    P.tubes.push({ id, bone: "crown", sections: sweep(pts, [Math.cos(az * D2R), 0, -Math.sin(az * D2R)]), fields: { ...FROND, tone: 0.6 } });
    region(id, "fronds");
    group(`spear-${i + 1}`, "crown", 0.6, "body", [id], "fronds", "palm-frond");
  }

  for (const f of fronds) {
    const g = new FrondGeom(f, F[`${f.id}a`]!, F[`${f.id}b`]!);
    const L = g.L;
    const ranges: Record<"a" | "b", [number, number]> = { a: [-0.15, f.La + 0.05], b: [f.La - 0.05, L + (lod.merged ? 0.07 : 0)] };
    for (const half of ["a", "b"] as const) {
      const bone = `${f.id}${half}`;
      const [s0, s1b] = ranges[half];
      const S = steps(s0, s1b, lod.frondStep);
      const geo = S.map((s) => ({ s, ...g.at(s, half) }));
      if (!lod.merged) {
        const rachis: Tube = { id: `${bone}-rachis`, bone, sections: geo.map((q) => ({ p: q.p, x: q.n, y: q.w, a: Math.max(g.rd(q.s), scale === 50 ? 0.0045 : 0.008), b: Math.max(g.rw(q.s), scale === 50 ? 0.005 : 0.009) })), fields: { ...FROND, tone: 0.6 } };
        P.tubes.push(rachis);
        region(rachis.id, "fronds");
        const parts = [rachis.id];
        for (const side of [1, -1]) {
          const from = Math.max(s0, 0.2 * L);
          if (from >= s1b - 0.05) continue;
          const VS = steps(from, s1b, lod.frondStep);
          const sec: Section[] = VS.map((s) => {
            const q = g.at(s, half);
            const e = unit(add(mul(q.w, side * Math.cos(g.beta)), mul(q.n, Math.sin(g.beta))));
            const reach = Math.max(g.vaneReach(s, lod.vaneShare), 0.03);
            const y = cross(q.t, e);
            return { p: add(q.p, mul(e, reach / 2)), x: e, y, a: reach / 2, b: lod.vaneThick, n: 3 };
          });
          const id = `${bone}-vane-${side > 0 ? "l" : "r"}`;
          P.tubes.push({ id, bone, sections: sec, fields: { ...FROND } });
          region(id, "fronds");
          parts.push(id);
        }
        group(`${bone}-blade`, bone, 0.52, "body", parts, "fronds", "palm-frond");
      } else {
        const sec: Section[] = S.map((s) => {
          const q = g.at(s, half);
          const reach = g.vaneReach(s, lod.vaneShare);
          const across = Math.max(reach * Math.cos(g.beta), g.rw(s), feature * 0.6);
          const lift = Math.max(0, reach * Math.sin(g.beta) * 0);
          return { p: add(q.p, mul(q.n, lift)), x: q.w, y: mul(q.n, -1), a: across, b: Math.max(lod.vaneThick, feature * 0.55), n: 4 };
        }).map((s) => ({ ...s, y: cross(s.x, cross(s.y, s.x)) }));
        const blade: Tube = { id: `${bone}-blade`, bone, sections: sec.map((s, i) => ({ ...s, y: unit(cross(unit(sub(sec[Math.min(sec.length - 1, i + 1)]!.p, sec[Math.max(0, i - 1)]!.p)), s.x)) })), fields: { ...FROND } };
        P.tubes.push(blade);
        region(blade.id, "fronds");
        group(`${bone}-blade`, bone, 0.52, "body", [blade.id], "fronds", "palm-frond");
      }

      const Lbone = half === "a" ? f.La : f.Lb;
      const segs = steps(s0, s1b, (s1b - s0) / lod.plateSegs);
      for (let i = 0; i + 1 < segs.length; i++) {
        const [a0, a1] = [segs[i]! - (i > 0 ? 0.01 : 0), segs[i + 1]! + (i + 2 < segs.length ? 0.01 : 0)];
        const xs = steps(a0, a1, 0.06);
        const loc = (p: V3) => toLocal2(F[bone]!, p);
        const top: P2[] = [], bottom: P2[] = [];
        const vs: { x: number; lo: number; hi: number }[] = [];
        for (const s of xs) {
          const q = g.at(s, half);
          const reach = g.vaneReach(s, lod.vaneShare);
          const hasVane = s >= 0.2 * L;
          const edgeL = hasVane ? loc(add(q.p, mul(g.e(s, 1), reach))) : loc(add(q.p, mul(q.w, g.rw(s))));
          const edgeR = hasVane ? loc(add(q.p, mul(g.e(s, -1), reach))) : loc(add(q.p, mul(q.w, -g.rw(s))));
          const c = loc(q.p);
          const halfW = Math.max(Math.abs(edgeL[2]), g.rw(s), feature * 0.5);
          top.push({ x: c[0], y: Math.max(edgeL[2], halfW) });
          bottom.push({ x: c[0], y: Math.min(edgeR[2], -halfW) });
          const rise = hasVane ? reach * Math.sin(g.beta) : g.rd(s);
          const thick = Math.max(lod.vaneThick, feature * 0.55);
          const lift = Math.max(0, reach * Math.sin(g.beta) * 0);
          vs.push(lod.merged ? { x: c[0], lo: c[1] + lift - thick, hi: c[1] + lift + thick } : { x: c[0], lo: c[1] - g.rd(s), hi: c[1] + Math.max(g.rd(s), rise * 0.8 + lod.vaneThick) });
        }
        const [p0, p1] = [vs[0]!, vs[vs.length - 1]!];
        const m = ((p1.lo + p1.hi) / 2 - (p0.lo + p0.hi) / 2) / Math.max(1e-6, p1.x - p0.x);
        const cc = (p0.lo + p0.hi) / 2 - m * p0.x;
        const lo = Math.min(...vs.map((v) => v.lo - (m * v.x + cc)));
        const hi = Math.max(...vs.map((v) => v.hi - (m * v.x + cc)), lo + feature * 0.6);
        const ring = [...top, ...bottom.reverse()];
        const mid = g.at((a0 + a1) / 2, half).p;
        P.plates.push({ id: `${bone}-blade${lod.plateSegs > 1 ? `-${i + 1}` : ""}`, bone, frond: f.id, ring, z0: cc + lo, z1: cc + hi, zTip: [cc + m * Lbone + lo, cc + m * Lbone + hi], centre: mid });
      }

      if (lod.leafletStep > 0) {
        const from = Math.max(s0 + (half === "b" ? 0.05 : 0), 0.2 * L);
        const to = half === "a" ? f.La : L - 0.04;
        if (from < to - 0.05) {
          const groupLen = scale === 50 ? 0.24 : 0.45;
          const groups = steps(from, to, groupLen);
          const lineSets: { lines: P2[][]; v: number[] }[] = [];
          for (let gi = 0; gi + 1 < groups.length; gi++) {
            const [ga, gb] = [groups[gi]!, groups[gi + 1]!];
            const count = Math.max(1, Math.floor((gb - ga) / lod.leafletStep));
            const stepS = (gb - ga) / count;
            const sm = ga + (gb - ga) / 2;
            const ell = g.leaflet(sm) * (scale === 50 ? 1 : 0.97);
            const al = g.alpha(sm);
            const wl = Math.min(0.016, Math.max(0.009, ell * 0.035)) * (scale === 50 ? 1 : 1.5);
            for (const side of [1, -1]) {
              const q0 = g.at(ga + stepS / 2, half);
              const q1 = g.at(ga + stepS / 2 + stepS * (count - 1), half);
              const e = g.e(ga + stepS / 2, side);
              const d = unit(add(mul(q0.t, Math.cos(al)), mul(e, Math.sin(al))));
              const nv = unit(cross(q0.t, e));
              const yv = cross(nv, d);
              const base = add(q0.p, mul(e, g.rw(ga)));
              const Fb = F[bone]!;
              const prof: [number, number][] = [[0, -wl * 0.35], [ell * 0.22, -wl], [ell * 0.7, -wl * 0.6], [ell, 0], [ell * 0.7, wl * 0.6], [ell * 0.22, wl], [0, wl * 0.35]];
              const stepV = count > 1 ? mul(sub(q1.p, q0.p), 1 / (count - 1)) : [0, 0, 0] as V3;
              const stepL = [dot(stepV, Fb.u), dot(stepV, Fb.v), dot(stepV, Fb.w)].map((x) => Math.round(x * 1e5) / 1e5);
              const xL = [dot(d, Fb.u), dot(d, Fb.v), dot(d, Fb.w)] as V3, yL = [dot(yv, Fb.u), dot(yv, Fb.v), dot(yv, Fb.w)] as V3, zL = [dot(nv, Fb.u), dot(nv, Fb.v), dot(nv, Fb.w)] as V3;
              const { quaternion } = KIT;
              P.repeats.push({ id: `${bone}-leaflets-${side > 0 ? "l" : "r"}${gi + 1}`, kind: "extrude", bone, material: "palm-frond", role: "texture", tone: 0.44, texture: true, plane: "xy", profile: prof.map(([a, b]) => [Math.round(a * 1e5) / 1e5, Math.round(b * 1e5) / 1e5]), from: -0.0035 * (scale === 50 ? 1 : 1.6), to: 0.0035 * (scale === 50 ? 1 : 1.6), transform: { translation: toLocal2(Fb, base).map((x) => Math.round(x * 1e5) / 1e5), rotation: quaternion(xL, yL, zL) }, ...(count > 1 ? { repeat: { count, translation: stepL } } : {}) });
              const lines: P2[][] = [];
              const vv: number[] = [];
              for (let k = 0; k < count; k++) {
                const b0 = add(base, mul(stepV, k));
                const tip = add(b0, mul(d, ell));
                const [lb, lt] = [toLocal2(Fb, b0), toLocal2(Fb, tip)];
                lines.push([{ x: lb[0], y: lb[2] }, { x: lt[0], y: lt[2] }]);
                vv.push((lb[1] + lt[1]) / 2);
              }
              lineSets.push({ lines, v: vv });
            }
          }
          if (lod.platesStroke) {
            const all = lineSets.flatMap((l) => l.lines);
            const vv = lineSets.flatMap((l) => l.v);
            const xs2 = all.map((l) => l[0]!.x);
            const n = xs2.length;
            const mx = xs2.reduce((a, b) => a + b, 0) / n, my = vv.reduce((a, b) => a + b, 0) / n;
            const sxx = xs2.reduce((a, x) => a + (x - mx) ** 2, 0), sxy = xs2.reduce((a, x, i) => a + (x - mx) * (vv[i]! - my), 0);
            const m = sxx > 1e-9 ? sxy / sxx : 0;
            const c = my - m * mx;
            P.strokes.push({ id: `${bone}-leaflets`, bone, lines: all, z0: c, z1: c + 0.002, zTip: [c + m * Lbone, c + m * Lbone + 0.002], role: "texture", centre: g.at((from + to) / 2, half).p });
          }
        }
      }

      if (lod.spines && half === "a") {
        const lines: P2[][] = [];
        for (const side of [1, -1]) for (const [ga, gb, sl] of [[0.06, 0.13, 0.07], [0.13, 0.195, 0.12]] as [number, number, number][]) {
          const [sa, sb] = [ga * L, gb * L];
          const count = Math.max(2, Math.round((sb - sa) / 0.055));
          const stepS = (sb - sa) / count;
          const q0 = g.at(sa + stepS / 2, "a"), q1 = g.at(sa + stepS / 2 + stepS * (count - 1), "a");
          const e = unit(add(mul(q0.w, side * Math.cos(26 * D2R)), mul(q0.n, Math.sin(26 * D2R))));
          const d = unit(add(mul(q0.t, Math.cos(36 * D2R)), mul(e, Math.sin(36 * D2R))));
          const nv = unit(cross(q0.t, e));
          const yv = cross(nv, d);
          const base = add(q0.p, mul(e, g.rw(sa) * 0.9));
          const Fb = F[bone]!;
          const stepV = mul(sub(q1.p, q0.p), 1 / (count - 1));
          const prof: [number, number][] = [[0, -0.006], [sl * 0.3, -0.004], [sl, 0], [sl * 0.3, 0.004], [0, 0.006]];
          P.repeats.push({ id: `${bone}-spines-${side > 0 ? "l" : "r"}${ga < 0.1 ? 1 : 2}`, kind: "extrude", bone, material: "palm-frond", role: "texture", tone: 0.66, texture: true, plane: "xy", profile: prof, from: -0.004, to: 0.004, transform: { translation: toLocal2(Fb, base).map((x) => Math.round(x * 1e5) / 1e5), rotation: KIT.quaternion([dot(d, Fb.u), dot(d, Fb.v), dot(d, Fb.w)], [dot(yv, Fb.u), dot(yv, Fb.v), dot(yv, Fb.w)], [dot(nv, Fb.u), dot(nv, Fb.v), dot(nv, Fb.w)]) }, repeat: { count, translation: [dot(stepV, Fb.u), dot(stepV, Fb.v), dot(stepV, Fb.w)].map((x) => Math.round(x * 1e5) / 1e5) } });
          for (let k = 0; k < count; k++) {
            const b0 = add(base, mul(stepV, k));
            const [lb, lt] = [toLocal2(Fb, b0), toLocal2(Fb, add(b0, mul(d, sl)))];
            lines.push([{ x: lb[0], y: lb[2] }, { x: lt[0], y: lt[2] }]);
          }
        }
        const q = g.at(0.13 * L, "a");
        const c = toLocal2(F[bone]!, q.p)[1];
        P.strokes.push({ id: `${bone}-spines`, bone, lines, z0: c, z1: c + 0.002, zTip: [c, c + 0.002], role: "texture", centre: q.p });
      }
    }
  }

  const anchorFronds = anchorsOn;
  anchorFronds.forEach((fid, i) => {
    const f = fronds.find((x) => x.id === fid)!;
    const g = new FrondGeom(f, F[`${f.id}a`]!, F[`${f.id}b`]!);
    const bone = `${f.id}b`;
    const centre = f.La + f.Lb * 0.5;
    const span = Math.min(0.7, f.Lb * 0.45);
    const S = steps(centre - span, centre + span, lod.frondStep);
    const lift = (s: number) => (lod.merged ? Math.max(0, g.vaneReach(s, lod.vaneShare) * Math.sin(g.beta) * 0) + Math.max(lod.vaneThick, feature * 0.55) : Math.max(g.rd(s), scale === 50 ? 0.0045 : 0.008)) + 0.0025;
    const patch = Math.max(0.0025, feature * 0.52);
    const sections: Section[] = S.map((s) => {
      const q = g.at(s, "b");
      return { p: add(q.p, mul(q.n, lift(s) + patch - 0.0025)), x: q.n, y: q.w, a: patch, b: Math.max(g.rw(s) * 1.1, 0.012, feature * 0.6) };
    });
    const q = g.at(centre, "b");
    const local = toLocal2(F[bone]!, add(q.p, mul(q.n, lift(centre) + 2 * patch)));
    P.emitters.push({ id: `th-tp-fronds-${i + 1}`, anchor: `tp-fronds-${i + 1}`, bone, tube: { id: `th-tp-fronds-${i + 1}`, bone, sections, fields: { material: "palm-frond", role: "detail", tone: 0.5 } }, anchorX: Math.round(local[0] * 1e4) / 1e4, anchorY: Math.round(local[1] * 1e4) / 1e4 });
  });

  for (const [bunch, side] of [["bunch-l", -1], ["bunch-r", 1]] as [string, number][]) {
    const fr = F[bunch]!;
    const rad: V3 = [side, 0, 0];
    const top = add(crown.o, add(mul(rad, 0.64), [0, 7.84 - crown.o[1], 0]));
    const stalk = [[0.2, 8.68], [0.34, 8.63], [0.46, 8.5], [0.555, 8.3], [0.61, 8.06], [0.64, 7.84]].map(([r, y]) => add([crown.o[0], 0, crown.o[2]], add(mul(rad, r!), [0, y!, 0])));
    const stalkStep = scale === 50 ? 1 : 2;
    const pts = stalk.filter((_, i) => i % stalkStep === 0 || i === stalk.length - 1).map((p, i, all) => ({ p, a: scale === 500 ? 0.04 : scale === 200 ? 0.02 : 0.016, b: scale === 500 ? 0.06 : scale === 200 ? 0.034 : 0.026 + 0.004 * (i / Math.max(1, all.length - 1)) }));
    const stalkTube: Tube = { id: `${bunch}-stalk`, bone: bunch, sections: sweep(pts, [0, 0, 1]), fields: { material: "date-fruit", role: "detail", tone: 0.62 } };
    P.tubes.push(stalkTube);
    region(stalkTube.id, "fruit");
    const parts = [stalkTube.id];
    if (lod.strands > 0) {
      for (let i = 0; i < lod.strands; i++) {
        const az = (i * 360) / lod.strands + 11;
        const ring = i % 2 === 0 ? 1 : 0.55;
        const dir: V3 = [Math.cos(az * D2R), 0, -Math.sin(az * D2R)];
        const p0 = add(top, add(mul(dir, 0.05 * ring), [0, -0.02 - 0.03 * (i % 3), 0]));
        const lengthM = 0.6 - 0.08 * (i % 3) - (ring < 1 ? 0.05 : 0);
        const p1 = add(p0, add(mul(dir, 0.14 * ring + 0.02), [0, -lengthM, 0]));
        const r = scale === 50 ? 0.036 : 0.055;
        const id = `${bunch}-strand-${i + 1}`;
        P.rods.push({ id, bone: bunch, p0, p1, r0: r * 0.8, r1: r, fields: { ...FRUIT } });
        region(id, "fruit");
        parts.push(id);
        if (lod.dates) {
          const axis = unit(sub(p1, p0));
          const L = len(sub(p1, p0));
          const count = Math.floor((L - 0.04) / 0.022);
          const perp = unit(cross(axis, [0, 0, 1]));
          const c0 = add(add(p0, mul(axis, 0.035)), mul(perp, r + 0.004));
          const dateAxes: [V3, V3, V3] = [axis, perp, cross(axis, perp)];
          const toL = (v: V3): V3 => [dot(v, fr.u), dot(v, fr.v), dot(v, fr.w)];
          P.repeats.push({ id: `${bunch}-dates-${i + 1}`, kind: "ellipsoid", bone: bunch, material: "date-fruit", role: "texture", tone: 0.3, texture: true, radii: [0.02, 0.014, 0.014], transform: { translation: toLocal2(fr, c0).map((x) => Math.round(x * 1e5) / 1e5), rotation: KIT.quaternion(toL(dateAxes[0]), toL(dateAxes[1]), toL(dateAxes[2])) }, repeat: { count, rotationAxis: toL(axis).map((x) => Math.round(x * 1e5) / 1e5), rotationRad: 2.0944 + 0.35, origin: toLocal2(fr, p0).map((x) => Math.round(x * 1e5) / 1e5), translation: toL(mul(axis, (L - 0.05) / count)).map((x) => Math.round(x * 1e5) / 1e5) } });
          P.worldTexture.push({
            id: `${bunch}-dates-${i + 1}`, bone: bunch, kind: "date",
            polys: (view) => {
              const toward = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] }[view] as V3;
              const lines: P2[][] = [];
              const depth: number[] = [];
              for (let k = 0; k < count; k++) {
                const ang = k * (2.0944 + 0.35);
                const off = add(mul(perp, Math.cos(ang)), mul(cross(axis, perp), Math.sin(ang)));
                if (dot(off, toward) < 0.2) continue;
                const c = add(add(p0, mul(axis, 0.035 + ((L - 0.05) / count) * k)), mul(off, r + 0.004));
                const ring2: P2[] = Array.from({ length: 9 }, (_, j) => {
                  const t = (2 * Math.PI * j) / 8;
                  const p = add(c, add(mul(axis, 0.019 * Math.cos(t)), mul(cross(toward, axis), 0.0125 * Math.sin(t))));
                  return project(view, p);
                });
                lines.push(ring2);
                depth.push(dot(c, toward));
              }
              return { lines, depth };
            },
          });
        }
      }
    } else if (scale === 200) {
      const c = add(top, [side * 0.06, -0.33, 0]);
      const id = `${bunch}-cluster`;
      P.blobs.push({ id, bone: bunch, c, axes: [[0, 1, 0], [1, 0, 0], [0, 0, -1]], radii: [0.34, 0.2, 0.2], fields: { ...FRUIT } });
      region(id, "fruit");
      parts.push(id);
    } else {
      const c = add(top, [side * 0.05, -0.32, 0]);
      const id = `${bunch}-cluster`;
      P.blobs.push({ id, bone: bunch, c, axes: [[0, 1, 0], [1, 0, 0], [0, 0, -1]], radii: [0.36, 0.21, 0.21], fields: { ...FRUIT } });
      region(id, "fruit");
      parts.push(id);
    }
    group(bunch, bunch, 0.36, "body", parts, "fruit", "date-fruit");
    if (lod.strands > 0) P.worldTexture.push({
      id: `${bunch}-strands`, bone: bunch, kind: "strand",
      polys: (view) => {
        const lines = P.rods.filter((r) => r.id.startsWith(`${bunch}-strand-`)).map((r) => [r.p0, r.p1].map((p) => project(view, p)));
        return { lines, depth: lines.map(() => 0) };
      },
    });
  }
  void minMass;
  void axisAt;
  return P;
}

import * as KIT from "./r3-date-palm-kit.ts";
