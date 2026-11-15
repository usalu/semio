/** 🌾 Author 47: miscanthus-grass body per scale, each blade, culm and plume a visual-hull loft of that scale's own drawn polygons. Usage: `bun author-47-grass.ts --scale N [--write]`. */
import { openCtx, saveLists, flag, has, drawn, hullLoft, capsuleOf, extrudeOf, type Json } from "./kit-lib.ts";
import { pieceHull, polyOf, clipHalf, simplifyRing, spanOf, inUnion, type Poly } from "./author-47-lib.ts";

const args = process.argv.slice(2);
const scale = Number(flag(args, "--scale"));
const c = openCtx("vegetation/miscanthus-grass", scale);
const BLADE = "miscanthus-blade";
const body: Json[] = [];
const add = (s: Json | null, why: string): void => {
  if (s === null) process.stderr.write(`[grass] skipped ${why}\n`);
  else body.push(s);
};
const P = (v: "elevation" | "front" | "plan", id: string): Poly | null => polyOf(c, v, id);
const some = <T>(xs: (T | null)[]): T[] => xs.filter((x): x is T => x !== null);
const W50 = Number(flag(args, "--wscale", "1"));
const SAMEB = has(args, "--sameb");
const TK = Number(flag(args, "--tk", "1.4"));
const sides = ["l1", "l2", "l3", "r1", "r2", "r3"];

function tuft(): void {
  const elev = some([P("elevation", "tuft")]);
  const front = some([P("front", "tuft")]);
  add(pieceHull(c, { id: "tuft", bone: "root-plate", material: "miscanthus-culm-mat", tone: 0.3, axis: 1, polys: { elevation: elev, front }, n: 8, e: 2.4 }), "tuft");
}

function culms(ids: (n: number, s: string) => { elev: string; front: string }): void {
  for (const n of [1, 2, 3, 4]) for (const s of ["a", "b"]) {
    const k = ids(n, s);
    add(pieceHull(c, { id: `culm-${n}${s}`, bone: `culm-${n}${s}`, material: "miscanthus-culm-mat", tone: 0.4, axis: 1, polys: { elevation: some([P("elevation", k.elev)]), front: some([P("front", k.front)]) }, n: 6, e: 2, minR: 0.5 * c.feature, slender: scale >= 200 }), `culm-${n}${s}`);
  }
}

function plumes(): void {
  for (const n of [1, 2, 3, 4]) add(pieceHull(c, { id: `plume-${n}`, bone: `plume-${n}`, material: "miscanthus-plume", tone: 0.85, axis: 1, polys: { elevation: some([P("elevation", `plume-${n}`)]), front: some([P("front", `plume-${n}`)]) }, n: 10, e: 2.2 }), `plume-${n}`);
}

import { extentOf } from "./author-47-lib.ts";

function ribbon(id: string, bone: string, elev: Poly[], plan: Poly[], front: Poly[], wscale = 1, n = 24): void {
  const [x0, x1] = extentOf(elev, "x");
  const [y0, y1] = extentOf(elev, "y");
  const [z0, z1] = extentOf(plan, "y");
  const steep = y1 - y0 > 1.2 * Math.hypot(x1 - x0, z1 - z0);
  add(pieceHull(c, { id, bone, material: BLADE, tone: 0.55, axis: steep ? 1 : 0, polys: { elevation: elev, plan, front }, n, e: 3, ribbon: true, wscale, tk: TK }), id);
}

function blades50(): void {
  for (const b of sides) {
    const ks = b.startsWith("l") ? ["1", "2", "4"] : ["1", "3", "4"];
    for (const k of ks) for (const s of ["a", "b"]) {
      const id = `blade-${b}-${k}${s}`;
      const bone = `bl-${b}${s}`;
      const elev = some([P("elevation", id)]);
      if (k === "1") {
        ribbon(id, bone, elev, some([P("plan", id)]), some([P("front", id)]));
        continue;
      }
      for (const [j, sign] of [[1, -1], [2, 1]] as const) {
        const plan = P("plan", `${id}-${j}`) ?? clipHalf(P("plan", id)!, "y", sign);
        const front = P("front", `${id}-${j}`) ?? clipHalf(P("front", id)!, "x", sign);
        ribbon(`${id}-${j}`, bone, elev, [plan], [front], j === 1 ? W50 : 1);
      }
    }
  }
}

