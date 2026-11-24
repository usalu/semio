/** 🧪️ Tries recovery strategies for the unions of the reference faces that polygon-clipping refuses: halving a refused chunk down to pairs, then per pair the swapped order, the regridded operands and the difference identity a ∪ b = a + (b − a). Prints how often each strategy was needed and what remained refused.
 *  bun strategies.ts <dump.json> [unit] */
import polygonClipping from "polygon-clipping";
type P = { x: number; y: number };
type G = [number, number][][][];
const area = (ring: readonly P[]): number => { let s = 0; for (let i = 0; i < ring.length; i++) { const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!]; s += a.x * b.y - b.x * a.y; } return s / 2; };
const ringArea = (ring: [number, number][]): number => Math.abs(area(ring.map(([x, y]) => ({ x, y }))));
const snapped = (ring: readonly P[], unit: number): [number, number][] => ring.map((p) => [Math.round(p.x / unit) * unit, Math.round(p.y / unit) * unit] as [number, number]).filter((p, i, all) => i === 0 || p[0] !== all[i - 1]![0] || p[1] !== all[i - 1]![1]);
const total = (multi: G): number => multi.reduce((sum, poly) => sum + poly.reduce((a, ring, i) => a + (i === 0 ? 1 : -1) * ringArea(ring), 0), 0);
const count: Record<string, number> = {};
const hit = (k: string): void => { count[k] = (count[k] ?? 0) + 1; };
const regrid = (g: G, unit: number): G => g.map((poly) => poly.map((ring) => snapped(ring.slice(0, -1).map(([x, y]) => ({ x, y })), unit)).filter((ring) => ring.length >= 3 && ringArea(ring) > 0).map((ring) => [...ring, ring[0]!])).filter((poly) => poly.length > 0) as G;
function merge(geoms: G[], unit: number): G {
  if (geoms.length === 1) return geoms[0]!;
  try { return polygonClipping.union(geoms[0]!, ...geoms.slice(1)) as G; } catch (error) {
    if (geoms.length > 2) { hit(`halved ${geoms.length}`); const h = geoms.length >> 1; return merge([merge(geoms.slice(0, h), unit), merge(geoms.slice(h), unit)], unit); }
    const [a, b] = [geoms[0]!, geoms[1]!];
    try { const r = polygonClipping.union(b, a) as G; hit("swapped"); return r; } catch {}
    try { const r = [...a, ...(polygonClipping.difference(b, a) as G)]; hit("difference b-a"); return r; } catch {}
    try { const r = [...b, ...(polygonClipping.difference(a, b) as G)]; hit("difference a-b"); return r; } catch {}
    try { const r = polygonClipping.union(regrid(a, unit), regrid(b, unit)) as G; hit("regrid"); return r; } catch {}
    hit("REFUSED"); throw error;
  }
}
function union(groups: readonly (readonly P[][])[], unit: number): G {
  let level: G[] = [];
  for (const g of groups) {
    const rings = g.map((ring) => snapped(ring, unit)).filter((ring) => ring.length >= 3 && ringArea(ring) > 4 * unit * unit).map((ring) => [...ring, ring[0]!]);
    if (rings.length === 0) continue;
    level.push(rings.length === 1 ? [[rings[0]!]] : polygonClipping.xor([rings[0]!], ...rings.slice(1).map((r) => [r]) as any) as G);
  }
  while (level.length > 1) { const next: G[] = []; for (let i = 0; i < level.length; i += 16) next.push(merge(level.slice(i, i + 16), unit)); level = next; }
  return level[0] ?? [];
}
const unit = Number(process.argv[3] ?? 1e-5);
for (const v of await Bun.file(process.argv[2]!).json()) {
  for (const k of Object.keys(count)) delete count[k];
  const t = performance.now();
  try {
    const [filled, ink, cut] = [union(v.filled, unit), union(v.outlines, unit), union(v.reference, unit)];
    const drawn = polygonClipping.union(filled, ink) as G;
    const either = total(drawn) + total(cut) - total(polygonClipping.intersection(drawn, cut) as G);
    console.log(`${v.id} 1:${v.scale} ${v.view} unit ${unit}: exact ${(total(polygonClipping.intersection(filled, cut) as G) / either).toFixed(9)} owned ${v.iou.toFixed(6)} ${Math.round(performance.now() - t)} ms ${JSON.stringify(count)}`);
  } catch (e) { console.log(`${v.id} 1:${v.scale} ${v.view} unit ${unit}: THROWS ${String(e).slice(0, 80)} ${JSON.stringify(count)}`); }
}
