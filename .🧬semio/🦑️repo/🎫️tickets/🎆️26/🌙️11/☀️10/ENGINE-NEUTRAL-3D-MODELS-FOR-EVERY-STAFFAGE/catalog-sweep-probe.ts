/** 🔬️ Decides the missing `sweep` of every extruded hint of an asset from the geometry: for each hint the outline under `w` and under `v` is compared with the silhouette of the authored `volume` (an independent 3D reference) in the same view, and with the asset's `dimensions`.
 * Usage: bun catalog-sweep-probe.ts <slug>... [--scale 50] [--json <file>] -> per hint: share of the outline inside the volume silhouette and outline bounds for each candidate, the verdict and the confidence. */
import { writeFileSync } from "node:fs";
import { VIEWS, loadAsset, resolveAsset, restState, type Json, type P2, type View } from "./volume-lib.ts";

const args = process.argv.slice(2);
const scaleAt = args.indexOf("--scale");
const denominator = scaleAt >= 0 ? Number(args[scaleAt + 1]) : 50;
const jsonAt = args.indexOf("--json");
const jsonOut = jsonAt >= 0 ? args[jsonAt + 1] : undefined;
const slugs = args.filter((a, i) => !a.startsWith("--") && args[i - 1] !== "--scale" && args[i - 1] !== "--json");

function withSweep(asset: Json, sweep: "v" | "w"): Json {
  const clone = structuredClone(asset);
  const walk = (x: Json): void => {
    if (Array.isArray(x)) x.forEach(walk);
    else if (x !== null && typeof x === "object") {
      if (x.kind === "extruded" && x.sweep === undefined) x.sweep = sweep;
      Object.values(x).forEach(walk);
    }
  };
  walk(clone.representations);
  return clone;
}

function withoutSolid(asset: Json): Json {
  const clone = structuredClone(asset);
  const walk = (x: Json): void => {
    if (Array.isArray(x)) x.forEach(walk);
    else if (x !== null && typeof x === "object") {
      if (x.solid?.kind === "extruded" && x.solid.sweep === undefined) delete x.solid;
      Object.values(x).forEach(walk);
    }
  };
  walk(clone.representations);
  return clone;
}

type Row = { slug: string; view: View; id: string; bone: string; errorM: { w: number | null; v: number | null }; areaM2: { w: number; v: number }; reference: string; verdict: "w" | "v" | "unclear"; note: string };
const rows: Row[] = [];

const rangeOf = (rings: readonly (readonly P2[])[], axis: "x" | "y"): [number, number] | null => {
  const values = rings.flat().map((p) => p[axis]);
  return values.length === 0 ? null : [Math.min(...values), Math.max(...values)];
};
const areaOf = (rings: readonly (readonly P2[])[]): number => rings.reduce((sum, r) => { let a = 0; for (let i = 0; i < r.length; i++) { const p = r[i]!; const q = r[(i + 1) % r.length]!; a += p.x * q.y - q.x * p.y; } return sum + Math.abs(a) / 2; }, 0);

