import { entryOf, loadSource, poseState, toWorld } from "./volumes-lib.ts";
const [spec, scaleS, id] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const src = loadSource(entry)!;
const st = poseState(asset, Number(scaleS));
const s = src.volumes[scaleS!].body.find((q: any) => q.id === id);
console.log(s.bone, JSON.stringify(st.frames[s.bone]));
for (const q of s.sections ?? []) { const w = toWorld(st.frames[s.bone]!, q.at); console.log("u", q.at[0], "v", q.at[1], "z", q.at[2], "rx", q.rx, "ry", q.ry, "world", w.map((x: number) => +x.toFixed(3)).join(",")); }
