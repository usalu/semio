import { entryOf, poseState } from "./volumes-lib.ts";
const { asset } = entryOf(process.argv[2]!);
const st = poseState(asset, 50);
const f = (v: number[]) => v.map((x) => x.toFixed(2)).join(",");
for (const [id, fr] of Object.entries(st.frames)) console.log(id.padEnd(14), "o", f(fr.o), "u", f(fr.u), "v", f(fr.v), "w", f(fr.w));
