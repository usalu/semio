/** 🧵️ Shows how the analysis scenes of assets get their colour-independent cue after the shared repair: `bun cue-after.ts <tests-dir> <ids,…>` prints per asset, authored scale, mode (forces, moments) and viewpoint whether the scene is hatched or dashed, how many diagram areas are hatched, and for an unhatched scene the strongest valued path as a share of the range of its legend (a share above 15 % in an unhatched scene would be a colour-only strong value). */
import { join, resolve } from "node:path";

const [testsArg, idsArg] = process.argv.slice(2) as [string, string];
const tests = resolve(testsArg);
const engine = await import(join(tests, "🔭️multiview-coverage", "🧰️.ts"));
const { resolveViews } = await import(join(tests, "..", "🔨️modules", "🔭️multiview", "🟦️.ts"));
for (const id of idsArg.split(",")) {
  const asset = engine.loadCatalog().find((candidate: any) => candidate.id === id);
  for (const key of Object.keys(asset.representations).filter((k) => engine.authoredOf(asset, k) !== undefined))
    for (const mode of ["forces", "moments"]) {
      const multi = resolveViews(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", denominator: Number(key) }, mode, engine.drawOptions(mode, Number(key)));
      const cells: string[] = [];
      for (const view of ["elevation", "front", "plan"]) {
        const drawing = multi.views[view];
        if (drawing.kind !== "drawing") {
          cells.push(`${view} ${drawing.code}`);
          continue;
        }
        const paths = drawing.scene.layers.flatMap((layer: any) => layer.items.filter((item: any) => item.kind === "path"));
        const hatchedDiagrams = paths.filter((item: any) => item.id.startsWith("diagram:") && item.fill?.pattern !== undefined).length;
        const cue = engine.hasCue(drawing.scene);
        const share = (prefix: string, legend: any): number => {
          const span = Math.max(Math.abs(legend?.min ?? 0), Math.abs(legend?.max ?? 0));
          const strongest = Math.max(0, ...paths.filter((item: any) => item.value !== undefined && item.id.startsWith(prefix)).map((item: any) => Math.abs(item.value.value)));
          return span > 0 ? (100 * strongest) / span : 0;
        };
        const member = share("shape:", drawing.scene.legends[0]);
        const diagram = share("diagram:", drawing.scene.legends.find((legend: any) => legend.quantity.endsWith("-diagram")));
        cells.push(`${view} ${cue ? "cue" : `UNHATCHED strongest member fill ${member.toFixed(1)} %, diagram ${diagram.toFixed(1)} %, rule ${engine.cueFinding(mode, drawing.scene) === undefined ? "ok" : "FAIL"}`}${hatchedDiagrams > 0 ? ` (${hatchedDiagrams} hatched diagram areas)` : ""}`);
      }
      console.log(`${id} 1:${key} ${mode}: ${cells.join("; ")}`);
    }
}
