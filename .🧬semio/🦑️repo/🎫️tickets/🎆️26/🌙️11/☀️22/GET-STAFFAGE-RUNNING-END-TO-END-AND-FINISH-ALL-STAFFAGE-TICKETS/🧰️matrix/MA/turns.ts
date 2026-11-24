/** 🧭️ How far every bone of an asset turns from its rest frame over every subject of the matrix (poses, clip instants, transitions): `bun turns.ts <id|file.json> [--json]` prints per bone the range of the sagittal angle of its axis against rest (degrees, about the lateral axis), the largest tilt of its lateral axis out of the rest direction (spread, twist) and the subject of each; `--json` prints `{ bone: { min, max, lateral } }` for `fit.ts --turns`. Run it from `T`. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const asset = readDoc(process.argv[2]!).doc.asset;
const rest = restFrames(asset.rig) as Record<string, { frame: { u: number[]; v: number[]; w: number[] } }>;
const key = Object.keys(asset.representations).find((candidate) => engine.authoredOf(asset, candidate) !== undefined)!;
const found: Record<string, { min: number; max: number; lateral: number; minAt: string; maxAt: string; lateralAt: string }> = {};
for (const subject of engine.subjects(asset, Number(key))) {
  let state: any;
  try {
    state = evaluate(asset, subject.query, engine.scaleSelector(Number(key)));
  } catch {
    continue;
  }
  for (const [bone, value] of Object.entries(state.bones) as [string, any][]) {
    const [now, was] = [value.frame, rest[bone]!.frame];
    const angle = ((Math.atan2(now.u.y, now.u.x) - Math.atan2(was.u[1]!, was.u[0]!)) * 180) / Math.PI;
    const sagittal = ((angle + 540) % 360) - 180;
    const lateral = (Math.acos(Math.min(1, Math.abs(now.w.x * was.w[0]! + now.w.y * was.w[1]! + now.w.z * was.w[2]!))) * 180) / Math.PI;
    const entry = (found[bone] ??= { min: 0, max: 0, lateral: 0, minAt: "", maxAt: "", lateralAt: "" });
    if (sagittal < entry.min) Object.assign(entry, { min: sagittal, minAt: subject.id });
    if (sagittal > entry.max) Object.assign(entry, { max: sagittal, maxAt: subject.id });
    if (lateral > entry.lateral) Object.assign(entry, { lateral, lateralAt: subject.id });
  }
}
if (process.argv.includes("--json")) console.log(JSON.stringify(Object.fromEntries(Object.entries(found).map(([bone, e]) => [bone, { min: Number(e.min.toFixed(1)), max: Number(e.max.toFixed(1)), lateral: Number(e.lateral.toFixed(1)) }]))));
else for (const [bone, e] of Object.entries(found)) console.log(`${asset.id} ${bone}: sagittal ${e.min.toFixed(0)}..${e.max.toFixed(0)} deg (${e.minAt} | ${e.maxAt}), lateral ${e.lateral.toFixed(0)} deg${e.lateral >= 1 ? ` (${e.lateralAt})` : ""}`);
