/** ✏️ Round 3 london plane (pollarded): redraws the 2D shapes of every scale and view as the true orthographic views of that scale's body (ruling E1), shape by shape with the ids, roles, bone bindings and line hierarchy of the drawing.
 *
 * Usage: `bun r5-london-plane-pollarded-draw.ts` writes `🗑️generated/r5-london-plane-pollarded/geometry.json` = `{ "50": { elevation, front, plan }, … }` for `r3-london-plane-pollarded-apply.py --geometry`.
 * Outlines are the projected outline of the named solids (raster cell a quarter of the smallest feature, reduced to 0.35 feature), strokes are the projected centre lines; cluster arcs and leaf marks are placed on the clusters nearest to the viewer.
 * @see ./r5-london-plane-pollarded-model.ts */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { TICKET, featureM, frames, meshesAt, toWorld, type Json, type V3 } from "./volumes-lib.ts";
import { GROUPS, REGS, asset, build } from "./r5-london-plane-pollarded-model.ts";

type P = [number, number];
type View = "elevation" | "front" | "plan";
const F = frames(asset);
const lists = build();
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const proj = (view: View, p: readonly number[]): P => (view === "elevation" ? [p[0]!, p[1]!] : view === "front" ? [p[2]!, p[1]!] : [p[0]!, p[2]!]);
/** 🧭 A view point in the coordinates of a shape: bone-local (u, v) in the elevation, world rest coordinates in front and plan. */
const toShape = (view: View, bone: string, q: P): P => {
  if (view !== "elevation") return q;
  const f = F[bone]!, dx = q[0] - f.o[0], dy = q[1] - f.o[1];
  return [dx * f.u[0] + dy * f.u[1], dx * f.v[0] + dy * f.v[1]];
};
const path = (pts: readonly P[], closed: boolean): string => `M ${pts.map((p) => `${r4(p[0])} ${r4(p[1])}`).join(" L ")}${closed ? " Z" : ""}`;

/** 🪚 Douglas-Peucker on an open polyline. */
function reduce(pts: readonly P[], tol: number): P[] {
  if (pts.length < 3) return [...pts];
  const keep = new Uint8Array(pts.length);
  keep[0] = keep[pts.length - 1] = 1;
  const stack: [number, number][] = [[0, pts.length - 1]];
  while (stack.length) {
    const [a, b] = stack.pop()!;
    const [ax, ay] = pts[a]!, [bx, by] = pts[b]!, L = Math.hypot(bx - ax, by - ay) || 1e-12;
    let worst = -1, far = tol;
    for (let i = a + 1; i < b; i++) { const d = Math.abs((pts[i]![0] - ax) * (by - ay) - (pts[i]![1] - ay) * (bx - ax)) / L; if (d > far) { far = d; worst = i; } }
    if (worst > 0) { keep[worst] = 1; stack.push([a, worst], [worst, b]); }
  }
  return pts.filter((_, i) => keep[i]);
}

