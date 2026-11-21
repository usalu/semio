/** ✏️ Round 5 roe deer (round 3 redraw): redraws the 2D shapes of every scale and view as the true orthographic view of that scale's body (ruling E1). Per shape: the union silhouette of the body solids the model files assign to it is traced, cleaned to an economical outline and written in the local frame of its shape; ids, roles, bone bindings, layer order and tones of the existing shapes stay, shapes for parts that had none (croup, antlers, thigh, shoulder and upper arm in front and plan, the lower legs at 1:500) are added beside a sibling, the detail and texture strokes are restated by hand along their bones, the hoof-cleft strokes of the side elevation (a cleft cannot be seen from the side) and the eyes of the plan (they sit under the brow on the side of the head and cannot be seen from above) are dropped. The outlines are solved through the product's own placement (`inverse` of r3-project2d.ts), so shapes keep their `followTip` and still stretch along a swinging limb in front and plan.
 * Usage: bun r3-roe-deer-draw2d.ts   (writes 🗑️generated/r3-roe-deer/reps.json = { representations })
 * @see ./r3-roe-deer-apply.py */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { REGION } from "./r3-roe-deer-50.ts";
import { SPEC } from "./r3-roe-deer-kit.ts";
import { inverse } from "./r3-project2d.ts";
import { GENERATED, Mask, entryOf, featureM, loadSource, merged, meshMask, meshesAt, poseState, projectWorld, type Json, type SolidMesh, type View } from "./volumes-lib.ts";

type P = [number, number];
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5 + 0;
const OUT = join(GENERATED, "r3-roe-deer");
const regionOf = (id: string): string | undefined => REGION.find(([re]) => re.test(id))?.[1];

/** 🧬️ The sibling a new shape takes its role, layer and tone from, with what differs. */
function template(id: string, view: View, have: (id: string) => boolean): { from: string; set: Json } {
  const pick = (...ids: string[]): string => ids.find(have) ?? (() => { throw new Error(`no template for ${id} among ${ids}`); })();
  const nf = id.split("-")[0]!;
  if (id === "croup") return { from: "hindbody", set: { zAdd: -0.02 } };
  if (id === "near-antler") return { from: "head", set: { z: 4.6, tone: 0.4 } };
  if (id === "far-antler") return { from: "head", set: { z: 1.9, tone: 0.3 } };
  if (id === "far-ear") return { from: "ears", set: { z: 2, tone: 0.5 } };
  if (id === "eye") return { from: "nose-leather", set: { role: "detail", z: 5, tone: 0.04 } };
  if (id === "eye-f") return { from: "eye", set: {} };
  if (/-thigh$/.test(id)) return { from: pick(`${nf}-shank`, `${nf}-hind-leg`), set: {} };
  if (/-(scapula|humerus|fore-upper)$/.test(id)) return { from: pick(`${nf}-forearm`, `${nf}-fore-leg`), set: {} };
  if (/-hind-lower$/.test(id)) return { from: `${nf}-hind-leg`, set: {} };
  if (/-fore-lower$/.test(id)) return { from: `${nf}-fore-leg`, set: {} };
  throw new Error(`no template rule for ${id} in ${view}`);
}

/** 🧭️ The outer boundary of the largest filled component of a mask as a polygon along the cell edges (world units of the view). */
function contour(mask: Mask): { ring: P[]; parts: number; rings: P[][] } {
  const { width: w, height: h, cells } = mask;
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= w || j >= h ? 0 : cells[j * w + i]!);
  const next = new Map<number, number[]>();
  const key = (i: number, j: number): number => j * (w + 1) + i;
  const edge = (a: number, b: number): void => void (next.get(a) ?? next.set(a, []).get(a)!).push(b);
  for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) {
    if (at(i, j) === 0) continue;
    if (at(i, j - 1) === 0) edge(key(i, j), key(i + 1, j));
    if (at(i + 1, j) === 0) edge(key(i + 1, j), key(i + 1, j + 1));
    if (at(i, j + 1) === 0) edge(key(i + 1, j + 1), key(i, j + 1));
    if (at(i - 1, j) === 0) edge(key(i, j + 1), key(i, j));
  }
  const loops: P[][] = [];
  for (const [start] of next) {
    while ((next.get(start)?.length ?? 0) > 0) {
      const loop: P[] = [];
      let a = start;
      let from = -1;
      for (;;) {
        const outs = next.get(a)!;
        let pickAt = 0;
        if (outs.length > 1 && from >= 0) {
          const dx = (a % (w + 1)) - (from % (w + 1));
          const dy = Math.floor(a / (w + 1)) - Math.floor(from / (w + 1));
          let best = -2;
          outs.forEach((b, k) => { const ex = (b % (w + 1)) - (a % (w + 1)); const ey = Math.floor(b / (w + 1)) - Math.floor(a / (w + 1)); const turn = dx * ey - dy * ex; if (turn > best) { best = turn; pickAt = k; } });
        }
        const b = outs.splice(pickAt, 1)[0]!;
        loop.push([mask.box.minX + (a % (w + 1)) * mask.cell, mask.box.minY + Math.floor(a / (w + 1)) * mask.cell]);
        from = a;
        a = b;
        if (a === start) break;
      }
      loops.push(loop);
    }
  }
  const area = (r: P[]): number => r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2;
  const outer = loops.filter((l) => area(l) > 0).sort((a, b) => area(b) - area(a));
  return { ring: outer[0] ?? [], parts: outer.length, rings: outer };
}

