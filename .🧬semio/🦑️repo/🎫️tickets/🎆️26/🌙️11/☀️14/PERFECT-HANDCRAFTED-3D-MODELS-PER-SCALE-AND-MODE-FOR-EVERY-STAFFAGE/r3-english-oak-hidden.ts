/** 🫥️ Round 3 English oak: for every foliage lobe of a scale, how far the crown outline in elevation, front and plan moves when that lobe is left out (largest outward distance in metres per view), so that a coarser scale can drop exactly the lobes that never shape an outline. Usage: bun r3-english-oak-hidden.ts [scale] [cellM] [greedyM] (greedyM: leave out the boss (and with `tufts` as fifth argument also the tufts) that moves the outline least, measure again, and repeat while that least move stays under greedyM; prints the bosses left out in that order) */
import { treeOf, type Lobe } from "./r3-english-oak-tree.ts";

const scale = Number(process.argv[2] ?? 50);
const cell = Number(process.argv[3] ?? 0.03);
const all = treeOf(scale).lobes;
const greedy = process.argv[4] === undefined ? undefined : Number(process.argv[4]);
let lobes = all;
const VIEWS = { elevation: [0, 1], front: [2, 1], plan: [0, 2] } as const;
type Grid = { w: number; h: number; x0: number; y0: number; cover: Uint16Array };

function grid(view: keyof typeof VIEWS): Grid {
  const [a, b] = VIEWS[view];
  const x0 = Math.min(...lobes.map((l) => l.c[a] - l.r[a])) - 1;
  const y0 = Math.min(...lobes.map((l) => l.c[b] - l.r[b])) - 1;
  const w = Math.ceil((Math.max(...lobes.map((l) => l.c[a] + l.r[a])) + 1 - x0) / cell);
  const h = Math.ceil((Math.max(...lobes.map((l) => l.c[b] + l.r[b])) + 1 - y0) / cell);
  return { w, h, x0, y0, cover: new Uint16Array(w * h) };
}
function cells(g: Grid, view: keyof typeof VIEWS, l: Lobe): number[] {
  const [a, b] = VIEWS[view];
  const out: number[] = [];
  const [i0, i1] = [Math.floor((l.c[a] - l.r[a] - g.x0) / cell), Math.ceil((l.c[a] + l.r[a] - g.x0) / cell)];
  const [j0, j1] = [Math.floor((l.c[b] - l.r[b] - g.y0) / cell), Math.ceil((l.c[b] + l.r[b] - g.y0) / cell)];
  for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) {
    const [x, y] = [g.x0 + (i + 0.5) * cell, g.y0 + (j + 0.5) * cell];
    if (((x - l.c[a]) / l.r[a]) ** 2 + ((y - l.c[b]) / l.r[b]) ** 2 <= 1) out.push(j * g.w + i);
  }
  return out;
}
function measure(): Record<string, Record<string, number>> {
  const result: Record<string, Record<string, number>> = {};
  for (const view of Object.keys(VIEWS) as (keyof typeof VIEWS)[]) {
    const g = grid(view);
    const own = lobes.map((l) => cells(g, view, l));
    for (const list of own) for (const k of list) g.cover[k]!++;
    lobes.forEach((l, n) => {
      const mine = new Uint8Array(g.cover.length);
      for (const k of own[n]!) mine[k] = 1;
      const kept = (q: number): boolean => g.cover[q]! - mine[q]! > 0;
      const lost = own[n]!.filter((k) => !kept(k));
      let worst = 0;
      for (const k of lost) {
        const [i, j] = [k % g.w, Math.floor(k / g.w)];
        let best = Infinity;
        for (let r = 1; r < 200 && best === Infinity; r++)
          for (let dj = -r; dj <= r; dj++) for (const di of Math.abs(dj) === r ? Array.from({ length: 2 * r + 1 }, (_, q) => q - r) : [-r, r]) {
            const [x, y] = [i + di, j + dj];
            if (x >= 0 && y >= 0 && x < g.w && y < g.h && kept(y * g.w + x)) best = Math.min(best, Math.hypot(di, dj) * cell);
          }
        worst = Math.max(worst, best === Infinity ? 9 : best);
      }
      (result[l.id] ??= {})[view] = worst;
    });
  }
  return result;
}
if (greedy === undefined) for (const [id, v] of Object.entries(measure())) console.log(id.padEnd(22), ["elevation", "front", "plan"].map((k) => (v[k] ?? 0).toFixed(2)).join("  "), Math.max(...Object.values(v)).toFixed(2));
else
  for (;;) {
    const worst = Object.entries(measure()).filter(([id]) => /-\d+$/.test(id) && (process.argv[5] === "tufts" || !id.includes("tuft"))).map(([id, v]) => [id, Math.max(...Object.values(v))] as const).sort((a, b) => a[1] - b[1]);
    if (worst.length === 0 || worst[0]![1] >= greedy) break;
    console.log(worst[0]![0], worst[0]![1].toFixed(2));
    lobes = lobes.filter((l) => l.id !== worst[0]![0]);
  }