/** 🖼️ The outer outlines of the projection of meshes in a view: filled raster, boundary loops, reduced. */
function outline(view: View, meshes: readonly { world: Float64Array; indices: Uint32Array }[], cell: number, tol: number): P[][] {
  let x0 = Infinity, y0 = Infinity, x1 = -Infinity, y1 = -Infinity;
  for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) { const [x, y] = proj(view, [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]); x0 = Math.min(x0, x); x1 = Math.max(x1, x); y0 = Math.min(y0, y); y1 = Math.max(y1, y); }
  x0 -= 2 * cell; y0 -= 2 * cell;
  const W = Math.ceil((x1 - x0) / cell) + 3, H = Math.ceil((y1 - y0) / cell) + 3;
  const g = new Uint8Array(W * H);
  for (const m of meshes) for (let t = 0; t < m.indices.length; t += 3) {
    const [a, b, c] = [0, 1, 2].map((k) => { const i = m.indices[t + k]! * 3; const q = proj(view, [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]); return [(q[0] - x0) / cell, (q[1] - y0) / cell] as P; });
    const lo = Math.max(0, Math.floor(Math.min(a![1], b![1], c![1]))), hi = Math.min(H - 1, Math.ceil(Math.max(a![1], b![1], c![1])));
    for (let j = lo; j <= hi; j++) {
      const y = j + 0.5, xs: number[] = [];
      for (const [p, q] of [[a!, b!], [b!, c!], [c!, a!]] as const) if ((p[1] <= y) !== (q[1] <= y)) xs.push(p[0] + ((y - p[1]) * (q[0] - p[0])) / (q[1] - p[1]));
      if (xs.length === 2) for (let i = Math.max(0, Math.round(Math.min(xs[0]!, xs[1]!))); i < Math.min(W, Math.round(Math.max(xs[0]!, xs[1]!))); i++) g[j * W + i] = 1;
    }
  }
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= W || j >= H ? 0 : g[j * W + i]!);
  const next = new Map<number, number[]>();
  const key = (i: number, j: number): number => j * (W + 1) + i;
  const edge = (ai: number, aj: number, bi: number, bj: number): void => void (next.get(key(ai, aj)) ?? next.set(key(ai, aj), []).get(key(ai, aj))!).push(key(bi, bj));
  for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) if (g[j * W + i]) {
    if (!at(i, j - 1)) edge(i, j, i + 1, j);
    if (!at(i + 1, j)) edge(i + 1, j, i + 1, j + 1);
    if (!at(i, j + 1)) edge(i + 1, j + 1, i, j + 1);
    if (!at(i - 1, j)) edge(i, j + 1, i, j);
  }
  const loops: P[][] = [];
  for (const start of [...next.keys()]) {
    while ((next.get(start) ?? []).length) {
      const loop: P[] = [];
      let k = start, prev: P | null = null;
      for (;;) {
        const outs = next.get(k);
        if (!outs || !outs.length) break;
        const here: P = [k % (W + 1), Math.floor(k / (W + 1))];
        let pick = 0;
        if (outs.length > 1 && prev) { const d: P = [here[0] - prev[0], here[1] - prev[1]]; pick = outs.findIndex((o) => { const e: P = [(o % (W + 1)) - here[0], Math.floor(o / (W + 1)) - here[1]]; return d[0] * e[1] - d[1] * e[0] > 0; }); if (pick < 0) pick = 0; }
        const to = outs.splice(pick, 1)[0]!;
        loop.push([x0 + here[0] * cell, y0 + here[1] * cell]);
        prev = here; k = to;
        if (k === start) break;
      }
      let area = 0;
      for (let i = 0; i < loop.length; i++) { const a = loop[i]!, b = loop[(i + 1) % loop.length]!; area += a[0] * b[1] - b[0] * a[1]; }
      if (area / 2 > 36 * cell * cell) {
        const far = loop.reduce((m, p, i) => (Math.hypot(p[0] - loop[0]![0], p[1] - loop[0]![1]) > Math.hypot(loop[m]![0] - loop[0]![0], loop[m]![1] - loop[0]![1]) ? i : m), 0);
        loops.push([...reduce(loop.slice(0, far + 1), tol).slice(0, -1), ...reduce([...loop.slice(far), loop[0]!], tol).slice(0, -1)]);
      }
    }
  }
  return loops;
}

const SHAPE_KEYS = ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const MASS_OF: Record<string, string> = { "fol-core": "core", "fol-top": "top", "fol-e2": "o-0", "fol-w2": "o-180", "fol-e1": "m-0", "fol-w1": "m-180", "fol-low-e": "lo-0", "fol-low-w": "lo-180" };
const out: Record<string, Record<View, Json[]>> = {};

