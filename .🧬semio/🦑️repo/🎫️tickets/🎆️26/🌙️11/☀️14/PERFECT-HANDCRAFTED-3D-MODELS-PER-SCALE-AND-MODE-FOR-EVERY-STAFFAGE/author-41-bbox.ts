/** 📦️ Author 41: world bounding box of every solid of one list of one scale (to find what sticks out of the body box). Usage: bun author-41-bbox.ts <asset> <scale> <body|thermal|structure> [min-outside-m] */
import { openCtx } from "./kit-lib.ts";
import { scaleMeshes } from "./volumes-lib.ts";
const [spec, scaleS, list, minS] = process.argv.slice(2);
const c = openCtx(spec!, Number(scaleS));
const sm = scaleMeshes(c.m, Number(scaleS), c.vol);
const box = (ms: any[]) => { const b = [1e9, 1e9, 1e9, -1e9, -1e9, -1e9]; for (const m of ms) { const p = m.world; for (let i = 0; i < p.length; i += 3) for (let k = 0; k < 3; k++) { b[k] = Math.min(b[k]!, p[i + k]); b[3 + k] = Math.max(b[3 + k]!, p[i + k]); } } return b; };
const bb = box(sm.body);
console.log("body box", bb.map((x) => +x.toFixed(3)).join(" "));
const ms = (sm as any)[list!] as any[];
const sol = (sm.solids as any)[list!] as any[];
ms.forEach((m, i) => { const b = box([m]); const out = Math.max(bb[0]! - b[0]!, bb[1]! - b[1]!, bb[2]! - b[2]!, b[3]! - bb[3]!, b[4]! - bb[4]!, b[5]! - bb[5]!); if (out > Number(minS ?? 0)) console.log(m.id, "outside", out.toFixed(4), b.map((x: number) => +x.toFixed(3)).join(" ")); });
