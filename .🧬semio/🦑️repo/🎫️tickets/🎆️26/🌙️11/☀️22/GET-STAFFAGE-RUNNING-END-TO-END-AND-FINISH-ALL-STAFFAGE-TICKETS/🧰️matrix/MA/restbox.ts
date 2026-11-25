/** 🧱️ Guards the rest drawing of a repaired copy against its backup: `bun restbox.ts <backup.json> <copy.json> [--drop <ops.json>] [--limit <mm>]` (run it from `T`) compares, per authored scale, the edges the three views share in the default pose (height: elevation y and front y; forward: elevation x and plan x; lateral: front x and plan y) and prints every edge whose disagreement between two views grew by more than the limit (default 2 mm) against the backup, with the changed shapes of the copy that lie on that edge. With `--drop` the ops of those shapes (same scale, view and id; `-nest-<n>` children count for their parent) are removed from the ops file; exit code 1 when an edge is worse.
 * @see ./nsettle.ts — the guard loop of the other checks */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const engine = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🧰️.ts"));
const args = process.argv.slice(2);
const option = (name: string): string | undefined => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : undefined);
const limit = Number(option("limit") ?? 2) / 1000;
const before = readDoc(args[0]!).doc.asset;
const after = readDoc(args[1]!).doc.asset;
const base = (name: string): string => name.replace(/-nest-\d+$/u, "");
type Box = { lo: number; hi: number };
type Drawn = { id: string; bone: string; x: Box; y: Box };
const PAIRS = [
  { name: "height", a: ["elevation", "y"], b: ["front", "y"] },
  { name: "forward", a: ["elevation", "x"], b: ["plan", "x"] },
  { name: "lateral", a: ["front", "x"], b: ["plan", "y"] },
] as const;

function drawn(asset: any, scale: string): Record<string, Drawn[]> {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, engine.scaleSelector(Number(scale)));
  const out: Record<string, Drawn[]> = {};
  for (const view of ["elevation", "front", "plan"])
    out[view] = (state.shapes[view] as any[]).flatMap((entry) => {
      const points = entry.rings.flat() as { x: number; y: number }[];
      if (points.length === 0) return [];
      const box = (axis: "x" | "y"): Box => ({ lo: Math.min(...points.map((p) => p[axis])), hi: Math.max(...points.map((p) => p[axis])) });
      return [{ id: entry.shape.id as string, bone: entry.shape.bone as string, x: box("x"), y: box("y") }];
    });
  return out;
}
const extent = (list: readonly Drawn[], axis: "x" | "y"): Box => ({ lo: Math.min(...list.map((d) => d[axis].lo)), hi: Math.max(...list.map((d) => d[axis].hi)) });

const culprits: { scale: string; view: string; id: string }[] = [];
for (const scale of Object.keys(after.representations).filter((key) => engine.authoredOf(after, key) !== undefined)) {
  const [was, now] = [drawn(before, scale), drawn(after, scale)];
  for (const pair of PAIRS)
    for (const end of ["lo", "hi"] as const) {
      const gap = (set: Record<string, Drawn[]>): number => Math.abs(extent(set[pair.a[0]]!, pair.a[1])[end] - extent(set[pair.b[0]]!, pair.b[1])[end]);
      const [old, next] = [gap(was), gap(now)];
      if (next <= Math.max(old, limit) + 0.0005) continue;
      const named: string[] = [];
      for (const [view, axis] of [pair.a, pair.b]) {
        const [edgeNow, edgeWas] = [extent(now[view]!, axis)[end], extent(was[view]!, axis)[end]];
        if (Math.abs(edgeNow - edgeWas) < 0.0005) continue;
        for (const shape of now[view]!.filter((d) => Math.abs(d[axis][end] - edgeNow) < 0.001)) {
          const twin = was[view]!.find((d) => d.id === shape.id && d.bone === shape.bone);
          if (twin !== undefined && Math.abs(twin[axis][end] - shape[axis][end]) < 0.0005) continue;
          named.push(`${view} ${shape.id}`);
          culprits.push({ scale, view, id: base(shape.id) });
        }
      }
      console.log(`1:${scale} ${pair.name} ${end}: ${pair.a[0]} and ${pair.b[0]} differ by ${(next * 1000).toFixed(1)} mm (backup ${(old * 1000).toFixed(1)} mm): ${named.join(", ") || "no changed shape on the edge"}`);
    }
}
const file = option("drop");
if (file !== undefined && culprits.length > 0) {
  const ops = JSON.parse(readFileSync(file, "utf8")) as any[];
  const next = ops.filter((op) => !culprits.some((hit) => hit.scale === op.scale && hit.view === (op.view ?? op.to) && hit.id === op.id));
  writeFileSync(file, JSON.stringify(next, null, 1));
  console.log(`${ops.length - next.length} ops removed from ${file}`);
}
if (culprits.length === 0) console.log("rest edges: as good as the backup");
process.exit(culprits.length > 0 ? 1 : 0);
