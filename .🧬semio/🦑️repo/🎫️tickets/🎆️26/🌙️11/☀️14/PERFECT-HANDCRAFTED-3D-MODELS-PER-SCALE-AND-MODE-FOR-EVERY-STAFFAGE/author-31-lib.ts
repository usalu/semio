/** 🌳️ Author 31 helpers on top of the author-02 kit: limbs with a 3D centre line, lobes per drawn scallop group, structure cores that follow the limb paths.
 * @see ./author-02-lib.ts */
import { toWorld, type Json, type V3 } from "./volumes-lib.ts";
import { Kit, ivalsAll, bboxOfShapes, type Iv, type Meta, type Sec, type Shapes } from "./author-02-lib.ts";

/** 🧭️ Radii given along the world axes, permuted to the bone's local (u, v, w) axes. */
export function localRadii(k: Kit, bone: string, r: V3): V3 {
  const es: V3[] = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
  const ds = es.map((e) => k.D(bone, e));
  return [0, 1, 2].map((a) => Math.max(...[0, 1, 2].map((w) => Math.abs(ds[w]![a]!) * r[w]!))) as V3;
}

export type Path = { bone: string; secs: Sec[] };

/** 🌿️ A limb loft along the dominant axis of its elevation shape; z centre from `zOf(param)` (default 0), depth from the half width (round) or the front/plan outline. Returns the section path. */
export function limb3(k: Kit, id: string, bone: string, E: Shapes, m: Meta, o: { n?: number; exp?: number; axis?: "x" | "y"; zOf?: (x: number) => number; F?: Shapes | null; P?: Shapes | null; depth?: number; list?: "body" | "thermal" | "structure"; extra?: Json; trim?: number; round?: boolean; adaptive?: number; yRange?: [number, number] } = {}): Sec[] {
  const b = bboxOfShapes(E);
  const axis = o.axis ?? (b.maxY - b.minY >= b.maxX - b.minX ? "y" : "x");
  let lo = axis === "y" ? b.minY : b.minX, hi = axis === "y" ? b.maxY : b.maxX;
  if (o.yRange && axis === "y") { lo = Math.max(lo, o.yRange[0]); hi = Math.min(hi, o.yRange[1]); }
  const n = o.n ?? Math.max(3, Math.min(9, Math.round((hi - lo) / Math.max(k.feat * 6, 0.3)) + 1));
  const stations: number[] = (() => {
    if (!o.adaptive) return Array.from({ length: n }, (_, i) => lo + ((hi - lo) * i) / (n - 1));
    const N = Math.max(80, Math.ceil((hi - lo) / Math.max(0.004, k.feat * 0.4)));
    const ts = Array.from({ length: N + 1 }, (_, i) => lo + ((hi - lo) * i) / N);
    const edgeOf = (S: Shapes, ax: "x" | "y") => ts.map((t) => { const iv = ivalsAll(S, ax, Math.min(hi - 1e-4, Math.max(lo + 1e-4, t))); return iv.length ? [iv[0]![0], iv[iv.length - 1]![1]] : [0, 0]; });
    const keep = new Set<number>([0, N]);
    const sources = [edgeOf(E, axis === "y" ? "y" : "x"), ...(axis === "y" && o.F && o.F.length ? [edgeOf(o.F, "y")] : [])];
    for (const edge of sources) for (const c of [0, 1]) {
      const rec = (a: number, b: number) => {
        let w = -1, wi = -1;
        for (let i = a + 1; i < b; i++) { const f = (ts[i]! - ts[a]!) / (ts[b]! - ts[a]!); const d = Math.abs(edge[a]![c]! + f * (edge[b]![c]! - edge[a]![c]!) - edge[i]![c]!); if (d > w) { w = d; wi = i; } }
        if (w > o.adaptive! && b - a > 1) { keep.add(wi); rec(a, wi); rec(wi, b); }
      };
      rec(0, N);
    }
    return [...keep].sort((a, b) => a - b).map((i) => ts[i]!);
  })();
  const secs: Sec[] = [];
  let prev: Iv | null = null;
  for (const st of stations) {
    const s = Math.min(hi - 1e-4, Math.max(lo + 1e-4, st));
    let iv = ivalsAll(E, axis === "y" ? "y" : "x", s);
    if (!iv.length) iv = prev ? [prev] : [];
    if (!iv.length) continue;
    const pick = prev ? iv.reduce((a, c) => (Math.abs((c[0] + c[1]) / 2 - (prev![0] + prev![1]) / 2) < Math.abs((a[0] + a[1]) / 2 - (prev![0] + prev![1]) / 2) ? c : a)) : iv.reduce((a, c) => (c[1] - c[0] > a[1] - a[0] ? c : a));
    prev = pick;
    const half = (pick[1] - pick[0]) / 2, mid = (pick[0] + pick[1]) / 2;
    let cz = o.zOf ? o.zOf(axis === "x" ? s : mid) : 0, hz = half * (o.depth ?? 1);
    let zi: Iv[] = [];
    if (axis === "y" && o.F) { const fb = bboxOfShapes(o.F); zi = ivalsAll(o.F, "y", Math.min(fb.maxY - 1e-4, Math.max(fb.minY + 1e-4, s))); }
    if (axis === "x" && o.P) zi = ivalsAll(o.P, "x", s);
    else if (axis === "x" && o.F) zi = ivalsAll(o.F, "y", Math.min(bboxOfShapes(o.F).maxY - 1e-4, Math.max(bboxOfShapes(o.F).minY + 1e-4, mid)));
    if (zi.length) { const w = zi.reduce((a, c) => (c[1] - c[0] > a[1] - a[0] ? c : a)); if (!o.zOf) cz = (w[0] + w[1]) / 2; if (!o.round) hz = (w[1] - w[0]) / 2; }
    secs.push(axis === "y" ? k.secY([mid, s, cz], half, hz, o.exp ?? 2) : k.secX([s, mid, cz], half, hz, o.exp ?? 2));
  }
  k.loft(o.list ?? "body", id, bone, secs, m, o.extra ?? {});
  return secs;
}

