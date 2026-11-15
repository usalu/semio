/** 🌳️ Author 53: hand-built broadleaf crowns (english-oak, apple-tree) from each scale's own drawings: scaffold limbs as tapering lofts along 3D centre lines (x, y from the elevation stripe, z from the front stripe), crowns as quadrant lobes (horizontal superelliptic slices read from elevation x-span and front z-span, kept where the plan drawing covers their footprint).
 * @see ./author-53.md */
import {
  bestOverlap, intervalsAt, drawn as drawnOf, ROUND, add, cross, dot, inside, lerp, len, limbPath, loftSolid, mul, openCtx, quatOfColumns, reduce, ringsOf, saveLists, sectionAlong, sectionLevel, simplify, spanAt, sphereSolid, stripe, sub, toLocal, unit, boxOf, dirLocal,
  type Ctx, type Json, type Path, type Ring, type Sec, type V3,
} from "./author-53-lib.ts";

export type TreeCfg = {
  slug: string;
  stems: string[];
  stemFront: string[];
  woody: string[];
  pairs: [string, string][];
  singles: string[];
  crown500: string;
  bark: { stem: string; limb: string; stemMat: string; limbMat: string };
  leaf: { sunRegion: string; shadeRegion: string; shade: (bone: string) => boolean };
  tone: (bone: string) => number;
  mat: (bone: string) => string;
  fruit?: { region: string; material: string; radius: number };
  scale: Record<number, { tolLobe: number; tolLimb: number; stations: number; bias: number }>;
  override?: Record<number, Partial<Pick<TreeCfg, "stems" | "stemFront" | "woody" | "pairs" | "singles">>>;
  barkOf?: (bone: string) => string;
  skipShape?: (id: string) => boolean;
  extra?: (c: Ctx, built: Built, woodMat: string) => { body: Json[]; thermal: Json[]; structure: Json[] };
};

export type Built = { body: Json[]; paths: Record<string, Path>; lobes: Record<string, { bone: string; centre: V3; r: V3; y: [number, number] }>; notes: string[] };

export const isBody = (s: { role: string }): boolean => s.role === "body";
const drawnPlan = (c: Ctx): { id: string; bone: string; rings: Ring[] }[] => drawnOf(c, "plan");

