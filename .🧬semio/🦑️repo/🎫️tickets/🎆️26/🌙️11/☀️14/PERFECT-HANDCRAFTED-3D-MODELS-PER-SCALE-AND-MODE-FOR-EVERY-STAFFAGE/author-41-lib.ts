/** 🐾️ Author 41: sculpting helpers on top of author-29-lib for quadrupeds and birds: lofts along slanted bones from the scale's own drawings (vertical-to-axis spans in elevation, lateral spans from the plan).
 * @see ./author-29-lib.ts
 * @see ./author-41.md */
import { Sculpt, type Look, type Station } from "./author-29-lib.ts";
import { drawn, type Json, type V3 } from "./kit-lib.ts";
import { lineSpan, medialPath, range, type P } from "./author-41-geo.ts";

export { range, type Look, type Station };

export class Zoo extends Sculpt {
  rings(view: "elevation" | "front" | "plan", ids: readonly string[]): P[][] {
    const set = new Set(ids);
    return drawn(this.c, view).filter((s) => set.has(s.id)).flatMap((s) => s.rings.map((r) => r.map((p) => [p.x, p.y] as P)));
  }

  /** 🦴️ Loft along bone `bone` between local distances `s0..s1` (m along the bone axis in elevation): sagittal extent from the elevation shapes `elev`, lateral extent from the plan shapes `plan` (or `half`), centre offset `zc`; `grow(s)` scales both radii (muscle bulge). */
  limb(id: string, bone: string, o: { elev: readonly string[]; plan?: readonly string[]; s0?: number; s1?: number; n: number; trim?: number; look: Look; e?: number; half?: (s: number, plan: number) => number; hv?: (s: number, v: number) => number; zc?: number; floor?: number; caps?: boolean; cv?: (s: number, c: number, h: number) => number; sectionAt?: readonly number[]; round?: boolean; front?: readonly string[] }): Json {
    const f = this.frame(bone);
    const u: P = [f.u[0]!, f.u[1]!];
    const nv: P = [-u[1], u[0]];
    const er = this.rings("elevation", o.elev);
    const pr = o.plan === undefined ? [] : this.rings("plan", o.plan);
    const floor = o.floor ?? 0.55 * this.feature;
    const st: Station[] = [];
    let s0 = o.s0;
    let s1 = o.s1;
    if (s0 === undefined || s1 === undefined) {
      const ss = er.flatMap((r) => r.map((p) => (p[0] - f.o[0]!) * u[0] + (p[1] - f.o[1]!) * u[1]));
      const lo = Math.min(...ss);
      const hi = Math.max(...ss);
      const t = (o.trim ?? 0.03) * (hi - lo);
      s0 ??= lo + t;
      s1 ??= hi - t;
    }
    for (const s of o.sectionAt ?? range(s0, s1, o.n)) {
      const at: P = [f.o[0]! + s * u[0], f.o[1]! + s * u[1]];
      const sp = lineSpan(er, at, u);
      if (sp === null) continue;
      let hv = (sp[1] - sp[0]) / 2;
      let cv = (sp[0] + sp[1]) / 2;
      if (o.cv !== undefined) cv = o.cv(s, cv, hv);
      if (o.hv !== undefined) hv = o.hv(s, hv);
      const c: P = [at[0] + cv * nv[0], at[1] + cv * nv[1]];
      let zc = o.zc ?? NaN;
      let hz = floor;
      if (pr.length > 0) {
        const ps = lineSpan(pr, [c[0], 0], [1, 0]);
        if (ps !== null) { hz = (ps[1] - ps[0]) / 2; if (o.zc === undefined) zc = (ps[0] + ps[1]) / 2; }
      }
      if (o.front !== undefined) {
        const fs = this.span("front", o.front, "y", c[1]);
        if (fs !== null) { hz = Math.min(hz, (fs[1] - fs[0]) / 2); if (o.zc === undefined && pr.length > 0) zc = (fs[0] + fs[1]) / 2; }
      }
      if (o.half !== undefined) hz = o.half(s, hz);
      if (o.round === true) hv = hz;
      st.push({ c: [c[0], c[1], zc], a: [0, 0, Math.max(floor, hz)], b: [nv[0] * Math.max(floor, hv), nv[1] * Math.max(floor, hv), 0], e: o.e });
    }
    if (st.length < 2) throw new Error(`${id}: no stations`);
    const known = st.filter((q) => !Number.isNaN(q.c[2]));
    for (const q of st) if (Number.isNaN(q.c[2])) q.c[2] = known.length === 0 ? 0 : known.reduce((a, b) => (Math.abs(b.c[0] - q.c[0]) < Math.abs(a.c[0] - q.c[0]) ? b : a)).c[2];
    return this.loft(id, bone, st, o.look, o.caps ?? true);
  }