/** 🍃️ Structure for trees: member cores along the recorded limb paths, joints at the bone origins, masses, fixed support. */
export function treeStructure(k: Kit, paths: Map<string, Sec[]>, o: { core?: number; joints?: boolean; masses?: { id: string; bone: string; c: V3; h: V3 }[]; massK?: number; supportBox?: V3; massMaterial?: string; clip?: (p: V3) => boolean; skipJoint?: RegExp; jointK?: number; coreOf?: Record<string, number> }) {
  const bones = k.memberBones();
  const feat = k.feat;
  const core = o.core ?? 0.62;
  const minR = feat * 0.55;
  const mat = (b: Json) => b.section?.material ?? k.asset.materials[0].id;
  for (const id of bones) {
    const b = k.boneDef(id);
    const p = paths.get(id);
    const meta: Meta = { material: mat(b), role: "body", tone: 0.45 };
    const extra = { member: id, part: "member" };
    if (p && p.length >= 2) {
      const cc = o.coreOf?.[id] ?? core;
      const secs = p.map((s) => ({ ...s, rx: Math.max(minR, s.rx * cc), ry: Math.max(minR, s.ry * cc), exp: 2 }));
      k.loft("structure", `member-${id}`, id, secs, meta, extra);
    } else {
      const f = k.fr[id]!;
      const r = Math.max(minR, (b.section?.extremeFibreM ?? 0.03) * core);
      let a = k.origin(id), t = k.tip(id);
      if (o.clip) {
        const pts = Array.from({ length: 41 }, (_, i) => [a[0] + ((t[0] - a[0]) * i) / 40, a[1] + ((t[1] - a[1]) * i) / 40, a[2] + ((t[2] - a[2]) * i) / 40] as V3);
        const ins = pts.map((q) => o.clip!(q));
        let best: [number, number] = [0, -1];
        for (let i = 0; i < ins.length; i++) if (ins[i]) { let j = i; while (j + 1 < ins.length && ins[j + 1]) j++; if (j - i > best[1] - best[0]) best = [i, j]; i = j; }
        if (best[1] >= best[0] && best[1] - best[0] >= 1) { a = pts[best[0]]!; t = pts[best[1]]!; }
        else if (best[1] >= best[0]) { a = pts[best[0]]!; t = [a[0], a[1] + 0.02, a[2]]; }
      }
      const mk = (q: V3): Sec => ({ c: q, ax: f.v, ay: f.w, rx: r, ry: r, exp: 2 });
      k.loft("structure", `member-${id}`, id, [mk(a), mk(t)], meta, extra);
    }
  }
  if (o.joints) {
    for (const id of bones) {
      const b = k.boneDef(id);
      if (!b.parent || !bones.includes(b.parent) || o.skipJoint?.test(id)) continue;
      const p = paths.get(id);
      const og = k.origin(id);
      const s0 = p?.length ? p.reduce((a, c) => (Math.hypot(c.c[0] - og[0], c.c[1] - og[1]) < Math.hypot(a.c[0] - og[0], a.c[1] - og[1]) ? c : a)) : undefined;
      const r = Math.max(minR, (s0 ? Math.min(s0.rx, s0.ry) : (b.section?.extremeFibreM ?? 0.03)) * core * (o.jointK ?? 1.25));
      const c = s0 ? s0.c : k.origin(id);
      k.ell("structure", `joint-${id}`, id, c, [r, r, r], { material: mat(b), role: "detail", tone: 0.35 }, { member: id, part: "joint" });
    }
  }
  const leafMat = o.massMaterial ?? k.asset.materials.find((x: Json) => /leaf|needle|foliage/.test(x.id))?.id ?? k.asset.materials[0].id;
  for (const g of o.masses ?? []) {
    const s = o.massK ?? 0.3;
    k.ell("structure", `mass-${g.id}`, g.bone, g.c, localRadii(k, g.bone, [Math.max(minR, g.h[0] * s), Math.max(minR, g.h[1] * s), Math.max(minR, g.h[2] * s)]), { material: leafMat, role: "detail", tone: 0.3 }, { member: g.bone, part: "mass" });
  }
  for (const sid of k.asset.structure.supports ?? []) {
    const a = k.asset.anchors.find((x: Json) => x.id === sid);
    if (!a) continue;
    const w = toWorld(k.fr[a.bone]!, [a.x, a.y, 0]);
    const fl = Math.min(a.footprintLengthM ?? 0.5, 0.6), fw = Math.min(a.footprintWidthM ?? 0.5, 0.6);
    const h = Math.max(2 * feat, 0.04);
    k.box("structure", `support-${sid}`, a.bone, [w[0], w[1] + h / 2, w[2]], (() => { const r = localRadii(k, a.bone, o.supportBox ?? [fl / 2, h / 2, fw / 2]); return [r[0] * 2, r[1] * 2, r[2] * 2] as V3; })(), { material: mat(k.boneDef(a.bone)), role: "detail", tone: 0.25 }, { member: a.bone, part: "support", anchor: sid, tags: [sid] });
  }
}

