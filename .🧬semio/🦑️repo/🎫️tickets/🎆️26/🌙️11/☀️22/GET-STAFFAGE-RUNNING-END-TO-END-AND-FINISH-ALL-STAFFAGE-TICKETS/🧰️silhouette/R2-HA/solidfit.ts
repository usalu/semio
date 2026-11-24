/** 🧬️ Refits drawn shapes that are solids (`revolved` about their own bone, `extruded`) to the body solids they draw, in every view that has the shape: a revolved shape gets the axis through the middle of the solids (`axisX`, `axisZ`) and the mean radius of every station as its profile; an extruded shape gets the exact outline of the solids seen along its sweep as its profile (`z=thin` makes it a slab of the smallest feature, or of `slab` mm, in the middle of the solids, for a part the view sees at a slant; `z=fit` also sets `z0`..`z1` to the extent of the solids along the sweep and drops `zTip`). Writes `shape-set` ops (points and solid) merged into the ops file.
 *  bun solidfit.ts <slug|file.json> <scale> <ops-out.json|-> <spec>… [--base ops.json]
 *  spec: <shape id>=<solid id>[+<solid id>…][@key=value,…]; a solid id may end in `*`; keys: parts (a revolved shape in that many lengths, each about its own axis, for a limb whose middle line leans against the bone; the further lengths are added as `<id>-2`, …), from, to (the part of the length, 0..1, whose stations set the axis of a revolved shape; the rest is hidden in the body), center (mean: the weighted mean instead of the middle of the extremes), round (1: one round fit for all views), views (joined by `+`, default every view that has the shape as a solid), z (fit), n (stations of a revolved profile, default 24), tol (mm of the outline simplification), grow (mm added to every radius), kind (rev or ext: turns a shape into that solid; ext needs sweep), layers (an extruded shape in that many slabs along its sweep, each with the outline of the solids of its depth, `<id>`, `<id>-l2`, …: the true stack for a part the view sees at a slant), holes (1: an extruded shape keeps the openings of its outline as further rings of a path), pieces (an extruded shape cut across its length into that many shapes `<id>`, `<id>-2`, …: a bent slender bar drawn in one piece is taken by the guard for a straight strip as wide as its bend), sweep (u, v or w; auto: the bone axis nearest to the view; thin: the axis along which the solids are thinnest, the true prism of a flat or slender part), like (an existing shape of the view after which a shape that does not exist yet is added as an extrusion, with tone, material and region of its solid), zorder (the z of such a new shape) */
