/** ⚖️ The paper and dimension findings of the default pose of one asset per authored scale, with the module of `ROOT` (worktree or a mirror with `03-paper-tie.patch`).
 * usage: ROOT=<mirror> bun measure-tie.ts used-harmonium
 */
import { mod, readAsset } from "./lib.ts";

const engine = await mod("🧪️tests/🔭️multiview-coverage/🧰️.ts");
const { resolveViews } = await mod("🔨️modules/🔭️multiview/🟦️.ts");
const asset = readAsset(process.argv[2]!);
const query = { kind: "pose", pose: asset.presentation.defaultPose };
for (const key of Object.keys(asset.representations)) {
  if (asset.representations[key].status !== "authored") continue;
  const denominator = Number(key);
  const multi = resolveViews(asset, query, { kind: "scale", denominator }, "light-line", engine.drawOptions("light-line", denominator, false, "en"));
  const dimension = engine.geometryFindings(asset, multi.state, true).filter((f: any) => f.check.startsWith("dimension-")).map((f: any) => f.check);
  for (const view of engine.VIEWPOINT_LIST) {
    const drawing = multi.views[view];
    if (drawing.kind === "unavailable") continue;
    const box = engine.paperBox(drawing.scene, drawing.state.shapes[view]);
    const findings = engine.paperFindings(asset, view, drawing.scene, drawing.state.shapes[view]);
    const k = drawing.scene.metadata.mmPerMetre;
    const h = view === "plan" ? asset.dimensions.depthM : asset.dimensions.heightM;
    const got = box.maxY - box.minY;
    console.log(`1:${key} ${view}: drawn height ${got} mm, nominal ${h * k} mm, |difference| ${Math.abs(got - h * k)}, tolerance ${engine.d18(h) * k}, excess ${Math.abs(got - h * k) - engine.d18(h) * k} -> paper findings [${findings.map((f: any) => f.check).join(", ")}]`);
  }
  console.log(`1:${key} dimension findings in metres: [${dimension.join(", ")}]`);
}
