/** ✏️ Round 3 goat: redraws the 2D shapes of every scale and view as the true orthographic view of that scale's body (ruling E1). Per bone: the union silhouette of the body solids of the bone is traced, cleaned to an economical outline and written in the local frame of its shape; ids, roles, bone bindings, layer order and tones of the existing shapes stay, shapes for bones that had none are added beside their sibling, the two detail strokes are restated by hand.
 * Usage: bun r3-goat-draw2d.ts   (writes 🗑️generated/r3-goat/reps.json = { representations })
 * @see ./r3-goat-apply.py */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { REGION_OF } from "./r3-goat-50-lists.ts";
import { GENERATED, Mask, entryOf, featureM, loadSource, merged, meshMask, meshesAt, poseState, projectWorld, type Json, type SolidMesh, type View } from "./volumes-lib.ts";

type P = [number, number];
const SPEC = "animals/goat";
const r5 = (x: number): number => Math.round(x * 1e5) / 1e5 + 0;
const nearFar = (s: string): string => (s === "n" ? "near" : "far");

/** 🏷️ The shape a body solid is drawn in at a scale and view (null: hidden by its twin). */
function shapeOf(scale: number, view: View, id: string): string | null {
  const elev = view === "elevation";
  let m: RegExpMatchArray | null;
  if (/^(ribcage|withers|chest)$/.test(id)) return "chest";
  if (/^(abdomen|loin|hindbody)$/.test(id)) return "hindbody";
  if (id === "croup") return "croup";
  if (/^(udder|teat-[nf])$/.test(id)) return "udder";
  if (id === "neck" || id === "head" || id === "tail-root" || id === "tail-tip" || id === "tail") return id;
  if (id === "jaw") return "jaw";
  if (id === "beard") return scale === 50 ? "beard" : "head";
  if (/^nostril-[nf]$/.test(id)) return "head";
  if ((m = id.match(/^eye-([nf])$/))) return elev ? (m[1] === "n" ? "eye" : null) : id;
  if ((m = id.match(/^horn-([nf])$/))) return scale <= 100 ? id : elev || m[1] === "n" ? "horn" : "horn-f";
  if ((m = id.match(/^ear-([nf])$/))) return scale === 200 ? (m[1] === "n" ? "ear" : "ear-f") : id;
  if ((m = id.match(/^(thigh|gaskin|shoulder|arm|forearm)-([nf])$/))) return elev ? id : `${nearFar(m[2]!)}-${m[1] === "arm" ? "humerus" : m[1]}`;
  if ((m = id.match(/^(hock-|cannon-h)([nf])$/))) return elev ? `cannon-h-${m[2]}` : scale === 200 ? `${nearFar(m[2]!)}-hind-lower` : `${nearFar(m[2]!)}-cannon-h`;
  if ((m = id.match(/^cannon-f([nf])$/))) return elev ? `cannon-f-${m[1]}` : scale === 200 ? `${nearFar(m[1]!)}-fore-lower` : `${nearFar(m[1]!)}-cannon-f`;
  if ((m = id.match(/^(pastern|dewclaw)-([hf])([nf])/))) return elev ? `pastern-${m[2]}-${m[3]}` : `${nearFar(m[3]!)}-pastern-${m[2]}`;
  if ((m = id.match(/^(claw|hoof)-([hf])([nf])/))) return elev ? `hoof-${m[2]}-${m[3]}` : `${nearFar(m[3]!)}-hoof-${m[2]}`;
  if ((m = id.match(/^(hind|fore)-(upper|lower)-([nf])$/))) return elev ? `leg-${m[1]}-${m[2]}-${m[3]}` : `${nearFar(m[3]!)}-leg-${m[1]}-${m[2]}`;
  throw new Error(`no shape for solid ${id}`);
}