/** ✂️ Douglas-Peucker on a closed ring, split at its two farthest points. */
function simplify(ring: P[], tol: number): P[] {
  const dp = (pts: P[]): P[] => {
    if (pts.length < 3) return pts;
    const [a, b] = [pts[0]!, pts[pts.length - 1]!];
    const len = Math.hypot(b[0] - a[0], b[1] - a[1]) || 1e-12;
    let worst = 0;
    let at = 0;
    for (let i = 1; i < pts.length - 1; i++) {
      const d = Math.abs((b[0] - a[0]) * (a[1] - pts[i]![1]) - (a[0] - pts[i]![0]) * (b[1] - a[1])) / len;
      if (d > worst) { worst = d; at = i; }
    }
    return worst <= tol ? [a, b] : [...dp(pts.slice(0, at + 1)).slice(0, -1), ...dp(pts.slice(at))];
  };
  let i0 = 0;
  for (let i = 1; i < ring.length; i++) if (ring[i]![0] < ring[i0]![0]) i0 = i;
  let i1 = 0;
  let far = -1;
  ring.forEach((p, i) => { const d = Math.hypot(p[0] - ring[i0]![0], p[1] - ring[i0]![1]); if (d > far) { far = d; i1 = i; } });
  const turned = [...ring.slice(i0), ...ring.slice(0, i0)];
  const k = (i1 - i0 + ring.length) % ring.length;
  return [...dp(turned.slice(0, k + 1)).slice(0, -1), ...dp([...turned.slice(k), turned[0]!]).slice(0, -1)];
}

/** ➰️ Chaikin-free rounding of the traced stair: every vertex moves to the mean of itself and its neighbours along the dense ring before simplification. */
function relax(ring: P[], passes: number): P[] {
  let r = ring;
  for (let n = 0; n < passes; n++) r = r.map((p, i) => { const a = r[(i - 1 + r.length) % r.length]!; const b = r[(i + 1) % r.length]!; return [(a[0] + 2 * p[0] + b[0]) / 4, (a[1] + 2 * p[1] + b[1]) / 4] as P; });
  return r;
}

/** 🫁️ The costal arch: the rear edge of the chest shape in the side elevation runs from the loin obliquely down and forward as the last rib does; what lies behind it is drawn by the hindbody shape, which the chest overlaps. World x of the line at height y. */
const COSTAL: P[] = [[0.3, 0.48], [0.39, 0.475], [0.44, 0.44], [0.48, 0.408], [0.52, 0.384], [0.57, 0.365], [0.62, 0.357], [0.67, 0.358], [0.71, 0.365], [0.75, 0.376], [0.9, 0.38]];
const costalX = (y: number): number => {
  for (let i = 0; i < COSTAL.length - 1; i++) if (y <= COSTAL[i + 1]![0]) { const t = Math.max(0, (y - COSTAL[i]![0]) / (COSTAL[i + 1]![0] - COSTAL[i]![0])); return COSTAL[i]![1] + t * (COSTAL[i + 1]![1] - COSTAL[i]![1]); }
  return COSTAL[COSTAL.length - 1]![1];
};

function outline(view: View, meshes: SolidMesh[], feature: number, keep?: (x: number, y: number) => boolean): { ring: P[]; parts: number; rings: P[][] } {
  const pts = meshes.flatMap((m) => Array.from({ length: m.world.length / 3 }, (_, i) => projectWorld(view, [m.world[3 * i]!, m.world[3 * i + 1]!, m.world[3 * i + 2]!])));
  const cell = feature / 16;
  const box = { minX: Math.min(...pts.map((p) => p.x)) - 3 * cell, minY: Math.min(...pts.map((p) => p.y)) - 3 * cell, maxX: Math.max(...pts.map((p) => p.x)) + 3 * cell, maxY: Math.max(...pts.map((p) => p.y)) + 3 * cell };
  const mask = meshMask(view, meshes, box, cell);
  if (keep !== undefined) for (let j = 0; j < mask.height; j++) for (let i = 0; i < mask.width; i++) if (!keep(box.minX + (i + 0.5) * cell, box.minY + (j + 0.5) * cell)) mask.cells[j * mask.width + i] = 0;
  const { ring, parts, rings } = contour(mask);
  const clean = (r: P[]): P[] => simplify(relax(r, 6), feature * 0.2);
  return { ring: clean(ring), parts, rings: rings.filter((r) => r.length * cell * cell > 0 && Math.abs(r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2) > feature * feature).map(clean) };
}

