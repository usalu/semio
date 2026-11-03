import { readFileSync, writeFileSync } from "node:fs";
import { fileOf } from "./lib.ts";
import { catalog, extentOf, sweep, consistency, authoredScales } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧊️three-views/🧰️.ts";
import { validateAsset } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const META = new Set(["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm", "solid"]);
const r3 = (n: number) => Math.round(n * 1000) / 1000;
type Box = { x0: number; x1: number; y0: number; y1: number };
const pts = (s: any): [number, number][] => {
  if (s.kind === "rect") return [[s.x, s.y], [s.x + s.w, s.y + s.h]];
  if (s.kind === "ellipse") return [[s.cx - s.rx, s.cy - s.ry], [s.cx + s.rx, s.cy + s.ry]];
  if (s.kind === "poly") return s.points;
  if (s.kind === "capsule") return [[s.x0 - s.r0, s.y0 - s.r0], [s.x0 + s.r0, s.y0 + s.r0], [s.x1 - s.r1, s.y1 - s.r1], [s.x1 + s.r1, s.y1 + s.r1]];
  if (s.kind === "path") {
    if (/[^MLCQZmlcqz0-9eE.,\s+-]/.test(s.d)) throw new Error(`unsupported path ${s.id}: ${s.d}`);
    const n = (s.d.match(/-?\d*\.?\d+(?:e[+-]?\d+)?/gi) ?? []).map(Number);
    const out: [number, number][] = [];
    for (let i = 0; i + 1 < n.length; i += 2) out.push([n[i], n[i + 1]]);
    return out;
  }
  throw new Error(`kind ${s.kind}`);
};
const box = (list: [number, number][]): Box => ({ x0: Math.min(...list.map((p) => p[0])), x1: Math.max(...list.map((p) => p[0])), y0: Math.min(...list.map((p) => p[1])), y1: Math.max(...list.map((p) => p[1])) });
const union = (boxes: Box[]): Box | undefined => (boxes.length === 0 ? undefined : { x0: Math.min(...boxes.map((b) => b.x0)), x1: Math.max(...boxes.map((b) => b.x1)), y0: Math.min(...boxes.map((b) => b.y0)), y1: Math.max(...boxes.map((b) => b.y1)) });

const flatten = (d: string): [number, number][] | undefined => {
  if (/[^MLCQ0-9eE.,\s+-]/.test(d)) return undefined;
  const out: [number, number][] = [];
  for (const seg of d.match(/[MLCQ][^MLCQ]*/g) ?? []) {
    const n = (seg.slice(1).match(/-?\d*\.?\d+(?:e[+-]?\d+)?/gi) ?? []).map(Number);
    const c = seg[0];
    if (c === "M" || c === "L") for (let i = 0; i + 1 < n.length; i += 2) out.push([n[i], n[i + 1]]);
    else {
      const k = c === "C" ? 6 : 4;
      for (let i = 0; i + k - 1 < n.length; i += k) {
        const a = out[out.length - 1];
        if (!a) return undefined;
        for (let j = 1; j <= 8; j++) {
          const u = j / 8, v = 1 - u;
          if (c === "C") out.push([v ** 3 * a[0] + 3 * v * v * u * n[i] + 3 * v * u * u * n[i + 2] + u ** 3 * n[i + 4], v ** 3 * a[1] + 3 * v * v * u * n[i + 1] + 3 * v * u * u * n[i + 3] + u ** 3 * n[i + 5]]);
          else out.push([v * v * a[0] + 2 * v * u * n[i] + u * u * n[i + 2], v * v * a[1] + 2 * v * u * n[i + 1] + u * u * n[i + 3]]);
        }
      }
    }
  }
  return out;
};

const thinQuad = (s: any, w: number): any | undefined => {
  let p: [number, number][] | undefined;
  if (s.kind === "poly" && s.closed === false) p = s.points;
  else if (s.kind === "path") p = flatten(s.d);
  if (!p || p.length < 2) return undefined;
  const n = p.map((q, i) => { const a = p![Math.max(0, i - 1)], b = p![Math.min(p!.length - 1, i + 1)]; const dx = b[0] - a[0], dy = b[1] - a[1], l = Math.hypot(dx, dy) || 1; return [-dy / l, dx / l]; });
  const left = p.map((q, i) => [r3(q[0] + (n[i][0] * w) / 2), r3(q[1] + (n[i][1] * w) / 2)]);
  const right = p.map((q, i) => [r3(q[0] - (n[i][0] * w) / 2), r3(q[1] - (n[i][1] * w) / 2)]).reverse();
  return { kind: "poly", points: [...left, ...right], closed: true };
};

