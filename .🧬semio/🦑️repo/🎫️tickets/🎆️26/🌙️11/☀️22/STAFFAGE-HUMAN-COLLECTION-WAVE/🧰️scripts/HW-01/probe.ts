/** 🔎️ Prints bone origins, tips and frame axes, anchors, rig and drawn bounds of one asset pose or clip instant.
 *  bun probe.ts <slug|path.json> [pose|clip:t] [bone ...] */
import { join, resolve } from "node:path";
import { evaluate, rigBounds } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const catalog = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans");
const [target, which, ...only] = process.argv.slice(2);
const path = target!.endsWith(".json") ? target! : join(catalog, target!, "🔣️.json");
const asset = (await Bun.file(path).json()).asset;
const query = which === undefined || which === "-" ? { kind: "pose" as const, pose: asset.presentation.defaultPose } : which.includes(":") ? { kind: "clip" as const, clip: which.split(":")[0]!, timeS: Number(which.split(":")[1]) } : { kind: "pose" as const, pose: which };
const f = (n: number) => n.toFixed(4);
const v = (p: { x: number; y: number; z: number }) => `(${f(p.x)} ${f(p.y)} ${f(p.z)})`;
for (const scale of [50] as const) {
  const state = evaluate(asset, query, { kind: "scale", scale: { denominator: scale } } as never);
  for (const [id, b] of Object.entries(state.bones)) {
    if (only.length > 0 && !only.includes(id)) continue;
    console.log(`${id.padEnd(22)} o${v(b.frame.o)} tip${v(b.tip)} u${v(b.frame.u)} v${v(b.frame.v)} w${v(b.frame.w)}`);
  }
  for (const [id, a] of Object.entries(state.anchors)) console.log(`anchor ${id.padEnd(16)} ${f(a.at.x)} ${f(a.at.y)} ${f(a.z)}`);
  const r = rigBounds(state);
  console.log(`rig x ${f(r.minX)}..${f(r.maxX)} y ${f(r.minY)}..${f(r.maxY)}  drawn x ${f(state.bounds.minX)}..${f(state.bounds.maxX)} y ${f(state.bounds.minY)}..${f(state.bounds.maxY)}  root ${JSON.stringify(state.root)} contacts ${state.contacts.join(",")}`);
  for (const vp of ["elevation", "front", "plan"] as const) {
    let a = Infinity, b = -Infinity, c = Infinity, d = -Infinity;
    for (const s of state.shapes[vp]) for (const ring of s.rings) for (const p of ring) { a = Math.min(a, p.x); b = Math.max(b, p.x); c = Math.min(c, p.y); d = Math.max(d, p.y); }
    console.log(`${vp} x ${f(a)}..${f(b)} (${f(b - a)}) y ${f(c)}..${f(d)} (${f(d - c)})`);
  }
}
