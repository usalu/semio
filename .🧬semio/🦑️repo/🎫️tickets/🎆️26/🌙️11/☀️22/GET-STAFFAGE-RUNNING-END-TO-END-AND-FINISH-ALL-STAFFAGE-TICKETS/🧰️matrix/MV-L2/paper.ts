/** 📐️ What sets the paper box of one view of an asset at a requested scale: `bun paper.ts <id|file.json> <elevation|front|plan> <pose> <denominator,…>` prints per scale the paper box of the matrix (`paperBox`: fills and outline runs) against the dimensions, the evaluated extent, and the shapes that reach the four sides of the evaluated extent with whether the scene draws them.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts — `paperFindings` */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";

const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const { resolveViews } = await import(join(S, "🔨️modules", "🔭️multiview", "🟦️.ts"));
const [id, view, pose, denominators] = process.argv.slice(2) as [string, "elevation" | "front" | "plan", string, string];
const asset = JSON.parse(readFileSync(fileOf(id), "utf8")).asset;
const d = asset.dimensions;
const [w, h] = view === "elevation" ? [d.widthM, d.heightM] : view === "front" ? [d.depthM, d.heightM] : [d.widthM, d.depthM];
for (const denominator of denominators.split(",").map(Number)) {
  const multi = resolveViews(asset, { kind: "pose", pose }, { kind: "scale", denominator }, "light-line", engine.drawOptions("light-line", denominator));
  const drawing = multi.views[view];
  if (drawing.kind !== "drawing") { console.log(`1:${denominator} ${drawing.kind} ${drawing.code}`); continue; }
  const k = drawing.scene.metadata.mmPerMetre;
  const shapes = drawing.state.shapes[view].filter((evaluated: any) => evaluated.shape.role !== "ground");
  const ids = engine.paperIds(drawing.scene, shapes);
  const box = engine.paperBox(drawing.scene, shapes);
  const extent = engine.extentOf(shapes);
  console.log(`1:${denominator} ${view} ${pose}: dimensions ${w} x ${h} m; paper box ${((box.maxX - box.minX) / k).toFixed(4)} x ${((box.maxY - box.minY) / k).toFixed(4)} m; evaluated ${(extent.maxX - extent.minX).toFixed(4)} x ${(extent.maxY - extent.minY).toFixed(4)} m (x ${extent.minX.toFixed(4)} … ${extent.maxX.toFixed(4)}, y ${extent.minY.toFixed(4)} … ${extent.maxY.toFixed(4)}); findings ${JSON.stringify(engine.paperFindings(asset, view, drawing.scene, shapes).map((finding: any) => finding.message))}`);
  const drawn = (shape: string): string => { const own = [...ids].filter((item) => item === `shape:${shape}` || item.startsWith(`outline:${shape}:`)); return own.length === 0 ? "NOT DRAWN" : `drawn (${own.length} paths)`; };
  for (const [side, pick] of [["top", (p: any) => p.y], ["bottom", (p: any) => -p.y], ["right", (p: any) => p.x], ["left", (p: any) => -p.x]] as const) {
    const reach = shapes.map((evaluated: any) => ({ id: evaluated.shape.id, far: Math.max(...evaluated.rings.flat().map(pick)) })).filter((entry: any) => Number.isFinite(entry.far)).sort((a: any, b: any) => b.far - a.far).slice(0, 5);
    console.log(`  ${side}: ${reach.map((entry: any) => `${entry.id} ${entry.far.toFixed(4)} ${drawn(entry.id)}`).join("; ")}`);
  }
  const items = drawing.scene.layers.flatMap((layer: any) => layer.items.filter((item: any) => item.kind === "path" && ids.has(item.id)).map((item: any) => ({ id: item.id, ys: item.commands.filter((c: any) => c.op !== "Z").flatMap((c: any) => (c.op === "C" ? [c.y, c.y1, c.y2] : [c.y])) })));
  const tops = items.map((item: any) => ({ id: item.id, min: Math.min(...item.ys), max: Math.max(...item.ys) }));
  console.log(`  paper top (smallest y): ${tops.sort((a: any, b: any) => a.min - b.min).slice(0, 4).map((t: any) => `${t.id} ${t.min.toFixed(3)} mm`).join("; ")}`);
  console.log(`  paper bottom (largest y): ${tops.sort((a: any, b: any) => b.max - a.max).slice(0, 4).map((t: any) => `${t.id} ${t.max.toFixed(3)} mm`).join("; ")}`);
}
