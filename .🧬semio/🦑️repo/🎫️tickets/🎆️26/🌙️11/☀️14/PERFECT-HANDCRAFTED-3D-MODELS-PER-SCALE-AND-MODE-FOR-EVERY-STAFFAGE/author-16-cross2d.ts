import { entryOf, restState, Mask, iouOf, hausdorff } from "./volumes-lib.ts";
const [spec] = process.argv.slice(2);
const { asset } = entryOf(spec!);
const ROLES = new Set(["body", "detail", "glazing"]);
const scales = [50, 100, 200, 500].filter((s) => asset.representations[String(s)].status === "authored");
for (const view of ["elevation", "front", "plan"] as const) {
  const sh = scales.map((s) => restState(asset, s)[view].filter((e: any) => e.closed && ROLES.has(e.shape.role) && e.rings.length > 0));
  const pts = sh.flat().flatMap((e: any) => e.rings.flat());
  const b = { minX: 1e9, minY: 1e9, maxX: -1e9, maxY: -1e9 };
  for (const p of pts) { b.minX = Math.min(b.minX, p.x); b.maxX = Math.max(b.maxX, p.x); b.minY = Math.min(b.minY, p.y); b.maxY = Math.max(b.maxY, p.y); }
  const cell = Math.max(b.maxX - b.minX, b.maxY - b.minY) / 400;
  const box = { minX: b.minX - 2 * cell, minY: b.minY - 2 * cell, maxX: b.maxX + 2 * cell, maxY: b.maxY + 2 * cell };
  const masks = sh.map((list: any[]) => { const m = new Mask(box, cell); for (const e of list) m.fill(e.rings); return m; });
  let line = `${view}:`;
  for (let i = 1; i < scales.length; i++) line += ` ${scales[i - 1]}->${scales[i]} iou=${iouOf(masks[i - 1]!, masks[i]!).toFixed(3)} H=${(hausdorff(masks[i - 1]!, masks[i]!) * 1000).toFixed(0)}mm;`;
  console.log(line);
}
