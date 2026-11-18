/** ✂️ Fits the drawn polygons of one view to the volume: every deep drawn-only region (more than tol from the volume boundary) is cut out of the polygons covering it, every deep volume-only region is added as a traced polygon on the bone of the neighbouring shape. World outlines are traced from the masks and carried back to the rest frame of the bone by the exact inverse of the evaluator carry (probe plus Newton). Writes ops for humans-try.ts / humans-ops.py.
 *  bun humansb-B-fit.ts <asset> <scale> <view> <out.json> [--in patch.json] [--tol mm] [--rounds n] [--grow cells] [--skip id,id] [--solids id,id] [--noadd] [--nocut] */
import { readFileSync, writeFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, distanceField, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const argv = process.argv.slice(2);
const [name, scaleArg, view, out] = argv as [string, string, "elevation" | "front" | "plan", string];
const opt = (k: string): string | undefined => (argv.indexOf(k) >= 0 ? argv[argv.indexOf(k) + 1] : undefined);
const scale = Number(scaleArg);
const feature = featureOf(scale as never);
const tol = Number(opt("--tol") ?? 5) / 1000;
const rounds = Number(opt("--rounds") ?? 3);
const growCells = Number(opt("--grow") ?? 2);
const solidsOk = new Set(opt("--solids")?.split(",") ?? []);
const skip = new Set(opt("--skip")?.split(",") ?? []);
const noadd = argv.includes("--noadd"), nocut = argv.includes("--nocut");
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const base = ((await readStreamedAsset(entry)) as { asset: Asset }).asset;
const geometry = (a: Asset): any[] => (a as any).representations[String(scale)].geometry[view];
const pre = opt("--in");
const preOps: any[] = pre ? JSON.parse(readFileSync(pre, "utf8")) : [];
{
  const find = (list: any[], id: string) => list.find((x) => x.id === id);
  for (const o of preOps) {
    if (String(o.scale) !== String(scale)) continue;
    if (o.op.startsWith("solid")) {
      const body: any[] = (base as any).volumes[String(scale)].body;
      Object.assign(find(body, o.id), o.set);
      continue;
    }
    if (!o.op.startsWith("shape")) continue;
    const list: any[] = (base as any).representations[String(scale)].geometry[o.view];
    if (o.op === "shape-add") list.splice(list.indexOf(find(list, o.after)) + 1, 0, o.shape);
    else if (o.op === "shape-set") Object.assign(find(list, o.id), o.set);
    else list[list.indexOf(find(list, o.id))] = o.shape;
  }
}
const own = new Set<string>();
const scanOffsets = (offsets: any) => { for (const [id, off] of Object.entries(offsets ?? {}) as [string, any][]) if ((off.abductionDeg ?? 0) !== 0 || (off.twistDeg ?? 0) !== 0) own.add(id); };
for (const p of (base as any).poses) scanOffsets(p.bones);
for (const c of (base as any).clips) for (const k of c.keys) scanOffsets(k.bones);
const boneById = new Map<string, any>((base as any).rig.bones.map((b: any) => [b.id, b]));
const spread = new Set<string>();
for (const b of (base as any).rig.bones) for (let c: any = b; c !== undefined; c = c.parent === null ? undefined : boneById.get(c.parent)) if (own.has(c.id)) { spread.add(b.id); break; }
const stable = (bone: string): string | undefined => { for (let c: any = boneById.get(bone); c !== undefined; c = c.parent === null ? undefined : boneById.get(c.parent)) if (!spread.has(c.id)) return c.id; return undefined; };
const orig = new Map<string, string>(geometry(base).map((s) => [s.id, JSON.stringify(s)]));
let current: any[] = geometry(base).map((s) => structuredClone(s));
const build = (list: any[]): Asset => {
  const a = structuredClone(base) as any;
  a.representations[String(scale)].geometry[view] = structuredClone(list);
  return a as Asset;
};
const evalAsset = (a: Asset) => evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never) as any;
type Pt = { x: number; y: number };
const GEOM = new Set(["x", "y", "w", "h", "r", "cx", "cy", "rx", "ry", "d", "points", "x0", "y0", "x1", "y1", "r0", "r1", "solid", "followTip", "kind", "closed", "rotationDeg"]);

