/** 🪞️ The derived-view measure of the suite on every authored band of an asset in its default pose, without the thresholds: per band the box of each view, the normalised Hausdorff distance of front and plan to the elevation and of plan to front under the best candidate transform (limit 0.03, `derived-view-in-band`) and of front and plan to the same view of every other band (limit 0.005, `derived-view-band-copy`), each also in millimetres of the longest side; `--far` adds per pair the point of either set that lies farthest from the other set.
 *  bun derived.ts <id | file.json>… [--far]
 * @see ../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🪞️derived.ts — the measure */
import { S, fileOf } from "./files.ts";
const T = `${S}/🧪️tests/🔭️multiview-coverage`;
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { pointSet, bestTransform, hausdorff, TRANSFORMS, NEAR_MATCH, COPY } = await import(`${T}/🪞️derived.ts`);
const { authoredOf } = await import(`${T}/🧰️.ts`);
type P = { x: number; y: number };
const far = process.argv.includes("--far");
const sideOf = (shapes: any[]): { side: number; w: number; h: number; n: number } => {
  const pts: P[] = shapes.filter((s) => s.shape.role !== "ground").flatMap((s) => s.rings.flat());
  const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y);
  const w = Math.max(...xs) - Math.min(...xs), h = Math.max(...ys) - Math.min(...ys);
  return { side: Math.max(w, h), w, h, n: pts.length };
};
const farthest = (a: P[], b: P[]): string => {
  let worst = { d: 0, p: a[0]! };
  for (const p of a) {
    const d = Math.min(...b.map((q) => Math.hypot(p.x - q.x, p.y - q.y)));
    if (d > worst.d) worst = { d, p };
  }
  return `${worst.d.toFixed(5)} at (${worst.p.x.toFixed(4)}, ${worst.p.y.toFixed(4)})`;
};
for (const arg of process.argv.slice(2).filter((a) => !a.startsWith("--"))) {
  const asset = (await Bun.file(fileOf(arg)).json()).asset;
  const bands = Object.keys(asset.representations).filter((key) => authoredOf(asset, key) !== undefined);
  const shapes = Object.fromEntries(bands.map((key) => [key, evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(key) } }).shapes]));
  console.log(`== ${asset.id} (default pose ${asset.presentation.defaultPose}; in-band limit ${NEAR_MATCH}, copy limit ${COPY})`);
  for (const key of bands) {
    const sets: Record<string, P[] | undefined> = {}, sides: Record<string, ReturnType<typeof sideOf>> = {};
    for (const view of ["elevation", "front", "plan"]) { sets[view] = pointSet(shapes[key][view]); sides[view] = sideOf(shapes[key][view]); }
    console.log(`1:${key} ` + ["elevation", "front", "plan"].map((v) => `${v} ${sides[v]!.w.toFixed(3)} × ${sides[v]!.h.toFixed(3)} m, ${sides[v]!.n} ring points, ${sets[v]?.length ?? 0} sampled`).join(" | "));
    for (const [view, against] of [["front", "elevation"], ["plan", "elevation"], ["plan", "front"]] as const) {
      const a = sets[view], b = sets[against];
      if (a === undefined || b === undefined) continue;
      const best = bestTransform(a, b);
      console.log(`   ${view} against ${against}: ${best.distance.toFixed(5)} under ${best.name} (${(best.distance * sides[view]!.side * 1000).toFixed(0)} mm of ${view}, limit ${(NEAR_MATCH * sides[view]!.side * 1000).toFixed(0)} mm)${best.distance < NEAR_MATCH ? "  FLAGGED" : ""}`);
      if (far) {
        const m = TRANSFORMS.find((t: any) => t.name === best.name).m;
        const moved = b.map((p) => ({ x: m[0] * p.x + m[1] * p.y, y: m[2] * p.x + m[3] * p.y }));
        console.log(`      farthest of ${view}: ${farthest(a, moved)}; farthest of ${against}: ${farthest(moved, a)}`);
      }
    }
    for (const other of bands) {
      if (other === key) continue;
      for (const view of ["front", "plan"]) {
        const a = sets[view], b = pointSet(shapes[other][view]);
        if (a === undefined || b === undefined) continue;
        const d = hausdorff(a, b);
        if (d < 4 * COPY) console.log(`   ${view} against 1:${other} ${view}: ${d.toFixed(5)}${d < COPY ? "  FLAGGED" : ""}`);
      }
    }
  }
}