/** 🧬️ The sibling a new shape takes its role, layer and tone from, with what differs. */
function template(id: string, have: (id: string) => boolean): { from: string; set: Json } {
  const pick = (...ids: string[]): string => ids.find(have) ?? (() => { throw new Error(`no template for ${id} among ${ids}`); })();
  if (id === "croup") return { from: "hindbody", set: { zAdd: -0.02 } };
  if (/^pastern-[hf]-[nf]$/.test(id)) return { from: `hoof-${id.slice(8)}`, set: { zAdd: -0.005 } };
  if (/^(near|far)-pastern-[hf]$/.test(id)) return { from: id.replace("pastern", "hoof"), set: {} };
  if (id === "udder") return { from: "hindbody", set: { zAdd: -0.05, tone: 0.82 } };
  if (id === "jaw") return { from: "head", set: { z: 3.9 } };
  if (id === "tail") return { from: "hindbody", set: { z: 2.8, tone: 0.66 } };
  if (id === "ear-n") return { from: "head", set: { z: 4.5, tone: 0.6 } };
  if (id === "ear-f") return { from: pick("ear-n", "ear"), set: { tone: 0.5, zAdd: -0.1 } };
  if (id === "horn-f") return { from: pick("horn-n", "horn"), set: { zAdd: -0.1 } };
  if (id === "arm-n") return { from: "shoulder-n", set: {} };
  if (id === "shoulder-n") return { from: "thigh-n", set: {} };
  if (id.endsWith("-f") && have(`${id.slice(0, -2)}-n`)) return { from: `${id.slice(0, -2)}-n`, set: { tone: 0.55, zAdd: -4 } };
  if (id.startsWith("far-") && have(`near-${id.slice(4)}`)) return { from: `near-${id.slice(4)}`, set: { tone: 0.55 } };
  if (/^(near|far)-hoof-h$/.test(id)) return { from: pick(`${id.split("-")[0]}-hind-lower`), set: {} };
  if (/^(near|far)-hoof-f$/.test(id)) return { from: pick(`${id.split("-")[0]}-fore-lower`), set: {} };
  throw new Error(`no template rule for ${id}`);
}