/** Detail and texture strokes, restated by hand in the frame of their bone (m along the bone, m across). */
const STROKES: Record<string, { bone: string; local: P[] }> = {
  brow: { bone: "head", local: [[0.07, 0.043], [0.09, 0.047], [0.108, 0.04]] },
  "mouth-line": { bone: "head", local: [[0.214, -0.012], [0.18, -0.011], [0.145, -0.008], [0.115, -0.002]] },
  "inner-ear": { bone: "ear-n", local: [[0.02, 0], [0.06, 0.007], [0.1, 0.001], [0.06, -0.005], [0.02, 0]] },
  "coat-1": { bone: "thorax", local: [[0.165, 0.01], [0.147, -0.059], [0.151, -0.121]] },
  "coat-2": { bone: "lumbar", local: [[0.07, -0.007], [0.112, -0.0655], [0.129, -0.13]] },
  "neck-fur": { bone: "neck", local: [[0.08, -0.03], [0.15, -0.02], [0.22, -0.015]] },
  "shoulder-line": { bone: "thorax", local: [[0.165, 0.01], [0.147, -0.059], [0.151, -0.121]] },
  "hip-line": { bone: "lumbar", local: [[0.07, -0.007], [0.112, -0.0655], [0.129, -0.13]] },
};
/** Lateral half span of the neck at its base and at the poll per scale: the neck is drawn as one extruded solid, so front and plan are projections of it in every pose (a flat plan shape would vanish when the neck points down to graze). */
const NECK_SPAN: Record<number, [number, number]> = { 50: [0.057, 0.031], 100: [0.057, 0.031], 200: [0.057, 0.032], 500: [0.058, 0.04] };
/** Lateral span of the near shoulder-and-upper-arm mass at 1:500: drawn as an extruded solid like the neck, because a flat front shape collapses to a line when the upper arm swings forward to the horizontal (foreleg raised) and the drawing stops being printable. */
const UPPER_SPAN: Record<number, Record<string, [number, number]>> = { 500: { "near-fore-upper": [-0.104, -0.024] } };
const PROBE: P[] = [[0, 0], [0.1, 0], [0, 0.1], [0.1, 0.1]];
const TONE: Record<string, number> = { "rump-patch": 0.95 };