export function buildBody(c: Ctx, cfg0: TreeCfg, opt: { exp: number; cross: boolean; merge?: boolean; split?: boolean }): Built {
  const cfg: TreeCfg = { ...cfg0, ...(cfg0.override?.[c.scale] ?? {}) };
  const sc = cfg.scale[c.scale]!;
  const f = c.feature;
  const rmin = 0.55 * f;
  const body: Json[] = [];
  const paths: Record<string, Path> = {};
  const lobes: Built["lobes"] = {};
  const notes: string[] = [];
  const keepShape = (id: string): boolean => cfg.skipShape === undefined || !cfg.skipShape(id);
  const E = (bones: string[]): Ring[] => ringsOf(c, "elevation", (s) => isBody(s) && bones.includes(s.bone) && keepShape(s.id));
  const F = (bones: string[]): Ring[] => ringsOf(c, "front", (s) => isBody(s) && bones.includes(s.bone) && keepShape(s.id));
  const frontPool = ringsOf(c, "front", isBody);
  const rig = c.raw.rig.bones as Json[];
  const order = rig.map((b) => b.id as string);
  const parentOf = (id: string): string | null => rig.find((b) => b.id === id)?.parent ?? null;

  const emitPath = (bone: string, path: Path, extra: Json = {}, material?: string, tone?: number, id?: string): void => {
    const secs = pathSections(c, bone, path, Math.max(0.35 * f, sc.tolLimb), (q) => [q.rLat, q.rVert], rmin);
    body.push(loftSolid(id ?? bone, bone, material ?? cfg.mat(bone), tone ?? cfg.tone(bone), secs, extra));
  };

  for (const bone of order) {
    if (!cfg.woody.includes(bone) && !cfg.stems.includes(bone)) continue;
    const fr = c.state.frames[bone]!;
    const isStem = cfg.stems.includes(bone);
    const lenBone = (rig.find((q) => q.id === bone)?.lengthM as number | undefined) ?? 0;
    const ownE = E([bone]);
    const pooled = isStem && ownE.length === 0;
    const eR = pooled ? E(cfg.stems) : ownE;
    if (eR.length === 0) continue;
    const parent = parentOf(bone);
    const pp = parent === null ? undefined : paths[parent];
    let z0 = 0;
    let slope = 0;
    if (pp !== undefined) {
      let best = Infinity;
      for (const q of pp) {
        const d = Math.hypot(q.P[0] - fr.o[0], q.P[1] - fr.o[1]);
        if (d < best) [best, z0] = [d, q.P[2]];
      }
      const a = pp[Math.floor(pp.length * 0.5)]!.P;
      const b = pp[pp.length - 1]!.P;
      slope = Math.abs(b[0] - a[0]) > 0.15 ? (b[2] - a[2]) / (b[0] - a[0]) : 0;
    }
    if (isStem) {
      const box = boxOf(eR);
      const span: [number, number] = [fr.o[1], fr.o[1] + lenBone * fr.u[1]];
      const siblingStarts = cfg.stems.filter((q) => q !== bone && c.state.frames[q] !== undefined).map((q) => c.state.frames[q]!.o[1]);
      const exceeds = !pooled && box.maxY > span[1] + 0.25 && siblingStarts.some((y0) => y0 >= span[1] - 0.05 && y0 < box.maxY - 0.1);
      const range: [number, number] = pooled ? span : exceeds ? [Math.max(box.minY, span[0] - 0.02), Math.min(box.maxY, span[1] + 0.02)] : [box.minY, box.maxY];
      const Es = stripe(eR, [fr.o[0], fr.o[1]], 60, [0, 1], range);
      const fR = F(cfg.stemFront);
      const Fs = fR.length > 0 ? stripe(fR, [0, fr.o[1]], 60, [0, 1], range) : null;
      if (Es === null) continue;
      const path: Path = [];
      const n = 40;
      for (let i = 0; i < n; i++) {
        const t = (i + 0.5) / n;
        const e = Es.at(t);
        const g = Fs === null ? null : Fs.at(t);
        path.push({ P: [e.p[0], e.p[1], g === null ? 0 : g.p[0]], rVert: e.h, rLat: g === null ? e.h : g.h });
      }
      paths[bone] = path;
      emitPath(bone, path);
      continue;
    }
    const fR = F([bone]);
    let zPrev = z0;
    const zIn = (x: number, y: number): number => {
      const iv = intervalsAt(frontPool, "y", y);
      const want = zPrev + slope * 0;
      let pick: [number, number] | null = null;
      let bd = Infinity;
      for (const q of iv) {
        const d = want >= q[0] && want <= q[1] ? 0 : Math.min(Math.abs(want - q[0]), Math.abs(want - q[1]));
        if (d < bd) [bd, pick] = [d, q];
      }
      if (pick !== null && bd > 0) zPrev = want < pick[0] ? pick[0] + 0.15 : pick[1] - 0.15;
      void x;
      return zPrev;
    };
    const res = limbPath(eR, fR, [fr.o[0], fr.o[1]], z0, zIn, 40);
    if (res === null) continue;
    paths[bone] = res.path;
    emitPath(bone, res.path);
  }

  const levelRings = (bone: string): { e: Ring[]; fr: Ring[] } => ({ e: E([bone]), fr: F([bone]) });
  const lobe = (id: string, bone: string, e: Ring[], fr: Ring[], material: string, tone: number, expo: number, xT?: [number, number], zT?: [number, number], clip?: { e: Ring[]; f: Ring[]; dy: number }): Json | null => {
    if (e.length === 0 || fr.length === 0) return null;
    const be = boxOf(e);
    const bf = boxOf(fr);
    const kx = xT === undefined ? 1 : (xT[1] - xT[0]) / (be.maxX - be.minX);
    const kz = zT === undefined ? 1 : (zT[1] - zT[0]) / (bf.maxX - bf.minX);
    const mx = (x: number): number => (xT === undefined ? x : xT[0] + (x - be.minX) * kx);
    const mz = (z: number): number => (zT === undefined ? z : zT[0] + (z - bf.minX) * kz);
    const raw: number[][] = [];
    const n = 36;
    for (let j = 0; j < n; j++) {
      const tau = (j + 0.5) / n;
      const yE = lerp(be.minY, be.maxY, tau);
      const yF = lerp(bf.minY, bf.maxY, tau);
      const xs = spanAt(e, "y", yE);
      const zs = spanAt(fr, "y", yF);
      if (xs === null || zs === null) continue;
      let xa = mx(xs[0]);
      let xb = mx(xs[1]);
      let za = mz(zs[0]);
      let zb = mz(zs[1]);
      const yy = (yE + yF) / 2 + (clip?.dy ?? 0);
      if (clip !== undefined) {
        const xi = bestOverlap(intervalsAt(clip.e, "y", yy), Math.min(xa, xb), Math.max(xa, xb));
        const zi = bestOverlap(intervalsAt(clip.f, "y", yy), Math.min(za, zb), Math.max(za, zb));
        if (xi === null || zi === null || xi[1] - xi[0] < 2 * rmin || zi[1] - zi[0] < 2 * rmin) continue;
        [xa, xb, za, zb] = [xi[0], xi[1], zi[0], zi[1]];
      }
      raw.push([yy, (xa + xb) / 2, (za + zb) / 2, Math.max((xb - xa) / 2 + sc.bias, rmin), Math.max((zb - za) / 2 + sc.bias, rmin)]);
    }
    if (raw.length < 3) return null;
    const keep = reduce(raw, sc.tolLobe);
    const rows = keep.map((i) => raw[i]!);
    const y0 = (be.minY + bf.minY) / 2 + (clip?.dy ?? 0);
    const y1 = (be.maxY + bf.maxY) / 2 + (clip?.dy ?? 0);
    const first = rows[0]!;
    const last = rows[rows.length - 1]!;
    const apex = (r: number[], y: number): number[] => [y, r[1]!, r[2]!, Math.max(r[3]! * 0.85, rmin), Math.max(r[4]! * 0.85, rmin)];
    const all = [apex(first, y0), ...rows, apex(last, y1)].filter((r, i, a) => i === 0 || r[0]! > a[i - 1]![0]! + 1e-3);
    const secs = all.map((r) => sectionLevel(c, bone, [r[1]!, r[0]!, r[2]!], r[3]!, r[4]!, expo));
    const cx = mx((be.minX + be.maxX) / 2);
    const cz = mz((bf.minX + bf.maxX) / 2);
    lobes[id] = { bone, centre: [cx, (y0 + y1) / 2, cz], r: [((be.maxX - be.minX) / 2) * kx, (y1 - y0) / 2, ((bf.maxX - bf.minX) / 2) * kz], y: [y0, y1] };
    return loftSolid(id, bone, material, tone, secs);
  };

  const splitCrown = (bone: string, e: Ring[], fr: Ring[]): Json[] => {
    const be = boxOf(e);
    const bf = boxOf(fr);
    const nY = 60;
    const rowsAt = (y: number): { ex: [number, number][]; fz: [number, number][] } => ({ ex: intervalsAt(e, "y", y), fz: intervalsAt(fr, "y", y) });
    let ym = be.maxY;
    for (let j = 0; j < nY; j++) {
      const y = lerp(be.minY, be.maxY, (j + 0.5) / nY);
      const r = rowsAt(y);
      if (r.ex.length === 1 && r.fz.length === 1) { ym = y; break; }
    }
    const out: Json[] = [];
    const low = rowsAt(lerp(be.minY, ym, 0.4));
    const tracks: { ex: number; fz: number }[] = [];
    const ec = low.ex.map((q) => (q[0] + q[1]) / 2);
    const fc = low.fz.map((q) => (q[0] + q[1]) / 2);
    ec.forEach((_, i) => fc.forEach((__, j) => tracks.push({ ex: ec[i]!, fz: fc[j]! })));
    const mk = (id: string, y0: number, y1: number, xRef: number | null, zRef: number | null): void => {
      const raw: number[][] = [];
      const n = 30;
      for (let k = 0; k < n; k++) {
        const t = (k + 0.5) / n;
        const y = lerp(y0, y1, t);
        const r = rowsAt(lerp(be.minY, be.maxY, (y - be.minY) / (be.maxY - be.minY)));
        const pickIv = (iv: [number, number][], ref: number | null): [number, number] | null => {
          if (iv.length === 0) return null;
          if (ref === null) return iv.reduce((a, b) => (b[1] - b[0] > a[1] - a[0] ? b : a));
          return iv.reduce((a, b) => (Math.abs((b[0] + b[1]) / 2 - ref) < Math.abs((a[0] + a[1]) / 2 - ref) ? b : a));
        };
        const xi = pickIv(r.ex, xRef);
        const zi = pickIv(r.fz, zRef);
        if (xi === null || zi === null) continue;
        raw.push([y, (xi[0] + xi[1]) / 2, (zi[0] + zi[1]) / 2, Math.max((xi[1] - xi[0]) / 2 + sc.bias, rmin), Math.max((zi[1] - zi[0]) / 2 + sc.bias, rmin)]);
      }
      if (raw.length < 3) return;
      const keep = reduce(raw, sc.tolLobe).map((i) => raw[i]!);
      const secs = keep.map((r) => sectionLevel(c, bone, [r[1]!, r[0]!, r[2]!], r[3]!, r[4]!, opt.exp));
      lobes[id] = { bone, centre: [keep[Math.floor(keep.length / 2)]![1]!, (y0 + y1) / 2, keep[Math.floor(keep.length / 2)]![2]!], r: [Math.max(...keep.map((r) => r[3]!)), (y1 - y0) / 2, Math.max(...keep.map((r) => r[4]!))], y: [y0, y1] };
      out.push(loftSolid(id, bone, cfg.mat(bone), cfg.tone(bone), secs));
    };
    if (ym > be.minY + 0.2) tracks.forEach((t, i) => mk(`${bone}-lobe-${i + 1}`, be.minY, Math.min(be.maxY, ym + 0.4), t.ex, t.fz));
    mk(`${bone}-crown`, Math.max(be.minY, ym - 0.4), be.maxY, null, null);
    void bf;
    return out;
  };

  const planAll = ringsOf(c, "plan", isBody);
  const covered = (x0: number, x1: number, z0: number, z1: number): number => {
    let hit = 0;
    let all = 0;
    const cx = (x0 + x1) / 2;
    const cz = (z0 + z1) / 2;
    for (let i = -4; i <= 4; i++) for (let j = -4; j <= 4; j++) {
      const u = i / 4.5;
      const v = j / 4.5;
      if (u * u + v * v > 1) continue;
      all++;
      if (inside(planAll, cx + u * ((x1 - x0) / 2), cz + v * ((z1 - z0) / 2))) hit++;
    }
    return hit / all;
  };

  const add1 = (s: Json | null): void => void (s !== null && body.push(s));
  for (const [w, e] of cfg.pairs) {
    const W = levelRings(w);
    const Eb = levelRings(e);
    add1(lobe(`${w}-mass`, w, W.e, W.fr, cfg.mat(w), cfg.tone(w), opt.exp));
    add1(lobe(`${e}-mass`, e, Eb.e, Eb.fr, cfg.mat(e), cfg.tone(e), opt.exp));
    if (opt.cross) {
      for (const [xb, zb, own] of [[W, Eb, w], [Eb, W, e]] as const) {
        const bx = boxOf(xb.e);
        const bz = boxOf(zb.fr);
        const cov = covered(bx.minX, bx.maxX, bz.minX, bz.maxX);
        if (cov >= 0.8) add1(lobe(`${own}-cross`, own, xb.e, zb.fr, cfg.mat(own), cfg.tone(own) - 0.03, opt.exp));
        else notes.push(`cross lobe of ${own} left out (plan covers ${(cov * 100).toFixed(0)} % of its footprint)`);
      }
    }
  }
  for (const s of cfg.singles) {
    const R = levelRings(s);
    if (opt.split === true && R.e.length > 0 && R.fr.length > 0) {
      for (const sd of splitCrown(s, R.e, R.fr)) add1(sd);
      continue;
    }
    const crownBone = /^fol/.test(s) ? s : "fol-top";
    add1(lobe(`${s}-mass`, s, R.e, R.fr, cfg.mat(crownBone), cfg.tone(crownBone), opt.exp));
  }
  const elevAll = ringsOf(c, "elevation", isBody);
  const frontAll = ringsOf(c, "front", isBody);
  const frac = (rings: Ring[], lo: number, hi: number, y0: number, y1: number): number => {
    let hit = 0;
    let all = 0;
    for (let i = -4; i <= 4; i++) for (let j = -4; j <= 4; j++) {
      const u = i / 4.5;
      const v = j / 4.5;
      if (u * u + v * v > 1) continue;
      all++;
      if (inside(rings, (lo + hi) / 2 + (u * (hi - lo)) / 2, (y0 + y1) / 2 + (v * (y1 - y0)) / 2)) hit++;
    }
    return hit / all;
  };
  const winLobes = { ...lobes };
  const dome = (tau: number): number => Math.max(0.3, Math.sqrt(Math.max(0, 1 - (2 * tau - 1) ** 2)) ** 0.7);
  for (const s of drawnPlan(c).filter((q) => /fill/.test(q.id))) {
    const pb = boxOf(s.rings);
    const hx = (pb.maxX - pb.minX) / 2;
    const hz = (pb.maxY - pb.minY) / 2;
    const cx = (pb.maxX + pb.minX) / 2;
    const cz = (pb.maxY + pb.minY) / 2;
    let best = { a: 0, h: 4, score: -1 };
    for (const lb of Object.values(winLobes)) for (let dy = -1.5; dy <= 1.51; dy += 0.5) {
      const a0 = lb.y[0] + dy;
      const hh = lb.y[1] - lb.y[0];
      const score = Math.min(frac(elevAll, pb.minX, pb.maxX, a0, a0 + hh), frac(frontAll, pb.minY, pb.maxY, a0, a0 + hh)) - 0.02 * Math.abs(dy);
      if (score > best.score) best = { a: a0, h: hh, score };
    }
    const rows: number[][] = [];
    const n = 20;
    for (let j = 0; j < n; j++) {
      const tau = (j + 0.5) / n;
      const p = dome(tau);
      const y = best.a + tau * best.h;
      const xi = bestOverlap(intervalsAt(elevAll, "y", y), cx - p * hx, cx + p * hx);
      const zi = bestOverlap(intervalsAt(frontAll, "y", y), cz - p * hz, cz + p * hz);
      if (xi === null || zi === null || xi[1] - xi[0] < 2 * rmin || zi[1] - zi[0] < 2 * rmin) continue;
      rows.push([y, (xi[0] + xi[1]) / 2, (zi[0] + zi[1]) / 2, (xi[1] - xi[0]) / 2 + sc.bias, (zi[1] - zi[0]) / 2 + sc.bias]);
    }
    if (rows.length < 3) {
      notes.push(`${s.id}: no consistent height found, left out`);
      continue;
    }
    const kept = reduce(rows, sc.tolLobe).map((i) => rows[i]!);
    rows.length = 0;
    rows.push(...kept);
    const secs = rows.map((r) => sectionLevel(c, s.bone, [r[1]!, r[0]!, r[2]!], r[3]!, r[4]!, opt.exp));
    lobes[s.id] = { bone: s.bone, centre: [cx, best.a + best.h / 2, cz], r: [hx, best.h / 2, hz], y: [best.a, best.a + best.h] };
    body.push(loftSolid(s.id, s.bone, cfg.mat(s.bone), cfg.tone(s.bone) - 0.03, secs));
    notes.push(`${s.id}: plan fill lobe at heights ${best.a.toFixed(2)}-${(best.a + best.h).toFixed(2)} m, footprint = the fill's bounding ellipse (elevation and front cover ${(best.score * 100).toFixed(0)} % of it)`);
  }
  void [add, cross, dot, len, mul, quatOfColumns, toLocal, unit, sphereSolid, openCtx, saveLists];
  return { body, paths, lobes, notes };
}

