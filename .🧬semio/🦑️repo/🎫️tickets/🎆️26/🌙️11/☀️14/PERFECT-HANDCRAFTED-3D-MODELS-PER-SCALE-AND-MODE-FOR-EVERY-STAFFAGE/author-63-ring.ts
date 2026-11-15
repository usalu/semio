/** bun author-63-ring.ts <asset> <scale> <view> <shapeId> [ymin] : the drawn ring in world coords */
import { drawn, openCtx } from "./kit-lib.ts";
const [spec, sc, view, id, ymin = "-9"] = process.argv.slice(2);
const c = openCtx(spec!, Number(sc));
for (const s of drawn(c, view as never)) if (s.id === id) for (const r of s.rings) console.log(r.filter((p) => p.y >= Number(ymin)).map((p) => `${p.x.toFixed(3)},${p.y.toFixed(3)}`).join(" "));
