"""🩹️ The repaired pocket rule of the silhouette guard as one patch of `S/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`: `python3 patch.py --trial` writes the patched module as `silhouette.ts` beside this file with its imports made absolute (the trial copy the tests and the measurement read); `python3 patch.py --land` writes the patched module over the real one (the landing; refused while the anchors of the patch are not found exactly once, and a second run changes nothing).

The patch: (0) `drawsBody` (request "multiview MU"): a slender solid whose projected centre line lies in a filled closed drawn shape of its bone and material (a `loft` or `member` drawn from its own sections) is compared as the solid itself, its projected triangles, instead of a stroke of one width; (1) `agreedPockets`: the small enclosed holes of the drawn mask and of the body mask are filled by one decision per cell (filled where the other mask is covered or has such a hole too, kept where the other mask is open), used by `boundarySpread` in place of two separate `fillSmallHoles`; (2) `Mask.sealRings`: the hairline closing of `Mask.seal` (an eighth of a cell) for filled drawn rings, called for the drawn fills in `measureView`, so a hairline between two drawn shapes is closed as the hairline between two solids is."""
import os, re, sys
HERE = os.path.dirname(os.path.abspath(__file__))
S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage"
REAL = f"{S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"

AGREED = '''/** 🤝️ Both masks with their small enclosed holes filled by ONE decision per cell: a cell of an enclosed hole of at most `maxCells` cells of one mask is filled when the other mask is covered there or has such a hole there too (a seam between overlapping shapes, a gap between slats, a void both agree on); where the other mask is open (the outside, or a void too large to be a pocket) the cell stays empty in both, so the rim of that pocket is outline in both masks and is compared like every other outline. A pocket that one mask encloses and the other leaves open by a slit, and a hole one cell above the limit in one mask and at it in the other, are therefore never filled in one mask alone. */
export function agreedPockets(a: Uint8Array, b: Uint8Array, width: number, height: number, maxCells: number): readonly [Uint8Array, Uint8Array] {
  const [fa, fb] = [fillSmallHoles(a, width, height, maxCells), fillSmallHoles(b, width, height, maxCells)];
  const [oa, ob] = [Uint8Array.from(a), Uint8Array.from(b)];
  for (let k = 0; k < a.length; k++) {
    if (fa[k] && !a[k] && fb[k]) oa[k] = 1;
    if (fb[k] && !b[k] && fa[k]) ob[k] = 1;
  }
  return [oa, ob];
}

'''

SEAL_RINGS = '''  /** 🧽️ The hairline closing of {@link seal} for filled rings (the drawn shapes): a cell that no ring covers, that is covered on both sides along a row, a column or a diagonal, and whose disc of an eighth of a cell lies entirely within an eighth of a cell of the filled rings, is covered. One ring is filled by the non-zero winding rule, several by the even-odd rule, as {@link fill} fills them. A gap wider than a quarter of a cell, a hole and the outer boundary are never changed. */
  sealRings(groups: readonly Group[]): void {
    const { width, height, cells, box, cell } = this;
    const reach = cell / 8;
    const candidates: number[] = [];
    for (let j = 1; j < height - 1; j++)
      for (let i = 1; i < width - 1; i++) {
        const k = j * width + i;
        if (cells[k]) continue;
        if ((cells[k - 1] && cells[k + 1]) || (cells[k - width] && cells[k + width]) || (cells[k - width - 1] && cells[k + width + 1]) || (cells[k - width + 1] && cells[k + width - 1])) candidates.push(k);
      }
    if (candidates.length === 0) return;
    const boxes = groups.map((g) => boundsOf(g));
    const inside = (g: Group, p: Point): boolean => {
      let [winding, crossings] = [0, 0];
      for (const ring of g)
        for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
          const [a, b] = [ring[k]!, ring[i]!];
          if (a.y <= p.y === b.y <= p.y) continue;
          if (a.x + ((p.y - a.y) / (b.y - a.y)) * (b.x - a.x) <= p.x) continue;
          crossings++;
          winding += b.y > a.y ? 1 : -1;
        }
      return g.length === 1 ? winding !== 0 : crossings % 2 === 1;
    };
    const toEdge = (g: Group, p: Point): number => {
      let best = Infinity;
      for (const ring of g)
        for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
          const [a, b] = [ring[k]!, ring[i]!];
          const [dx, dy] = [b.x - a.x, b.y - a.y];
          const t = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
          best = Math.min(best, Math.hypot(p.x - a.x - t * dx, p.y - a.y - t * dy));
        }
      return best;
    };
    const probes = [[0, 0], ...[0.5, 1].flatMap((r) => [[r, 0], [-r, 0], [0, r], [0, -r], [0.7071 * r, 0.7071 * r], [-0.7071 * r, 0.7071 * r], [0.7071 * r, -0.7071 * r], [-0.7071 * r, -0.7071 * r]])] as const;
    for (const k of candidates) {
      const c: Point = { x: box.minX + ((k % width) + 0.5) * cell, y: box.minY + (Math.floor(k / width) + 0.5) * cell };
      const near = groups.filter((_, n) => c.x >= boxes[n]![0] - 2 * reach && c.x <= boxes[n]![1] + 2 * reach && c.y >= boxes[n]![2] - 2 * reach && c.y <= boxes[n]![3] + 2 * reach);
      if (near.length === 0) continue;
      if (probes.every(([dx, dy]) => near.some((g) => inside(g, { x: c.x + dx * reach, y: c.y + dy * reach }) || toEdge(g, { x: c.x + dx * reach, y: c.y + dy * reach }) <= reach))) cells[k] = 1;
    }
  }

'''

