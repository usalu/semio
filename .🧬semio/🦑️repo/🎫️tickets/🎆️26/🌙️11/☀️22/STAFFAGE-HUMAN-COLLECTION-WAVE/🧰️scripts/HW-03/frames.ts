/** 🦴️ Prints the world frames of chosen bones of an asset in a pose (or at a clip time), optionally with pose overrides as JSON.
 *  bun frames.ts <file.json> <pose> [bone,bone] [--clip id --time s] [--override '{"arm-l":{"angleDeg":-30}}'] */
import { parseArgs } from "node:util";
import { S } from "./lib.ts";
const { evaluate } = await import(`${S}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { values, positionals } = parseArgs({ allowPositionals: true, options: { clip: { type: "string" }, time: { type: "string" }, override: { type: "string" } } });
const [file, pose, list] = positionals;
const asset = (await Bun.file(file!).json()).asset;
if (values.override) {
  const p = asset.poses.find((x: { id: string }) => x.id === pose);
  for (const [k, v] of Object.entries(JSON.parse(values.override))) p.bones[k] = { ...(p.bones[k] ?? {}), ...(v as object) };
}
const state = evaluate(asset, values.clip ? { kind: "clip", clip: values.clip, timeS: Number(values.time ?? 0) } : { kind: "pose", pose: pose! });
const f = (v: { x: number; y: number; z: number }): string => `(${v.x.toFixed(3)}, ${v.y.toFixed(3)}, ${v.z.toFixed(3)})`;
for (const id of (list ?? Object.keys(state.bones).join(",")).split(",")) {
  const b = state.bones[id];
  if (!b) continue;
  console.log(`${id.padEnd(16)} o ${f(b.frame.o)} tip ${f(b.tip)} u ${f(b.frame.u)} v ${f(b.frame.v)} w ${f(b.frame.w)}`);
}
for (const [id, x] of Object.entries(state.anchors) as [string, { at: { x: number; y: number }; z: number }][]) console.log(`anchor ${id} (${x.at.x.toFixed(3)}, ${x.at.y.toFixed(3)}, ${x.z.toFixed(3)})`);
