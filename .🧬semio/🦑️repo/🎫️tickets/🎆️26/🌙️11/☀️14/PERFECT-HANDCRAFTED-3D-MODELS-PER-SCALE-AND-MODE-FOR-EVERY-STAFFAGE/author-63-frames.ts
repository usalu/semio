/** bun author-63-frames.ts <asset> <scale> bone,bone : bone frames in the default pose */
import { openCtx } from "./kit-lib.ts";
const [spec, sc, bones] = process.argv.slice(2);
const c = openCtx(spec!, Number(sc));
const fr = (c.state as any).frames;
for (const b of bones ? bones.split(",") : Object.keys(fr)) { const f = fr[b]; if (f) console.log(b.padEnd(14), "o", f.o.map((x: number) => x.toFixed(3)).join(","), "u", f.u.map((x: number) => x.toFixed(3)).join(","), "v", f.v.map((x: number) => x.toFixed(3)).join(","), "w", f.w.map((x: number) => x.toFixed(3)).join(",")); }