const affineOf = (bone: string): { a: number; b: number; c: number; d: number; tx: number; ty: number } | undefined => {
  const list = [...current, { kind: "poly", points: [[0, 0], [1, 0], [0, 1]], closed: true, id: "__probe", bone, role: "body", z: 9, filled: true, tone: 0.5 }];
  const s = evalAsset(build(list)).shapes[view].find((x: any) => x.shape.id === "__probe");
  if (s === undefined) return undefined;
  const [p0, p1, p2] = s.rings[0] as Pt[];
  const aff = { a: p1!.x - p0!.x, c: p1!.y - p0!.y, b: p2!.x - p0!.x, d: p2!.y - p0!.y, tx: p0!.x, ty: p0!.y };
  return Math.abs(aff.a * aff.d - aff.b * aff.c) < 0.2 ? undefined : aff;
};
const invert = (m: NonNullable<ReturnType<typeof affineOf>>, p: Pt): [number, number] => {
  const det = m.a * m.d - m.b * m.c;
  const dx = p.x - m.tx, dy = p.y - m.ty;
  return [(m.d * dx - m.b * dy) / det, (-m.c * dx + m.a * dy) / det];
};
const delta = (m: NonNullable<ReturnType<typeof affineOf>>, dx: number, dy: number): [number, number] => {
  const det = m.a * m.d - m.b * m.c;
  return [(m.d * dx - m.b * dy) / det, (-m.c * dx + m.a * dy) / det];
};

const traceLoops = (cells: Uint8Array, w: number, h: number): Pt[][] => {
  const key = (i: number, j: number) => j * (w + 1) + i;
  const next = new Map<number, [number, number][]>();
  const push = (a: [number, number], b: [number, number]) => { const k = key(a[0], a[1]); if (!next.has(k)) next.set(k, []); next.get(k)!.push(b); };
  const on = (i: number, j: number) => i >= 0 && j >= 0 && i < w && j < h && cells[j * w + i] === 1;
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (cells[j * w + i]) {
    if (!on(i, j - 1)) push([i, j], [i + 1, j]);
    if (!on(i + 1, j)) push([i + 1, j], [i + 1, j + 1]);
    if (!on(i, j + 1)) push([i + 1, j + 1], [i, j + 1]);
    if (!on(i - 1, j)) push([i, j + 1], [i, j]);
  }
  const loops: Pt[][] = [];
  for (const [k0, list] of next) {
    while (list.length > 0) {
      const start: [number, number] = [k0 % (w + 1), Math.floor(k0 / (w + 1))];
      const loop: [number, number][] = [start];
      let cur: [number, number] = list.shift()!;
      let prev = start;
      while (!(cur[0] === start[0] && cur[1] === start[1])) {
        loop.push(cur);
        const outs = next.get(key(cur[0], cur[1]))!;
        const dir = [cur[0] - prev[0], cur[1] - prev[1]];
        let pick = 0;
        if (outs.length > 1) {
          let best = -Infinity;
          outs.forEach((o, n) => { const d2 = [o[0] - cur[0], o[1] - cur[1]]; const turn = dir[0]! * d2[1]! - dir[1]! * d2[0]!; if (turn > best) { best = turn; pick = n; } });
        }
        prev = cur;
        cur = outs.splice(pick, 1)[0]!;
      }
      loops.push(loop.map(([i, j]) => ({ x: i, y: j })));
    }
  }
  return loops;
};
const area = (l: Pt[]) => { let s = 0; for (let i = 0; i < l.length; i++) { const a = l[i]!, b = l[(i + 1) % l.length]!; s += a.x * b.y - b.x * a.y; } return s / 2; };
const rdpOpen = (pts: Pt[], eps: number): Pt[] => {
  if (pts.length < 3) return pts;
  const a = pts[0]!, b = pts[pts.length - 1]!;
  const dx = b.x - a.x, dy = b.y - a.y, l = Math.hypot(dx, dy) || 1;
  let best = -1, idx = -1;
  for (let i = 1; i < pts.length - 1; i++) { const d = l === 0 ? Math.hypot(pts[i]!.x - a.x, pts[i]!.y - a.y) : Math.abs((pts[i]!.x - a.x) * dy - (pts[i]!.y - a.y) * dx) / l; if (d > best) { best = d; idx = i; } }
  if (best <= eps) return [a, b];
  return [...rdpOpen(pts.slice(0, idx + 1), eps).slice(0, -1), ...rdpOpen(pts.slice(idx), eps)];
};
const rdp = (pts: Pt[], eps: number): Pt[] => {
  if (pts.length < 6) return pts;
  let far = 0, fd = -1;
  for (let i = 1; i < pts.length; i++) { const d = Math.hypot(pts[i]!.x - pts[0]!.x, pts[i]!.y - pts[0]!.y); if (d > fd) { fd = d; far = i; } }
  return [...rdpOpen(pts.slice(0, far + 1), eps).slice(0, -1), ...rdpOpen([...pts.slice(far), pts[0]!], eps).slice(0, -1)];
};

