/** 🔎️ Lists slender candidates per asset and scale: ratio >= 20 at any scale, unflagged, or thicker at a coarser scale than at 1:50 (author 65). Usage: bun author-65-scan.ts [category/slug ...] */
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { SCALES, SOURCES, entryOf, isAuthored, loadSource, meshesAt, merged, resolveList, type Json } from "./volumes-lib.ts";

type Ext = { e: [number, number, number]; axis: [number, number, number]; c: [number, number, number] };
function extents(m: { world: Float64Array }): Ext {
  const n = m.world.length / 3;
  if (n === 0) return { e: [0, 0, 0], axis: [1, 0, 0], c: [0, 0, 0] };
  const mean = [0, 0, 0];
  for (let i = 0; i < n; i++) for (let k = 0; k < 3; k++) mean[k]! += m.world[i * 3 + k]! / n;
  const c = [0, 0, 0, 0, 0, 0];
  for (let i = 0; i < n; i++) {
    const d = [0, 1, 2].map((k) => m.world[i * 3 + k]! - mean[k]!);
    c[0]! += d[0]! * d[0]!; c[1]! += d[0]! * d[1]!; c[2]! += d[0]! * d[2]!; c[3]! += d[1]! * d[1]!; c[4]! += d[1]! * d[2]!; c[5]! += d[2]! * d[2]!;
  }
  const a = [[c[0]!, c[1]!, c[2]!], [c[1]!, c[3]!, c[4]!], [c[2]!, c[4]!, c[5]!]];
  const v = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
  for (let sweep = 0; sweep < 30; sweep++) for (const [p, q] of [[0, 1], [0, 2], [1, 2]] as const) {
    if (Math.abs(a[p]![q]!) < 1e-14) continue;
    const theta = (a[q]![q]! - a[p]![p]!) / (2 * a[p]![q]!);
    const t = Math.sign(theta || 1) / (Math.abs(theta) + Math.sqrt(theta * theta + 1));
    const cs = 1 / Math.sqrt(t * t + 1); const sn = t * cs;
    for (let k = 0; k < 3; k++) { const x = a[k]![p]!, y = a[k]![q]!; a[k]![p] = cs * x - sn * y; a[k]![q] = sn * x + cs * y; }
    for (let k = 0; k < 3; k++) { const x = a[p]![k]!, y = a[q]![k]!; a[p]![k] = cs * x - sn * y; a[q]![k] = sn * x + cs * y; }
    for (let k = 0; k < 3; k++) { const x = v[k]![p]!, y = v[k]![q]!; v[k]![p] = cs * x - sn * y; v[k]![q] = sn * x + cs * y; }
  }
  const ex: { x: number; ax: number }[] = [];
  for (let axis = 0; axis < 3; axis++) {
    let lo = Infinity, hi = -Infinity;
    for (let i = 0; i < n; i++) { const d = m.world[i * 3]! * v[0]![axis]! + m.world[i * 3 + 1]! * v[1]![axis]! + m.world[i * 3 + 2]! * v[2]![axis]!; if (d < lo) lo = d; if (d > hi) hi = d; }
    ex.push({ x: hi - lo, ax: axis });
  }
  ex.sort((p, q) => p.x - q.x);
  const la = ex[2]!.ax;
  return { e: [ex[0]!.x, ex[1]!.x, ex[2]!.x], axis: [v[0]![la]!, v[1]![la]!, v[2]![la]!], c: [mean[0]!, mean[1]!, mean[2]!] };
}

type Rec = { list: string; id: string; kind: string; len: number; cross: number; min: number; axis: [number, number, number]; c: [number, number, number]; slender: boolean; ref: boolean; repeat: number };
const specs = process.argv.slice(2);
const all: string[] = [];
if (specs.length === 0) {
  for (const cat of readdirSync(SOURCES)) for (const f of readdirSync(join(SOURCES, cat))) if (f.endsWith(".json")) all.push(`${cat}/${f.slice(0, -5)}`);
} else all.push(...specs);
const dot = (a: number[], b: number[]) => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;
const out: Record<string, unknown[]> = {};
for (const spec of all) {
  try {
    const { entry, asset } = entryOf(spec);
    const source = loadSource(entry);
    if (source === null) continue;
    const m = merged(asset, source);
    const per: Record<number, Rec[]> = {};
    for (const scale of SCALES) {
      const vol = source.volumes?.[String(scale)];
      if (!isAuthored(vol)) continue;
      per[scale] = [];
      for (const list of ["body", "thermal", "structure"] as const) {
        const raw: Json[] = vol[list] ?? [];
        for (const s of resolveList(vol, list, [])) {
          const { repeat: _r, ...single } = s;
          let mesh;
          try { [mesh] = meshesAt(m, scale, [single]); } catch { continue; }
          if (mesh === undefined) continue;
          const x = extents(mesh);
          const rw = raw.find((q) => q.id === s.id);
          per[scale]!.push({ list, id: s.id, kind: s.kind, len: x.e[2], cross: x.e[1], min: x.e[0], axis: x.axis, c: x.c, slender: s.slender === true, ref: rw?.of !== undefined, repeat: s.repeat?.count ?? 1 });
        }
      }
    }
    const rows: unknown[] = [];
    const fine = per[50] ?? [];
    for (const scale of SCALES) {
      for (const r of per[scale] ?? []) {
        if (r.ref) continue;
        const ratio = r.len / Math.max(r.cross, 1e-9);
        const sameId = fine.find((f) => f.list === r.list && f.id === r.id);
        let match: Rec | undefined;
        for (const f of fine) {
          if (f.list !== r.list || f.len / Math.max(f.cross, 1e-9) < 20) continue;
          if (Math.abs(dot(f.axis, r.axis)) < 0.96) continue;
          const d = [r.c[0] - f.c[0], r.c[1] - f.c[1], r.c[2] - f.c[2]];
          const along = dot(d, f.axis);
          const perp = Math.sqrt(Math.max(0, dot(d, d) - along * along));
          if (perp > Math.max(0.6 * r.cross, 0.03)) continue;
          if (r.len / f.len < 0.7 || r.len / f.len > 1.4) continue;
          if (!match || f.id === r.id) match = f;
        }
        const thick = scale !== 50 && match !== undefined && (r.cross > 1.25 * match.cross || r.cross < 0.8 * match.cross) && Math.abs(r.cross - match.cross) >= 0.003;
        const unflagged = ratio >= 20 && !r.slender;
        const falseFlag = r.slender && ratio < 20;
        if (thick || unflagged || falseFlag) rows.push({ scale, list: r.list, id: r.id, kind: r.kind, lenMm: +(r.len * 1000).toFixed(1), crossMm: +(r.cross * 1000).toFixed(1), minMm: +(r.min * 1000).toFixed(1), ratio: +ratio.toFixed(1), slender: r.slender, repeat: r.repeat, match: match ? { id: match.id, crossMm: +(match.cross * 1000).toFixed(1), minMm: +(match.min * 1000).toFixed(1), lenMm: +(match.len * 1000).toFixed(1) } : null, sameIdAt50: sameId ? { crossMm: +(sameId.cross * 1000).toFixed(1) } : null, thick, unflagged, falseFlag });
      }
    }
    if (rows.length > 0) out[spec] = rows;
  } catch (e) {
    out[spec] = [{ error: (e as Error).message.slice(0, 160) }];
  }
}
process.stdout.write(JSON.stringify(out));
