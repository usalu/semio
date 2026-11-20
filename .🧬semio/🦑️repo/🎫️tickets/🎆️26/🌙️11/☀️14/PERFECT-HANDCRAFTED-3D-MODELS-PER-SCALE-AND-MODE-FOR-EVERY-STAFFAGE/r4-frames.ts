/** 🧭️ Prints the pose frames of some bones of an asset (origin, axes), to see how the rig's lateral offsets arrive in the kit. Usage: bun r4-frames.ts <asset> <scale> <bone...> */
import { openCtx } from "./kit-lib.ts";
const [spec, scale, ...bones] = process.argv.slice(2);
const c = openCtx(spec!, Number(scale));
for (const b of bones) console.log(b, JSON.stringify(c.state.frames[b]));
console.log("feature", c.feature, "views", Object.keys(c.state));
