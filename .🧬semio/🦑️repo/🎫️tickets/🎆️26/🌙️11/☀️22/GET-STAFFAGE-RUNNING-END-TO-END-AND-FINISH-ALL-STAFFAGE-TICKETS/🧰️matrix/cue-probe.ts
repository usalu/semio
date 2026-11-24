/** 🔦️ Shows why an analysis scene has no colour-independent cue: `bun cue-probe.ts <tests-dir> <asset> <denominator> <pose> <mode>` prints per viewpoint the member values of the scene, their share of the legend range, the pattern and dash of every valued path and the legends. */
import { join, resolve } from "node:path";

const [testsArg, id, denominatorArg, pose, mode] = process.argv.slice(2) as [string, string, string, string, string];
const tests = resolve(testsArg);
const engine = await import(join(tests, "🔭️multiview-coverage", "🧰️.ts"));
const { resolveViews } = await import(join(tests, "..", "🔨️modules", "🔭️multiview", "🟦️.ts"));
const asset = engine.loadCatalog().find((candidate: any) => candidate.id === id);
const denominator = Number(denominatorArg);
console.log(`bones: ${asset.rig.bones.map((b: any) => `${b.id}(${b.lengthM} m${b.section ? ", section" : ""})`).join(" ")}`);
console.log(`structure keys: ${Object.keys(asset.structure).join(", ")}; supports: ${asset.anchors.filter((a: any) => a.support).map((a: any) => `${a.id}:${a.support}`).join(" ")}`);
const multi = resolveViews(asset, { kind: "pose", pose: pose === "default" ? asset.presentation.defaultPose : pose }, { kind: "scale", denominator }, mode, engine.drawOptions(mode, denominator));
for (const view of ["elevation", "front", "plan"]) {
  const drawing = multi.views[view];
  if (drawing.kind !== "drawing") {
    console.log(`${view}: ${drawing.kind} ${drawing.code}`);
    continue;
  }
  const rows = new Map<string, string>();
  let patterns = 0;
  let dashes = 0;
  const layers = new Map<string, number>();
  for (const layer of drawing.scene.layers)
    for (const item of layer.items) {
      if (item.kind !== "path") continue;
      layers.set(layer.id, (layers.get(layer.id) ?? 0) + 1);
      if (item.fill?.pattern !== undefined) patterns++;
      if ((item.stroke?.dashMm?.length ?? 0) > 0) dashes++;
      if (item.value !== undefined) rows.set(`${layer.id}|${item.value.quantity}|${item.value.label?.en ?? ""}`, `${item.value.value.toPrecision(4)} ${item.value.unit}${item.fill?.pattern ? ` pattern ${item.fill.pattern}` : ""}${item.stroke?.dashMm ? ` dash ${item.stroke.dashMm}` : ""}`);
    }
  console.log(`\n${view}: paths per layer ${[...layers].map(([k, v]) => `${k}:${v}`).join(" ")}; patterns ${patterns}, dashes ${dashes}`);
  for (const [key, value] of rows) console.log(`  ${key} = ${value}`);
  for (const legend of drawing.scene.legends) console.log(`  legend ${legend.title.en} [${legend.scale?.min}, ${legend.scale?.max}] ${legend.unit}: ${legend.interpretation?.en?.slice(0, 200)}`);
}
