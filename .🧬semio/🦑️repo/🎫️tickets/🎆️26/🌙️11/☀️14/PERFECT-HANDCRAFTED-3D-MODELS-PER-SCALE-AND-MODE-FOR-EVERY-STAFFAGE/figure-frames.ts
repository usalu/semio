/** 📐️ Default-pose frames (origin, u, v, w, length, anterior sign) of bones: bun figure-frames.ts <asset> <scale> <bone>... */
import { openCtx } from "./kit-lib.ts";
import { boneInfos } from "./figure-lib.ts";
const c = openCtx(process.argv[2]!, Number(process.argv[3]));
const inf = boneInfos(c);
for (const b of process.argv.slice(4)) { const i = inf[b]; console.log(b, "o", i?.frame.o.map((x) => x.toFixed(3)), "u", i?.frame.u.map((x) => x.toFixed(2)), "v", i?.frame.v.map((x) => x.toFixed(2)), "w", i?.frame.w.map((x) => x.toFixed(2)), "L", i?.L, "ant", i?.ant); }
