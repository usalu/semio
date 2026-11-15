/** 🔬️ Bone frames of the default pose: bun author-33-frames.ts <asset> <scale> */
import { openCtx } from "./kit-lib.ts";
const c = openCtx(process.argv[2]!, Number(process.argv[3]));
const f = (c.state as any).frames;
const r = (v: number[]) => v.map((x) => x.toFixed(3)).join(",");
for (const [b, fr] of Object.entries<any>(f)) console.log(b.padEnd(12), "o", r(fr.origin ?? fr.o ?? fr.p ?? []), "u", r(fr.u), "v", r(fr.v), "w", r(fr.w));
console.log(Object.keys((c.state as any)));
