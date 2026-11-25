/** 📦️ The evaluated boxes of every drawn shape of assets, for the offline look at a finding: `bun boxes.ts <out dir> <id|file.json>…` writes `<out dir>/<id>.json` with, per authored scale and pose, per view the rows `[shape id, bone, x0, x1, y0, y1]` in millimetres and the evaluated anchors `[id, bone, x, y, z]`. Run it from `T`.
 * @see ../MR/gate.ts — the gate that finds what this explains */
import { join } from "node:path";
import { mkdirSync, writeFileSync } from "node:fs";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const [out, ...args] = process.argv.slice(2);
mkdirSync(out!, { recursive: true });
const mm = (n: number): number => Math.round(n * 1000);
for (const arg of args) {
  const asset = readDoc(arg).doc.asset;
  const result: Record<string, Record<string, unknown>> = {};
  for (const key of Object.keys(asset.representations).filter((k) => engine.authoredOf(asset, k) !== undefined)) {
    result[key] = {};
    for (const pose of asset.poses) {
      let state: any;
      try {
        state = evaluate(asset, { kind: "pose", pose: pose.id }, engine.scaleSelector(Number(key)));
      } catch (error) {
        result[key]![pose.id] = { error: (error as Error).message };
        continue;
      }
      const views: Record<string, unknown[]> = {};
      for (const view of ["elevation", "front", "plan"]) {
        views[view] = state.shapes[view].flatMap((s: any) => {
          const pts = s.rings.flat();
          if (pts.length === 0) return [];
          const xs = pts.map((p: any) => p.x), ys = pts.map((p: any) => p.y);
          return [[s.shape.id, s.shape.bone, mm(Math.min(...xs)), mm(Math.max(...xs)), mm(Math.min(...ys)), mm(Math.max(...ys))]];
        });
      }
      views.anchors = Object.entries(state.anchors ?? {}).map(([id, v]: [string, any]) => [id, asset.anchors.find((n: any) => n.id === id)?.bone, mm(v.x ?? v.position?.x ?? 0), mm(v.y ?? v.position?.y ?? 0), mm(v.z ?? 0)]);
      result[key]![pose.id] = views;
    }
  }
  writeFileSync(join(out!, `${asset.id}.json`), JSON.stringify(result));
  console.log(`[DEBUG] boxes ${asset.id} written`);
}