export function convert(asset: any, log: string[], theta: Map<string, number> = new Map()): number {
  const bones: any[] = asset.rig.bones;
  const byId = new Map(bones.map((b) => [b.id, b]));
  const seeds = new Set<string>();
  const scan = (o: any, owner?: string): void => { if (o === null || typeof o !== "object") return; for (const [k, v] of Object.entries<any>(o)) { if ((k === "twistDeg" || k === "abductionDeg") && v !== 0 && owner) seeds.add(owner); else scan(v, /^[a-z0-9-]+$/.test(k) && byId.has(k) ? k : owner); } };
  scan(asset.poses.map((p: any) => p.bones)); scan(asset.clips.map((c: any) => c.keys.map((k: any) => k.bones)));
  const spread = new Set<string>();
  const rides = (id: string): boolean => { for (let b = byId.get(id); b; b = b.parent ? byId.get(b.parent) : undefined) if (seeds.has(b.id)) return true; return false; };
  for (const b of bones) if (rides(b.id)) spread.add(b.id);
  const rest = new Map<string, number[]>();
  const restOf = (id: string): number[] => {
    if (rest.has(id)) return rest.get(id)!;
    const b = byId.get(id);
    const p = b.parent ? restOf(b.parent) : [1, 0, 0, 1, 0, 0];
    const a = (b.angleDeg * Math.PI) / 180, c = Math.cos(a), s = Math.sin(a);
    const l = [c, s, -s, c, b.x, b.y];
    const m = [p[0] * l[0] + p[2] * l[1], p[1] * l[0] + p[3] * l[1], p[0] * l[2] + p[2] * l[3], p[1] * l[2] + p[3] * l[3], p[0] * l[4] + p[2] * l[5] + p[4], p[1] * l[4] + p[3] * l[5] + p[5]];
    rest.set(id, m);
    return m;
  };
  const toLocal = (id: string, x: number, y: number): [number, number] => { const m = restOf(id); const det = m[0] * m[3] - m[1] * m[2]; const dx = x - m[4], dy = y - m[5]; return [(m[3] * dx - m[2] * dy) / det, (-m[1] * dx + m[0] * dy) / det]; };
  let changed = 0;
  for (const rep of Object.values<any>(asset.representations)) {
    if (rep.status !== "authored") continue;
    const G = rep.geometry;
    const lat = (view: string, s: any): [number, number] => { const b = box(pts(s)); return view === "front" ? [b.x0, b.x1] : [b.y0, b.y1]; };
    const sameBone = (view: string, bone: string) => G[view].filter((s: any) => s.bone === bone && !s.solid && s.role !== "ground");
    const original = { elevation: [...G.elevation], front: [...G.front], plan: [...G.plan] } as Record<string, any[]>;
    const latOf = (view: string, s: any): [number, number] => {
      if (view !== "elevation") return lat(view, s);
      const f = original.front.find((x) => x.id === s.id && x.bone === s.bone), p = original.plan.find((x) => x.id === s.id && x.bone === s.bone);
      if (f && !f.solid) return lat("front", f);
      if (p && !p.solid) return lat("plan", p);
      if (f?.solid) return [f.solid.z0, f.solid.z1];
      if (p?.solid) return [p.solid.z0, p.solid.z1];
      const u = union([...sameBone("front", s.bone).map((x: any) => { const [a, b] = lat("front", x); return { x0: a, x1: b, y0: 0, y1: 0 }; }), ...sameBone("plan", s.bone).map((x: any) => { const [a, b] = lat("plan", x); return { x0: a, x1: b, y0: 0, y1: 0 }; })]);
      const fs = original.front.concat(original.plan).filter((x) => x.bone === s.bone && x.solid?.kind === "extruded");
      if (u) return [u.x0, u.x1];
      if (fs.length) return [Math.min(...fs.map((x) => x.solid.z0)), Math.max(...fs.map((x) => x.solid.z1))];
      return [-0.02, 0.02];
    };
    for (const view of ["elevation", "front", "plan"]) {
      G[view] = G[view].map((s: any) => {
        if (!spread.has(s.bone) || s.solid !== undefined || s.role === "ground") return s;
        const pb = box(pts(s));
        const err = (Math.max(pb.x1 - pb.x0, pb.y1 - pb.y0) * Math.sin(((theta.get(s.bone) ?? 90) * Math.PI) / 180) * 1000) / rep.denominator;
        if (err <= 0.2) return s;
        if (view !== "elevation") {
          const isOpen = (s.kind === "poly" && s.closed === false) || (s.kind === "path" && !/[Zz]/.test(s.d));
          const b0 = box(pts(s));
          const dx = b0.x1 - b0.x0, dy = b0.y1 - b0.y0;
          if (isOpen && Math.min(dx, dy) > 0.1 * Math.max(dx, dy) && Math.min(dx, dy) > 0.05) { log.push(`${asset.id} 1:${rep.denominator} ${view} ${s.id}: diagonal open line left flat`); return s; }
        }
        let [z0, z1] = latOf(view, s);
        if (z1 - z0 < 0.02) { const m = (z0 + z1) / 2; z0 = m - 0.01; z1 = m + 0.01; }
        const solid = { kind: "extruded", z0: r3(z0), z1: r3(z1) };
        changed++;
        if (view === "elevation") {
          const open = (s.kind === "poly" && s.closed === false) || (s.kind === "path" && !/[Zz]/.test(s.d));
          if (open) {
            const w = Math.max(0.01, ((s.penMm ?? rep.pens[s.role] ?? 0.13) * rep.denominator) / 1000);
            const q = thinQuad(s, w);
            if (!q) { log.push(`${asset.id} 1:${rep.denominator} elevation ${s.id}: open curve left flat`); changed--; return s; }
            return { ...q, ...Object.fromEntries(Object.entries(s).filter(([k]) => META.has(k))), solid };
          }
          return { ...s, solid };
        }
        const twin = original.elevation.find((x) => x.id === s.id && x.bone === s.bone);
        const meta = Object.fromEntries(Object.entries(s).filter(([k]) => META.has(k)));
        if (twin) return { ...Object.fromEntries(Object.entries(twin).filter(([k]) => !META.has(k))), ...meta, solid };
        const own = box(pts(s));
        const pool = (view === "front" ? original.plan : original.front).filter((x) => x.bone === s.bone);
        const myLat = lat(view, s);
        const score = (x: any): number => { const l = lat(view === "front" ? "plan" : "front", x); return Math.abs(l[0] - myLat[0]) + Math.abs(l[1] - myLat[1]); };
        const other = pool.find((x) => x.id === s.id) ?? [...pool].sort((p, q) => score(p) - score(q)).find((x) => score(x) < 0.15 * (myLat[1] - myLat[0] + 0.1));
        const ob = other ? box(pts(other)) : undefined;
        const bonePlan = union(original.plan.filter((x) => x.bone === s.bone).map((x) => box(pts(x))));
        const boneFront = union(original.front.filter((x) => x.bone === s.bone).map((x) => box(pts(x))));
        const fx = view === "plan" ? [own.x0, own.x1] : ob ? [ob.x0, ob.x1] : bonePlan ? [bonePlan.x0, bonePlan.x1] : undefined;
        const hy = view === "front" ? [own.y0, own.y1] : ob ? [ob.y0, ob.y1] : boneFront ? [boneFront.y0, boneFront.y1] : undefined;
        if (!fx || !hy) throw new Error(`${asset.id} ${rep.denominator} ${view} ${s.id}: no forward/height range`);
        const c = [toLocal(s.bone, fx[0], hy[0]), toLocal(s.bone, fx[1], hy[0]), toLocal(s.bone, fx[1], hy[1]), toLocal(s.bone, fx[0], hy[1])].map(([a, b]) => [r3(a), r3(b)]);
        log.push(`${asset.id} 1:${rep.denominator} ${view} ${s.id} (${s.bone}): synthesised box profile`);
        return { kind: "poly", points: c, closed: true, ...meta, solid };
      });
    }
  }
  log.push(`${asset.id}: spread bones [${[...spread].join(", ")}]; ${changed} shapes made solids`);
  return changed;
}

