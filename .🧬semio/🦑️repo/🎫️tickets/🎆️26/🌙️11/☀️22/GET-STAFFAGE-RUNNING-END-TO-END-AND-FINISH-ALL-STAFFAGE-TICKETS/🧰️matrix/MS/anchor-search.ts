/** ⚓️ Finds where an anchor stays on the drawn shapes of its bone: `bun anchor-search.ts <tests-dir> <id> <anchor> [reach=0.6] [step=0.15]` puts candidates on a grid of `step` metres up to `reach` around the anchor in the frame of its bone, evaluates every pose at every authored scale once and measures each candidate as the matrix measures an anchor (distance to the shapes of the bone and of its descendant bones in every view that draws the bone). It prints the anchor as it is and the candidates nearest to it whose worst distance is at most 0.4 of the tolerance. */
import { join, resolve } from "node:path";

const tests = resolve(process.argv[2]!);
const [id, anchorId] = [process.argv[3]!, process.argv[4]!];
const reach = Number(process.argv[5] ?? 0.6), step = Number(process.argv[6] ?? 0.15);
const engine = await import(join(tests, "🔭️multiview-coverage", "🧰️.ts"));
const { distanceToShapes, subtrees } = await import(join(tests, "🔭️multiview-coverage", "⚓️landmarks.ts"));
const { evaluate } = await import(join(tests, "..", "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const asset = structuredClone(engine.loadCatalog().find((x: any) => x.id === id));
const anchor = asset.anchors.find((a: any) => a.id === anchorId);
const tolerance = engine.toleranceM(asset);
const n = Math.round(reach / step);
const candidates: { id: string; dx: number; dy: number; dz: number }[] = [];
for (let i = -n; i <= n; i++) for (let j = -n; j <= n; j++) for (let k = -n; k <= n; k++) {
  const c = { id: `cand-${candidates.length}`, dx: i * step, dy: j * step, dz: k * step };
  candidates.push(c);
  asset.anchors.push({ ...anchor, id: c.id, x: anchor.x + c.dx, y: anchor.y + c.dy, z: (anchor.z ?? 0) + c.dz });
}
const worst = new Map<string, { d: number; where: string }>();
const subtree = subtrees(asset.rig.bones)(anchor.bone);
for (const key of Object.keys(asset.representations).filter((k) => engine.authoredOf(asset, k) !== undefined))
  for (const pose of asset.poses) {
    const state = evaluate(asset, { kind: "pose", pose: pose.id }, engine.scaleSelector(Number(key)));
    for (const view of ["elevation", "front", "plan"] as const) {
      const shapes = state.shapes[view].filter((s: any) => s.shape.role !== "ground" && subtree.has(s.shape.bone));
      if (!state.shapes[view].some((s: any) => s.shape.role !== "ground" && s.shape.bone === anchor.bone)) continue;
      for (const c of [{ id: anchorId }, ...candidates]) {
        const { at, z } = state.anchors[c.id];
        const p = view === "elevation" ? { x: at.x, y: at.y } : view === "front" ? { x: z, y: at.y } : { x: at.x, y: z };
        const d = distanceToShapes(p, shapes);
        if (Number.isFinite(d) && d > (worst.get(c.id)?.d ?? -1)) worst.set(c.id, { d, where: `1:${key} ${pose.id} ${view}` });
      }
    }
  }
console.log(`${id} ${anchorId} on ${anchor.bone} at (${anchor.x}, ${anchor.y}, ${anchor.z ?? 0}): worst ${(worst.get(anchorId)!.d * 1000).toFixed(0)} mm at ${worst.get(anchorId)!.where}; tolerance ${(tolerance * 1000).toFixed(0)} mm`);
const good = candidates.filter((c) => worst.get(c.id)!.d <= 0.4 * tolerance).sort((a, b) => Math.hypot(a.dx, a.dy, a.dz) - Math.hypot(b.dx, b.dy, b.dz));
for (const c of good.slice(0, 6)) console.log(`  (${(anchor.x + c.dx).toFixed(4)}, ${(anchor.y + c.dy).toFixed(4)}, ${((anchor.z ?? 0) + c.dz).toFixed(4)}) moved ${(Math.hypot(c.dx, c.dy, c.dz) * 1000).toFixed(0)} mm: worst ${(worst.get(c.id)!.d * 1000).toFixed(0)} mm at ${worst.get(c.id)!.where}`);
console.log(`${good.length} of ${candidates.length} candidates hold`);
