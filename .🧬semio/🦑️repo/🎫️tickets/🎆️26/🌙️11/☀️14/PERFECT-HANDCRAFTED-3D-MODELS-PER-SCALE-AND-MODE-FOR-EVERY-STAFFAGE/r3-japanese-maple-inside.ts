/** 🔍️ Lists the structure (or thermal) solids of one scale whose projected vertices leave the projected body, with the largest distance, for the japanese maple.
 * Usage: bun r3-japanese-maple-inside.ts <scale> [structure|thermal]
 * @see ./r3-vegetation-japanese-maple.md */
import { entryOf, loadSource, merged, meshesAt, meshMask, projectWorld, resolveList, type View } from "./volumes-lib.ts";

const scale = Number(process.argv[2] ?? 50);
const list = (process.argv[3] ?? "structure") as "structure" | "thermal";
const { entry, asset: raw } = entryOf("vegetation/japanese-maple");
const asset = merged(raw, loadSource(entry)!);
const volume = asset.volumes[String(scale)];
const body = meshesAt(asset, scale, resolveList(volume, "body").filter((s: any) => !s.texture && !s.slender));
const other = meshesAt(asset, scale, resolveList(volume, list));
for (const view of ["elevation", "front"] as View[]) {
  const cell = 0.005;
  const box = { minX: -4, minY: -4, maxX: 6, maxY: 6 };
  const mask = meshMask(view, body, box, cell);
  const at = (x: number, y: number): boolean => {
    const i = Math.floor((x - box.minX) / cell), j = Math.floor((y - box.minY) / cell);
    return (mask as any).cells[j * mask.width + i] === 1;
  };
  for (const m of other) {
    let worst = 0;
    for (let i = 0; i < m.world.length; i += 3) {
      const p = projectWorld(view, [m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]);
      if (at(p.x, p.y)) continue;
      let d = 0;
      while (d < 1 && ![[1, 0], [-1, 0], [0, 1], [0, -1], [0.7, 0.7], [-0.7, 0.7], [0.7, -0.7], [-0.7, -0.7]].some(([a, b]) => at(p.x + a! * d, p.y + b! * d))) d += cell;
      worst = Math.max(worst, d);
    }
    if (worst > 0.006) console.log(`${view} ${m.id} ${worst.toFixed(3)}`);
  }
}
