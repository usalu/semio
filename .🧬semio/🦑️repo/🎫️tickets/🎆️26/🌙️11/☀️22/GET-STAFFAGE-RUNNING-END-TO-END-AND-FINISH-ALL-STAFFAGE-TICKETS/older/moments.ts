/** 🏗️ Lists what the moments view lets a reader read per member in the three views of one pose: `bun moments.ts <category-folder>/<asset> <pose> [mode]` prints every valued path with a bone, per view, and the invariant peak of the diagram. */
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

const root = resolve(import.meta.dir, "../../../../../../../..");
const staffage = join(root, "🎓️teaching/🏛️architecture/🧍️staffage");
const { resolveViews } = await import(join(staffage, "🔨️modules/🔭️multiview/🟦️.ts"));
const { drawOptions } = await import(join(staffage, "🧪️tests/🔭️multiview-coverage/🧰️.ts"));
const { UNCHECKED_PROFILE } = await import(join(staffage, "🧫️fixtures/🧱️synthetic-figure/🟦️.ts"));
const [path, pose, mode = "moments"] = process.argv.slice(2);
const { asset } = JSON.parse(readFileSync(join(staffage, "🗂️catalog", path!, "🔣️.json"), "utf8"));
const multi = resolveViews(asset, { kind: "pose", pose: pose ?? asset.presentation.defaultPose }, { kind: "scale", denominator: 50 }, mode, { ...drawOptions(mode, 50), profile: UNCHECKED_PROFILE });
const members = multi.analysis.structure?.members ?? {};
let peak = 0;
for (const member of Object.values(members) as { stations: { momentVector: { x: number; y: number; z: number } }[] }[]) for (const s of member.stations) peak = Math.max(peak, Math.hypot(s.momentVector.x, s.momentVector.y, s.momentVector.z));
console.log(`${asset.id} ${mode}: invariant moment peak ${peak.toPrecision(6)} N·m`);
for (const view of ["elevation", "front", "plan"] as const) {
  const drawn = multi.views[view];
  if (drawn.kind !== "drawing") continue;
  const rows: string[] = [];
  for (const layer of drawn.scene.layers) for (const item of layer.items) if (item.kind === "path" && item.value !== undefined && item.bone) rows.push(`${item.id}=${item.value.value.toPrecision(5)}`);
  console.log(`${view}: ${rows.join(" ")}`);
}