for (const spec of slugs) {
  const entry = resolveAsset(spec);
  const asset = loadAsset(entry.path);
  const state = { w: restState(withSweep(asset, "w"), denominator), v: restState(withSweep(asset, "v"), denominator) };
  const flat = restState(withoutSolid(asset), denominator);
  for (const view of VIEWS) {
    for (const s of asset.representations[String(denominator)].geometry[view]) {
      if (s.solid?.kind !== "extruded" || s.solid.sweep !== undefined) continue;
      const pick = (st: ReturnType<typeof restState>, v: View, match: (e: { shape: Json }) => boolean) => st[v].filter(match);
      const own = { w: pick(state.w, view, (e) => e.shape.id === s.id)[0], v: pick(state.v, view, (e) => e.shape.id === s.id)[0] };
      const wr = own.w?.rings ?? [];
      const vr = own.v?.rings ?? [];
      const axis = view === "front" ? "y" : view === "plan" ? "x" : null;
      let reference = "none";
      let twin = pick(state.w, "elevation", (e) => e.shape.id === s.id && e.shape.bone === s.bone && e.closed);
      if (twin.length > 0) reference = "elevation twin of the same id";
      else if (view !== "elevation") {
        twin = pick(state.w, "elevation", (e) => e.shape.bone === s.bone && e.closed && e.shape.role !== "ground");
        if (twin.length > 0) reference = "elevation shapes of the same bone";
      }
      const elevAxis = view === "front" ? "y" : "x";
      const refRange = twin.length > 0 && axis !== null ? rangeOf(twin.flatMap((t) => t.rings), elevAxis) : null;
      const err = (rings: readonly (readonly P2[])[]): number | null => {
        const r = axis === null ? null : rangeOf(rings, axis);
        if (r === null || refRange === null) return null;
        return reference === "elevation twin of the same id" ? Math.abs(r[0] - refRange[0]) + Math.abs(r[1] - refRange[1]) : Math.max(0, refRange[0] - r[0]) + Math.max(0, r[1] - refRange[1]);
      };
      const e: { w: number | null; v: number | null } = { w: err(wr), v: err(vr) };
      const area = { w: areaOf(wr), v: areaOf(vr) };
      let verdict: Row["verdict"] = "w";
      let note = "";
      if (view === "elevation") {
        const plain = pick(flat, "elevation", (e) => e.shape.id === s.id)[0]?.rings ?? [];
        const box = (r: readonly (readonly P2[])[]) => { const x = rangeOf(r, "x"); const y = rangeOf(r, "y"); return x && y ? [x[0], x[1], y[0], y[1]] : null; };
        const dist = (a: number[] | null, b: number[] | null): number => (a && b ? a.reduce((t, v, i) => t + Math.abs(v - b[i]!), 0) : Infinity);
        const dw = dist(box(wr), box(plain));
        const dv = dist(box(vr), box(plain));
        e.w = Number.isFinite(dw) ? dw : null;
        e.v = Number.isFinite(dv) ? dv : null;
        reference = "the shape's own flat elevation outline (solid removed)";
        if (dw < 0.004 && dv < 0.004) note = "flat outline equal under both: w by the convention (profile in the bone u,v plane)";
        else if (!Number.isFinite(dw) && Number.isFinite(dv) && dv > 0.02) note = "line stroke: a zero-area outline under w (the line itself, seen edge-on), a displaced sliver under v: w";
        else verdict = dw <= dv ? "w" : "v";
      } else if (e.w !== null && e.v !== null) {
        if (Math.abs(e.w - e.v) < 0.004) note = "no discriminating evidence from the twin (both within 4 mm): w by the convention";
        else verdict = e.w < e.v ? "w" : "v";
      } else note = "no elevation reference: w by the convention";
      rows.push({ slug: entry.slug, view, id: s.id, bone: s.bone, errorM: { w: e.w === null ? null : Number(e.w.toFixed(4)), v: e.v === null ? null : Number(e.v.toFixed(4)) }, areaM2: { w: Number(area.w.toFixed(5)), v: Number(area.v.toFixed(5)) }, reference, verdict, note });
    }
  }
}
const tally = new Map<string, number>();
for (const r of rows) tally.set(`${r.slug} ${r.verdict}${r.note.startsWith("no ") ? " (weak)" : ""}`, (tally.get(`${r.slug} ${r.verdict}${r.note.startsWith("no ") ? " (weak)" : ""}`) ?? 0) + 1);
console.log([...tally].sort().map(([k, v]) => `${k}: ${v}`).join("\n"));
for (const r of rows.filter((r) => r.verdict !== "w")) console.log(JSON.stringify(r));
if (jsonOut) writeFileSync(jsonOut, JSON.stringify(rows, null, 1));