  /** 🔗️ Continuous tapered chain along the bones `bones` (joint to joint in the default pose, z = `z`): one tube per bone with radii `r[i]` at the joints (length bones + 1). */
  chain(prefix: string, bones: readonly string[], lengths: readonly number[], r: readonly number[], look: Look, z = 0, ids?: readonly string[]): void {
    bones.forEach((bone, i) => {
      const f = this.frame(bone);
      const a: V3 = [f.o[0]!, f.o[1]!, z];
      const b: V3 = [f.o[0]! + f.u[0]! * lengths[i]!, f.o[1]! + f.u[1]! * lengths[i]!, z];
      this.tube(ids?.[i] ?? `${prefix}-${i + 1}`, bone, a, b, r[i]!, r[i + 1]!, look);
    });
  }

  /** 🐂️ Round-sectioned loft along the medial path of a thin curved drawn strip (horn): `shape` is the elevation shape id, `from` the end that is the root, `z0 -> z1` the lateral offset along the path, radii `r0 -> r1` (lateral) with the sagittal radius from the strip width. */
  strip(id: string, bone: string, o: { shape: string; from: P; n: number; z0: number; z1: number; r0: number; r1: number; look: Look; e?: number; cut?: number; taper?: number }): Json {
    const ring = this.rings("elevation", [o.shape])[0]!;
    const path = medialPath(ring, o.n, o.from).slice(0, o.n - (o.cut ?? 0));
    const st: Station[] = path.map((p, i) => {
      const a = path[Math.max(0, i - 1)]!.c;
      const b = path[Math.min(path.length - 1, i + 1)]!.c;
      const t: P = [b[0] - a[0], b[1] - a[1]];
      const l = Math.hypot(t[0], t[1]) || 1;
      const nv: P = [-t[1] / l, t[0] / l];
      const u = i / Math.max(1, path.length - 1);
      const tp = 1 - (1 - (o.taper ?? 1)) * u;
      const rz = (o.r0 + (o.r1 - o.r0) * u) * tp;
      const rv = Math.max(p.half * tp, 0.55 * this.feature);
      return { c: [p.c[0], p.c[1], o.z0 + (o.z1 - o.z0) * u], a: [0, 0, Math.max(rz, 0.55 * this.feature)], b: [nv[0] * rv, nv[1] * rv, 0], e: o.e };
    });
    return this.loft(id, bone, st, o.look, true);
  }

  override tube(id: string, bone: string, a: V3, b: V3, r0: number, r1: number, look: Look): Json {
    const d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const l = Math.hypot(...d) || 1;
    if (Math.abs(d[2]! / l) < 0.9) return super.tube(id, bone, a, b, r0, r1, look);
    const dn = d.map((v) => v / l) as V3;
    const p1: V3 = [1, 0, 0];
    const p2: V3 = [dn[1]! * p1[2]! - dn[2]! * p1[1]!, dn[2]! * p1[0]! - dn[0]! * p1[2]!, dn[0]! * p1[1]! - dn[1]! * p1[0]!];
    const ext = 0.4 * Math.min(r0, r1);
    const a2 = a.map((v, i) => v - dn[i]! * ext) as V3;
    const b2 = b.map((v, i) => v + dn[i]! * ext) as V3;
    return this.loft(id, bone, [{ c: a2, a: p1.map((v) => v * r0) as V3, b: p2.map((v) => v * r0) as V3 }, { c: b2, a: p1.map((v) => v * r1) as V3, b: p2.map((v) => v * r1) as V3 }], look);
  }

  worldOf(bone: string, s: number): V3 {
    const f = this.frame(bone);
    return [f.o[0]! + s * f.u[0]!, f.o[1]! + s * f.u[1]!, f.o[2]! + s * f.u[2]!];
  }
}

/** 🫁️ Barrel torso piece: a back loft (upper part of the drawn depth, full width) and a belly loft (lower part, narrower), both from the scale's drawings, so the cross-section reads as an egg with a spine line and a belly instead of a tube. */
export function torso2(S: Zoo, id: string, bone: string, o: { ids: readonly string[]; xs: readonly number[]; look: Look; e?: number; split?: number; bellyW?: number; backH?: number; minY?: number }): void {
  const split = o.split ?? 0.3;
  const bw = o.bellyW ?? 0.84;
  S.hullX(`${id}-back`, bone, { elev: o.ids, plan: o.ids, xs: o.xs, look: o.look, e: o.e ?? 2.4, vfix: (_x, lo, hi) => [lo + split * (hi - lo), hi] });
  S.hullX(`${id}-belly`, bone, { elev: o.ids, plan: o.ids, xs: o.xs, look: o.look, e: o.e ?? 2.2, vfix: (_x, lo, hi) => { const l = Math.max(lo, o.minY ?? -1); return [l, l + (1 - split * 0.55) * (hi - l)]; }, half: (_x, zh) => zh * bw });
}
