/** 🔬️ Names the stage of the unchanged oracle that throws on a dumped view: the per-group ring xor, the 16-way union of a level (with level and chunk), or the final overlay. Copies the union of the oracle as it stood before the fix.
 *  bun stages.ts <dump.json> */
import polygonClipping from "polygon-clipping";
type P = { x: number; y: number };
const area = (ring: readonly P[]): number => { let s = 0; for (let i = 0; i < ring.length; i++) { const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!]; s += a.x * b.y - b.x * a.y; } return s / 2; };
const snapped = (ring: readonly P[], unit: number): [number, number][] => ring.map((p) => [Math.round(p.x / unit) * unit, Math.round(p.y / unit) * unit] as [number, number]).filter((p, i, all) => i === 0 || p[0] !== all[i - 1]![0] || p[1] !== all[i - 1]![1]);
function union(name: string, groups: readonly (readonly P[][])[], unit: number): any {
  let level: any[] = [];
  groups.forEach((g, k) => {
    const rings = g.map((ring) => snapped(ring, unit)).filter((ring) => ring.length >= 3 && Math.abs(area(ring.map(([x, y]) => ({ x, y })))) > 4 * unit * unit).map((ring) => [...ring, ring[0]!]);
    if (rings.length === 0) return;
    try { level.push(rings.length === 1 ? [rings[0]!] : polygonClipping.xor([rings[0]!], ...rings.slice(1).map((r) => [r]) as any)); }
    catch (e) { console.log(`  ${name} unit ${unit}: xor of group ${k} (${rings.length} rings) throws`); throw e; }
  });
  let depth = 0;
  while (level.length > 1) {
    const next: any[] = [];
    for (let i = 0; i < level.length; i += 16) {
      try { next.push(polygonClipping.union(level[i]!, ...level.slice(i + 1, i + 16))); }
      catch (e) { console.log(`  ${name} unit ${unit}: union level ${depth} chunk ${i / 16} of ${Math.ceil(level.length / 16)} throws: ${String(e).slice(0, 90)}`); throw e; }
    }
    level = next; depth++;
  }
  return level[0] ?? [];
}
for (const v of await Bun.file(process.argv[2]!).json()) {
  console.log(`${v.id} 1:${v.scale} ${v.view}`);
  for (const unit of [1e-5, 1e-4, 5e-4]) {
    try {
      const [filled, ink, cut] = [union("filled", v.filled, unit), union("outlines", v.outlines, unit), union("reference", v.reference, unit)];
      try { polygonClipping.intersection(polygonClipping.union(filled, ink), cut); polygonClipping.intersection(filled, cut); console.log(`  unit ${unit}: passes`); }
      catch (e) { console.log(`  unit ${unit}: final overlay throws: ${String(e).slice(0, 90)}`); }
    } catch {}
  }
}