export function pathSections(c: Ctx, bone: string, path: Path, tol: number, rf: (q: Path[number]) => [number, number], rmin: number): Sec[] {
  const keep = simplify(path, tol);
  return keep.map((q, i) => {
    const a = keep[Math.max(0, i - 1)]!.P;
    const b = keep[Math.min(keep.length - 1, i + 1)]!.P;
    const [lat, vert] = rf(q);
    let t = unit(sub(b, a));
    if (Math.abs(t[1]) > 0.88) t = [0, t[1] >= 0 ? 1 : -1, 0];
    return sectionAlong(c, bone, q.P, t, Math.max(lat, rmin), Math.max(vert, rmin), 2.2);
  });
}

export function boxAlong(c: Ctx, id: string, bone: string, material: string, p0: V3, p1: V3, r: number, extra: Json): Json {
  const d = unit(sub(p1, p0));
  const ref: V3 = Math.abs(d[2]) < 0.9 ? [0, 0, 1] : [0, 1, 0];
  const y = unit(cross(ref, d));
  const z = cross(d, y);
  const q = quatOfColumns(dirLocal(c, bone, d), dirLocal(c, bone, y), dirLocal(c, bone, z));
  const mid = mul(add(p0, p1), 0.5);
  return { id, kind: "box", bone, material, role: "body", tone: 0.4, size: [len(sub(p1, p0)), 2 * r, 2 * r].map(ROUND), transform: { translation: toLocal(c, bone, mid).map(ROUND), rotation: q.map((v) => Math.round(v * 1e6) / 1e6) }, ...extra };
}

