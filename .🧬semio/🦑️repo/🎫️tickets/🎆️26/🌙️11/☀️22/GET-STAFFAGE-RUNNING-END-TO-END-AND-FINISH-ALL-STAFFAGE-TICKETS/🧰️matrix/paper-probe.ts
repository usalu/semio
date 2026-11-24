/** 📏️ Shows what the `paper-width` and `paper-height` checks of the matrix measure: `bun paper-probe.ts <tests-dir> <asset> <view> <denominators>` prints per scale the evaluated extent in paper mm, the box of the `shape:<id>` paths (what the matrix reads), the box of the `shape:` and `outline:` paths together, and the shapes whose only paths are outlines. */
import { join, resolve } from "node:path";

const [testsArg, id, view, denominators] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", string];
const tests = resolve(testsArg);
const engine = await import(join(tests, "🔭️multiview-coverage", "🧰️.ts"));
const { resolveViews } = await import(join(tests, "..", "🔨️modules", "🔭️multiview", "🟦️.ts"));
const asset = engine.loadCatalog().find((candidate: any) => candidate.id === id);
const d = asset.dimensions;
const [w, h] = view === "elevation" ? [d.widthM, d.heightM] : view === "front" ? [d.depthM, d.heightM] : [d.widthM, d.depthM];
for (const denominator of denominators.split(",").map(Number)) {
  const multi = resolveViews(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", denominator }, "light-line", engine.drawOptions("light-line", denominator));
  const drawing = multi.views[view];
  if (drawing.kind !== "drawing") {
    console.log(`1:${denominator} ${drawing.kind} ${drawing.code}`);
    continue;
  }
  const k = drawing.scene.metadata.mmPerMetre;
  const shapes = drawing.state.shapes[view].filter((evaluated: any) => evaluated.shape.role !== "ground");
  const extent = engine.extentOf(drawing.state.shapes[view]);
  const fills = new Set(shapes.map((evaluated: any) => `shape:${evaluated.shape.id}`));
  const named = engine.sceneBox(drawing.scene, fills);
  const both = new Set<string>(fills);
  const outlined = new Set<string>();
  const filled = new Set<string>();
  for (const layer of drawing.scene.layers)
    for (const item of layer.items) {
      if (item.kind !== "path") continue;
      const match = /^(shape|outline):([^:]+)/u.exec(item.id);
      if (match === null || !fills.has(`shape:${match[2]}`)) continue;
      both.add(item.id);
      (match[1] === "shape" ? filled : outlined).add(match[2]!);
    }
  const all = engine.sceneBox(drawing.scene, both);
  const onlyOutline = [...outlined].filter((shape) => !filled.has(shape));
  const fmt = (box: any): string => (box === undefined ? "none" : `${(box.maxX - box.minX).toFixed(2)} x ${(box.maxY - box.minY).toFixed(2)} mm`);
  console.log(`1:${denominator} dimensions ${(w * k).toFixed(2)} x ${(h * k).toFixed(2)} mm (tolerance ${(engine.d18(w) * k).toFixed(2)} x ${(engine.d18(h) * k).toFixed(2)}); evaluated ${((extent.maxX - extent.minX) * k).toFixed(2)} x ${((extent.maxY - extent.minY) * k).toFixed(2)} mm; shape: paths ${fmt(named)}; shape: and outline: paths ${fmt(all)}; drawn as outline only: ${onlyOutline.join(", ") || "none"}; never drawn: ${shapes.map((e: any) => e.shape.id).filter((shape: string) => !filled.has(shape) && !outlined.has(shape)).join(", ") || "none"}`);
}