const { entry, asset: catalog } = entryOf(SPEC);
const asset: Json = merged(catalog, loadSource(entry)!);
const before: Json = JSON.parse(readFileSync(join(OUT, "asset-before.json"), "utf8")).asset;
const representations: Json = JSON.parse(JSON.stringify(before.representations));
const log: string[] = [];
for (const scale of [50, 100, 200, 500]) {
  const body: Json[] = asset.volumes[String(scale)].body;
  const map = JSON.parse(readFileSync(join(OUT, `shapes.${scale}.json`), "utf8")) as Record<string, string>;
  const meshOf = new Map(meshesAt(asset, scale, body).map((m, i) => [body[i]!.id as string, m]));
  const feature = featureM(scale, asset);
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const old: Json[] = representations[String(scale)].geometry[view];
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const id = map[s.id];
      if (id === undefined) throw new Error(`1:${scale}: solid ${s.id} has no shape`);
      if (view === "elevation" && id === "eye-f") continue;
      if (view === "plan" && (id === "eye" || id === "eye-f")) continue;
      (groups.get(id) ?? groups.set(id, []).get(id)!).push(s);
    }
    const list: Json[] = old.filter((s) => groups.has(s.id) || s.id in STROKES);
    for (const s of old) if (!list.includes(s)) log.push(`1:${scale} ${view}: removed ${s.id}`);
    const have = (id: string): boolean => list.some((s) => s.id === id);
    let open = [...groups.keys()].filter((id) => !have(id));
    while (open.length > 0) {
      const n = open.length;
      for (const id of [...open]) {
        let t: { from: string; set: Json };
        try { t = template(id, view, have); } catch (error) { if (open.length === 1 || n === 0) throw error; continue; }
        const src = list.find((s) => s.id === t.from)!;
        const { zAdd, ...set } = t.set;
        const made: Json = { ...src, id, ...set, ...(zAdd === undefined ? {} : { z: r5(src.z + zAdd) }) };
        delete made.penMm;
        delete made.regions;
        list.splice(list.indexOf(src) + 1, 0, made);
        log.push(`1:${scale} ${view}: added ${id} (bone ${groups.get(id)![0]!.bone}) beside ${t.from}`);
        open = open.filter((o) => o !== id);
      }
      if (open.length === n) throw new Error(`1:${scale} ${view}: no template for ${open}`);
    }
    const shapes: Json[] = list.map((s) => {
      const solids = groups.get(s.id);
      const out: Json = {};
      for (const k of ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "penMm"]) if (s[k] !== undefined) out[k] = s[k];
      if (solids !== undefined) {
        const bones = new Set(solids.map((q) => q.bone));
        if (bones.size !== 1) throw new Error(`1:${scale} ${view} ${s.id}: solids on several bones ${[...bones]}`);
        const bone = [...bones][0] as string;
        if (out.bone !== bone) { log.push(`1:${scale} ${view}: ${s.id} rebound ${out.bone} -> ${bone}`); out.bone = bone; }
        const isNew = !old.some((o) => o.id === s.id);
        const region = regionOf(solids[0]!.id);
        if (isNew) { delete out.regions; if (region === undefined) delete out.region; else out.region = region; }
        if (s.filled !== true) delete out.penMm;
        out.filled = true;
        if (TONE[s.id] !== undefined) out.tone = TONE[s.id];
        if (s.id === "rump-patch") out.z = 2.9;
      }
      out.kind = "poly";
      if (s.followTip !== undefined) out.followTip = s.followTip;
      out.points = PROBE;
      out.closed = solids !== undefined;
      return out;
    });
    const probe: Json = { ...asset, representations };
    const frames = (poseState(asset, scale) as Json).frames;
    const jobs = shapes.map((s) => {
      const { points: _points, kind: _kind, closed: _closed, ...meta } = s;
      const solids = groups.get(s.id);
      if (solids === undefined) {
        const st = STROKES[s.id]!;
        const f = frames[st.bone]!;
        s.bone = st.bone;
        meta.bone = st.bone;
        return { meta, target: st.local.map(([u, v]) => projectWorld(view, [f.o[0] + f.u[0] * u + f.v[0] * v, f.o[1] + f.u[1] * u + f.v[1] * v, f.o[2] + f.u[2] * u + f.v[2] * v])) };
      }
      const { ring, parts } = outline(view, solids.map((q) => meshOf.get(q.id)!), feature);
      if (parts !== 1) log.push(`1:${scale} ${view}: ${s.id} has ${parts} separate parts, the largest is drawn`);
      return { meta, target: ring.map(([x, y]) => ({ x, y })) };
    });
    const solved = inverse(probe, scale, view, jobs);
    shapes.forEach((s, i) => {
      const r = solved[i]!;
      if (r.residualM > 2e-4) log.push(`1:${scale} ${view}: ${s.id} placed with a residual of ${(r.residualM * 1000).toFixed(2)} mm${r.edgeOn ? " (edge-on)" : ""}`);
      s.points = r.points.map((q) => [r5(q.x), r5(q.y)]);
    });
    const neck = shapes.find((q) => q.id === "neck")!;
    delete neck.followTip;
    if (view === "elevation") neck.solid = { kind: "extruded", sweep: "w", z0: -NECK_SPAN[scale]![0], z1: NECK_SPAN[scale]![0], zTip: [-NECK_SPAN[scale]![1], NECK_SPAN[scale]![1]] };
    else { const twin = (representations[String(scale)].geometry.elevation as Json[]).find((q) => q.id === "neck")!; neck.points = twin.points; neck.solid = twin.solid; }
    for (const [id, [z0, z1]] of Object.entries(UPPER_SPAN[scale] ?? {})) {
      const upper = shapes.find((q) => q.id === id)!;
      delete upper.followTip;
      if (view === "elevation") upper.solid = { kind: "extruded", sweep: "w", z0, z1 };
      else { const twin = (representations[String(scale)].geometry.elevation as Json[]).find((q) => q.id === id)!; upper.points = twin.points; upper.solid = twin.solid; }
    }
    representations[String(scale)].geometry[view] = shapes;
    console.log(`1:${scale} ${view}: ${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  }
}
writeFileSync(join(OUT, "reps.json"), JSON.stringify({ representations }));
writeFileSync(join(OUT, "draw2d-log.txt"), log.join("\n") + "\n");
console.log(log.join("\n"));