export function emitters(c: Ctx): Json[] {
  const out: Json[] = [];
  const anchors = c.raw.anchors as Json[];
  for (const em of c.raw.thermal.emitters as Json[]) {
    for (const aid of em.anchors ?? [em.anchor]) {
      const a = anchors.find((x) => x.id === aid)!;
      const ang = ((em.directionDeg ?? 0) * Math.PI) / 180;
      const d: V3 = [-Math.sin(ang), Math.cos(ang), 0];
      const y: V3 = [0, 0, 1];
      const z = cross(d, y);
      const q = quatOfColumns(dirLocal(c, a.bone, d), dirLocal(c, a.bone, y), dirLocal(c, a.bone, z));
      const fr = c.state.frames[a.bone]!;
      const wp = add(add(add(fr.o, mul(fr.u, a.x)), mul(fr.v, a.y)), mul(fr.w, a.z ?? 0));
      const dm = c.raw.dimensions;
      const clear = Math.min(dm.widthM / 2 - Math.abs(wp[0]), dm.depthM / 2 - Math.abs(wp[2]), dm.heightM - wp[1], wp[1]) - 2 * c.feature;
      const r = Math.max(Math.min(em.diameterM / 2, clear), 2 * c.feature);
      out.push({ id: `th-${aid}`, kind: "ellipsoid", bone: a.bone, material: c.raw.materials.find((m: Json) => m.id === (c.raw.thermal.regions as Json[]).find((g) => g.id === em.region)?.material)?.id ?? "oak-bark", role: "detail", tone: 0.5, radii: [ROUND(Math.max(0.6 * c.feature, 0.005)), ROUND(r), ROUND(r)], transform: { translation: [a.x, a.y, a.z ?? 0].map(ROUND), rotation: q.map((v) => Math.round(v * 1e6) / 1e6) }, region: em.region, tags: [em.id] });
    }
  }
  return out;
}

