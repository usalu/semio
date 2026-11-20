/** 🕳️ Round 3, vegetation/hosta-clump: lists the enclosed holes of the drawing and of the projected body in one view with their areas, to see which holes lie near the validator's fill threshold (16 x 16 cells of a quarter feature). Usage: bun r3-hosta-clump-holes.ts <scale> <view>
 * @see ./r3-vegetation-hosta-clump.md */
import { entryOf, loadSource, merged, poseState, scaleMeshes, viewMasks, type View } from "./volumes-lib.ts";
const [scale, view] = [Number(process.argv[2]), process.argv[3] as View];
const { entry, asset: catalog } = entryOf("vegetation/hosta-clump");
const asset = merged(catalog, loadSource(entry)!);
const sm = scaleMeshes(asset, scale, asset.volumes[String(scale)]);
const m = viewMasks(view, poseState(asset, scale), sm.body.filter((_, i) => sm.solids.body[i]!.texture !== true));
const limit = 256 * ((0.15 * scale) / 1000 / 4) ** 2 * 1e4;
for (const [name, mask] of [["drawn", m.truth], ["body", m.volume]] as const) {
  const { width: w, height: h, cells } = mask;
  const seen = new Uint8Array(w * h);
  const out: string[] = [];
  for (let start = 0; start < w * h; start++) {
    if (cells[start] || seen[start]) continue;
    const queue = [start];
    seen[start] = 1;
    let [n, sx, sy, open] = [0, 0, 0, false];
    while (queue.length > 0) {
      const k = queue.pop()!;
      const [x, y] = [k % w, Math.floor(k / w)];
      n++; sx += x; sy += y;
      if (x === 0 || y === 0 || x === w - 1 || y === h - 1) open = true;
      for (const [dx, dy] of [[1, 0], [-1, 0], [0, 1], [0, -1]]) {
        const [nx, ny] = [x + dx!, y + dy!];
        if (nx < 0 || ny < 0 || nx >= w || ny >= h) continue;
        const j = ny * w + nx;
        if (!cells[j] && !seen[j]) { seen[j] = 1; queue.push(j); }
      }
    }
    const area = n * mask.cell * mask.cell * 1e4;
    if (!open && area > limit * 0.35 && area < limit * 3) out.push(`${area.toFixed(1)} cm2 at ${(m.box.minX + (sx / n) * mask.cell).toFixed(3)}, ${(m.box.minY + (sy / n) * mask.cell).toFixed(3)}`);
  }
  console.log(`[DEBUG] ${name} holes near the fill limit of ${limit.toFixed(1)} cm2: ${out.join("; ")}`);
}
