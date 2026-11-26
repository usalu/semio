/** ⚖️ The flat front and plan shapes of assets evaluated by the engine before (`old-mvl1.ts`, the module with the elevation of a loft or solid left out of the depth twins) and after the change of MV-L1: compare.ts <max poses> <asset id>… prints per asset the shapes compared, the shapes that differ and the largest coordinate difference in metres. */
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate as after } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { evaluate as before } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/old-mvl1.ts";
const [limit, ...ids] = process.argv.slice(2);
const root = process.env.S + "/🗂️catalog";
for (const id of ids) {
  const file = readdirSync(root).map((c) => join(root, c, id, "🔣️.json")).find((f) => { try { readFileSync(f); return true; } catch { return false; } })!;
  const asset = JSON.parse(readFileSync(file, "utf8")).asset;
  let compared = 0, differ = 0, worst = 0; const kinds = new Set<string>();
  for (const scale of [50, 100, 200, 500]) {
    if ((asset.representations[String(scale)] as any)?.status !== "authored") continue;
    for (const pose of asset.poses.slice(0, Number(limit))) {
      const query = { kind: "pose", pose: pose.id } as const, select = { kind: "scale", scale: { denominator: scale } } as const;
      let a, b;
      try { a = before(asset, query, select); b = after(asset, query, select); } catch (e) { console.log(id, scale, pose.id, "ERROR", String(e).slice(0, 120)); continue; }
      for (const view of ["front", "plan"] as const) a.shapes[view].forEach((s: any, i: number) => {
        compared++;
        const t = b.shapes[view][i]!;
        if (JSON.stringify(s.rings) === JSON.stringify(t.rings)) return;
        differ++; kinds.add(`${view}:${s.shape.kind}${s.shape.solid ? "+solid" : ""}`);
        s.rings.flat().forEach((p: any, k: number) => { const q = t.rings.flat()[k]; if (q) worst = Math.max(worst, Math.hypot(p.x - q.x, p.y - q.y)); });
      });
    }
  }
  console.log(`${id}: compared ${compared} differ ${differ} worst ${worst.toFixed(4)} m ${[...kinds].join(",")}`);
}
