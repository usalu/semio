type Multi = polygonClipping.MultiPolygon;
type Overlay = (a: Multi, b: Multi) => Multi;
const SNAPS = [1e-5, 1e-4, 5e-4];
const gridded = (ring: readonly Point[], unit: number): [number, number][] => ring.map((p) => [Math.round(p.x / unit), Math.round(p.y / unit)] as [number, number]).filter((p, i, all) => i === 0 || p[0] !== all[i - 1]![0] || p[1] !== all[i - 1]![1]);
const closed = (rings: readonly [number, number][][], least: number): polygonClipping.Ring[] => rings.filter((ring) => ring.length >= 3 && Math.abs(area(ring.map(([x, y]) => ({ x, y })))) > least).map((ring) => [...ring, ring[0]!]);
const total = (multi: Multi): number => multi.reduce((sum, poly) => sum + poly.reduce((a, ring, i) => a + (i === 0 ? 1 : -1) * Math.abs(area(ring.map(([x, y]) => ({ x, y })))), 0), 0);
const regridded = (multi: Multi): Multi => multi.map((poly) => closed(poly.map((ring) => gridded(ring.slice(0, -1).map(([x, y]) => ({ x, y })), 1)), 0)).filter((poly) => poly.length > 0);

/** 🔁️ One overlay of two regions by polygon-clipping; where its sweep line fails on the operands as they are (the vertices its earlier overlays computed lie off the grid), once more with both operands rounded back to the grid. */
function overlay(operation: Overlay, a: Multi, b: Multi): Multi {
  try {
    return operation(a, b);
  } catch {
    return operation(regridded(a), regridded(b));
  }
}

/** 🌲️ The union of regions by polygon-clipping in one sweep; where the sweep fails, the union of the unions of its two halves, down to single pairs. */
function merged(regions: readonly Multi[]): Multi {
  if (regions.length < 2) return regions[0] ?? [];
  if (regions.length === 2) return overlay(polygonClipping.union, regions[0]!, regions[1]!);
  try {
    return polygonClipping.union(regions[0]!, ...regions.slice(1));
  } catch {
    const half = regions.length >> 1;
    return merged([merged(regions.slice(0, half)), merged(regions.slice(half))]);
  }
}

function union(groups: readonly Group[], unit: number): Multi {
  let level: Multi[] = [];
  for (const g of groups) {
    const rings = closed(g.map((ring) => gridded(ring, unit)), 4);
    if (rings.length > 0) level.push(rings.length === 1 ? [[rings[0]!]] : polygonClipping.xor([rings[0]!], ...rings.slice(1).map((r) => [r] as polygonClipping.Polygon)));
  }
  while (level.length > 1) {
    const next: Multi[] = [];
    for (let i = 0; i < level.length; i += 16) next.push(merged(level.slice(i, i + 16)));
    level = next;
  }
  return level[0] ?? [];
}

/** 📐️ The exact agreement by polygon-clipping on the integer grid of 10 micrometres (and, where an overlay fails even pair by pair, of 0.1 and 0.5 millimetres): the faces are united sixteen at a time, a failing sweep is split into halves and a failing pair is rounded back to the grid, so the areas are those of the library throughout. */
export function exactIou(m: Measured): number {
  let failure: unknown;
  for (const unit of SNAPS) {
    try {
      const [filled, ink, cut] = [union(m.filled, unit), union(m.outlines, unit), union(m.reference, unit)];
      const [visible, full] = m.section ? [union(m.visible, unit), union(m.full, unit)] : [cut, cut];
      const drawn = merged([filled, ink]);
      const inside = merged([overlay(polygonClipping.intersection, filled, visible), overlay(polygonClipping.intersection, ink, full)]);
      const either = total(drawn) + total(cut) - total(overlay(polygonClipping.intersection, drawn, cut));
      return either === 0 ? 1 : total(inside) / either;
    } catch (error) {
      failure = error;
    }
  }
  throw failure;
}
