/** 🔬️ [DEBUG] probe: traces the torso coat solids of walking-commuter and compares with the authored 2D shape. */
import { readDoc, shapesFrom, ringArea } from "./lib.ts";
const doc = await readDoc("walking-commuter");
const a = doc.asset;
for (const view of ["elevation", "front", "plan"] as const) {
  const t0 = performance.now();
  const sh = shapesFrom(a, "50", view, ["coat"], { id: "coat", bone: "torso", region: "coat", material: "wool-coat", tone: 0.3, z: 4 });
  const old = a.representations["50"].geometry[view].find((s: any) => s.id === "coat" || (s.bone === "torso" && s.region === "coat"));
  console.log(view, Math.round(performance.now() - t0), "ms", sh.length, sh[0]?.points.length, "area", sh.map((s: any) => ringArea(s.points).toFixed(4)), "old", old?.id, old && ringArea(old.points).toFixed(4));
  const bb = (pts: number[][]) => [0, 1].map((k) => [Math.min(...pts.map((p) => p[k]!)), Math.max(...pts.map((p) => p[k]!))].map((v) => v.toFixed(3)).join(".."));
  console.log("  new", bb(sh[0].points), "old", old && bb(old.points));
}