/** 🌫️ Transpiration patches of foliage emitters (thermal solids at the emitter anchors). */
export function emitterPatches(k: Kit, regionOf: (region: string) => string | undefined) {
  const t = Math.max(0.02, k.feat * 2.2);
  for (const e of k.asset.thermal.emitters ?? []) {
    const reg = k.asset.thermal.regions.find((r: Json) => r.id === e.region);
    if (!reg) continue;
    const anchors: string[] = e.anchors ?? [e.anchor];
    anchors.forEach((aid, i) => {
      const a = k.asset.anchors.find((x: Json) => x.id === aid);
      if (!a) return;
      const w = toWorld(k.fr[a.bone]!, [a.x, a.y, a.z ?? 0]);
      const r = Math.max(k.feat * 1.2, Math.min(e.diameterM / 2, 0.3));
      k.loft("thermal", `emit-${e.id}-${i + 1}`, a.bone, [k.secY([w[0], w[1] - t / 2, w[2]], r, r, 2), k.secY([w[0], w[1] + t / 2, w[2]], r, r, 2)], { material: reg.material, role: "body", tone: 0.5 }, { region: e.region, tags: [e.id, "emitter"] });
    });
  }
  void regionOf;
}

/** 📍️ z centre line of a drawn plan strip as a function of x (clamped to the strip). */
export function zFromPlan(P: Shapes | null): ((x: number) => number) | null {
  if (!P || !P.length) return null;
  const b = bboxOfShapes(P);
  return (x: number) => {
    const xc = Math.min(b.maxX - 1e-4, Math.max(b.minX + 1e-4, x));
    const iv = ivalsAll(P, "x", xc);
    if (!iv.length) return (b.minY + b.maxY) / 2;
    return (iv[0]![0] + iv[iv.length - 1]![1]) / 2;
  };
}
