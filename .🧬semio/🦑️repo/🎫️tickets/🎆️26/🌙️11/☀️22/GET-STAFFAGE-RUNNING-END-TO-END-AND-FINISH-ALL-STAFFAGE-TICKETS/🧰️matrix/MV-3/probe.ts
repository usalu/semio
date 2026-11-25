/** 🔬️ Timing and convention probe: `bun probe.ts <id>` prints the subjects of the matrix per scale, the time of one evaluation and whether an anchor is `o + x·u + y·v + z·w` of the current frame of its bone. */
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";
const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const asset = readDoc(process.argv[2]!).doc.asset;
for (const key of Object.keys(asset.representations).filter((k) => engine.authoredOf(asset, k) !== undefined)) {
  const all = engine.subjects(asset, Number(key));
  const kinds: Record<string, number> = {};
  for (const s of all) kinds[s.query.kind] = (kinds[s.query.kind] ?? 0) + 1;
  const t0 = performance.now();
  const state = evaluate(asset, all[all.length - 1].query, engine.scaleSelector(Number(key)));
  const t1 = performance.now();
  const a = asset.anchors.find((x: any) => x.kind === "emitter");
  const f = state.bones[a.bone].frame;
  const p = [0, 1, 2].map((i) => f.o[i] + a.x * f.u[i] + a.y * f.v[i] + (a.z ?? 0) * f.w[i]);
  console.log(`[DEBUG] 1:${key} subjects ${all.length} ${JSON.stringify(kinds)} eval ${(t1 - t0).toFixed(0)} ms; anchor ${a.id} state (${state.anchors[a.id].at.x.toFixed(4)}, ${state.anchors[a.id].at.y.toFixed(4)}, ${state.anchors[a.id].z.toFixed(4)}) mine (${p.map((v) => v.toFixed(4)).join(", ")}); last subject ${JSON.stringify(all[all.length - 1].query).slice(0, 200)}`);
}
