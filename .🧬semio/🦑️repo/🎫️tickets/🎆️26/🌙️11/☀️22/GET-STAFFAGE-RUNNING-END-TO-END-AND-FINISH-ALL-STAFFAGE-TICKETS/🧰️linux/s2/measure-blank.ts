/** 🕳️ The `blank-shapes` verdict of the symbol alternatives of one asset per pose, view and mode: the test of the worktree (fills `shape:<id>` only) beside the test of `ROOT` when it exports `drawnIds` (`04-blank-shapes.patch`), and the reason text.
 * usage: ROOT=<mirror> bun measure-blank.ts kneeling-gardener gardener-block 200,500
 */
import { mod, readAsset } from "./lib.ts";

const engine = await mod("🧪️tests/🔭️multiview-coverage/🧰️.ts");
const { resolveViews } = await mod("🔨️modules/🔭️multiview/🟦️.ts");
const [id, symbol, list] = process.argv.slice(2);
const asset = readAsset(id!);
for (const denominator of list!.split(",").map(Number)) {
  let [cells, oldBlank, newBlank] = [0, 0, 0];
  const failing = new Set<string>();
  let reason = "";
  for (const pose of asset.poses)
    for (const mode of engine.MODE_LIST) {
      const multi = resolveViews(asset, { kind: "pose", pose: pose.id }, { kind: "symbol", symbol, denominator }, mode, engine.drawOptions(mode, denominator, false, "en"));
      for (const view of engine.VIEWPOINT_LIST) {
        const drawing = multi.views[view];
        if (drawing.kind === "unavailable" || mode === "forces" || mode === "moments") continue;
        cells++;
        const shapes = drawing.state.shapes[view];
        const fills = new Set(shapes.filter((e: any) => e.shape.role !== "ground").map((e: any) => `shape:${e.shape.id}`));
        if (engine.sceneBox(drawing.scene, fills) === undefined) {
          oldBlank++;
          failing.add(`${view}|${mode}`);
          reason = engine.blankReason(drawing.state, view, mode);
        }
        if (engine.drawnIds !== undefined && engine.sceneBox(drawing.scene, engine.drawnIds(drawing.scene, shapes)) === undefined) newBlank++;
      }
    }
  console.log(`${id} symbol ${symbol} 1:${denominator}: ${cells} pose x view x mode checks; blank by fills only ${oldBlank} (${failing.size} cells of view x mode); blank by ${engine.drawnIds === undefined ? "n/a (module has no drawnIds)" : `drawnIds ${newBlank}`}`);
  console.log(`  reason text: ${reason}`);
}