export function supportSolids(c: Ctx): Json[] {
  const out: Json[] = [];
  for (const a of (c.raw.anchors as Json[]).filter((x) => x.kind === "support")) {
    const L = Math.max(a.footprintLengthM ?? 1, 4 * c.feature);
    const W = Math.max(a.footprintWidthM ?? L, 4 * c.feature);
    const t = Math.max(c.feature, 0.04);
    out.push({ id: `support-${a.id}`, kind: "box", bone: a.bone, material: c.raw.thermal.regions[0].material, role: "detail", tone: 0.25, size: [t, L, W].map(ROUND), transform: { translation: [-t / 2, 0, 0] }, member: a.bone, part: "support", anchor: a.id });
  }
  return out;
}

export function buildAll(c: Ctx, cfg: TreeCfg, opt: { exp: number; cross: boolean; joints: boolean; core: number; category?: string; split?: boolean }): { body: Json[]; thermal: Json[]; structure: Json[]; built: Built } {
  const built = buildBody(c, cfg, opt);
  const f = c.feature;
  const sc = cfg.scale[c.scale]!;
  const rmin = 0.55 * f;
  const fine = c.scale === 50 ? built : buildBody(openCtx(`${opt.category ?? "vegetation"}/${cfg.slug}`, 50), cfg, { ...opt, joints: true });
  const regionOf = (s: Json): string => (cfg.barkOf !== undefined && !/leaf/.test(s.material) ? cfg.barkOf(s.bone) : /leaf/.test(s.material) ? (cfg.leaf.shade(s.bone) || /shade/.test(s.material) ? cfg.leaf.shadeRegion : cfg.leaf.sunRegion) : cfg.stems.includes(s.bone) ? cfg.bark.stem : cfg.woody.includes(s.bone) ? cfg.bark.limb : cfg.leaf.shade(s.bone) ? cfg.leaf.shadeRegion : cfg.leaf.sunRegion);
  const thermal: Json[] = built.body.map((s) => ({ id: `th-${s.id}`, of: s.id, region: regionOf(s) }));
  const woodMat = c.raw.materials.find((m: Json) => /wood/.test(m.id))?.id ?? cfg.mat(cfg.stems[0]!);
  const crown = Object.values(built.lobes).sort((p, q) => q.r[0] * q.r[2] - p.r[0] * p.r[2])[0];
  if (!built.body.some((s) => regionOf(s) === cfg.leaf.shadeRegion) && crown !== undefined) {
    const k = 0.5;
    thermal.push({ id: "th-inner-crown", kind: "ellipsoid", bone: crown.bone, material: cfg.mat("fol-core"), role: "detail", tone: 0.4, radii: [crown.r[0] * k, crown.r[1] * k, crown.r[2] * k].map(ROUND), transform: { translation: toLocal(c, crown.bone, crown.centre).map(ROUND) }, region: cfg.leaf.shadeRegion });
  }
  const ex = cfg.extra?.(c, built, woodMat) ?? { body: [], thermal: [], structure: [] };
  thermal.push(...ex.thermal);
  thermal.push(...emitters(c));
  const rig = c.raw.rig.bones as Json[];
  const structure: Json[] = [...ex.structure];
  const eAll = ringsOf(c, "elevation", isBody);
  const fAll = ringsOf(c, "front", isBody);
  const pAll = ringsOf(c, "plan", isBody);
  const tipOf = (bone: string): V3 | undefined => {
    const pa = built.paths[bone] ?? fine.paths[bone];
    return pa === undefined ? undefined : pa[pa.length - 1]!.P;
  };
  for (const b of rig) {
    const id = b.id as string;
    const secR = b.section?.areaM2 === undefined ? Infinity : Math.sqrt(b.section.areaM2 / Math.PI);
    const path = built.paths[id] ?? fine.paths[id];
    if (path !== undefined) {
      const own = built.paths[id] !== undefined;
      const rf = (q: Path[number]): [number, number] => [Math.min(opt.core * q.rLat, secR), Math.min(opt.core * q.rVert, secR)];
      let use = path;
      let rfUse = rf;
      if (!own) {
        const par = (b.parent as string | null) === null ? undefined : built.paths[b.parent as string];
        if (par !== undefined) {
          const a0 = Math.floor(par.length * 0.5);
          use = par.slice(a0, Math.max(a0 + 2, Math.floor(par.length * 0.9)));
          const childR = Math.min(...path.map((q) => Math.min(rf(q)[0], rf(q)[1])));
          rfUse = (q) => [Math.min(childR, 0.6 * q.rLat), Math.min(childR, 0.6 * q.rVert)];
        } else {
          const half = (q: Path[number]): number => {
            const iv = intervalsAt(eAll, "y", q.P[1]).find((w) => w[0] <= q.P[0] && w[1] >= q.P[0]);
            return iv === undefined ? 0 : Math.min(q.P[0] - iv[0], iv[1] - q.P[0]);
          };
          const rf0 = rf;
          rfUse = (q) => [Math.min(rf0(q)[0], 0.5 * half(q)), Math.min(rf0(q)[1], 0.5 * half(q))];
          const solidAt = (P: V3): boolean =>
            Object.values(built.lobes).some((l) => ((P[0] - l.centre[0]) / (0.7 * l.r[0])) ** 2 + ((P[1] - l.centre[1]) / (0.7 * l.r[1])) ** 2 + ((P[2] - l.centre[2]) / (0.7 * l.r[2])) ** 2 <= 1) ||
            Object.values(built.paths).some((pp) => pp.some((w) => Math.hypot(P[0] - w.P[0], P[1] - w.P[1], P[2] - w.P[2]) < 0.6 * Math.min(w.rLat, w.rVert)));
          const ok = (q: Path[number]): boolean => {
            const r = Math.max(rfUse(q)[0], rfUse(q)[1], rmin) + 2 * f;
            if (!solidAt(q.P)) return false;
            return [[0, 0], [r, 0], [-r, 0], [0, r], [0, -r]].every(([dx, dy]) => inside(eAll, q.P[0] + dx!, q.P[1] + dy!) && inside(fAll, q.P[2] + dx!, q.P[1] + dy!) && inside(pAll, q.P[0] + dx!, q.P[2] + dy!));
          };
          let k = 0;
          while (k < path.length && ok(path[k]!)) k++;
          use = path.slice(0, Math.max(k, 0));
        }
      }
      if (use.length >= 2 && c.scale === 500 && use.length <= 3) {
        const q0 = use[0]!;
        const q1 = use[use.length - 1]!;
        const r = Math.max(rfUse(q1)[0], rfUse(q1)[1], 1.1 * rmin);
        structure.push(boxAlong(c, `member-${id}`, id, woodMat, q0.P, q1.P, r, { member: id, part: "member" }));
      } else if (use.length >= 2) {
        const secs = pathSections(c, id, use, Math.max(0.35 * f, sc.tolLimb), rfUse, rmin);
        structure.push(loftSolid(`member-${id}`, id, woodMat, 0.4, secs, { member: id, part: "member" }));
      } else {
        const q = use[0] ?? path[0]!;
        const r = Math.max(rf(q)[0], rf(q)[1], 1.1 * rmin);
        const qc: V3 = [q.P[0], Math.max(q.P[1], r + 0.5 * f), q.P[2]];
        structure.push(c.scale === 500 ? boxAlong(c, `member-${id}`, id, woodMat, sub(qc, [0, r, 0]), add(qc, [0, r, 0]), r, { member: id, part: "member" }) : sphereSolid(c, `member-${id}`, id, woodMat, 0.4, qc, r, { member: id, part: "member", role: "body" }));
      }
      continue;
    }
    if (/^fol-/.test(id)) {
      const lobe = built.lobes[`${id}-mass`] ?? crown;
      if (lobe === undefined) continue;
      const parent = b.parent as string | null;
      const fr = c.state.frames[id]!;
      const p0: V3 = (parent === null ? undefined : tipOf(parent)) ?? [fr.o[0], fr.o[1], 0];
      const dir = sub(p0, lobe.centre);
      const p1: V3 = add(lobe.centre, mul(dir, 0.35));
      const pStart: V3 = sub(lobe.centre, mul(dir, 0.15));
      const r0 = Math.max(0.045 * Math.sqrt(c.scale / 50), rmin);
      if (c.scale === 500) {
        structure.push(boxAlong(c, `member-${id}`, id, woodMat, pStart, p1, Math.max(r0 * 0.7, rmin), { member: id, part: "member" }));
        continue;
      }
      structure.push({ id: `member-${id}`, kind: "capsule", bone: id, material: woodMat, role: "body", tone: 0.4, p0: toLocal(c, id, pStart).map(ROUND), p1: toLocal(c, id, p1).map(ROUND), r0, r1: Math.max(r0 * 0.5, rmin), member: id, part: "member" });
    }
  }
  if (opt.joints) {
    for (const b of rig) {
      const id = b.id as string;
      const path = built.paths[id];
      const parent = b.parent as string | null;
      if (path === undefined || parent === null || built.paths[parent] === undefined || parent === "root-plate") continue;
      const m = structure.find((x) => x.id === `member-${id}`);
      if (m === undefined || m.kind !== "loft") continue;
      const r = Math.max(0.95 * Math.min(m.sections[0].rx, m.sections[0].ry), 1.1 * rmin);
      structure.push(sphereSolid(c, `joint-${id}`, id, woodMat, 0.35, path[0]!.P, r, { member: id, part: "joint" }));
    }
  }
  structure.push(...supportSolids(c));
  const covered = new Set(thermal.map((t) => t.region as string));
  for (const reg of c.raw.thermal.regions as Json[]) {
    if (covered.has(reg.id)) continue;
    for (const m of structure.filter((x) => x.part === "member" && regionOf({ bone: x.bone, material: "" }) === reg.id)) {
      const t = { ...m, id: `th-${m.id}`, role: "detail", region: reg.id };
      delete t.member;
      delete t.part;
      thermal.push(t);
    }
  }
  return { body: [...built.body, ...ex.body], thermal, structure, built };
}

