/** 🧪️ Ticket-local validation of an authored volume: Ajv schema conformance, owned `validateAsset` volume rules and pose checks (bounding box and detachment by solid-box connectivity).
 * Usage: bun authoring-components-validate.ts <slug> [poseId ...] */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { tessellateSolid } from "../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { validateAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🟦️.ts";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { QUALITY_FOR, STAFFAGE, frames, loadAsset, resolveAsset, toWorld, type V3 } from "./volume-lib.ts";

const slug = process.argv[2]!;
const entry = resolveAsset(slug);
const asset = loadAsset(entry.path);
const schema = JSON.parse(readFileSync(join(STAFFAGE, "🧬️schema/🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true, allErrors: true });
ajv.addSchema(schema);
const check = ajv.compile({ $ref: `${schema.$id}#/$defs/Asset` });
const okSchema = check(asset);
const schemaErrors = okSchema ? [] : (check.errors ?? []).slice(0, 8).map((e) => `${e.instancePath} ${e.message}`);
const issues = validateAsset(asset).filter((i: any) => i.code.startsWith("volume-"));
console.log(`${slug}: schema ${okSchema ? "ok" : "FAIL"} ${schemaErrors.join(" | ")}; volume issues ${issues.length}`);
for (const i of issues) console.log("  ", JSON.stringify(i));

const maxDim = Math.max(asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM);
const quality = QUALITY_FOR(maxDim);
const local = asset.volume.solids.map((s: any) => {
  const g = tessellateSolid({ ...s, node: s.bone } as never, quality);
  return { id: s.id as string, bone: s.bone as string, pos: g.positions as ArrayLike<number> };
});
const poses: string[] = [asset.poses[0].id, ...(process.argv.length > 3 ? process.argv.slice(3) : [asset.poses[1]?.id].filter(Boolean))];
const tol = 0.006;
type Box = { id: string; min: V3; max: V3 };
function boxes(poseId: string | null): Box[] {
  const state = poseId === null ? null : evaluate(asset, { kind: "pose", pose: poseId } as never, { kind: "scale", scale: { denominator: 50 } } as never);
  return local.map((l: any) => {
    const g = (state as any)?.bones[l.bone].frame;
    const a = (q: any): V3 => [q.x, q.y, q.z];
    const frame = g ? { o: a(g.o), u: a(g.u), v: a(g.v), w: a(g.w) } : frames(asset)[l.bone]!;
    const min: V3 = [Infinity, Infinity, Infinity];
    const max: V3 = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < l.pos.length; i += 3) {
      const p = toWorld(frame, [l.pos[i], l.pos[i + 1], l.pos[i + 2]]);
      for (let k = 0; k < 3; k++) { min[k] = Math.min(min[k]!, p[k]!); max[k] = Math.max(max[k]!, p[k]!); }
    }
    return { id: l.id, min, max };
  });
}
function components(list: Box[]): string[][] {
  const parent = list.map((_, i) => i);
  const find = (i: number): number => (parent[i] === i ? i : (parent[i] = find(parent[i]!)));
  for (let i = 0; i < list.length; i++) for (let j = i + 1; j < list.length; j++) {
    const a = list[i]!, b = list[j]!;
    if ([0, 1, 2].every((k) => a.min[k]! <= b.max[k]! + tol && b.min[k]! <= a.max[k]! + tol)) parent[find(i)] = find(j);
  }
  const groups = new Map<number, string[]>();
  list.forEach((b, i) => { const r = find(i); groups.set(r, [...(groups.get(r) ?? []), b.id]); });
  return [...groups.values()];
}
const extent = (list: Box[]) => [0, 1, 2].map((k) => (Math.max(...list.map((b) => b.max[k]!)) - Math.min(...list.map((b) => b.min[k]!))).toFixed(3));
const rest = components(boxes(null));
console.log(`  rest: ${rest.length} component(s), extent ${extent(boxes(null)).join(" x ")}`);
if (rest.length > 1) for (const c of rest) console.log("    component: " + (c.length > 6 ? c.slice(0, 6).join(",") + ",... (" + c.length + ")" : c.join(",")));
for (const id of poses) {
  const list = boxes(id);
  const comp = components(list);
  const lone = comp.length > rest.length ? comp.filter((c) => c.length < 3).map((c) => c.join("+")).slice(0, 8) : [];
  if (comp.length !== rest.length) for (const c of comp) console.log("    component: " + (c.length > 6 ? c.slice(0, 6).join(",") + ",... (" + c.length + ")" : c.join(",")));
  console.log(`  pose ${id}: ${comp.length} component(s)${comp.length > rest.length ? " DETACHED? " + lone.join(",") : ""}, extent ${extent(list).join(" x ")}, lowest y ${Math.min(...list.map((b) => b.min[1])).toFixed(3)}`);
}