/** 🧭️ The outer boundary of the largest filled component of a mask as a polygon along the cell edges (world units of the view). */
function contour(mask: Mask, retainHoles = false): { ring: P[]; parts: number; rings: P[][] } {
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
  return { ring: outer[0] ?? [], parts: outer.length, rings: retainHoles ? loops : outer };
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

function outline(view: View, meshes: SolidMesh[], feature: number, keep?: (x: number, y: number) => boolean, retainHoles = false): { ring: P[]; parts: number; rings: P[][] } {
  const pts = meshes.flatMap((m) => Array.from({ length: m.world.length / 3 }, (_, i) => projectWorld(view, [m.world[3 * i]!, m.world[3 * i + 1]!, m.world[3 * i + 2]!])));
  const cell = feature / 16;
  const box = { minX: Math.min(...pts.map((p) => p.x)) - 3 * cell, minY: Math.min(...pts.map((p) => p.y)) - 3 * cell, maxX: Math.max(...pts.map((p) => p.x)) + 3 * cell, maxY: Math.max(...pts.map((p) => p.y)) + 3 * cell };
  const mask = meshMask(view, meshes, box, cell);
  if (keep !== undefined) for (let j = 0; j < mask.height; j++) for (let i = 0; i < mask.width; i++) if (!keep(box.minX + (i + 0.5) * cell, box.minY + (j + 0.5) * cell)) mask.cells[j * mask.width + i] = 0;
  const { ring, parts, rings } = contour(mask,retainHoles);
  const clean = (r: P[]): P[] => simplify(relax(r, 6), feature * 0.2);
  return { ring: clean(ring), parts, rings: rings.filter((r) => r.length * cell * cell > 0 && Math.abs(r.reduce((s, p, i) => s + p[0] * r[(i + 1) % r.length]![1] - r[(i + 1) % r.length]![0] * p[1], 0) / 2) > feature * feature).map(clean) };
}

const STROKES: Record<string, { bone: string; local: P[] }> = { "mouth-line": { bone: "head", local: [[0.243, -0.013], [0.2, -0.019], [0.152, -0.027]] }, "hock-line": { bone: "shank-hn", local: [[0.14, -0.012], [0.225, -0.012]] } };
const PROBE: P[] = [[0, 0], [0.1, 0], [0, 0.1], [0.1, 0.1]];

const { entry, asset: catalog } = entryOf(SPEC);
const asset: Json = merged(catalog, loadSource(entry)!);
const representations: Json = JSON.parse(JSON.stringify(asset.representations));
const log: string[] = [];
for (const scale of [50, 100, 200, 500]) {
  const body: Json[] = asset.volumes[String(scale)].body;
  const meshOf = new Map(meshesAt(asset, scale, body).map((m, i) => [body[i]!.id as string, m]));
  const feature = featureM(scale, asset);
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const old: Json[] = representations[String(scale)].geometry[view];
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const id = shapeOf(scale, view, s.id);
      if (id !== null) (groups.get(id) ?? groups.set(id, []).get(id)!).push(s);
    }
    const list: Json[] = old.filter((s) => groups.has(s.id) || s.id in STROKES);
    for (const s of old) if (!list.includes(s)) log.push(`1:${scale} ${view}: removed ${s.id} (no body part of its own)`);
    const have = (id: string): boolean => list.some((s) => s.id === id);
    let open = [...groups.keys()].filter((id) => !have(id));
    while (open.length > 0) {
      const before = open.length;
      for (const id of [...open]) {
        let t: { from: string; set: Json };
        try { t = template(id, have); } catch (error) { if (before === 1 || open.every((o) => { try { template(o, have); return false; } catch { return true; } })) throw error; continue; }
        const src = list.find((s) => s.id === t.from)!;
        const { zAdd, ...set } = t.set;
        const made: Json = { ...src, id, ...set, ...(zAdd === undefined ? {} : { z: r5(src.z + zAdd) }) };
        delete made.penMm;
        list.splice(list.indexOf(src) + 1, 0, made);
        log.push(`1:${scale} ${view}: added ${id} (bone ${groups.get(id)![0]!.bone}) beside ${t.from}`);
        open = open.filter((o) => o !== id);
      }
    }
    const tails = have("tail") || have("tail-root");
    const earShapes = have("ear-n") || have("ear");
    const shapes: Json[] = list.map((s) => {
      const solids = groups.get(s.id);
      const out: Json = {};
      for (const k of ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "penMm"]) if (s[k] !== undefined) out[k] = s[k];
      if (view === "front" && s.id === "neck") out.z = 3.4;
      if (view !== "elevation" && /^(near|far)-/.test(s.id)) out.tone = view === "front" && /^(near|far)-(shoulder|humerus|forearm|cannon-f|pastern-f|hoof-f|fore-lower|leg-fore)/.test(s.id) ? 0.72 : 0.55;
      if (view === "front" && /^(near|far)-(shoulder|humerus|forearm|cannon-f|pastern-f|hoof-f|fore-lower|leg-fore)/.test(s.id)) out.z = 3.6;
      if (solids !== undefined) {
        const bones = new Set(solids.map((q) => q.bone));
        if (bones.size !== 1) throw new Error(`1:${scale} ${view} ${s.id}: solids on several bones ${[...bones]}`);
        const bone = [...bones][0] as string;
        if (out.bone !== bone) { log.push(`1:${scale} ${view}: ${s.id} rebound ${out.bone} -> ${bone}`); out.bone = bone; }
        const regions = [...new Set(solids.map((q) => REGION_OF(q.id)[0]))];
        const isNew = !old.some((o) => o.id === s.id);
        if (isNew || (out.regions === undefined && out.region !== regions[0])) { delete out.regions; out.region = regions[0]; }
        if (out.regions !== undefined) { out.regions = (out.regions as string[]).filter((r) => !(r === "tail" && tails && s.id !== "tail") && !(r === "ears" && earShapes && !s.id.startsWith("ear"))); }
        out.filled = true;
        if (s.id === "horn") delete out.penMm;
      }
      out.kind = "poly";
      if (s.followTip !== undefined) out.followTip = s.followTip;
      out.points = PROBE;
      out.closed = solids !== undefined;
      return out;
    });
    const probe: Json = { ...asset, representations: { ...asset.representations, [String(scale)]: { ...asset.representations[String(scale)], geometry: { ...asset.representations[String(scale)].geometry, [view]: shapes } } } };
    const drawn: Json[] = (poseState(probe, scale) as Json)[view];
    const frames = poseState(probe, scale).frames;
    const more: { after: Json; shape: Json }[] = [];
    for (const s of shapes) {
      const ev = drawn.find((d) => d.shape.id === s.id);
      const q: { x: number; y: number }[] = ev?.rings?.[0] ?? [];
      if (q.length < 4) throw new Error(`1:${scale} ${view} ${s.id}: probe gave ${q.length} points`);
      const o = q[0]!;
      const ax: P = [(q[1]!.x - o.x) / 0.1, (q[1]!.y - o.y) / 0.1];
      const ay: P = [(q[2]!.x - o.x) / 0.1, (q[2]!.y - o.y) / 0.1];
      const err = Math.hypot(o.x + 0.1 * ax[0] + 0.1 * ay[0] - q[3]!.x, o.y + 0.1 * ax[1] + 0.1 * ay[1] - q[3]!.y);
      if (err > 1e-6) throw new Error(`1:${scale} ${view} ${s.id}: placement not affine (${err})`);
      const det = ax[0] * ay[1] - ax[1] * ay[0];
      const toLocal = (p: P): P => { const dx = p[0] - o.x; const dy = p[1] - o.y; return [r5((ay[1] * dx - ay[0] * dy) / det), r5((-ax[1] * dx + ax[0] * dy) / det)]; };
      const solids = groups.get(s.id);
      if (solids === undefined) {
        const st = STROKES[s.id]!;
        const f = frames[st.bone]!;
        s.points = st.local.map(([u, v]) => { const w = projectWorld(view, [f.o[0] + f.u[0] * u + f.v[0] * v, f.o[1] + f.u[1] * u + f.v[1] * v, f.o[2] + f.u[2] * u + f.v[2] * v]); return toLocal([w.x, w.y]); });
        continue;
      }
      const { ring, rings } = outline(view, solids.map((q2) => meshOf.get(q2.id)!), feature, view === "elevation" && s.id === "chest" && scale <= 100 ? (x, y) => x >= costalX(y) : undefined);
      s.points = ring.map(toLocal);
      rings.slice(1).forEach((extra, n) => { more.push({ after: s, shape: { ...s, id: `${s.id}-${"bcdefgh"[n]}`, points: extra.map(toLocal) } }); log.push(`1:${scale} ${view}: ${s.id} has a separate part, drawn as ${s.id}-${"bcdefgh"[n]}`); });
    }
    for (const m of more) shapes.splice(shapes.indexOf(m.after) + 1, 0, m.shape);
    representations[String(scale)].geometry[view] = shapes;
    console.log(`1:${scale} ${view}: ${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  }
}
const decisions: Json = JSON.parse(readFileSync(join(import.meta.dir, "r3-goat-rep-decisions.json"), "utf8"));
for (const scale of [50, 100, 200, 500]) representations[String(scale)].decisions = decisions[String(scale)];
mkdirSync(join(GENERATED, "r3-goat"), { recursive: true });
writeFileSync(join(GENERATED, "r3-goat", "reps.json"), JSON.stringify({ representations }));
writeFileSync(join(GENERATED, "r3-goat", "draw2d-log.txt"), log.join("\n") + "\n");
console.log(log.join("\n"));
