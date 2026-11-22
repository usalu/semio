/** 📏️ Measures the 1:50 chair primitives against the reference table of the report (overall height, width, depth, seat height). */
import { bow, frontLeg, rearLeg, SEAT } from "./author-geometry.ts";
const tubes = [bow(10, 16), rearLeg(1, 8), rearLeg(-1, 8), frontLeg(1), frontLeg(-1)];
const ext = (k: number, s: number) => Math.max(...tubes.flatMap((t) => t.pts.map((p, i) => s * p[k]! + t.r[i]!)));
const x0 = Math.min(-SEAT.ro, -ext(0, -1));
const x1 = Math.max(SEAT.ro, ext(0, 1));
console.log(`[DEBUG] height ${ext(1, 1).toFixed(4)} width ${(ext(2, 1) + ext(2, -1)).toFixed(4)} depth ${(x1 - x0).toFixed(4)} (x ${x0.toFixed(4)}..${x1.toFixed(4)}) seat ${SEAT.y1}`);