import polygonClipping from "polygon-clipping";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { restFrames } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { worldFrameOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/🦴️skeleton/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { fileOf } from "./people.ts";

type P = [number, number];
const args = process.argv.slice(2);
const baseAt = args.indexOf("--base");
const base = baseAt < 0 ? undefined : args.splice(baseAt, 2)[1];
const [name, scaleArg, out, ...specs] = args as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (base !== undefined) applyOps(asset, await readOps(base));
const scale = Number(scaleArg), feature = featureOf(scale);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const worlds = worldSolids(asset, state.bones as never, scale) as any[];
const REST = restFrames(asset.rig) as Record<string, { frame: any }>;
const r5 = (n: number): number => Math.round(n * 1e5) / 1e5;
const pick = (ids: string): any[] => ids.split("+").flatMap((id) => {
  const hit = worlds.filter((w) => (id.endsWith("*") ? w.id.startsWith(id.slice(0, -1)) : w.id === id));
  if (hit.length === 0) throw new Error(`no solid "${id}"`);
  return hit;
});
const localOf = (bone: string, list: any[]): { pts: number[][]; tris: number[][] }[] => {
  const f = state.bones[bone].frame as { o: any; u: any; v: any; w: any };
  return list.map((w) => {
    const pts: number[][] = [];
    for (let i = 0; i < w.positions.length; i += 3) { const d = [w.positions[i] - f.o.x, w.positions[i + 1] - f.o.y, w.positions[i + 2] - f.o.z]; pts.push([d[0]! * f.u.x + d[1]! * f.u.y + d[2]! * f.u.z, d[0]! * f.v.x + d[1]! * f.v.y + d[2]! * f.v.z, d[0]! * f.w.x + d[1]! * f.w.y + d[2]! * f.w.z]); }
    const tris: number[][] = [];
    for (let t = 0; t < w.indices.length; t += 3) tris.push([w.indices[t], w.indices[t + 1], w.indices[t + 2]]);
    return { pts, tris };
  });
};
const simplify = (ring: P[], tol: number): P[] => {
  const pts = ring.slice(0, -1);
  let far = 0, best = -1;
  for (let i = 1; i < pts.length; i++) { const d = Math.hypot(pts[i]![0] - pts[0]![0], pts[i]![1] - pts[0]![1]); if (d > best) [best, far] = [d, i]; }
  const dp = (list: P[]): P[] => {
    if (list.length < 3) return list;
    const [a, b] = [list[0]!, list[list.length - 1]!];
    const l = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1;
    let [worst, at] = [0, 0];
    for (let i = 1; i < list.length - 1; i++) { const d = Math.abs((b[0] - a[0]) * (list[i]![1] - a[1]) - (b[1] - a[1]) * (list[i]![0] - a[0])) / l; if (d > worst) [worst, at] = [d, i]; }
    return worst <= tol ? [a, b] : [...dp(list.slice(0, at + 1)).slice(0, -1), ...dp(list.slice(at))];
  };
  return [...dp(pts.slice(0, far + 1)).slice(0, -1), ...dp([...pts.slice(far), pts[0]!]).slice(0, -1)];
};
const areaOf = (r: P[]): number => { let s = 0; for (let i = 0; i < r.length; i++) { const [a, b] = [r[i]!, r[(i + 1) % r.length]!]; s += a[0] * b[1] - b[0] * a[1]; } return Math.abs(s / 2); };

const ops: Op[] = [];
for (const spec of specs) {
  const [head, tail] = spec.split("@") as [string, string | undefined];
  const [id, ids] = head.split("=") as [string, string | undefined];
  const opt = Object.fromEntries((tail ?? "").split(",").filter(Boolean).map((kv) => kv.split("=") as [string, string]));
  const list = pick(ids ?? id);
  const views = (opt.views?.split("+") ?? ["elevation", "front", "plan"]).filter((v) => opt.like !== undefined || (asset.representations[scaleArg!].geometry[v] as any[]).some((s) => s.id === id && (s.solid !== undefined || opt.kind !== undefined)));
  if (views.length === 0) throw new Error(`no solid shape "${id}"`);
  for (const view of views) {
    const drawn = asset.representations[scaleArg!].geometry[view] as any[];
    const like = opt.like === undefined ? undefined : drawn.find((s) => s.id === opt.like);
    if (opt.like !== undefined && like === undefined) throw new Error(`${view}: no shape "${opt.like}" to draw "${id}" like`);
    const source = (asset.volumes[scaleArg!].body as any[]).find((s) => s.id === list[0].id);
    const region = (asset.volumes[scaleArg!].thermal as any[]).find((t) => t.of === list[0].id)?.region ?? like?.region;
    const fresh = drawn.find((s) => s.id === id) === undefined;
    const shape = drawn.find((s) => s.id === id) ?? { id, bone: list[0].bone, role: source.role === "detail" ? "detail" : "body", z: Number(opt.zorder ?? like.z), filled: true, tone: source.tone ?? like.tone, ...(region === undefined ? {} : { region }), ...(source.material === undefined ? {} : { material: source.material }) };
    if (fresh) opt.kind ??= "ext";
    const bone = shape.bone as string;
    const local = localOf(bone, list);
    const all = local.flatMap((l) => l.pts);
    const kind = opt.kind ?? (shape.solid.kind === "revolved" ? "rev" : "ext");
    const rest = worldFrameOf(REST[bone]!.frame) as { o: number[]; u: number[]; v: number[]; w: number[] };
    if (kind === "rev") {
      if (opt.kind === undefined && shape.solid !== undefined && (shape.solid.kind !== "revolved" || shape.solid.axis !== "u" || shape.solid.axisBone !== bone || shape.solid.mirror !== true)) throw new Error(`${view} ${id}: not a mirrored body of revolution about its own bone`);
      const n = Number(opt.n ?? 24);
      const [lo, hi] = [Math.min(...all.map((q) => q[0]!)), Math.max(...all.map((q) => q[0]!))];
      const step = (hi - lo) / n;
      const dirs = Array.from({ length: 36 }, (_, i) => [Math.cos((i * Math.PI) / 18), Math.sin((i * Math.PI) / 18)] as P);
      const bins = Array.from({ length: n + 1 }, (_, i) => all.filter((q) => Math.abs(q[0]! - (lo + i * step)) <= step * 0.75));
      const support = (bin: number[][], c: P): number[] => dirs.map((d) => Math.max(...bin.map((q) => (q[1]! - c[0]) * d[0] + (q[2]! - c[1]) * d[1])));
      let c: P = [0, 0];
      const weights = bins.map((bin) => (bin.length < 3 ? 0 : (() => { const h = support(bin, [0, 0]); return (Math.max(...h) + Math.max(...h.map((_, k) => h[(k + 18) % 36]!))) / 2; })()));
      {
        let [sv, sw, sum] = [0, 0, 0];
        bins.forEach((bin, i) => { if (bin.length < 3) return; const [v0, v1, w0, w1] = [Math.min(...bin.map((q) => q[1]!)), Math.max(...bin.map((q) => q[1]!)), Math.min(...bin.map((q) => q[2]!)), Math.max(...bin.map((q) => q[2]!))]; const g = weights[i]! ** 2; sv += g * (v0 + v1) / 2; sw += g * (w0 + w1) / 2; sum += g; });
        c = [sv / sum, sw / sum];
      }
      const f = state.bones[bone].frame as { o: any; u: any; v: any; w: any };
      const d = view === "elevation" ? [0, 0, 1] : view === "front" ? [1, 0, 0] : [0, 1, 0];
      const dl: P = [d[0]! * f.v.x + d[1]! * f.v.y + d[2]! * f.v.z, d[0]! * f.w.x + d[1]! * f.w.y + d[2]! * f.w.z];
      const across = Math.hypot(dl[0], dl[1]);
      const e: P | undefined = opt.round === "1" || across < 0.35 ? undefined : [-dl[1] / across, dl[0] / across];
      const det = rest.v[0]! * rest.w[2]! - rest.v[2]! * rest.w[0]!;
      if (Math.abs(det) < 0.2) throw new Error(`${view} ${id}: the bone rests too flat for an offset axis (det ${det.toFixed(3)})`);
      const grow = Number(opt.grow ?? 0) / 1000;
      const parts = Number(opt.parts ?? 1);
      const filled = bins.map((bin, i) => (bin.length >= 3 ? i : -1)).filter((i) => i >= 0);
      let previous = id;
      for (let part = 0; part < parts; part++) {
        const [a, b] = [filled[Math.floor((part * (filled.length - 1)) / parts)]!, filled[Math.ceil(((part + 1) * (filled.length - 1)) / parts)]!];
        const inPart = (i: number): boolean => i >= a && i <= b && bins[i]!.length >= 3;
        const halves = bins.map((bin, i) => (!inPart(i) || e === undefined ? undefined : [Math.max(...bin.map((q) => (q[1]! - c[0]) * e[0] + (q[2]! - c[1]) * e[1])), Math.max(...bin.map((q) => -(q[1]! - c[0]) * e[0] - (q[2]! - c[1]) * e[1]))] as P));
        let bend = 0;
        let cp: P = c;
        if (e !== undefined) {
          const span = (i: number): boolean => parts > 1 || (weights[i]! > 0.5 * Math.max(...weights) && i >= Number(opt.from ?? 0) * n && i <= Number(opt.to ?? 1) * n);
          const mids = halves.filter((h, i) => h !== undefined && span(i)).map((h) => (h![0] - h![1]) / 2);
          const ce = (Math.max(...mids) + Math.min(...mids)) / 2;
          mids.forEach((m) => { bend = Math.max(bend, Math.abs(m - ce)); });
          cp = [c[0] + ce * e[0], c[1] + ce * e[1]];
        }
        const axisX = r5((cp[0] * rest.w[2]! - cp[1] * rest.v[2]!) / det), axisZ = r5((-cp[0] * rest.w[0]! + cp[1] * rest.v[0]!) / det);
        const L = [axisX * rest.u[0]! + axisZ * rest.u[2]!, axisX * rest.v[0]! + axisZ * rest.v[2]!, axisX * rest.w[0]! + axisZ * rest.w[2]!];
        const gap = Math.abs(axisZ - L[2]!);
        let clamp = 0, spread = 0;
        const upper: P[] = [];
        bins.forEach((bin, i) => {
          if (!inPart(i)) return;
          const h = support(bin, [L[1]!, L[2]!]);
          const r = (halves[i] === undefined ? (Math.max(...h) + Math.min(...h)) / 2 : (halves[i]![0] + halves[i]![1]) / 2) + grow;
          spread = Math.max(spread, e === undefined ? (Math.max(...h) - Math.min(...h)) / 2 : bend);
          clamp = Math.max(clamp, gap - r);
          upper.push([r5(lo + i * step), r5(L[1]! + Math.sqrt(Math.max(r * r - gap * gap, 0)))]);
        });
        const points: P[] = [[upper[0]![0], r5(L[1]!)], ...upper, [upper[upper.length - 1]![0], r5(L[1]!)]];
        const set = { kind: "poly", points, closed: true, solid: { kind: "revolved", axisBone: bone, axis: "u", axisX, axisZ, mirror: true } };
        const pid = part === 0 ? id : `${id}-${part + 1}`;
        ops.push(part === 0 ? { op: "shape-set", scale: scaleArg, view, id, set } : { op: "shape-add", scale: scaleArg, view, after: previous, shape: { ...shape, id: pid, ...set } });
        previous = pid;
        console.log(`${view} ${pid}: revolved about the axis at v ${(cp[0] * 1000).toFixed(1)} w ${(cp[1] * 1000).toFixed(1)} mm (axisX ${axisX}, axisZ ${axisZ}), ${upper.length} stations, radii ${(Math.min(...upper.map((p) => p[1] - L[1]!)) * 1000).toFixed(1)}..${(Math.max(...upper.map((p) => p[1] - L[1]!)) * 1000).toFixed(1)} mm, ${e === undefined ? "round fit, out of round" : "fit across the view, bend"} ${(spread * 1000).toFixed(1)} mm, clamped ${(Math.max(clamp, 0) * 1000).toFixed(1)} mm`);
      }
    } else {
      const fr = state.bones[bone].frame as { o: any; u: any; v: any; w: any };
      const dv = view === "elevation" ? [0, 0, 1] : view === "front" ? [1, 0, 0] : [0, 1, 0];
      const along = (["u", "v", "w"] as const).map((x) => Math.abs(dv[0]! * fr[x].x + dv[1]! * fr[x].y + dv[2]! * fr[x].z));
      const auto = (["u", "v", "w"] as const)[along.indexOf(Math.max(...along))]!;
      const extents = [0, 1, 2].map((x) => Math.max(...all.map((q) => q[x]!)) - Math.min(...all.map((q) => q[x]!)));
      const least = (["u", "v", "w"] as const)[extents.indexOf(Math.min(...extents))]!;
      const sweep = (opt.sweep === "thin" ? least : opt.sweep === "auto" ? auto : opt.sweep ?? shape.solid?.sweep ?? auto) as "u" | "v" | "w";
      console.log(`${view} ${id}: the view runs ${along.map((x, n) => `${"uvw"[n]} ${x.toFixed(2)}`).join(" ")} along the bone axes`);
      if (shape.solid?.weights !== undefined) throw new Error(`${view} ${id}: a skinned extrusion`);
      const [i, j, k] = sweep === "w" ? [0, 1, 2] : sweep === "u" ? [1, 2, 0] : [0, 2, 1];
      const layers = Number(opt.layers ?? 1);
      const [k0, k1] = [Math.min(...all.map((q) => q[k!]!)), Math.max(...all.map((q) => q[k!]!))];
      for (let layer = 0; layer < layers; layer++) {
      const [lo, hi] = [k0 + ((k1 - k0) * layer) / layers, k0 + ((k1 - k0) * (layer + 1)) / layers];
      const inLayer = (l: { pts: number[][] }, t: number[]): boolean => { const m = (l.pts[t[0]!]![k!]! + l.pts[t[1]!]![k!]! + l.pts[t[2]!]![k!]!) / 3; return layers === 1 || (m >= lo - 1e-9 && m <= hi + 1e-9); };
      const tris: P[][][] = [];
      for (const l of local) for (const t of l.tris) {
        if (!inLayer(l, t)) continue;
        const [a, b, c] = t.map((x) => [Math.round(l.pts[x!]![i!]! * 1e6) / 1e6, Math.round(l.pts[x!]![j!]! * 1e6) / 1e6] as P) as [P, P, P];
        if (Math.abs((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])) > 1e-10) tris.push([[a, b, c, a]]);
      }
      const unite = (list: P[][][], digits: number): P[][][] => {
        const snap = (p: P): P => [Math.round(p[0] * digits) / digits, Math.round(p[1] * digits) / digits];
        let acc: P[][][] = [];
        const clean = list.map((poly) => poly.map((ring) => ring.map(snap))).filter((poly) => { const [a, b, c] = poly[0] as [P, P, P]; return Math.abs((b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])) > 1e-10; });
        for (let t = 0; t < clean.length; t += 200) acc = polygonClipping.union(acc as never, ...(clean.slice(t, t + 200) as never[])) as never;
        return acc;
      };
      let acc: P[][][] = [];
      for (const digits of [1e6, 1e5, 2e4, 1e4]) { try { acc = unite(tris, digits); break; } catch (error) { if (digits === 1e4) throw error; } }
      const tol = (opt.tol === undefined ? (feature * 1000) / 60 : Number(opt.tol)) / 1000;
      const rings = acc.map((poly) => simplify(poly[0]!, tol)).filter((r) => r.length >= 3).sort((a, b) => areaOf(b) - areaOf(a));
      if (rings.length > 1 && Number(opt.pieces ?? 1) > 1) console.log(`${view} ${id}: ${rings.length} outlines, the largest is kept (areas ${rings.map((r) => areaOf(r).toFixed(5)).join(" ")})`);
      const zb = sweep === "w" ? rest.o[2]! : 0;
      const [z0, z1] = layers === 1 ? [r5(k0 + zb), r5(k1 + zb)] : [r5(lo + zb), r5(hi + zb)];
      if (rings.length === 0) continue;
      const thin = Math.min((z1 - z0) / 2, Math.max(feature / 2, Number(opt.slab ?? 0) / 2000));
      const solid = layers > 1 ? { kind: "extruded", sweep, z0, z1 } : opt.z === "thin" ? { kind: "extruded", sweep, z0: r5((z0 + z1) / 2 - thin), z1: r5((z0 + z1) / 2 + thin) } : opt.z === "fit" || shape.solid === undefined || shape.solid.kind !== "extruded" || shape.solid.sweep !== sweep ? { kind: "extruded", sweep, z0, z1 } : shape.solid;
      const pieces = Number(opt.pieces ?? 1);
      const [x0, x1] = [Math.min(...rings[0]!.map((p) => p[0])), Math.max(...rings[0]!.map((p) => p[0]))];
      let previous = layer === 0 ? opt.like ?? id : `${id}-l${layer}`;
      const lid = layer === 0 ? id : `${id}-l${layer + 1}`;
      for (let piece = 0; piece < pieces; piece++) {
        const [a, b] = [x0 + ((x1 - x0) * piece) / pieces - 1e-4, x0 + ((x1 - x0) * (piece + 1)) / pieces + 1e-4];
        const cutRing = pieces === 1 ? rings[0]! : ((polygonClipping.intersection([[...rings[0]!, rings[0]![0]!]] as never, [[[a, -9], [b, -9], [b, 9], [a, 9], [a, -9]]] as never) as unknown as P[][][])[0]?.[0]?.slice(0, -1) ?? []);
        if (cutRing.length < 3) continue;
        const big = acc.slice().sort((p, q) => areaOf(q[0]!) - areaOf(p[0]!))[0]!;
        const holes = opt.holes === "1" && pieces === 1 ? big.slice(1).map((ring) => simplify(ring, tol)).filter((r) => r.length >= 3 && areaOf(r) > (feature / 4) ** 2) : [];
        if (pieces === 1) holes.push(...rings.slice(1).filter((r) => areaOf(r) > (feature / 4) ** 2));
        const set = holes.length > 0 ? { kind: "path", d: [cutRing, ...holes].map((r) => `M ${r.map((p) => `${r5(p[0])} ${r5(p[1])}`).join(" L ")} Z`).join(" "), solid } : { kind: "poly", points: cutRing.map((p) => [r5(p[0]), r5(p[1])]), closed: true, solid };
        if (holes.length > 0) console.log(`${view} ${id}: ${holes.length} further ring(s) kept (openings and separate parts)`);
        const pid = piece === 0 ? lid : `${lid}-${piece + 1}`;
        const isNew = drawn.find((x) => x.id === pid) === undefined;
        const whole: any = { ...(drawn.find((x) => x.id === pid) ?? shape), id: pid, ...set };
        if (whole.kind === "path") { delete whole.points; delete whole.closed; } else delete whole.d;
        ops.push({ op: "shape-add", scale: scaleArg, view, after: isNew ? (drawn.find((x) => x.id === previous) === undefined && !ops.some((o) => o.view === view && o.shape?.id === previous) ? id : previous) : pid, shape: whole });
        previous = pid;
      }
      if (layer === layers - 1) console.log(`${view} ${id}: extruded along ${sweep}, ${rings[0]!.length} points, solids span ${z0}..${z1}, shape spans ${solid.z0}..${solid.z1}${solid.zTip ? ` tip ${solid.zTip}` : ""}`);
      }
    }
  }
}
if (out !== "-") await writeOps(out!, mergeOps(await readOps(out!), ops));
else console.log(ops.map((o) => JSON.stringify(o)).join("\n"));
