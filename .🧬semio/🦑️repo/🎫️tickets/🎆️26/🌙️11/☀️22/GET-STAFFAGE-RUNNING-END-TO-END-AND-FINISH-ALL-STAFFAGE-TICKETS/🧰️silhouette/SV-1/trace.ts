/** 🧵️ Raster helpers of the SV-1 repair: connected components, dilation, and the outline of a cell set as simplified rings (outer rings counter-clockwise, holes clockwise). */
export type P = { x: number; y: number };

/** 🏷️ Labels the 4-connected components of the set cells; returns the label per cell (0 = unset) and the count. */
export function label(mask: Uint8Array, width: number, height: number): { labels: Int32Array; count: number } {
  const labels = new Int32Array(mask.length);
  let count = 0;
  const stack: number[] = [];
  for (let s = 0; s < mask.length; s++) {
    if (!mask[s] || labels[s]) continue;
    labels[s] = ++count;
    stack.push(s);
    while (stack.length > 0) {
      const k = stack.pop()!;
      const [i, j] = [k % width, (k / width) | 0];
      if (i > 0 && mask[k - 1] && !labels[k - 1]) { labels[k - 1] = count; stack.push(k - 1); }
      if (i < width - 1 && mask[k + 1] && !labels[k + 1]) { labels[k + 1] = count; stack.push(k + 1); }
      if (j > 0 && mask[k - width] && !labels[k - width]) { labels[k - width] = count; stack.push(k - width); }
      if (j < height - 1 && mask[k + width] && !labels[k + width]) { labels[k + width] = count; stack.push(k + width); }
    }
  }
  return { labels, count };
}

/** 🎈️ Grows the set cells by `r` cells (square neighbourhood). */
export function dilate(mask: Uint8Array, width: number, height: number, r: number): Uint8Array {
  const rows = new Uint8Array(mask.length);
  for (let j = 0; j < height; j++) for (let i = 0; i < width; i++) if (mask[j * width + i]) for (let d = Math.max(0, i - r); d <= Math.min(width - 1, i + r); d++) rows[j * width + d] = 1;
  const out = new Uint8Array(mask.length);
  for (let j = 0; j < height; j++) for (let i = 0; i < width; i++) if (rows[j * width + i]) for (let d = Math.max(0, j - r); d <= Math.min(height - 1, j + r); d++) out[d * width + i] = 1;
  return out;
}

/** 🫧️ Shrinks the set cells by `r` cells (square neighbourhood; outside the grid counts as set). */
export function erode(mask: Uint8Array, width: number, height: number, r: number): Uint8Array {
  return dilate(mask.map((x) => (x ? 0 : 1)), width, height, r).map((x) => (x ? 0 : 1));
}

const cross = (p: P, q: P, r: P): number => (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
const off = (p: P, q: P, r: P): number => Math.abs(cross(q, r, p)) / (Math.hypot(r.x - q.x, r.y - q.y) || 1);
const rdp = (l: P[], t: number): P[] => {
  if (l.length < 3) return l;
  let [mi, md] = [0, 0];
  for (let i = 1; i < l.length - 1; i++) { const d = off(l[i]!, l[0]!, l[l.length - 1]!); if (d > md) { md = d; mi = i; } }
  return md > t ? [...rdp(l.slice(0, mi + 1), t).slice(0, -1), ...rdp(l.slice(mi), t)] : [l[0]!, l[l.length - 1]!];
};

/** ✂️ Simplifies a closed ring by Ramer-Douglas-Peucker between its two extreme points. */
export function simplify(ring: P[], tolerance: number): P[] {
  if (ring.length < 5) return ring;
  let [a, b] = [0, 0];
  for (let i = 0; i < ring.length; i++) { if (ring[i]!.x + ring[i]!.y < ring[a]!.x + ring[a]!.y) a = i; if (ring[i]!.x + ring[i]!.y > ring[b]!.x + ring[b]!.y) b = i; }
  if (a === b) return ring;
  const rot = [...ring.slice(a), ...ring.slice(0, a)];
  const k = (b - a + ring.length) % ring.length;
  const out = [...rdp(rot.slice(0, k + 1), tolerance).slice(0, -1), ...rdp([...rot.slice(k), rot[0]!], tolerance).slice(0, -1)];
  return out.length >= 3 ? out : ring;
}

/** 🖊️ The outline of the set cells as rings in grid coordinates (cell corners): outer rings counter-clockwise, holes clockwise; cells that touch only by a corner stay separate. */
export function outline(mask: Uint8Array, width: number, height: number): P[][] {
  const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= width || j >= height ? 0 : mask[j * width + i]!);
  const V = width + 1;
  const next = new Map<number, number[]>();
  const add = (x0: number, y0: number, x1: number, y1: number): void => { const k = y0 * V + x0; const l = next.get(k); if (l === undefined) next.set(k, [y1 * V + x1]); else l.push(y1 * V + x1); };
  for (let j = 0; j < height; j++) for (let i = 0; i < width; i++) {
    if (!mask[j * width + i]) continue;
    if (!at(i, j - 1)) add(i, j, i + 1, j);
    if (!at(i + 1, j)) add(i + 1, j, i + 1, j + 1);
    if (!at(i, j + 1)) add(i + 1, j + 1, i, j + 1);
    if (!at(i - 1, j)) add(i, j + 1, i, j);
  }
  const rings: P[][] = [];
  for (const [start, list] of next) {
    while (list.length > 0) {
      const ring: P[] = [];
      let [from, to] = [start, list.pop()!];
      let [dx, dy] = [(to % V) - (from % V), ((to / V) | 0) - ((from / V) | 0)];
      ring.push({ x: from % V, y: (from / V) | 0 });
      while (to !== start) {
        const outs = next.get(to)!;
        let pick = 0;
        if (outs.length > 1) {
          const want = [-dy, dx];
          pick = outs.findIndex((n) => (n % V) - (to % V) === want[0]! && ((n / V) | 0) - ((to / V) | 0) === want[1]!);
          if (pick < 0) pick = 0;
        }
        const n = outs.splice(pick, 1)[0]!;
        const [ndx, ndy] = [(n % V) - (to % V), ((n / V) | 0) - ((to / V) | 0)];
        if (ndx !== dx || ndy !== dy) ring.push({ x: to % V, y: (to / V) | 0 });
        [from, to, dx, dy] = [to, n, ndx, ndy];
      }
      const [fx, fy] = [ring[1] ? ring[1].x - ring[0]!.x : 0, ring[1] ? ring[1].y - ring[0]!.y : 0];
      if (Math.sign(fx) === Math.sign(dx) && Math.sign(fy) === Math.sign(dy) && ring.length > 1) ring.shift();
      if (ring.length >= 3) rings.push(ring);
    }
  }
  return rings;
}

/** 📐️ The signed area of a ring (positive counter-clockwise). */
export function area(ring: readonly P[]): number {
  let s = 0;
  for (let i = 0; i < ring.length; i++) { const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!]; s += a.x * b.y - b.x * a.y; }
  return s / 2;
}