export type Words = { en: string; de: string };

export function treeDecisions(c: Ctx, r: { body: Json[]; thermal: Json[]; structure: Json[]; built: Built }, w: { name: Words; coarse: Record<number, Words>; hard: Words; full?: boolean }): Words[] {
  const s = c.scale;
  const n = (k: string): number => r.body.filter((x) => x.kind === k).length;
  const lobes = r.body.filter((x) => x.material !== undefined && /leaf/.test(x.material)).length;
  const woody = r.body.length - lobes - r.body.filter((x) => x.role === "detail").length;
  const fills = Object.keys(r.built.lobes).filter((k) => /fill/.test(k)).length;
  const members = r.structure.filter((x) => x.part === "member").length;
  const joints = r.structure.filter((x) => x.part === "joint").length;
  const emit = r.thermal.filter((x) => x.tags !== undefined).length;
  const own = w.full === true ? { en: `Body of 1:${s} (${r.body.length} solids): ${w.coarse[s]?.en ?? ""}`, de: `Körper von 1:${s} (${r.body.length} Körper): ${w.coarse[s]?.de ?? ""}` } : undefined;
  return [
    own ?? {
      en: `Body of 1:${s} (${r.body.length} solids, ${n("loft")} lofts): ${woody} tapering lofts for trunk and limbs follow the centre lines read from this scale's elevation (x, y) and front (z) outlines with the half widths of both views as section radii, ${lobes} crown lobes are stacks of horizontal superelliptic slices whose widths are the elevation x span and the front z span of the drawn scallop groups${fills > 0 ? `, ${fills} of them standing for the plan drawing's crown fills, clipped to what elevation and front draw at that height` : ""}. ${w.coarse[s]?.en ?? ""}`,
      de: `Körper von 1:${s} (${r.body.length} Körper, ${n("loft")} Lofts): ${woody} verjüngte Lofts für Stamm und Äste folgen den Mittellinien aus Aufriss (x, y) und Front (z) dieses Maßstabs, die Schnittradien sind die Halbbreiten beider Ansichten, ${lobes} Kronenlappen sind Stapel waagrechter superelliptischer Scheiben, deren Breiten die x-Spanne des Aufrisses und die z-Spanne der Front der gezeichneten Laubgruppen sind${fills > 0 ? `, ${fills} davon stehen für die Kronenfüllungen des Grundrisses, auf das beschnitten, was Aufriss und Front in dieser Höhe zeichnen` : ""}. ${w.coarse[s]?.de ?? ""}`,
    },
    {
      en: `Thermal (${r.thermal.length} solids): bark and leaf regions are references to the exposed body solids of the same bone family, ${emit} flat discs of the emitters' own diameter stand at their anchors inside the crown${r.thermal.some((x) => x.id === "th-inner-crown") ? ", and an inner crown ellipsoid carries the shade leaves that this scale's body does not draw separately" : ""}. Foliage regions are exchange areas (areaBasis exchange) and are covered, not compared.`,
      de: `Thermisch (${r.thermal.length} Körper): Rinden- und Laubregionen verweisen auf die freien Körper-Solids derselben Knochenfamilie, ${emit} flache Scheiben mit dem Durchmesser der Emitter stehen an ihren Ankern in der Krone${r.thermal.some((x) => x.id === "th-inner-crown") ? ", und ein Ellipsoid der Innenkrone trägt die Schattenblätter, die der Körper dieses Maßstabs nicht eigens zeichnet" : ""}. Laubregionen sind Austauschflächen (areaBasis exchange) und werden nur abgedeckt, nicht verglichen.`,
    },
    {
      en: `Structure (${r.structure.length} solids): ${members} member solids follow the same centre lines as the body at ${Math.round(100 * 0.75)} % of the local radius, capped at the bone's section; ${joints > 0 ? `${joints} joint spheres sit at the limb forks; ` : ""}twig bundles run from the branch tips into the crown lobes; one fixed support slab lies under the root plate at its anchor. Members of bones this scale does not draw (inside the crown) keep the centre lines of the finest scale. ${w.hard.en}`,
      de: `Tragwerk (${r.structure.length} Körper): ${members} Stabkörper folgen denselben Mittellinien wie der Körper mit ${Math.round(100 * 0.75)} % des örtlichen Radius, begrenzt auf den Querschnitt des Knochens; ${joints > 0 ? `${joints} Gelenkkugeln sitzen an den Astgabeln; ` : ""}Zweigbündel laufen von den Astspitzen in die Kronenlappen; eine feste Auflagerplatte liegt unter dem Wurzelteller an ihrem Anker. Stäbe von Knochen, die dieser Maßstab nicht zeichnet (im Inneren der Krone), behalten die Mittellinien des feinsten Maßstabs. ${w.hard.de}`,
    },
  ];
}
