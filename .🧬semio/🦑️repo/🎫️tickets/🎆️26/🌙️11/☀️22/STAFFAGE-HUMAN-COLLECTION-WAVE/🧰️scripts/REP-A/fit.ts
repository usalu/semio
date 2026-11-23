/** 🧵️ Local silhouette repair of the drawings of one asset file against its own volumes, as the catalog-silhouette guard measures them (default pose, largest boundary distance at most two features, IoU at least 0.92).
 *  The worst boundary point of a failing view is repaired where it lies: an area the body has and the drawing lacks gets a drawn patch on the bone of the solid there (the traced area, widened into the body by the print feature);
 *  an area the drawing has and the body lacks is cut out of the flat shapes that draw it. A change is kept only when the view improves and `validateAsset` stays at its issue count. Writes the ops for `apply.py`.
 *  bun fit.ts <asset.json> <ops-out.json> [scales=50,100,200,500] [views=elevation,front,plan] [maxOps=14] */
import polygonClipping from "polygon-clipping";
import { validateAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { restFrames } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { DEFAULT_PRINT_PROFILE, measureSuitability } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
import { consistency, sweep } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { Mask, boundaryOf, distanceField, distanceToTriangle, fillSmallHoles, measureView, project, worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

type P = { x: number; y: number };
const [path, outFile, scalesArg, viewsArg, maxArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(path!).json()).asset;
const pose = asset.presentation.defaultPose;
const maxOps = Number(maxArg ?? 14);
const ops: any[] = [];
const log = (s: string) => console.log(`[DEBUG] ${s}`);
let lastIssue = "";
const issuesOf = (): number => { const l = validateAsset(structuredClone(asset)); lastIssue = l.length === 0 ? "" : `${l[0]!.code}: ${l[0]!.message.en.slice(0, 260)}`; return l.length; };
const baseIssues = issuesOf();
const queries = [...asset.poses.filter((p: any) => !p.key).map((p: any) => ({ kind: "pose", pose: p.id })), ...asset.clips.flatMap((c: any) => Array.from({ length: 9 }, (_, i) => ({ kind: "clip", clip: c.id, timeS: (c.durationS * i) / 8 })))];
const parentOf = new Map<string, string | null>(asset.rig.bones.map((b: any) => [b.id, b.parent ?? null]));
const vec = (v: any): [number, number, number] => (Array.isArray(v) ? v : [v.x, v.y, v.z]) as [number, number, number];
const frameOf = (f: any) => ({ o: vec(f.o), u: vec(f.u), v: vec(f.v), w: vec(f.w) });
const restAll: any = restFrames(asset.rig);
const rest: any = Object.fromEntries(Object.entries(restAll).map(([k, f]: any) => [k, { frame: frameOf(f.frame) }]));
const cross = (p: P, q: P, r: P) => (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
const lineDist = (p: P, q: P, r: P) => Math.abs(cross(q, r, p)) / (Math.hypot(r.x - q.x, r.y - q.y) || 1);
const rdp = (l: P[], t: number): P[] => { if (l.length < 3) return l; let mi = 0, md = 0; for (let i = 1; i < l.length - 1; i++) { const d = lineDist(l[i]!, l[0]!, l[l.length - 1]!); if (d > md) { md = d; mi = i; } } return md > t ? [...rdp(l.slice(0, mi + 1), t).slice(0, -1), ...rdp(l.slice(mi), t)] : [l[0]!, l[l.length - 1]!]; };
const rdpRing = (r: P[], t: number): P[] => { let a = 0, b = 0; for (let i = 0; i < r.length; i++) { if (r[i]!.x < r[a]!.x) a = i; if (r[i]!.x > r[b]!.x) b = i; } if (a === b) return r; const rot = [...r.slice(a), ...r.slice(0, a)]; const k = rot.indexOf(r[b]!); return [...rdp(rot.slice(0, k + 1), t).slice(0, -1), ...rdp([...rot.slice(k), rot[0]!], t).slice(0, -1)]; };
const areaOf = (r: readonly P[]): number => { let a = 0; for (let i = 0; i < r.length; i++) { const p = r[i]!, q = r[(i + 1) % r.length]!; a += p.x * q.y - q.x * p.y; } return a / 2; };
const insideRing = (p: P, ring: readonly P[]) => { let c = false; for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) if ((ring[i]!.y > p.y) !== (ring[j]!.y > p.y) && p.x < ((ring[j]!.x - ring[i]!.x) * (p.y - ring[i]!.y)) / (ring[j]!.y - ring[i]!.y) + ring[i]!.x) c = !c; return c; };
const insideRings = (p: P, rings: readonly (readonly P[])[]) => rings.reduce((n, r) => n + (insideRing(p, r) ? 1 : 0), 0) % 2 === 1;

/** 🧭️ The outer loops (counter-clockwise, in metres) of the set cells of a grid. */
function loopsOf(g: Uint8Array, W: number, H: number, x0: number, y0: number, cell: number, tol: number): P[][] {
  const at = (i: number, j: number) => (i < 0 || j < 0 || i >= W || j >= H ? 0 : g[j * W + i]!);
  const next = new Map<number, number[]>();
  const key = (i: number, j: number) => j * (W + 1) + i;
  const add = (a: number, b: number) => { const l = next.get(a); if (l) l.push(b); else next.set(a, [b]); };
  for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) if (at(i, j)) {
    if (!at(i, j - 1)) add(key(i, j), key(i + 1, j));
    if (!at(i + 1, j)) add(key(i + 1, j), key(i + 1, j + 1));
    if (!at(i, j + 1)) add(key(i + 1, j + 1), key(i, j + 1));
    if (!at(i - 1, j)) add(key(i, j + 1), key(i, j));
  }
  const out: P[][] = [];
  while (next.size > 0) {
    const [start] = next.keys();
    let cur = start!;
    const loop: number[] = [];
    for (;;) { loop.push(cur); const l = next.get(cur)!; const n = l.pop()!; if (l.length === 0) next.delete(cur); cur = n; if (cur === start || !next.has(cur)) break; }
    const ring = loop.map((k) => ({ x: x0 + (k % (W + 1)) * cell, y: y0 + Math.floor(k / (W + 1)) * cell }));
    if (areaOf(ring) > 0) out.push(rdpRing(ring, tol));
  }
  return out.filter((r) => r.length >= 3).sort((a, b) => Math.abs(areaOf(b)) - Math.abs(areaOf(a)));
}

for (const sc of (scalesArg && scalesArg !== "-" ? scalesArg : "50,100,200,500").split(",")) {
  const scale = Number(sc);
  if (asset.volumes[sc]?.status !== "authored" || asset.representations[sc]?.status !== "authored") continue;
  const feature = featureOf(scale as never), limit = 2 * feature;
  const printM = 0.00026 * scale;
  const baseViews = sweep(asset, scale).length + consistency(asset, scale).length;
  const sel = { kind: "scale", scale: { denominator: scale } } as never;
  const geo = (view: string): any[] => asset.representations[sc].geometry[view];
  const solidBones = new Set<string>(asset.volumes[sc].body.map((s: any) => s.bone));
  const slenderBones = new Set<string>(asset.volumes[sc].body.filter((s: any) => s.slender === true).map((s: any) => s.bone));
  let state: any = evaluate(asset, { kind: "pose", pose }, sel);
  const solids = worldSolids(asset, state.bones as never, scale as never).filter((w: any) => w.texture !== true);
  const tris = new Map<string, Map<string, [P, P, P][]>>();
  const trisOf = (view: string): Map<string, [P, P, P][]> => {
    let m = tris.get(view);
    if (m) return m;
    m = new Map();
    for (const s of solids as any[]) {
      if (s.bone === undefined || s.slender === true) continue;
      const l = m.get(s.bone) ?? [];
      for (let k = 0; k < s.indices.length; k += 3) l.push([0, 1, 2].map((j) => { const i = s.indices[k + j]! * 3; return project(view as never, s.positions[i]!, s.positions[i + 1]!, s.positions[i + 2]!); }) as [P, P, P]);
      m.set(s.bone, l);
    }
    tris.set(view, m);
    return m;
  };
  const look = (view: string) => {
    state = evaluate(asset, { kind: "pose", pose }, sel);
    const m = measureView(asset, state, solids, view as never, scale as never, undefined, feature);
    return { m, score: Math.max(m.hausdorffM / limit, m.iou < 0.925 ? 1 + (0.925 - m.iou) * 10 : 0) };
  };
  /** 🪞️ Bone-local points whose drawing in `view` is `targets` (the placement probed with a temporary shape). */
  const toLocal = (view: string, bone: string, targets: P[], solid?: any): P[] | null => {
    const forward = (points: P[]): P[] | null => {
      const probe = { kind: "poly", points: points.map((p) => [p.x, p.y]), closed: true, id: "rep-a-probe", bone, role: "body", z: 99, filled: true, tone: 0.5, ...(solid === undefined ? {} : { solid }) };
      geo(view).push(probe);
      try {
        const ps: any = evaluate(asset, { kind: "pose", pose }, sel);
        const got = ps.shapes[view].find((s: any) => s.shape.id === "rep-a-probe");
        return got === undefined || got.rings[0].length < points.length ? null : got.rings[0].slice(0, points.length);
      } catch { return null; } finally { geo(view).pop(); }
    };
    const base = forward([{ x: 0, y: 0 }, { x: 0.05, y: 0 }, { x: 0, y: 0.05 }]);
    if (base === null) return null;
    const [o, a, b] = base as [P, P, P];
    const m = { a: (a.x - o.x) / 0.05, b: (a.y - o.y) / 0.05, c: (b.x - o.x) / 0.05, d: (b.y - o.y) / 0.05 };
    const det = m.a * m.d - m.b * m.c;
    if (Math.abs(det) < 0.2) return null;
    const inv = (p: P) => { const dx = p.x - o.x, dy = p.y - o.y; return { x: (m.d * dx - m.c * dy) / det, y: (-m.b * dx + m.a * dy) / det }; };
    let local = targets.map(inv);
    for (let it = 0; it < 5; it++) {
      const got = forward(local);
      if (got === null) return null;
      const err = targets.map((t, i) => ({ x: t.x - got[i]!.x, y: t.y - got[i]!.y }));
      if (Math.max(...err.map((e) => Math.hypot(e.x, e.y))) < Math.max(2e-4, feature / 30)) return local;
      local = local.map((l, i) => { const d = inv({ x: o.x + err[i]!.x, y: o.y + err[i]!.y }); return { x: l.x + d.x, y: l.y + d.y }; });
    }
    return null;
  };
  const extents = new Map<string, Record<"u" | "v" | "w", [number, number]>>();
  const extentOf = (bone: string) => {
    let e = extents.get(bone);
    if (e) return e;
    e = { u: [Infinity, -Infinity], v: [Infinity, -Infinity], w: [Infinity, -Infinity] };
    const f = frameOf(state.bones[bone].frame);
    for (const s of solids as any[]) if (s.bone === bone && s.slender !== true) for (let i = 0; i < s.positions.length; i += 3) for (const k of ["u", "v", "w"] as const) { const d = (s.positions[i]! - f.o[0]) * f[k][0] + (s.positions[i + 1]! - f.o[1]) * f[k][1] + (s.positions[i + 2]! - f.o[2]) * f[k][2]; if (d < e[k][0]) e[k][0] = d; if (d > e[k][1]) e[k][1] = d; }
    extents.set(bone, e);
    return e;
  };
  /** 🍽️ Profile points of a thin plate (an `extruded` shape) of a bone whose middle plane draws `targets` in `view`. */
  const plate = (view: string, f: any, solid: any, restZ: number, targets: P[]): P[] | null => {
    const [pa, pb] = solid.sweep === "w" ? ["u", "v"] : solid.sweep === "v" ? ["u", "w"] : ["v", "w"];
    const c = (solid.z0 + solid.z1) / 2 - (solid.sweep === "w" ? restZ : 0);
    const at = (a: number, b: number): P => project(view as never, f.o[0] + a * f[pa!][0] + b * f[pb!][0] + c * f[solid.sweep][0], f.o[1] + a * f[pa!][1] + b * f[pb!][1] + c * f[solid.sweep][1], f.o[2] + a * f[pa!][2] + b * f[pb!][2] + c * f[solid.sweep][2]);
    const [o, ea, eb] = [at(0, 0), at(1, 0), at(0, 1)];
    const m = { a: ea.x - o.x, b: ea.y - o.y, c: eb.x - o.x, d: eb.y - o.y };
    const det = m.a * m.d - m.b * m.c;
    if (Math.abs(det) < 0.5) return null;
    return targets.map((p) => { const dx = p.x - o.x, dy = p.y - o.y; return { x: (m.d * dx - m.c * dy) / det, y: (-m.b * dx + m.a * dy) / det }; });
  };
  const pathOf = (loops: P[][]): string => loops.map((l) => "M" + l.map((p) => `${p.x.toFixed(4)} ${p.y.toFixed(4)}`).join(" L") + " Z").join(" ");
  const flat = (s: any): boolean => s.solid === undefined && s.followTip === undefined && s.kind !== "member" && s.kind !== "capsule" && s.role !== "ground";

  for (const view of (viewsArg && viewsArg !== "-" ? viewsArg : "elevation,front,plan").split(",")) {
    const tried = new Set<string>();
    let n = 0;
    for (let iter = 0; iter < 3 * maxOps && n < maxOps; iter++) {
      const cur = look(view);
      if (iter === 0) log(`1:${sc} ${view} start iou ${cur.m.iou.toFixed(3)} haus ${(cur.m.hausdorffM * 1000).toFixed(1)} mm`);
      if (cur.score <= 1) break;
      const m = cur.m, cell = m.cell, box = m.box;
      const drawn = new Mask(box, cell);
      for (const g of m.filled) drawn.fill(g);
      for (const g of m.outlines) drawn.fill(g);
      const vol = new Mask(box, cell);
      for (const g of m.reference) vol.fill(g, true);
      if (m.section) for (const g of m.visible) vol.fill(g, true);
      vol.seal();
      const [w, h] = [vol.width, vol.height];
      const holes = Math.round((4 * feature / cell) ** 2);
      const D = fillSmallHoles(drawn.cells, w, h, holes), V = fillSmallHoles(vol.cells, w, h, holes);
      const bd = boundaryOf(D, w, h), bv = boundaryOf(V, w, h);
      const toV = distanceField(bv, w, h), toD = distanceField(bd, w, h);
      const list: { i: number; j: number; d: number }[] = [];
      for (const [from, field] of [[bd, toV], [bv, toD]] as const) for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > limit) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * cell });
      if (list.length === 0 && m.iou >= 0.925) for (const [from, field] of [[bd, toV], [bv, toD]] as const) for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (from[j * w + i] && Math.sqrt(field[j * w + i]!) * cell > 0.8 * limit) list.push({ i, j, d: Math.sqrt(field[j * w + i]!) * cell });
      if (list.length === 0) for (let j = 0; j < h; j += 2) for (let i = 0; i < w; i += 2) if (D[j * w + i] !== V[j * w + i]) list.push({ i, j, d: Math.sqrt(Math.min(toV[j * w + i]!, toD[j * w + i]!)) * cell });
      list.sort((a, b) => b.d - a.d);
      let acted = false;
      let attempts = 0;
      for (const c of list.flatMap((x) => [{ ...x, want: 1 }, { ...x, want: 0 }])) {
        const key = `${c.want},${Math.round(c.i * cell / 0.03)},${Math.round(c.j * cell / 0.03)}`;
        if (tried.has(key)) continue;
        tried.add(key);
        if (++attempts > 14) break;
        let seed = -1, best = Infinity;
        const R = Math.ceil(c.d / cell) + 3;
        for (let j = Math.max(0, c.j - R); j <= Math.min(h - 1, c.j + R); j++) for (let i = Math.max(0, c.i - R); i <= Math.min(w - 1, c.i + R); i++) if (D[j * w + i] !== V[j * w + i] && V[j * w + i] === c.want) { const d2 = (i - c.i) ** 2 + (j - c.j) ** 2; if (d2 < best) { best = d2; seed = j * w + i; } }
        const missing = V[seed] === 1;
        const reach = Math.max(4 * c.d, 0.12) / cell;
        const comp = new Uint8Array(w * h);
        const queue = [seed];
        comp[seed] = 1;
        for (let q = 0; q < queue.length; q++) {
          const k = queue[q]!, i = k % w, j = (k - i) / w;
          for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) {
            const [ni, nj] = [i + di, j + dj];
            if (ni < 0 || nj < 0 || ni >= w || nj >= h || Math.hypot(ni - c.i, nj - c.j) > reach) continue;
            const nk = nj * w + ni;
            if (!comp[nk] && D[nk] !== V[nk] && (V[nk] === 1) === missing) { comp[nk] = 1; queue.push(nk); }
          }
        }
        const centre = { x: box.minX + (queue.reduce((s, k) => s + (k % w), 0) / queue.length + 0.5) * cell, y: box.minY + (queue.reduce((s, k) => s + Math.floor(k / w), 0) / queue.length + 0.5) * cell };
        const at = `(${(box.minX + c.i * cell).toFixed(3)}, ${(box.minY + c.j * cell).toFixed(3)}) ${(c.d * 1000).toFixed(0)} mm, ${queue.length} cells`;
        const snapshot = structuredClone(geo(view));
        const opsLen = ops.length;
        const accept = (what: string): boolean => {
          let after: ReturnType<typeof look>;
          try { after = look(view); } catch (e) { after = { m: cur.m, score: Infinity }; }
          const ok = after.m.iou >= Math.min(cur.m.iou, 0.925) - 0.002 && (after.score < cur.score - 0.01 || (after.score <= cur.score + 1e-9 && after.m.iou > cur.m.iou + 0.0005));
          const drawable = ok && queries.every((q) => { try { evaluate(asset, q as never, sel); return true; } catch { lastIssue = `not drawable at ${JSON.stringify(q)}`; return false; } });
          const printable = ok && drawable && (() => { try { const su: any = measureSuitability(asset, asset.representations[sc], scale as never, DEFAULT_PRINT_PROFILE); if (!su.suitable) lastIssue = `print ${su.violations[0]?.check} ${su.violations[0]?.shapes.join("+")}`; return su.suitable; } catch (e) { lastIssue = `print ${String(e).slice(0, 80)}`; return false; } })();
          const oneBody = ok && drawable && printable && (() => { const found = [...sweep(asset, scale), ...consistency(asset, scale)]; if (found.length > baseViews) lastIssue = `three views: ${found[0]!.message.slice(0, 200)}`; return found.length <= baseViews; })();
          if (ok && drawable && printable && oneBody && issuesOf() <= baseIssues) { log(`1:${sc} ${view} ${what} at ${at}: ${(cur.m.hausdorffM * 1000).toFixed(1)} -> ${(after.m.hausdorffM * 1000).toFixed(1)} mm, iou ${after.m.iou.toFixed(3)}`); return true; }
          asset.representations[sc].geometry[view] = snapshot.map((s: any) => structuredClone(s));
          ops.length = opsLen;
          log(`1:${sc} ${view} rejected ${what} at ${at} (${ok ? `validator ${lastIssue}` : `${(after.m.hausdorffM * 1000).toFixed(1)} mm, iou ${after.m.iou.toFixed(3)}`})`);
          return false;
        };
        const dir = view === "elevation" ? 2 : view === "front" ? 0 : 1;
        const axisOf = (bone: string): "u" | "v" | "w" | undefined => { const f = frameOf(state.bones[bone].frame); const best = (["w", "v", "u"] as const).map((k) => ({ k, a: Math.abs(f[k][dir]) })).sort((p, q) => q.a - p.a)[0]!; return best.a > 0.5 ? best.k : undefined; };
        const slabOf = (bone: string, k: "u" | "v" | "w", thick: boolean) => { const ext = extentOf(bone); const c = (ext[k][0] + ext[k][1]) / 2, t = thick ? (ext[k][1] - ext[k][0]) / 2 : Number(process.env.SLAB ?? 0.0015), base = k === "w" ? rest[bone].frame.o[2] : 0; return { kind: "extruded", sweep: k, z0: +(c - t + base).toFixed(4), z1: +(c + t + base).toFixed(4) }; };
        if (c.want === 1) {
          const raw = new Uint8Array(w * h);
          for (let k = 0; k < w * h; k++) raw[k] = drawn.cells[k]! ? 1 : 0;
          const r = 3, Rw = Math.ceil(Math.max(2 * c.d, 0.05) / cell);
          const out = distanceField(raw, w, h);
          const inv = new Uint8Array(w * h);
          for (let k = 0; k < w * h; k++) inv[k] = out[k]! <= r * r ? 0 : 1;
          const back = distanceField(inv, w, h);
          const slit = new Uint8Array(w * h);
          let nSlit = 0;
          for (let k = 0; k < w * h; k++) if (!raw[k] && back[k]! > r * r && Math.abs((k % w) - c.i) <= Rw && Math.abs(Math.floor(k / w) - c.j) <= Rw) { slit[k] = 1; nSlit++; }
          if (nSlit > 0) {
            const near = distanceField(slit, w, h);
            const fill = new Uint8Array(w * h);
            for (let k = 0; k < w * h; k++) if (slit[k] || (raw[k] && near[k]! <= 9)) fill[k] = 1;
            const loops = loopsOf(fill, w, h, box.minX, box.minY, cell, cell * 0.4).filter((l) => Math.abs(areaOf(l)) > 4 * cell * cell);
            const here = { x: box.minX + (c.i + 0.5) * cell, y: box.minY + (c.j + 0.5) * cell };
            const host = (state.shapes[view] as any[]).filter((e) => e.closed && e.shape.filled !== false && e.shape.role === "body" && e.shape.followTip === undefined && e.shape.kind !== "member" && solidBones.has(e.shape.bone)).map((e) => { let d = insideRings(here, e.rings) ? 0 : Infinity; if (d > 0) for (const ring of e.rings) for (const p of ring) d = Math.min(d, Math.hypot(p.x - here.x, p.y - here.y)); return { e, d }; }).sort((p, q) => p.d - q.d)[0]?.e;
            if (host !== undefined && loops.length > 0) {
              const tmpl = geo(view).find((x: any) => x.id === host.shape.id && x.bone === host.shape.bone);
              const axis = axisOf(host.shape.bone);
              for (const solid of [undefined, ...(axis === undefined ? [] : [slabOf(host.shape.bone, axis, false)])]) {
                if (tmpl === undefined) break;
                const locals: P[][] = [];
                for (const l of loops) { const local = solid === undefined ? toLocal(view, tmpl.bone, l) : plate(view, frameOf(state.bones[tmpl.bone].frame), solid, rest[tmpl.bone].frame.o[2], l); if (local !== null) locals.push(local); }
                if (locals.length !== loops.length) continue;
                let id = `${tmpl.bone}-close-${view[0]}`;
                for (let k = 2; geo(view).some((x: any) => x.id === id); k++) id = `${tmpl.bone}-close-${view[0]}${k}`;
                const shape: any = { ...(locals.length === 1 ? { kind: "poly", points: locals[0]!.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]), closed: true } : { kind: "path", d: pathOf(locals) }), id, bone: tmpl.bone, role: tmpl.role, z: +(tmpl.z - 0.001).toFixed(4), filled: true, tone: tmpl.tone, ...(solid === undefined ? {} : { solid }) };
                for (const k of ["region", "regions", "material"]) if (tmpl[k] !== undefined) shape[k] = tmpl[k];
                geo(view).splice(geo(view).findIndex((x: any) => x.id === tmpl.id) + 1, 0, shape);
                ops.push({ op: "shape-add", scale: sc, view, after: tmpl.id, shape });
                if (accept(`slits closed by ${id} (${nSlit} cells)`)) { acted = true; n++; break; }
              }
              if (acted) break;
            }
          }
        }
        if (seed < 0) continue;
        if (missing) {
          const grow = Math.max(2, Math.ceil(printM / cell));
          const field = distanceField(comp, w, h);
          const patch = new Uint8Array(w * h);
          for (let k = 0; k < w * h; k++) if (comp[k] || (V[k] && field[k]! <= grow * grow)) patch[k] = 1;
          const loop = loopsOf(patch, w, h, box.minX, box.minY, cell, cell * 0.6)[0];
          if (loop === undefined) continue;
          const votes = new Map<string, number>();
          const cellsOf = queue.filter((_, q) => q % Math.max(1, Math.floor(queue.length / 40)) === 0);
          for (const [bone, ts] of trisOf(view)) { let nHit = 0; for (const k of cellsOf) { const p = { x: box.minX + (k % w + 0.5) * cell, y: box.minY + (Math.floor(k / w) + 0.5) * cell }; if (ts.some((t) => distanceToTriangle(p, t) === 0)) nHit++; } if (nHit > 0) votes.set(bone, nHit); }
          const bones: string[] = [];
          for (const [bone] of [...votes].sort((a, b) => b[1] - a[1]).slice(0, 2)) for (let b: string | null | undefined = bone; b; b = parentOf.get(b)) if (!bones.includes(b)) bones.push(b);
          for (const bone of bones.slice(0, 6)) {
            if (slenderBones.has(bone) || !solidBones.has(bone)) continue;
            const mine = geo(view).filter((s: any) => s.role === "body" && s.filled !== false && s.kind !== "member");
            const nearest = (l: any[]) => l.map((s) => { const e = state.shapes[view].find((x: any) => x.shape.id === s.id && x.shape.bone === s.bone); let d = Infinity; if (e) for (const ring of e.rings) for (const p of ring) d = Math.min(d, Math.hypot(p.x - centre.x, p.y - centre.y)); return { s, d }; }).sort((a, b) => a.d - b.d)[0]?.s;
            const tmpl = nearest(mine.filter((s: any) => s.bone === bone)) ?? nearest(mine);
            if (tmpl === undefined) continue;
            let id = `${bone}-fill-${view[0]}`;
            for (let k = 2; geo(view).some((s: any) => s.id === id); k++) id = `${bone}-fill-${view[0]}${k}`;
            const f = frameOf(state.bones[bone].frame);
            const ext = extentOf(bone);
            const dir = view === "elevation" ? 2 : view === "front" ? 0 : 1;
            const order = (["w", "v", "u"] as const).map((k) => ({ k, a: Math.abs(f[k][dir]) })).sort((p, q) => q.a - p.a).filter((x) => x.a > 0.5).map((x) => x.k);
            const slab = (k: "u" | "v" | "w") => { const c = (ext[k][0] + ext[k][1]) / 2, t = Number(process.env.SLAB ?? 0.0015), base = k === "w" ? rest[bone].frame.o[2] : 0; return { kind: "extruded", sweep: k, z0: +(c - t + base).toFixed(4), z1: +(c + t + base).toFixed(4) }; };
            for (const solid of [undefined, ...order.map(slab)]) {
              const local = solid === undefined ? toLocal(view, bone, loop) : plate(view, f, solid, rest[bone].frame.o[2], loop);
              if (local === null) continue;
              const shape: any = { kind: "poly", points: local.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]), closed: true, id, bone, role: tmpl.role, z: +(tmpl.z - 0.001).toFixed(4), filled: true, tone: tmpl.tone, ...(solid === undefined ? {} : { solid }) };
              for (const k of ["region", "regions", "material"]) if (tmpl[k] !== undefined) shape[k] = tmpl[k];
              const o = { op: "shape-add", scale: sc, view, after: tmpl.id, shape };
              geo(view).splice(geo(view).findIndex((x: any) => x.id === tmpl.id) + 1, 0, shape);
              ops.push(o);
              if (accept(`patch ${id}${solid ? ` swept ${(solid as any).sweep}` : ""} (like ${tmpl.id})`)) { acted = true; n++; break; }
            }
            if (acted) break;
          }
        } else {
          const cut = new Uint8Array(w * h);
          const field = distanceField(comp, w, h);
          for (let k = 0; k < w * h; k++) if (comp[k] || (!V[k] && field[k]! <= 2)) cut[k] = 1;
          const cutLoops = loopsOf(cut, w, h, box.minX, box.minY, cell, cell * 0.4);
          if (cutLoops.length === 0) continue;
          const cutter = cutLoops.map((l) => [l.map((p) => [p.x, p.y] as [number, number])]);
          const sample = queue.filter((_, q) => q % Math.max(1, Math.floor(queue.length / 60)) === 0).map((k) => ({ x: box.minX + (k % w + 0.5) * cell, y: box.minY + (Math.floor(k / w) + 0.5) * cell }));
          const covering = (state.shapes[view] as any[]).filter((e) => e.closed && e.shape.role !== "ground" && e.shape.followTip === undefined && e.shape.kind !== "member" && sample.some((p) => insideRings(p, e.rings)));
          if (covering.length === 0) {
            const here = { x: box.minX + (c.i + 0.5) * cell, y: box.minY + (c.j + 0.5) * cell };
            const stroke = (state.shapes[view] as any[]).filter((e) => !(e.closed && e.shape.filled !== false) && e.shape.role !== "ground").map((e) => { let d = Infinity; for (const ring of e.rings) for (let i = 0; i + 1 < ring.length + (e.closed ? 1 : 0); i++) { const p = ring[i]!, q = ring[(i + 1) % ring.length]!; const dx = q.x - p.x, dy = q.y - p.y, l = dx * dx + dy * dy; const t = l === 0 ? 0 : Math.max(0, Math.min(1, ((here.x - p.x) * dx + (here.y - p.y) * dy) / l)); d = Math.min(d, Math.hypot(here.x - p.x - t * dx, here.y - p.y - t * dy)); } return { e, d }; }).filter((x) => x.d <= 3 * cell).sort((p, q) => p.d - q.d)[0];
            if (stroke !== undefined) {
              const index = geo(view).findIndex((x: any) => x.id === stroke.e.shape.id && x.bone === stroke.e.shape.bone);
              if (index >= 0) { const old = geo(view)[index]; geo(view).splice(index, 1); ops.push({ op: "shape-delete", scale: sc, view, id: old.id }); if (accept(`stroke ${old.id} outside the body removed`)) { acted = true; n++; break; } }
            }
          }
          if (covering.length === 0 || covering.some((e) => e.shape.followTip !== undefined || e.shape.kind === "member" || e.shape.role === "ground")) { log(`1:${sc} ${view} excess at ${at} is drawn by ${covering.map((e) => `${e.shape.id}[${e.shape.bone}${e.shape.solid ? "," + e.shape.solid.kind : ""}]`).join(", ") || "strokes"}: not cut`); continue; }
          const pieces = covering.map((e) => {
            const subject = (e.rings as P[][]).map((r) => r.map((p) => [p.x, p.y] as [number, number]));
            try {
              const rest2 = polygonClipping.difference(subject.length === 1 ? [subject[0]!] : polygonClipping.xor([subject[0]!], ...subject.slice(1).map((r) => [r])) as never, ...(cutter as never[])) as unknown as [number, number][][][];
              return rest2.flatMap((poly) => poly.map((ring) => ring.slice(0, -1).map(([x, y]) => ({ x: x!, y: y! })))).filter((l) => Math.abs(areaOf(l)) > 4 * cell * cell).map((l) => rdpRing(l, cell * 0.25));
            } catch { return null; }
          });
          if (pieces.some((x) => x === null)) continue;
          for (const mode of ["keep", "thin", "plates"] as const) {
            let fine = true;
            for (const [q, e] of covering.entries()) {
              const loops = pieces[q]!;
              const index = geo(view).findIndex((x: any) => x.id === e.shape.id && x.bone === e.shape.bone);
              const old = geo(view)[index];
              if (old === undefined) { fine = false; break; }
              if (loops.length === 0) { geo(view).splice(index, 1); ops.push({ op: "shape-delete", scale: sc, view, id: old.id }); continue; }
              const axis = solidBones.has(old.bone) ? axisOf(old.bone) : undefined;
              const solid = (mode === "keep" && flat(old)) || axis === undefined ? undefined : slabOf(old.bone, axis, mode === "keep");
              if (solid === undefined && !flat(old)) { fine = false; break; }
              const locals: P[][] = [];
              for (const l of loops) { const local = solid === undefined ? toLocal(view, old.bone, l) : plate(view, frameOf(state.bones[old.bone].frame), solid, rest[old.bone].frame.o[2], l); if (local === null) { fine = false; break; } locals.push(local); }
              if (!fine) break;
              const { kind: _k, points: _p, d: _d, closed: _c, x: _x, y: _y, w: _w, h: _h, r: _r, x0: _x0, y0: _y0, x1: _x1, y1: _y1, r0: _r0, r1: _r1, cx: _cx, cy: _cy, rx: _rx, ry: _ry, rotationDeg: _rd, solid: _s, ...keep } = old;
              const shape = { ...(locals.length === 1 ? { kind: "poly", points: locals[0]!.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]), closed: true } : { kind: "path", d: pathOf(locals) }), ...keep, ...(solid === undefined ? {} : { solid }) };
              geo(view)[index] = shape;
              ops.push({ op: "shape", scale: sc, view, id: old.id, shape });
            }
            if (!fine) { asset.representations[sc].geometry[view] = snapshot.map((x: any) => structuredClone(x)); ops.length = opsLen; continue; }
            if (accept(`cut (${mode}) of ${covering.map((e) => e.shape.id).join(", ")}`)) { acted = true; n++; break; }
          }
        }
        if (!acted) {
          const here = { x: box.minX + (c.i + 0.5) * cell, y: box.minY + (c.j + 0.5) * cell };
          const usable = (e: any) => e.closed && e.shape.followTip === undefined && e.shape.kind !== "member" && e.shape.role !== "ground" && solidBones.has(e.shape.bone);
          const near = (state.shapes[view] as any[]).filter(usable).map((e) => { let d = insideRings(here, e.rings) ? 0 : Infinity; if (d > 0) for (const ring of e.rings) for (const p of ring) d = Math.min(d, Math.hypot(p.x - here.x, p.y - here.y)); return { e, d }; }).filter((x) => x.d <= c.d + 4 * cell).sort((p, q) => p.d - q.d).slice(0, 3);
          for (const { e } of near) {
            const own = new Mask(box, cell);
            own.fill(e.rings);
            const R = Math.ceil(Math.max(1.5 * c.d, limit) / cell);
            const field = distanceField(own.cells, w, h);
            const next = new Uint8Array(w * h);
            for (let k = 0; k < w * h; k++) if ((own.cells[k] && V[k]) || (V[k] && !D[k] && field[k]! <= R * R)) next[k] = 1;
            const loops = loopsOf(next, w, h, box.minX, box.minY, cell, cell * 0.5).filter((l) => Math.abs(areaOf(l)) > 6 * cell * cell);
            if (loops.length === 0) continue;
            const index = geo(view).findIndex((x: any) => x.id === e.shape.id && x.bone === e.shape.bone);
            const old = geo(view)[index];
            if (old === undefined) continue;
            const axis = axisOf(old.bone);
            for (const mode of ["keep", "thin"] as const) {
              const solid = (mode === "keep" && flat(old)) || axis === undefined ? undefined : slabOf(old.bone, axis, mode === "keep");
              if (solid === undefined && (!flat(old) || mode === "thin")) continue;
              const locals: P[][] = [];
              for (const l of loops) { const local = solid === undefined ? toLocal(view, old.bone, l) : plate(view, frameOf(state.bones[old.bone].frame), solid, rest[old.bone].frame.o[2], l); if (local !== null) locals.push(local); }
              if (locals.length !== loops.length) continue;
              const { kind: _k, points: _p, d: _d, closed: _c, x: _x, y: _y, w: _w, h: _h, r: _r, x0: _x0, y0: _y0, x1: _x1, y1: _y1, r0: _r0, r1: _r1, cx: _cx, cy: _cy, rx: _rx, ry: _ry, rotationDeg: _rd, solid: _s, ...keep } = old;
              const shape = { ...(locals.length === 1 ? { kind: "poly", points: locals[0]!.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]), closed: true } : { kind: "path", d: pathOf(locals) }), ...keep, ...(solid === undefined ? {} : { solid }) };
              geo(view)[index] = shape;
              ops.push({ op: "shape", scale: sc, view, id: old.id, shape });
              if (accept(`retrace (${mode}) of ${old.id}`)) { acted = true; n++; break; }
            }
            if (acted) break;
          }
        }
        if (!acted) {
          const Rw = Math.ceil(Math.max(3 * c.d, 0.06) / cell);
          const inWin = (k: number) => Math.abs((k % w) - c.i) <= Rw && Math.abs(Math.floor(k / w) - c.j) <= Rw;
          const E = new Uint8Array(w * h), M = new Uint8Array(w * h);
          let nE = 0, nM = 0;
          for (let k = 0; k < w * h; k++) if (inWin(k) && D[k] !== V[k]) { if (V[k]) { M[k] = 1; nM++; } else { E[k] = 1; nE++; } }
          const usable = (e: any) => e.closed && e.shape.followTip === undefined && e.shape.kind !== "member" && e.shape.role !== "ground" && solidBones.has(e.shape.bone);
          const cuts: { e: any; loops: P[][] }[] = [];
          if (nE > 0) {
            const fieldE = distanceField(E, w, h);
            const cutMask = new Uint8Array(w * h);
            for (let k = 0; k < w * h; k++) if (E[k] || (!V[k] && fieldE[k]! <= 2)) cutMask[k] = 1;
            const cutter = loopsOf(cutMask, w, h, box.minX, box.minY, cell, cell * 0.4).map((l) => [l.map((p) => [p.x, p.y] as [number, number])]);
            for (const e of (state.shapes[view] as any[]).filter(usable)) {
              const own = new Mask(box, cell);
              own.fill(e.rings);
              let hit = false;
              for (let k = 0; k < w * h && !hit; k++) if (E[k] && own.cells[k]) hit = true;
              if (!hit || cutter.length === 0) continue;
              const subject = (e.rings as P[][]).map((r) => r.map((p) => [p.x, p.y] as [number, number]));
              try {
                const left = polygonClipping.difference(subject.length === 1 ? [subject[0]!] : polygonClipping.xor([subject[0]!], ...subject.slice(1).map((r) => [r])) as never, ...(cutter as never[])) as unknown as [number, number][][][];
                cuts.push({ e, loops: left.flatMap((poly) => poly.map((ring) => ring.slice(0, -1).map(([x, y]) => ({ x: x!, y: y! })))).filter((l) => Math.abs(areaOf(l)) > 4 * cell * cell).map((l) => rdpRing(l, cell * 0.25)) });
              } catch { /* this shape keeps its outline */ }
            }
          }
          let patchLoops: P[][] = [];
          let patchBones: string[] = [];
          if (nM > 0) {
            const grow = Math.max(2, Math.ceil(printM / cell));
            const fieldM = distanceField(M, w, h);
            const patch = new Uint8Array(w * h);
            for (let k = 0; k < w * h; k++) if (M[k] || (V[k] && fieldM[k]! <= grow * grow)) patch[k] = 1;
            patchLoops = loopsOf(patch, w, h, box.minX, box.minY, cell, cell * 0.6).filter((l) => Math.abs(areaOf(l)) > 6 * cell * cell);
            const votes = new Map<string, number>();
            const cells: number[] = [];
            for (let k = 0; k < w * h; k++) if (M[k]) cells.push(k);
            for (const [bone, ts] of trisOf(view)) { let nHit = 0; for (const k of cells.filter((_, q) => q % Math.max(1, Math.floor(cells.length / 40)) === 0)) { const p = { x: box.minX + (k % w + 0.5) * cell, y: box.minY + (Math.floor(k / w) + 0.5) * cell }; if (ts.some((t) => distanceToTriangle(p, t) === 0)) nHit++; } if (nHit > 0 && !slenderBones.has(bone) && solidBones.has(bone)) votes.set(bone, nHit); }
            patchBones = [...votes].sort((a, b) => b[1] - a[1]).slice(0, 2).map((x) => x[0]);
          }
          const variants: (undefined | "slab")[] = [undefined, "slab"];
          for (const bone of patchLoops.length > 0 ? patchBones : [""]) {
            for (const variant of variants) {
              let fine = true;
              for (const { e, loops } of cuts) {
                const index = geo(view).findIndex((x: any) => x.id === e.shape.id && x.bone === e.shape.bone);
                const old = geo(view)[index];
                if (old === undefined) { fine = false; break; }
                if (loops.length === 0) { geo(view).splice(index, 1); ops.push({ op: "shape-delete", scale: sc, view, id: old.id }); continue; }
                const axis = axisOf(old.bone);
                const solid = flat(old) || axis === undefined ? undefined : slabOf(old.bone, axis, false);
                if (solid === undefined && !flat(old)) { fine = false; break; }
                const locals: P[][] = [];
                for (const l of loops) { const local = solid === undefined ? toLocal(view, old.bone, l) : plate(view, frameOf(state.bones[old.bone].frame), solid, rest[old.bone].frame.o[2], l); if (local !== null) locals.push(local); }
                if (locals.length !== loops.length) { fine = false; break; }
                const { kind: _k, points: _p, d: _d, closed: _c, x: _x, y: _y, w: _w, h: _h, r: _r, x0: _x0, y0: _y0, x1: _x1, y1: _y1, r0: _r0, r1: _r1, cx: _cx, cy: _cy, rx: _rx, ry: _ry, rotationDeg: _rd, solid: _s, ...keep } = old;
                const shape = { ...(locals.length === 1 ? { kind: "poly", points: locals[0]!.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]), closed: true } : { kind: "path", d: pathOf(locals) }), ...keep, ...(solid === undefined ? {} : { solid }) };
                geo(view)[index] = shape;
                ops.push({ op: "shape", scale: sc, view, id: old.id, shape });
              }
              if (fine && patchLoops.length > 0) {
                const axis = axisOf(bone);
                const solid = variant === undefined ? undefined : axis === undefined ? null : slabOf(bone, axis, false);
                const mine = geo(view).filter((x: any) => x.role === "body" && x.filled !== false && x.kind !== "member");
                const tmpl = mine.find((x: any) => x.bone === bone) ?? mine[0];
                if (solid === null || tmpl === undefined) fine = false;
                else {
                  const locals: P[][] = [];
                  for (const l of patchLoops) { const local = solid === undefined ? toLocal(view, bone, l) : plate(view, frameOf(state.bones[bone].frame), solid, rest[bone].frame.o[2], l); if (local !== null) locals.push(local); }
                  if (locals.length !== patchLoops.length) fine = false;
                  else {
                    let id = `${bone}-fill-${view[0]}`;
                    for (let k = 2; geo(view).some((x: any) => x.id === id); k++) id = `${bone}-fill-${view[0]}${k}`;
                    const shape: any = { ...(locals.length === 1 ? { kind: "poly", points: locals[0]!.map((p) => [+p.x.toFixed(4), +p.y.toFixed(4)]), closed: true } : { kind: "path", d: pathOf(locals) }), id, bone, role: tmpl.role, z: +(tmpl.z - 0.001).toFixed(4), filled: true, tone: tmpl.tone, ...(solid === undefined ? {} : { solid }) };
                    for (const k of ["region", "regions", "material"]) if (tmpl[k] !== undefined) shape[k] = tmpl[k];
                    geo(view).splice(geo(view).findIndex((x: any) => x.id === tmpl.id) + 1, 0, shape);
                    ops.push({ op: "shape-add", scale: sc, view, after: tmpl.id, shape });
                  }
                }
              }
              if (!fine) { asset.representations[sc].geometry[view] = snapshot.map((x: any) => structuredClone(x)); ops.length = opsLen; continue; }
              if (accept(`window (${cuts.length} cut, ${patchLoops.length} filled${bone ? ` on ${bone}` : ""}${variant ? " as plate" : ""})`)) { acted = true; n++; break; }
              if (patchLoops.length === 0) break;
            }
            if (acted) break;
          }
        }
        if (acted) break;
      }
      if (!acted) { log(`1:${sc} ${view}: no further repair`); break; }
    }
    const end = look(view);
    console.log(`${asset.id} 1:${sc} ${view} iou ${end.m.iou.toFixed(3)} haus ${(end.m.hausdorffM * 1000).toFixed(1)} mm ${end.score <= 1 && end.m.iou >= 0.92 ? "ok" : "FAIL"} (${n} ops)`);
  }
}
await Bun.write(outFile!, JSON.stringify(ops));
console.log(`${asset.id}: ${ops.length} ops, validateAsset ${issuesOf()} issues (before ${baseIssues})`);