let counter = current.filter((s) => String(s.id).startsWith("fit-")).length;
const stats = { cut: 0, added: 0, unplaced: 0 };
for (let round = 0; round < rounds; round++) {
  const asset = build(current);
  const state = evalAsset(asset);
  const solids = worldSolids(asset, state.bones as never, scale as never).filter((w: any) => w.texture !== true);
  const m = measureView(asset, state, solids, view as never, scale as never, undefined, feature, Infinity, false);
  const { cell, box } = m;
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
  const toVol = distanceField(boundaryOf(V, w, h), w, h), toDrawn = distanceField(boundaryOf(D, w, h), w, h);
  const N = w * h;
  const bdD = boundaryOf(D, w, h), bdV = boundaryOf(V, w, h);
  const nearestVol = (p: Pt, r: number): Pt | undefined => {
    const i = Math.floor((p.x - box.minX) / cell), j = Math.floor((p.y - box.minY) / cell), n = Math.ceil(r / cell);
    let best: Pt | undefined, b2 = r * r;
    for (let dj = -n; dj <= n; dj++) for (let di = -n; di <= n; di++) {
      const ii = i + di, jj = j + dj;
      if (ii < 0 || jj < 0 || ii >= w || jj >= h || !bdV[jj * w + ii]) continue;
      const q = { x: box.minX + (ii + 0.5) * cell, y: box.minY + (jj + 0.5) * cell };
      const d2 = (p.x - q.x) ** 2 + (p.y - q.y) ** 2;
      if (d2 < b2) { b2 = d2; best = q; }
    }
    return best;
  };
  const flag = new Uint8Array(N);
  for (let k = 0; k < N; k++) if ((bdD[k] && Math.sqrt(toVol[k]!) * cell > tol) || (bdV[k] && Math.sqrt(toDrawn[k]!) * cell > tol)) flag[k] = 1;
  for (let r = 0; r < 2; r++) { const src = Uint8Array.from(flag); for (let j = 1; j < h - 1; j++) for (let i = 1; i < w - 1; i++) { const k = j * w + i; if (!src[k] && (src[k - 1] || src[k + 1] || src[k - w] || src[k + w] || src[k - w - 1] || src[k - w + 1] || src[k + w - 1] || src[k + w + 1])) flag[k] = 1; } }
  const red = new Uint8Array(N), blue = new Uint8Array(N);
  for (let k = 0; k < N; k++) { red[k] = D[k] && !V[k] ? 1 : 0; blue[k] = V[k] && !D[k] ? 1 : 0; }
  const components = (set: Uint8Array, field: Float64Array): Uint8Array[] => {
    const seen = new Uint8Array(N), result: Uint8Array[] = [];
    for (let s = 0; s < N; s++) {
      if (!set[s] || seen[s]) continue;
      const comp = new Uint8Array(N), stack = [s];
      seen[s] = 1; let deep = false, count = 0;
      while (stack.length) {
        const k = stack.pop()!; comp[k] = 1; count++;
        if (flag[k]) deep = true;
        const i = k % w, j = Math.floor(k / w);
        for (const [di, dj] of [[1, 0], [-1, 0], [0, 1], [0, -1]] as const) { const ii = i + di, jj = j + dj; if (ii < 0 || jj < 0 || ii >= w || jj >= h) continue; const q = jj * w + ii; if (set[q] && !seen[q]) { seen[q] = 1; stack.push(q); } }
      }
      if (deep && count > 4) result.push(comp);
    }
    return result;
  };
  const cuts = nocut ? [] : components(red, toVol);
  const adds = noadd ? [] : components(blue, toDrawn);
  const cen = (c: Uint8Array) => { let cx = 0, cy = 0, n = 0; for (let k = 0; k < N; k++) if (c[k]) { cx += k % w; cy += Math.floor(k / w); n++; } return `(${(box.minX + cx / n * cell).toFixed(3)}, ${(box.minY + cy / n * cell).toFixed(3)}) n${n}`; };
  if (process.env.VERBOSE) console.log(`[DEBUG] cuts ${cuts.map(cen).join(" ")} adds ${adds.map(cen).join(" ")}`);
  const rings = new Map<string, Pt[]>(state.shapes[view].map((s: any) => [s.shape.id, s.rings[0]]));
  const allRings = new Map<string, Pt[][]>(state.shapes[view].map((s: any) => [s.shape.id, s.rings]));
  const masks = new Map<string, Uint8Array>();
  const maskOf = (id: string): Uint8Array => {
    let k = masks.get(id);
    if (k === undefined) { const mk = new Mask(box, cell); mk.fill(allRings.get(id)!); k = mk.cells; masks.set(id, k); }
    return k;
  };
  const templatable = (s: any): boolean => s.closed !== false && s.filled !== false && rings.has(s.id) && rings.get(s.id)!.length >= 3 && s.kind !== "member" && s.role !== "ground" && s.role !== "detail";
  const fillable = (s: any): boolean => (s.solid === undefined || solidsOk.has(s.id) || [...solidsOk].some((x) => x.endsWith("*") && s.id.startsWith(x.slice(0, -1)))) && s.closed !== false && s.filled !== false && rings.has(s.id) && rings.get(s.id)!.length >= 3 && !skip.has(s.id) && s.kind !== "member" && s.role !== "ground";
  const targets = new Map<string, Uint8Array>();
  const replaced = new Map<string, Pt[]>();
  const added: { template: any; world: Pt[] }[] = [];
  const toWorld = (loop: Pt[]): Pt[] => rdp(loop, 0.6).map((p) => ({ x: box.minX + p.x * cell, y: box.minY + p.y * cell }));
  const largest = (cells: Uint8Array): Pt[] | undefined => {
    const loops = traceLoops(cells, w, h).map((l) => ({ l, a: area(l) })).filter((x) => x.a > 0).sort((a, b) => b.a - a.a);
    return loops.length === 0 || loops[0]!.a < 6 ? undefined : loops[0]!.l;
  };
  for (const comp of cuts) {
    for (const s of current) {
      if (!fillable(s)) continue;
      const mk = targets.get(s.id) ?? maskOf(s.id);
      let hit = 0;
      for (let k = 0; k < N; k++) if (mk[k] && comp[k]) { hit++; if (hit > 2) break; }
      if (process.env.VERBOSE && hit > 0) console.log(`[DEBUG] cut comp ${cen(comp)} hits ${s.id} ${hit} fillable`);
      if (hit <= 2) continue;
      const nm = targets.get(s.id) ?? Uint8Array.from(mk);
      for (let k = 0; k < N; k++) if (comp[k]) nm[k] = 0;
      targets.set(s.id, nm);
    }
  }
  for (const [id, nm] of targets) {
    const loop = largest(nm);
    if (loop === undefined) { stats.unplaced++; console.log(`[DEBUG] unplaced cut ${id}`); continue; }
    replaced.set(id, toWorld(loop));
  }
  const strokeFix = new Map<string, Pt[]>();
  for (const s of current) {
    if (!((s.kind === "poly" && s.closed === false) || (s.kind === "path" && s.filled === false && !/Z/.test(s.d ?? ""))) || s.solid !== undefined || skip.has(s.id) || !rings.has(s.id)) continue;
    const ring = rings.get(s.id)!;
    const rep = (asset as any).representations[String(scale)];
    const pen = (((s.penMm ?? rep.pens[s.role]) ?? 0) * scale) / 1000;
    const samples: { p: Pt; ok: boolean }[] = [];
    for (let i = 0; i + 1 < ring.length; i++) {
      const a = ring[i]!, b = ring[i + 1]!;
      const n = Math.max(1, Math.ceil(Math.hypot(b.x - a.x, b.y - a.y) / cell));
      for (let u = 0; u < n; u++) {
        const p = { x: a.x + ((b.x - a.x) * u) / n, y: a.y + ((b.y - a.y) * u) / n };
        const ci = Math.floor((p.x - box.minX) / cell), cj = Math.floor((p.y - box.minY) / cell);
        const k = cj * w + ci;
        const out = ci < 0 || cj < 0 || ci >= w || cj >= h || (!V[k] && Math.sqrt(toVol[k]!) * cell > pen / 2 + 3 * cell);
        samples.push({ p, ok: !out });
      }
    }
    samples.push({ p: ring[ring.length - 1]!, ok: true });
    if (samples.every((x) => x.ok)) continue;
    const mapped: Pt[] = samples.map((x) => (x.ok ? x.p : nearestVol(x.p, 0.12) ?? x.p));
    const bestRun = mapped;
    strokeFix.set(s.id, rdpOpen(bestRun, 0.6 * cell));
  }
  for (const comp of adds) {
    const grown = Uint8Array.from(comp);
    for (let r = 0; r < growCells; r++) {
      const src = Uint8Array.from(grown);
      for (let j = 1; j < h - 1; j++) for (let i = 1; i < w - 1; i++) { const k = j * w + i; if (!src[k] && V[k] && (src[k - 1] || src[k + 1] || src[k - w] || src[k + w])) grown[k] = 1; }
    }
    const reachMask = Uint8Array.from(grown);
    for (let r = 0; r < 10; r++) {
      const src = Uint8Array.from(reachMask);
      for (let j = 1; j < h - 1; j++) for (let i = 1; i < w - 1; i++) { const k = j * w + i; if (!src[k] && (src[k - 1] || src[k + 1] || src[k - w] || src[k + w])) reachMask[k] = 1; }
    }
    let best: any, bestHit = 0;
    for (const s of current) {
      if (!templatable(s)) continue;
      const mk = maskOf(s.id);
      let hit = 0;
      for (let k = 0; k < N; k++) if (mk[k] && reachMask[k]) hit++;
      if (hit > bestHit) { bestHit = hit; best = s; }
    }
    const loop = largest(grown);
    if (best === undefined || loop === undefined) { stats.unplaced++; let cx = 0, cy = 0, n = 0; for (let k = 0; k < N; k++) if (comp[k]) { cx += k % w; cy += Math.floor(k / w); n++; } console.log(`[DEBUG] unplaced add at (${(box.minX + cx / n * cell).toFixed(3)}, ${(box.minY + cy / n * cell).toFixed(3)}) cells ${n} template ${best?.id} loop ${loop?.length}`); continue; }
    added.push({ template: best, world: toWorld(loop) });
  }
  if (replaced.size + added.length + strokeFix.size === 0) { console.log(`[DEBUG] round ${round + 1} nothing to do (iou ${m.iou.toFixed(3)} h ${(m.hausdorffM * 1000).toFixed(1)}mm)`); break; }
  const aff = new Map<string, ReturnType<typeof affineOf>>();
  const affFor = (bone: string) => { if (!aff.has(bone)) aff.set(bone, affineOf(bone)); return aff.get(bone); };
  const worldTarget = new Map<string, Pt[]>();
  const next: any[] = [];
  const keepProps = (s: any) => Object.fromEntries(Object.entries(s).filter(([k]) => !GEOM.has(k)));
  for (const s of current) {
    next.push(s);
    const r = replaced.get(s.id);
    if (r !== undefined) {
      const keepSolid = s.solid?.kind === "extruded" && view === "elevation";
      const flatten = s.solid !== undefined && !keepSolid;
      const bone = flatten && spread.has(s.bone) ? stable(s.bone) : s.bone;
      const A = bone === undefined ? undefined : affFor(bone);
      if (A === undefined) { stats.unplaced++; continue; }
      next[next.length - 1] = { kind: "poly", points: r.map((p) => invert(A, p)), closed: true, ...keepProps(s), ...(keepSolid ? { solid: s.solid } : {}), bone, id: s.id };
      worldTarget.set(s.id, r); stats.cut++;
    }
    const sf = strokeFix.get(s.id);
    if (sf !== undefined) {
      const A = affFor(s.bone);
      if (A !== undefined) { next[next.length - 1] = { ...s, kind: "poly", points: sf.map((p) => invert(A, p)), closed: false, followTip: undefined }; delete next[next.length - 1].followTip; delete next[next.length - 1].d; worldTarget.set(s.id, sf); stats.cut++; }
    }
    for (const a of added.filter((x) => x.template.id === s.id)) {
      const isSpread = spread.has(s.bone);
      const bone = isSpread && view !== "elevation" ? stable(s.bone) : s.bone;
      const solid = isSpread && view === "elevation" ? (s.solid?.kind === "extruded" ? s.solid : undefined) : undefined;
      if (isSpread && view === "elevation" && solid === undefined) { stats.unplaced++; continue; }
      const A = bone === undefined ? undefined : affFor(bone);
      if (A === undefined) { stats.unplaced++; continue; }
      const id = `fit-${bone}-${scale}-${++counter}`;
      const props: any = { ...keepProps(s), id, bone, z: (s.z ?? 1) + 0.001 };
      next.push({ kind: "poly", points: a.world.map((p) => invert(A, p)), closed: true, ...props, ...(solid === undefined ? {} : { solid }) });
      worldTarget.set(id, a.world); stats.added++;
    }
  }
  current = next;
  for (let it = 0; it < 12; it++) {
    const ids = new Set(worldTarget.keys());
    const ringsOfList = (list: any[]) => new Map<string, Pt[]>(evalAsset(build(list)).shapes[view].map((x: any) => [x.shape.id, x.rings[0]]));
    const shifted = (dx: number, dy: number) => ringsOfList(current.map((x) => (ids.has(x.id) ? { ...x, points: x.points.map((p: number[]) => [p[0]! + dx, p[1]! + dy]) } : x)));
    const eps = 1e-3;
    const r0 = ringsOfList(current), rx = shifted(eps, 0), ry = shifted(0, eps);
    let worst = 0;
    for (const s of current) {
      const tgt = worldTarget.get(s.id), g = r0.get(s.id);
      if (tgt === undefined || g === undefined || g.length !== s.points.length) continue;
      s.points = s.points.map((p: number[], k: number) => {
        const dx = tgt[k]!.x - g[k]!.x, dy = tgt[k]!.y - g[k]!.y;
        worst = Math.max(worst, Math.hypot(dx, dy));
        const a = (rx.get(s.id)![k]!.x - g[k]!.x) / eps, c = (rx.get(s.id)![k]!.y - g[k]!.y) / eps, b = (ry.get(s.id)![k]!.x - g[k]!.x) / eps, d = (ry.get(s.id)![k]!.y - g[k]!.y) / eps;
        const det = a * d - b * c;
        if (Math.abs(det) < 0.05) return p;
        const damp = it < 3 ? 0.5 : 0.7;
        return [p[0]! + (damp * (d * dx - b * dy)) / det, p[1]! + (damp * (-c * dx + a * dy)) / det];
      });
    }
    if (process.env.VERBOSE) console.log(`[DEBUG] newton ${it} worst ${(worst * 1000).toFixed(1)}mm`);
    if (worst < cell / 4) break;
  }
  const a2 = build(current);
  const st2 = evalAsset(a2);
  const m2 = measureView(a2, st2, solids, view as never, scale as never, undefined, feature, Infinity, false);
  console.log(`[DEBUG] round ${round + 1} cut ${stats.cut} added ${stats.added} unplaced ${stats.unplaced}: iou ${m.iou.toFixed(3)} -> ${m2.iou.toFixed(3)} h ${(m.hausdorffM * 1000).toFixed(1)} -> ${(m2.hausdorffM * 1000).toFixed(1)}mm`);
}
const r5 = (v: number) => Math.round(v * 1e5) / 1e5;
const ops: any[] = [];
let prev: string | undefined;
for (const s of current) {
  const fin = s.kind === "poly" && Array.isArray(s.points) ? { ...s, points: s.points.map((p: number[]) => [r5(p[0]!), r5(p[1]!)]) } : s;
  const o = orig.get(s.id);
  if (o === undefined) ops.push({ op: "shape-add", scale: String(scale), view, after: prev, shape: fin });
  else if (o !== JSON.stringify(fin)) ops.push({ op: "shape", scale: String(scale), view, id: s.id, shape: fin });
  prev = s.id;
}
writeFileSync(out, JSON.stringify([...preOps, ...ops], null, 1));
console.log(`[DEBUG] wrote ${ops.length} ops (cut ${stats.cut} added ${stats.added} unplaced ${stats.unplaced})`);
