/** 👁️ What the depth plates of a plant show beside its drawn outlines: `bun visible.ts <id|file.json>… [--prefix mvc-]` evaluates every pose but the default one at every authored scale and prints, per view that holds plates, the pieces of the plates outside the union of all other filled shapes.
 *
 * Per sheet (scale, view, pose): the number of pieces of at least (0.3 mm on paper) squared, their share of the silhouette area in percent, the slivers among them (mean thickness 2 × area / perimeter below 0.6 mm on paper), the islands (pieces that touch no other filled shape: a detached cut), and the longest straight edge on the free boundary in millimetres on paper. The last line per plant is its worst sheet, by the free area of its pieces on paper.
 * @see ./carry.ts — the plates
 * @see ./look.sh — the sheets */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const clipping: any = (await import(Bun.resolveSync("polygon-clipping", `${join(S, "🧪️tests", "🧊️catalog-silhouette", "🔍️oracles")}/`))).default;

type P = [number, number];
const args = process.argv.slice(2);
const at = args.indexOf("--prefix");
const PREFIX = at < 0 ? "mvc-" : args[at + 1]!;
const areaOf = (ring: readonly P[]): number => Math.abs(ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0)) / 2;

function thin(ring: readonly P[], eps: number): P[] {
  const pts = ring.slice() as P[];
  if (pts.length > 1 && pts[0]![0] === pts[pts.length - 1]![0] && pts[0]![1] === pts[pts.length - 1]![1]) pts.pop();
  if (pts.length < 5) return pts;
  let far = 0;
  pts.forEach((p, i) => { if (Math.hypot(p[0] - pts[0]![0], p[1] - pts[0]![1]) > Math.hypot(pts[far]![0] - pts[0]![0], pts[far]![1] - pts[0]![1])) far = i; });
  const line = (run: P[]): P[] => {
    const keep = new Uint8Array(run.length);
    keep[0] = keep[run.length - 1] = 1;
    const stack: P[] = [[0, run.length - 1]];
    while (stack.length > 0) {
      const [a, b] = stack.pop()!;
      const [p, q] = [run[a]!, run[b]!];
      const length = Math.hypot(q[0] - p[0], q[1] - p[1]) || 1e-12;
      let worst = eps, hit = -1;
      for (let i = a + 1; i < b; i++) {
        const d = Math.abs((q[0] - p[0]) * (run[i]![1] - p[1]) - (q[1] - p[1]) * (run[i]![0] - p[0])) / length;
        if (d > worst) { worst = d; hit = i; }
      }
      if (hit >= 0) { keep[hit] = 1; stack.push([a, hit], [hit, b]); }
    }
    return run.filter((_, i) => keep[i] === 1);
  };
  return [...line(pts.slice(0, far + 1)).slice(0, -1), ...line([...pts.slice(far), pts[0]!]).slice(0, -1)];
}

const union = (list: P[][][][]): P[][][] => {
  let level = list;
  while (level.length > 1) level = Array.from({ length: Math.ceil(level.length / 2) }, (_, i) => (level[2 * i + 1] === undefined ? level[2 * i]! : clipping.union(level[2 * i], level[2 * i + 1])));
  return level[0] ?? [];
};

for (const arg of args.filter((word, i) => !word.startsWith("--") && (at < 0 || i !== at + 1))) {
  const asset = readDoc(arg).doc.asset;
  let worst = { area: 0, text: "no plate shows beside the drawn outlines" };
  for (const [key, decision] of Object.entries(asset.representations) as [string, any][]) {
    if (decision.status !== "authored") continue;
    const paper = Number(key) / 1000;
    for (const view of ["front", "plan"]) {
      if (!decision.geometry[view].some((shape: any) => String(shape.id).startsWith(PREFIX))) continue;
      for (const pose of asset.poses) {
        if (pose.id === asset.presentation.defaultPose) continue;
        const state: any = evaluate(asset, { kind: "pose", pose: pose.id }, engine.scaleSelector(Number(key)));
        const [own, rest]: [P[][][][], P[][][][]] = [[], []];
        for (const s of state.shapes[view]) {
          if (s.shape.filled !== true || s.shape.role === "ground" || s.closed !== true) continue;
          const multi = s.rings.map((ring: any[]) => thin(ring.map((p) => [Math.round(p.x * 1e5) / 1e5, Math.round(p.y * 1e5) / 1e5] as P), 0.05 * paper)).filter((ring: P[]) => ring.length > 2 && areaOf(ring) > 1e-9).map((ring: P[]) => [ring]);
          if (multi.length > 0) (String(s.shape.id).startsWith(PREFIX) ? own : rest).push(...multi.map((polygon: P[][]) => [polygon]));
        }
        let pieces: P[][][] = [], sheet: P[][][] = [];
        try { sheet = union(rest); pieces = clipping.difference(union(own), sheet); } catch (error) { console.log(`${asset.id} 1:${key} ${view} ${pose.id}: not measured (${String(error).slice(0, 60)})`); continue; }
        const free = pieces.filter((polygon) => areaOf(polygon[0]!) >= (0.3 * paper) ** 2);
        const total = sheet.reduce((sum, polygon) => sum + areaOf(polygon[0]!), 0);
        const borders = sheet.flat().flatMap((ring) => ring.map((p, i) => [p, ring[(i + 1) % ring.length]!] as [P, P])).filter(([p, q]) => Math.hypot(p[0] - q[0], p[1] - q[1]) > 1e-9);
        const onSheet = (m: P): boolean => borders.some(([p, q]) => {
          const l = (q[0] - p[0]) ** 2 + (q[1] - p[1]) ** 2;
          const t = Math.min(1, Math.max(0, ((m[0] - p[0]) * (q[0] - p[0]) + (m[1] - p[1]) * (q[1] - p[1])) / l));
          return Math.hypot(m[0] - p[0] - t * (q[0] - p[0]), m[1] - p[1] - t * (q[1] - p[1])) < 0.02 * paper;
        });
        let [slivers, islands, edge, area] = [0, 0, 0, 0];
        for (const polygon of free) {
          const ring = polygon[0]!;
          const a = areaOf(ring);
          area += a;
          const around = ring.reduce((sum, p, i) => sum + Math.hypot(ring[(i + 1) % ring.length]![0] - p[0], ring[(i + 1) % ring.length]![1] - p[1]), 0);
          if ((2 * a) / around < 0.6 * paper) slivers++;
          let touches = false;
          for (let i = 0; i < ring.length; i++) {
            const [p, q] = [ring[i]!, ring[(i + 1) % ring.length]!];
            if (onSheet([(p[0] + q[0]) / 2, (p[1] + q[1]) / 2])) touches = true;
            else edge = Math.max(edge, Math.hypot(q[0] - p[0], q[1] - p[1]) / paper);
          }
          if (!touches) islands++;
        }
        const line = `1:${key} ${view} ${pose.id}: ${free.length} pieces, ${((100 * area) / Math.max(total, 1e-9)).toFixed(2)} % of the silhouette, ${slivers} slivers, ${islands} islands, longest straight free edge ${edge.toFixed(1)} mm on paper`;
        console.log(`${asset.id} ${line}`);
        if (area / paper ** 2 > worst.area) worst = { area: area / paper ** 2, text: `${line}, ${(area / paper ** 2).toFixed(0)} mm² on paper` };
      }
    }
  }
  console.log(`WORST ${asset.id} ${worst.text}`);
}
