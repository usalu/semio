/** 📐️ Closed drawn shapes of one scale with their world bounding boxes: bun figure-shapes.ts <asset> <scale> <regex on "bone/id"> */
import { openCtx, drawn } from "./kit-lib.ts";
const [spec, sc, re] = process.argv.slice(2);
const c = openCtx(spec!, Number(sc));
const rx = new RegExp(re!);
for (const view of ["elevation", "front", "plan"] as const) for (const s of drawn(c, view)) if (rx.test(s.bone + "/" + s.id)) { const xs = s.rings.flat().map((q) => q.x), ys = s.rings.flat().map((q) => q.y); console.log(view.padEnd(9), s.bone.padEnd(8), s.id.padEnd(14), String(s.region).padEnd(10), "x", Math.min(...xs).toFixed(3), Math.max(...xs).toFixed(3), " y", Math.min(...ys).toFixed(3), Math.max(...ys).toFixed(3)); }