function blades100(two: boolean): void {
  for (const b of sides) for (const sg of ["a", "b"]) {
    const bone = `bl-${b}${sg}`;
    const plan = some([P("plan", `${b}-1-${sg}`)]);
    const front = some([P("front", `${b}-1-${sg}`)]);
    for (const h of two ? [0, 1] : [0]) ribbon(`bl-${b}${sg}-${h}`, bone, some([P("elevation", `bl-${b}${sg}-${h}`)]), plan, front, 1, 24);
    for (const k of two ? ["2", "3"] : ["2"]) {
      const pl = some([P("plan", `${b}-${k}-${sg}`)]);
      const fr = some([P("front", `${b}-${k}-${sg}`)]);
      if (pl.length === 0 || fr.length === 0) continue;
      add(pieceHull(c, { id: `z-${b}-${k}${sg}`, bone, material: BLADE, tone: 0.55, axis: 2, polys: { plan: pl, front: fr }, n: 24, e: 3, ribbon: true, tk: TK }), `z-${b}-${k}${sg}`);
    }
  }
}

function maskPiecesLens(): void {
  const plan = drawn(c, "plan").filter((q) => !/^tuft|^culm|^plume/.test(q.id)).flatMap((q) => q.rings);
  const lenses = drawn(c, "elevation").filter((q) => /^bl-/.test(q.bone) && q.closed !== false);
  let count = 0;
  for (const fr of drawn(c, "front")) {
    if (/^tuft|^culm|^plume/.test(fr.id)) continue;
    const F = fr.rings;
    const [z0, z1] = extentOf(F, "x");
    const [y0, y1] = extentOf(F, "y");
    const alongZ = z1 - z0 > 1.2 * (y1 - y0);
    const side = fr.bone.slice(0, 4);
    for (const L of lenses) {
      if (!L.bone.startsWith(side) || (SAMEB && L.bone.slice(-1) !== fr.bone.slice(-1))) continue;
      const lens = L.rings;
      const [lx0, lx1] = extentOf(lens, "x");
      const scanX = (z: number, ya: number, yb: number): [number, number] | null => {
        let lo = Infinity;
        let hi = -Infinity;
        for (let x = lx0; x <= lx1; x += 0.01) {
          if (!inUnion(plan, x, z)) continue;
          if (inUnion(lens, x, ya) || inUnion(lens, x, (ya + yb) / 2) || inUnion(lens, x, yb)) {
            lo = Math.min(lo, x);
            hi = Math.max(hi, x);
          }
        }
        return Number.isFinite(lo) ? [lo - 0.005, hi + 0.005] : null;
      };
      const custom = alongZ
        ? { lo: z0, hi: z1, at: (s: number) => { const iy = spanOf(F, "x", s); if (iy === null) return null; const ix = scanX(s, iy[0], iy[1]); return ix === null ? null : { ib: ix, ic: iy }; } }
        : { lo: y0, hi: y1, at: (s: number) => { const iz = spanOf(F, "y", s); if (iz === null) return null; const ix = scanX((iz[0] + iz[1]) / 2, s, s); return ix === null ? null : { ib: ix, ic: iz }; } };
      const piece = pieceHull(c, { id: `mass-${fr.id}-${L.id}`, bone: fr.bone, material: BLADE, tone: 0.55, axis: alongZ ? 2 : 1, polys: {}, custom, n: alongZ ? 12 : 10, e: 2.6 });
      if (piece !== null && (piece.sections as unknown[]).length >= 3) {
        body.push(piece);
        count++;
      }
    }
  }
  process.stderr.write(`[grass] mask pieces ${count}\n`);
}

function maskPieces(): void {
  const plan = drawn(c, "plan").filter((q) => !/^tuft|^culm|^plume/.test(q.id)).flatMap((q) => q.rings);
  for (const fr of drawn(c, "front")) {
    if (/^tuft|^culm|^plume/.test(fr.id)) continue;
    const F = fr.rings;
    const [z0, z1] = extentOf(F, "x");
    const [y0, y1] = extentOf(F, "y");
    const alongZ = z1 - z0 > 1.2 * (y1 - y0);
    const same = (b: string): boolean => b.startsWith(fr.bone.slice(0, 4)) && b.endsWith(fr.bone.slice(-1));
    const mine = drawn(c, "elevation").filter((q) => same(q.bone)).flatMap((q) => q.rings);
    const [ex0, ex1] = extentOf(mine, "x");
    const sideEl = drawn(c, "elevation").filter((q) => q.bone.startsWith(fr.bone.slice(0, 4))).flatMap((q) => q.rings);
    const scanX = (z: number, ya: number, yb: number): [number, number] | null => {
      let lo = Infinity;
      let hi = -Infinity;
      for (let x = ex0 - 0.01; x <= ex1 + 0.01; x += 0.01) {
        if (!inUnion(plan, x, z)) continue;
        if (inUnion(sideEl, x, ya) || inUnion(sideEl, x, (ya + yb) / 2) || inUnion(sideEl, x, yb)) {
          lo = Math.min(lo, x);
          hi = Math.max(hi, x);
        }
      }
      if (!Number.isFinite(lo)) for (let x = ex0 - 0.01; x <= ex1 + 0.01; x += 0.01) if (inUnion(plan, x, z)) [lo, hi] = [Math.min(lo, x), Math.max(hi, x)];
      return Number.isFinite(lo) ? [lo - 0.005, hi + 0.005] : null;
    };
    const custom = alongZ
      ? { lo: z0, hi: z1, at: (s: number) => { const iy = spanOf(F, "x", s); if (iy === null) return null; const ix = scanX(s, iy[0], iy[1]); return ix === null ? null : { ib: ix, ic: iy }; } }
      : { lo: y0, hi: y1, at: (s: number) => { const iz = spanOf(F, "y", s); if (iz === null) return null; const ix = scanX((iz[0] + iz[1]) / 2, s, s); return ix === null ? null : { ib: ix, ic: iz }; } };
    add(pieceHull(c, { id: `mass-${fr.id}`, bone: fr.bone, material: BLADE, tone: 0.55, axis: alongZ ? 2 : 1, polys: {}, custom, n: scale === 200 ? (alongZ ? 8 : 6) : alongZ ? 18 : 14, e: 2.6 }), `mass-${fr.id}`);
  }
}