DRAWS_BODY = '''/** 🫧️ Whether a `loft` shape of the bone (and material), filled and closed, is the body of a slender solid itself (a drawing that carries the sections of the solid; any other filled shape is compared at the width it is drawn with): nine tenths of the points of its projected centre line lie inside the shape or within half the true width of its outline. A `loft` or `member` drawn from the sections of the solid is such a shape: the reference is then the solid as it is (its projected triangles, tapering included), not a stroke of one width. */
export function drawsBody(bone: string | undefined, centre: readonly Point[], drawn: Drawn, material: string | undefined, truthM: number): boolean {
  const inside = (p: Point, ring: readonly Point[]): boolean => {
    let odd = false;
    for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
      const [a, b] = [ring[k]!, ring[i]!];
      if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) odd = !odd;
    }
    return odd;
  };
  return drawn.some((d) => d.bone === bone && d.body === true && d.closed && d.filled && d.rings.length > 0 && (material === undefined || d.material === undefined || d.material === material) && centre.filter((p) => d.rings.some((ring) => inside(p, ring)) || distanceToPolylines(p, polylines(d)) <= truthM / 2).length >= 0.9 * centre.length);
}

'''

TYPE_OLD = 'export type Drawn = readonly { readonly bone: string; readonly material?: string; readonly closed: boolean;'
TYPE_NEW = 'export type Drawn = readonly { readonly bone: string; readonly material?: string; readonly body?: boolean; readonly closed: boolean;'
SHAPE_OLD = 'export function drawnOf(evaluated: readonly { readonly shape: { readonly bone?: string; readonly material?: string; readonly role: string;'
SHAPE_NEW = 'export function drawnOf(evaluated: readonly { readonly shape: { readonly kind?: string; readonly bone?: string; readonly material?: string; readonly role: string;'
MAP_OLD = '...(s.shape.material === undefined ? {} : { material: s.shape.material }), closed: s.closed,'
MAP_NEW = '...(s.shape.material === undefined ? {} : { material: s.shape.material }), ...(s.shape.kind === "loft" ? { body: true } : {}), closed: s.closed,'

PAIRS = [
    (TYPE_OLD, TYPE_NEW),
    (SHAPE_OLD, SHAPE_NEW),
    (MAP_OLD, MAP_NEW),
    ('/** 🧵️ The strokes a slender solid is drawn as (C24 refined)', DRAWS_BODY + '/** 🧵️ The strokes a slender solid is drawn as (C24 refined)'),
    ('    const followed = drawn === undefined ? undefined : followedWidth(solid.bone, centre, drawn, solid.material, truth);',
     '''    if (drawn !== undefined && drawsBody(solid.bone, centre, drawn, solid.material, truth)) {
      for (let t = 0; t + 2 < local.length; t += 3) out.push([0, 1, 2].map((k) => vertices[local[t + k]!]!));
      continue;
    }
    const followed = drawn === undefined ? undefined : followedWidth(solid.bone, centre, drawn, solid.material, truth);'''),
    ('''/** 📈️ The spread of the boundary distances of `a` and `b` over the cells of `core` (enclosed holes of at most `holeCells` cells are filled first: they are not outline);''',
     AGREED + '''/** 📈️ The spread of the boundary distances of `a` and `b` over the cells of `core` (enclosed holes of at most `holeCells` cells that both masks agree on are filled first, {@link agreedPockets}: they are not outline);'''),
    ('''  const [ba, bb] = [boundaryOf(fillSmallHoles(a, width, height, holes), width, height), boundaryOf(fillSmallHoles(b, width, height, holes), width, height)];''',
     '''  const [fa, fb] = agreedPockets(a, b, width, height, holes);
  const [ba, bb] = [boundaryOf(fa, width, height), boundaryOf(fb, width, height)];'''),
    ('''  /** 🖌️ Fills the rings: one ring by the non-zero winding rule''', SEAL_RINGS + '''  /** 🖌️ Fills the rings: one ring by the non-zero winding rule'''),
    ('''  for (const g of filled) area.fill(g);
''', '''  for (const g of filled) area.fill(g);
  area.sealRings(filled);
'''),
]


def patched(text):
    """🧵️ The module text with the patch applied; a text that already carries it is returned as it is."""
    if "export function drawsBody" in text:
        return text
    for old, new in PAIRS:
        if text.count(old) != 1:
            raise SystemExit(f"anchor found {text.count(old)} times, nothing written: {old[:80]}")
        text = text.replace(old, new)
    return text


source = open(REAL, encoding="utf8").read()
out = patched(source)
if "--land" in sys.argv:
    open(REAL, "w", encoding="utf8").write(out)
    print("landed" if out != source else "already landed")
elif "--trial" in sys.argv:
    base = os.path.dirname(REAL)
    absolute = re.sub(r'from "(\.\.?/[^"]+)"', lambda m: f'from "{os.path.normpath(os.path.join(base, m.group(1)))}"', out)
    open(f"{HERE}/silhouette.ts", "w", encoding="utf8").write(absolute)
    print(f"trial copy written, {out.count(chr(10)) - source.count(chr(10))} lines added")
else:
    raise SystemExit("say --trial or --land")
