/** 🔍️ The evaluated rings of the drawn shapes of one view in the default pose (id, role, pen, bounding box in paper metres) and, with `--solids`, the projected body solids; filtered by an id substring or by a point with a radius (`--at x y r`).
 *  bun rings.ts <id|file.json> <scale> <view> [substring] [--at x y r] [--solids] */
import { S, fileOf } from "./lib.ts";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { worldSolids, project } = await import(`${S}/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts`);
const args = process.argv.slice(2);
const wantSolids = args.includes("--solids");
const atIndex = args.indexOf("--at");
const at = atIndex < 0 ? undefined : { x: Number(args[atIndex + 1]), y: Number(args[atIndex + 2]), r: Number(args[atIndex + 3]) };
const rest = args.filter((a, i) => a !== "--solids" && (atIndex < 0 || i < atIndex || i > atIndex + 3));
const [name, scaleArg, view, like] = rest as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
const scale = Number(scaleArg);
const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const hit = (x0: number, x1: number, y0: number, y1: number): boolean => at === undefined || (x0 <= at.x + at.r && x1 >= at.x - at.r && y0 <= at.y + at.r && y1 >= at.y - at.r);
for (const s of state.shapes[view!]) {
  if (like !== undefined && like !== "-" && !s.shape.id.includes(like)) continue;
  const pts = s.rings.flat();
  if (pts.length === 0) continue;
  const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
  const [x0, x1, y0, y1] = [Math.min(...xs), Math.max(...xs), Math.min(...ys), Math.max(...ys)];
  if (hit(x0, x1, y0, y1)) console.log(`D ${s.shape.id} ${s.shape.role} ${s.shape.kind} ${s.closed ? "closed" : "open"}${s.shape.filled === false ? " unfilled" : ""} pen ${s.shape.penMm ?? ""} z ${s.shape.z} pts ${pts.length} x ${x0.toFixed(4)}..${x1.toFixed(4)} y ${y0.toFixed(4)}..${y1.toFixed(4)}`);
}
if (wantSolids) for (const w of worldSolids(asset, state.bones, scale)) {
  if (like !== undefined && like !== "-" && !w.id.includes(like)) continue;
  let [x0, x1, y0, y1] = [Infinity, -Infinity, Infinity, -Infinity];
  for (let i = 0; i < w.positions.length; i += 3) {
    const p = project(view, w.positions[i], w.positions[i + 1], w.positions[i + 2]);
    [x0, x1, y0, y1] = [Math.min(x0, p.x), Math.max(x1, p.x), Math.min(y0, p.y), Math.max(y1, p.y)];
  }
  if (hit(x0, x1, y0, y1)) console.log(`V ${w.id} bone ${w.bone}${w.slender ? " slender" : ""}${w.texture ? " texture" : ""} tris ${w.indices.length / 3} x ${x0.toFixed(4)}..${x1.toFixed(4)} y ${y0.toFixed(4)}..${y1.toFixed(4)}`);
}
