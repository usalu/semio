import { entryOf, poseState } from "./volumes-lib.ts";
const { asset } = entryOf(process.argv[2]!);
const st: any = poseState(asset, Number(process.argv[3] ?? 50));
const f = (v: number[]) => v.map((x) => x.toFixed(4)).join(",");
for (const [id, fr] of Object.entries<any>(st.frames)) console.log(id.padEnd(14), "o", f(fr.o), "u", f(fr.u), "v", f(fr.v), "w", f(fr.w));
