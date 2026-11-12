import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { catalogEntries, loadAsset, toWorld, projectWorld, type Json, type V3 } from "./volume-lib.ts";
import { localMeshes } from "./lateral-posed.ts";
import { select } from "./lateral-snap.ts";
const [slug, beforeFile, scale, view, pose, ...ids] = process.argv.slice(2);
const before = JSON.parse(readFileSync(beforeFile!, "utf8")).asset as Json;
const a = loadAsset(catalogEntries().find((x) => x.slug === slug)!.path);
const q = { kind: "pose", pose: pose || a.presentation.defaultPose } as never;
const sb = evaluate(before, q, select(Number(scale))) as Json, sa = evaluate(a, q, select(Number(scale))) as Json;
const arr = (p: Json) => [p.x, p.y, p.z] as V3;
const ext = (pts: { x: number; y: number }[]) => pts.length ? [Math.min(...pts.map((p) => p.x)), Math.max(...pts.map((p) => p.x)), Math.min(...pts.map((p) => p.y)), Math.max(...pts.map((p) => p.y))].map((x) => +x.toFixed(3)) : [];
for (const t of localMeshes(a)) {
  if (ids.length && !ids.includes(t.id)) continue;
  const f = sa.bones[t.bone].frame; const fr = { o: arr(f.o), u: arr(f.u), v: arr(f.v), w: arr(f.w) };
  const pts: { x: number; y: number }[] = [];
  for (let i = 0; i < t.local.length; i += 3) pts.push(projectWorld(view as never, toWorld(fr, [t.local[i]!, t.local[i + 1]!, t.local[i + 2]!])));
  const find = (s: Json) => (s.shapes[view] as Json[]).filter((x) => x.shape.id === t.id).flatMap((x) => x.rings.flat());
  console.log(t.id.padEnd(16), "volume", ext(pts), " before", ext(find(sb)), " after", ext(find(sa)));
}
