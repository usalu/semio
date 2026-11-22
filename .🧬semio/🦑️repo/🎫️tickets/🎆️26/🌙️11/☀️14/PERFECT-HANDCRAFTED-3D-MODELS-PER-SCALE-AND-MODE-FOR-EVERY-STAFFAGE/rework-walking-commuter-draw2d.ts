/** ✏️ Redraws the filled shapes of the 2D drawings of `humans/walking-commuter` that the reworked body invalidates: each one becomes the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement. Layer, tone, region, hatch and pen of every shape stay as drawn; which solids one shape unites is stated below. Strokes and the slender strap keep their drawn geometry.
 *
 * Usage: `bun rework-walking-commuter-draw2d.ts` reads and rewrites the candidate `🗑️generated/rework-walking-commuter/🔣️.json`.
 * @see ./rework-walking-commuter-build.ts */
import { join } from "node:path";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt } from "./r3-lib.ts";
import { area, GENERATED, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { readJsonFile, writeJsonFile } from "./rework-04-kit.ts";

const path = join(GENERATED, "rework-walking-commuter", "🔣️.json");
const doc = readJsonFile(path);
const asset = doc.asset as Json;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
/** 🧩 The drawn shape a body solid belongs to when no shape carries its own id. */
function groupOf(id: string, view: View, drawn: Set<string>): string | null {
  if (drawn.has(id)) return id;
  const hand = /^hand-([lr])-/.exec(id);
  if (hand) return `hand-${hand[1]}`;
  if (/^coat-(waist|skirt-[lr]|centre)$/.test(id)) return "coat";
  const sole = /^shoe-(heel|sole)-([lr])$/.exec(id);
  if (sole) return view === "plan" ? `shoe-rear-${sole[2]}` : `shoe-sole-${sole[2]}`;
  const toeSole = /^shoe-toe-sole-([lr])$/.exec(id);
  if (toeSole) return `shoe-toe-${toeSole[1]}`;
  if (/^head-(nose|chin|lips)$/.test(id)) return "head";
  const sock = /^sock-([lr])$/.exec(id);
  if (sock && view === "plan") return `shank-${sock[1]}`;
  if (view !== "front" && /^(lapel-[lr]|scarf-front|coat-button-\d|coat-pocket-[lr])$/.test(id)) return "coat";
  return null;
}
const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
for (const scale of [50, 100, 200, 500]) {
  const frames = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose }).frames;
  const body: Json[] = asset.volumes[String(scale)].body;
  const tol = TOL[scale]!;
  const cell = tol / 3;
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const shapes = asset.representations[String(scale)].geometry[view] as Json[];
    const drawn = new Set(shapes.filter((s) => s.filled === true && s.closed === true).map((s) => s.id as string));
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      if (s.slender === true) continue;
      const g = groupOf(s.id, view, drawn);
      if (g !== null && drawn.has(g)) (groups.get(g) ?? groups.set(g, []).get(g)!).push(s);
    }
    const jobs: { shape: Json; meta: Json; target: P2[] }[] = [];
    for (const [id, solids] of groups) {
      const shape = shapes.find((s) => s.id === id)!;
      const mask = projectMask(view, meshesIn(asset, frames, solids), cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const { kind: _kind, points: _points, closed: _closed, ...meta } = shape;
      jobs.push({ shape, meta, target: simplifyRing(rings[0]!, tol) });
    }
    const solved = inverse(asset, scale, view, jobs);
    jobs.forEach((j, k) => {
      if (solved[k]!.residualM > 1e-5 || solved[k]!.edgeOn) console.log(`  [DEBUG] ${scale} ${view} ${j.meta.id}: residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm edgeOn ${solved[k]!.edgeOn}`);
      j.shape.points = solved[k]!.points.map((p) => [r4(p.x), r4(p.y)]);
    });
    console.log(`[DEBUG] ${scale} ${view}: ${jobs.length} of ${shapes.length} shapes redrawn; kept ${shapes.filter((s) => !groups.has(s.id)).map((s) => s.id).join(" ")}`);
  }
}
writeJsonFile(path, doc);