function culmStrokes(): void {
  for (const n of [1, 2, 3, 4]) {
    const st = (c.state.elevation as Json[]).find((q) => q.shape.id === `culm-${n}a-axis`);
    if (st === undefined) continue;
    const pts = st.rings[0] as { x: number; y: number }[];
    const f = c.state.frames[`culm-${n}a`]!;
    const loc = (p: { x: number; y: number }): [number, number, number] => {
      const d = [p.x - f.o[0]!, p.y - f.o[1]!, 0 - f.o[2]!];
      return [d[0]! * f.u[0]! + d[1]! * f.u[1]! + d[2]! * f.u[2]!, d[0]! * f.v[0]! + d[1]! * f.v[1]! + d[2]! * f.v[2]!, d[0]! * f.w[0]! + d[1]! * f.w[1]! + d[2]! * f.w[2]!];
    };
    const k = Math.max(2, Math.round(pts.length / 5));
    const keep = pts.filter((_, i) => i % k === 0 || i === pts.length - 1);
    const r = 0.55 * c.feature;
    for (let i = 0; i + 1 < keep.length; i++) add(capsuleOf({ id: `culm-${n}a-${i}`, bone: `culm-${n}a`, material: "miscanthus-culm-mat", tone: 0.4 }, loc(keep[i]!), loc(keep[i + 1]!), r), `culm ${n} ${i}`);
  }
}

function hull500(): void {
  const h = Number(flag(args, "--skirt", "0.4"));
  const sp = Number(flag(args, "--spire", "1.15"));
  const plan = P("plan", "tuft")!;
  const ring = simplifyRing(plan, 0.6 * c.feature);
  add(extrudeOf({ id: "tuft-skirt", bone: "root-plate", material: "miscanthus-blade", tone: 0.5 }, "yz", ring.map((p) => [-p.x, p.y] as [number, number]), Number(flag(args, "--base", "0.06")), h), "skirt");
  const polys = { elevation: some([P("elevation", "tuft")]), front: some([P("front", "tuft")]) };
  const common = { bone: "root-plate", axis: 1 as const, polys, n: 28, e: Number(flag(args, "--e", "2")), minR: 0.55 * c.feature };
  add(pieceHull(c, { ...common, id: "tuft", material: "miscanthus-blade", tone: 0.5, range: [h - 0.02, sp] }), "tuft500");
  add(pieceHull(c, { ...common, id: "spire", material: "miscanthus-plume", tone: 0.8, n: 12, range: [sp - 0.01, 5] }), "spire500");
}

const per: Record<number, () => void> = {
  100: () => {
    tuft();
    culms((n, s) => ({ elev: `culm-${n}${s}-axis`, front: `culm-${n}${s}` }));
    plumes();
    maskPieces();
  },
  200: () => {
    tuft();
    culmStrokes();
    plumes();
    maskPieces();
  },
  500: () => hull500(),
  50: () => {
    tuft();
    culms((n, s) => ({ elev: `culm-${n}${s}`, front: `culm-${n}${s}` }));
    plumes();
    blades50();
  },
};
per[scale]!();
process.stderr.write(`[grass] 1:${scale} body ${body.length} solids\n`);
if (has(args, "--map")) {
  const g = (re: RegExp): string => body.filter((b) => re.test(b.id)).map((b) => b.id).join("+");
  process.stdout.write([`blades=${g(/^blade|^bl-|^l\d|^r\d/)}`, `plumes=${g(/^plume/)}`, `culms=${g(/^culm|^tuft/)}`].join(",") + "\n");
} else if (has(args, "--write")) saveLists(c, { body });
else process.stdout.write(JSON.stringify(body.map((s) => s.id)) + "\n");
void drawn;
void hullLoft;
void maskPiecesLens;