if (import.meta.main) {
  const ids = process.argv.slice(2).filter((a) => !a.startsWith("--"));
  const write = process.argv.includes("--write");
  for (const id of ids) {
    const file = fileOf(id);
    const doc = JSON.parse(readFileSync(file, "utf8"));
    const before = catalog().find((a) => a.id === id)!;
    const log: string[] = [];
    const ctx = { assets: new Set(catalog().map((a) => a.id)) };
    const theta = new Map<string, number>();
    for (const i of validateAsset(doc.asset, ctx)) { const m = /on bone "([^"]+)".*?up to ([0-9.]+) degrees/.exec(i.message.en); if (i.code === "lateral-ambiguous" && m) theta.set(m[1], Math.max(theta.get(m[1]) ?? 0, Number(m[2]))); }
    convert(doc.asset, log, theta);
    console.log(log.slice(-12).join("\n"));
    const issues = validateAsset(doc.asset, { assets: new Set(catalog().map((a) => a.id)) }).filter((i) => !["view-twin-missing"].includes(i.code));
    console.log(`${id}: remaining issues ${issues.length}`, [...new Set(issues.map((i) => i.code))].join(","));
    for (const i of issues.slice(0, 6)) console.log("   ", i.code, i.path, i.message.en.slice(0, 160));
    const f: string[] = [];
    for (const s of authoredScales(doc.asset)) for (const x of [...sweep(doc.asset, s), ...consistency(doc.asset, s)]) f.push(`[${x.check}] ${x.message}`);
    console.log(`${id}: guard findings ${f.length}`); for (const x of f.slice(0, 6)) console.log("   ", x.slice(0, 220));
    const ext = (a: any, s: number, v: string) => { const st: any = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: s } } as any); const e = extentOf(st.shapes[v]); return e ? [e.minX, e.maxX, e.minY, e.maxY].map((n) => +n.toFixed(2)) : null; };
    for (const s of authoredScales(doc.asset)) for (const v of ["elevation", "front", "plan"]) { const a = ext(before, s, v), b = ext(doc.asset, s, v); if (JSON.stringify(a) !== JSON.stringify(b)) console.log(`   extent change 1:${s} ${v}: ${JSON.stringify(a)} -> ${JSON.stringify(b)}`); }
    if (write) { writeFileSync(file, JSON.stringify(doc, null, 1) + "\n"); console.log("written", id); }
  }
}