for (const scale of [50, 100, 200, 500]) {
  const key = String(scale), feat = featureM(scale, asset), L = lists[key]!, reg = REGS[key]!;
  const meshes = new Map(meshesAt({ ...asset, volumes: Object.fromEntries(Object.entries(lists).map(([k, v]) => [k, { status: "authored", denominator: Number(k), ...v }])) }, scale, L.body).map((m) => [m.id, m]));
  const solids = (test: RegExp | ((id: string) => boolean)) => L.body.filter((s) => (test instanceof RegExp ? test.test(s.id) : test(s.id))).map((s) => meshes.get(s.id)!);
  const groupIds = (g: string): string[] => Object.entries(reg).filter(([, v]) => v.group === g).map(([id]) => id);
  const drawnGroups = [...new Set(Object.values(reg).map((v) => v.group).filter((x): x is string => x !== undefined))];
  const centre = (id: string): V3 => { const s = reg[id]!.secs; const a = s[0]!.c, b = s[s.length - 1]!.c; return [(a[0] + b[0]) / 2, (a[1] + b[1]) / 2, (a[2] + b[2]) / 2]; };
  const groupCentre = (g: string): V3 => { const ids = groupIds(g); return ids.map(centre).reduce((a, c) => [a[0] + c[0] / ids.length, a[1] + c[1] / ids.length, a[2] + c[2] / ids.length] as V3, [0, 0, 0] as V3); };
  const depth = (view: View, p: V3): number => (view === "elevation" ? -p[2] : view === "front" ? p[0] : p[1]);
  const zOf = (view: View, g: string): number => { const d = depth(view, groupCentre(g)); return r4(view === "elevation" ? 4.5 + d / 2.4 : view === "front" ? 4 + d / 3.4 : 3 + (d - 4.5) / 2.2); };
  const closedOf = (view: View, bone: string, ms: readonly { world: Float64Array; indices: Uint32Array }[], soft: boolean): string | null => {
    const loops = outline(view, ms, feat / 4, feat * (soft ? 0.35 : 0.2));
    return loops.length ? loops.map((l) => path(l.map((q) => toShape(view, bone, q)), true)).join(" ") : null;
  };
  /** 🌙 The upper arc of the cluster of a group that is nearest to the viewer (rank `n`), inside its outline. */
  const arc = (view: View, bone: string, g: string, n: number): string => {
    const ids = groupIds(g).filter((id) => !/tip$/.test(id)).sort((a, b) => depth(view, centre(b)) - depth(view, centre(a)));
    const id = ids[Math.min(n, ids.length - 1)]!, s = reg[id]!.secs, c = centre(id);
    const r = Math.max(...s.map((x) => x.r)), rz = Math.max(...s.map((x) => x.rz ?? x.r)), h = Math.abs(s[s.length - 1]!.c[1] - s[0]!.c[1]) / 2;
    const [cx, cy] = proj(view, c), [ax, ay] = view === "plan" ? [r, rz] : view === "front" ? [rz, h] : [r, h];
    const from = view === "plan" ? 200 + n * 70 : 40 + n * 25, pts: P[] = [];
    for (let i = 0; i <= 8; i++) { const t = ((from + (i * 80) / 8) * Math.PI) / 180; pts.push(toShape(view, bone, [cx + 0.72 * ax * Math.cos(t), cy + 0.72 * ay * Math.sin(t)])); }
    return path(pts, false);
  };
  const line = (view: View, bone: string, pts: readonly V3[]): string => path(reduce(pts.map((p) => toShape(view, bone, proj(view, p))), feat * 0.2), false);
  const sectionLine = (view: View, id: string, bone?: string): string | null => { const s = L.body.find((x) => x.id === id); return s ? line(view, bone ?? s.bone, (s.sections as Json[]).map((x) => toWorld(F[s.bone]!, x.at))) : null; };
  const tipLine = (view: View, bone: string): string => { const f = F[bone]!, len = asset.rig.bones.find((b: Json) => b.id === bone).lengthM; return line(view, bone, [f.o as V3, [f.o[0] + f.u[0] * len, f.o[1] + f.u[1] * len, f.o[2] + f.u[2] * len]]); };
  const old = asset.representations[key].geometry as Record<View, Json[]>;
  const base = (s: Json): Json => Object.fromEntries(SHAPE_KEYS.filter((k) => s[k] !== undefined).map((k) => [k, s[k]]));
  const asPath = (s: Json, d: string, patch: Json = {}): Json => { const b = { ...base(s), ...patch }; return { id: b.id, bone: b.bone, role: b.role, z: b.z, filled: b.filled, tone: b.tone, kind: "path", d, ...Object.fromEntries(Object.entries(b).filter(([k]) => !["id", "bone", "role", "z", "filled", "tone"].includes(k))) }; };
  out[key] = { elevation: [], front: [], plan: [] };
  for (const view of ["elevation", "front", "plan"] as const) {
    const res: Json[] = [];
    const seenGroups = new Set<string>();
    const mass = (s: Json, g: string, patch: Json = {}): void => {
      const d = closedOf(view, patch.bone ?? s.bone, solids((id) => groupIds(g).includes(id)), true);
      if (d === null) return;
      seenGroups.add(g);
      res.push(asPath(s, d, { z: g === "crown" ? s.z : zOf(view, g), ...patch }));
    };
    const added = (id: string): boolean => /^(e-|twig-)/.test(id) || (scale === 500 && /^shoot-/.test(id)) || (view === "plan" && /^(root-plate-mass|knuckle-)/.test(id)) || (view === "plan" && scale === 50 && /^plan-s-/.test(id)) || (scale === 100 && view === "elevation" && /^shoot-w1-b/.test(id));
    for (const s of old[view].filter((x) => !added(x.id))) {
      const id = s.id as string;
      let m: RegExpMatchArray | null;
      if (/^crown-fill|^s-w2a-mass$/.test(id) && view === "plan") continue;
      if (id === "crown") { mass(s, "crown", { bone: "fol-core" }); continue; }
      if ((m = id.match(/^(fol-[a-z0-9-]+)-mass$/))) { mass(s, MASS_OF[m[1]!]!); continue; }
      if ((m = id.match(/^[fp]-([a-z]+(?:-\d+)?)$/)) && GROUPS.some((g) => g.id === m![1])) { if (drawnGroups.includes(m[1]!)) mass(s, m[1]!); continue; }
      if ((m = id.match(/^[fp]-([a-z]+(?:-\d+)?)-arc$/))) { if (drawnGroups.includes(m[1]!)) res.push(asPath(s, arc(view, s.bone, m[1]!, 0), { z: r4(zOf(view, m[1]!) + 0.0005) })); continue; }
      if ((m = id.match(/^(fol-[a-z0-9-]+)-arc(\d)$/))) { const g = MASS_OF[m[1]!]!; res.push(asPath(s, arc(view, s.bone, g, Number(m[2]) - 1), { z: r4(zOf(view, g) + 0.0001 * Number(m[2])) })); continue; }
      if ((m = id.match(/^(fol-[a-z0-9-]+)-leaf(\d)$/))) {
        const g = MASS_OF[m[1]!]!, ids = groupIds(g).sort((a, b) => depth(view, centre(b)) - depth(view, centre(a)));
        const c = toShape(view, s.bone, proj(view, centre(ids[(Number(m[2]) - 1) % ids.length]!)));
        const nums = (s.d as string).match(/-?\d+(\.\d+)?/g)!.map(Number), xs = nums.filter((_, i) => i % 2 === 0), ys = nums.filter((_, i) => i % 2 === 1);
        const dx = c[0] - (Math.min(...xs) + Math.max(...xs)) / 2 + 0.1 * (Number(m[2]) % 2 ? 1 : -1), dy = c[1] - (Math.min(...ys) + Math.max(...ys)) / 2 - 0.12;
        let i = 0;
        res.push({ ...s, z: r4(zOf(view, g) + 0.0008), d: (s.d as string).replace(/-?\d+(\.\d+)?/g, (v) => String(r4(Number(v) + (i++ % 2 === 0 ? dx : dy)))) });
        continue;
      }
      if ((m = id.match(/^root-ridge-([we])$/))) { const d = sectionLine(view, m[1] === "w" ? "buttress-3" : "buttress-1", s.bone); if (d) res.push(asPath(s, d)); continue; }
      if ((id === "stem" || id === "trunk-dot" || id === "head") && view === "plan") {
        if (id === "head" && scale === 50) { res.push(asPath(s, closedOf(view, s.bone, solids(/^head(-burr-\d)?$/), false)!)); continue; }
        const w = meshes.get(id === "head" ? "head" : "stem")!.world, xs: number[] = [], zs: number[] = [];
        for (let i = 0; i < w.length; i += 3) { xs.push(w[i]!); zs.push(w[i + 2]!); }
        const [ax, bx, az, bz] = [Math.min(...xs), Math.max(...xs), Math.min(...zs), Math.max(...zs)];
        res.push({ ...s, cx: r4((ax + bx) / 2), cy: r4((az + bz) / 2), rx: r4((bx - ax) / 2), ry: r4((bz - az) / 2) });
        continue;
      }
      if (id === "stem") { const d = closedOf(view, s.bone, solids(view === "elevation" && scale !== 500 ? /^stem$/ : /^(stem|root-flare|buttress-\d)$/), false); res.push(asPath(s, d!)); continue; }
      if (id === "root-plate-mass") { res.push(asPath(s, closedOf(view, s.bone, solids(/^(root-flare|buttress-\d)$/), false)!)); continue; }
      if (id === "head") { res.push(asPath(s, closedOf(view, s.bone, solids(/^head(-burr-\d)?$/), false)!)); continue; }
      if ((m = id.match(/^knuckle-(e2|e1|c|w1|w2)$/))) { res.push(asPath(s, closedOf(view, s.bone, solids(new RegExp(`^knuckle-${m[1]}(-|$)`)), false)!)); continue; }
      if ((m = id.match(/^shoot-(e2|e1|c|w1|w2)-?([ab])$/)) || (m = id.match(/^plan-s-(e2|e1|c|w1|w2)(a)$/))) { const d = closedOf(view, s.bone, solids(new RegExp(`^shoot-${m[1]}-${m[2]}$|^shoot-${m[1]}-node$`.replace(m[2] === "b" ? /\|.*$/ : /^$/, ""))), false); if (d) res.push(asPath(s, d)); continue; }
      if ((m = id.match(/^shoot-(e2|w2)$/)) || (m = id.match(/^s-(e2|w2)a-mass$/))) { res.push(asPath(s, closedOf(view, s.bone, solids(new RegExp(`^shoot-${m[1]}$`)), false)!)); continue; }
      if ((m = id.match(/^shoot-(e2|e1|c|w1|w2)-b-line$/))) { res.push(view === "elevation" ? s : asPath(s, tipLine(view, s.bone))); continue; }
      if ((m = id.match(/^pl-shoot-(e2|e1|c|w1|w2)$/))) { res.push(asPath(s, tipLine(view, s.bone))); continue; }
      res.push(s);
    }
    const first = res.findIndex((s) => /^(fol-|[fp]-)/.test(s.id));
    const extra: Json[] = [];
    for (const g of drawnGroups) if (!seenGroups.has(g) && g !== "crown") {
      const G = GROUPS.find((x) => x.id === g)!, d = closedOf(view, G.bone, solids((id) => groupIds(g).includes(id)), true);
      if (d) extra.push({ id: `${view === "elevation" ? "e" : view === "front" ? "f" : "p"}-${g}`, bone: G.bone, role: "body", z: zOf(view, g), filled: true, tone: G.region === "leaf-shade" ? 0.45 : 0.6, kind: "path", d, region: G.region, material: G.region === "leaf-shade" ? "plane-leaf-shade" : "plane-leaf-sun" });
    }
    if (scale <= 100 && view !== "plan") for (const s of L.body.filter((x) => /^shoot-/.test(x.id) && !/^shoot-(e2|e1|c|w1|w2)-(a|b|node)$/.test(x.id))) extra.push({ id: `twig-${(s.id as string).replace(/^shoot-/, "")}`, bone: s.bone, role: "detail", z: 2.25, filled: false, tone: 0.35, kind: "path", d: sectionLine(view, s.id)! });
    if (scale === 500 && view === "elevation") for (const s of L.body.filter((x) => /^outer-shoot-/.test(x.id))) extra.push({ id: (s.id as string).replace(/^outer-/, ""), bone: s.bone, role: "body", z: 1.9, filled: true, tone: 0.35, kind: "path", d: closedOf(view, s.bone, solids(new RegExp(`^${s.id}$`)), false)!, region: "shoot-bark", material: "plane-twig" });
    if (scale === 100 && view === "elevation" && !res.some((s) => s.id === "shoot-w1-b")) {
      const like = res.find((s) => s.id === "shoot-e1-b")!, lineLike = res.find((s) => s.id === "shoot-e1-b-line")!;
      extra.push(asPath({ ...like, id: "shoot-w1-b", bone: "s-w1b" }, closedOf(view, "s-w1b", solids(/^shoot-w1-b$/), false)!), { ...lineLike, id: "shoot-w1-b-line", bone: "s-w1b", d: `M ${r4(asset.rig.bones.find((b: Json) => b.id === "s-w1b").lengthM)} 0 L 0 0` });
    }
    if (view === "plan" && scale === 50) for (const k of ["e2", "e1", "c", "w1", "w2"]) extra.push({ id: `plan-s-${k}a`, bone: `s-${k}a`, role: "detail", z: 5.5015, filled: true, tone: 0.35, kind: "path", d: closedOf(view, `s-${k}a`, solids(new RegExp(`^shoot-${k}-(a|node)$`)), false)!, region: "shoot-bark", material: "plane-twig" });
    if (view === "plan" && scale !== 500) {
      const flare = old.elevation.find((s) => s.id === "root-plate-mass")!;
      extra.push(asPath(flare, closedOf(view, "root-plate", solids(/^(root-flare|buttress-\d)$/), false)!, { region: "trunk-bark", material: "plane-bark" }));
      for (const s of old.elevation.filter((x) => /^knuckle-(e2|e1|c|w1|w2)$/.test(x.id))) extra.push(asPath(s, closedOf(view, s.bone, solids(new RegExp(`^${s.id}(-|$)`)), false)!));
    }
    res.splice(first < 0 ? res.length : first, 0, ...extra);
    out[key]![view] = res;
  }
  console.log(`1:${scale}`, (["elevation", "front", "plan"] as const).map((v) => `${v} ${out[key]![v].length} shapes (${old[v].length} before)`).join(", "));
}
writeFileSync(join(TICKET, "🗑️generated", "r5-london-plane-pollarded", "geometry.json"), `${JSON.stringify(out)}\n`);
