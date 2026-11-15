import { openCtx } from "./kit-lib.ts";
import { pathPoints } from "./author-54-lib.ts";
const c = openCtx("vegetation/hosta-clump", 50);
const sh = c.raw.representations["50"].geometry.elevation.find((x: any) => x.id === process.argv[2]);
const p = pathPoints(sh.d, 8);
for (let u = 0.004; u < 0.12; u += 0.012) { const ys: number[] = []; for (let i = 0, j = p.length - 1; i < p.length; j = i++) { const a = p[j]!, b = p[i]!; if (a[0] <= u !== b[0] <= u) ys.push(a[1] + ((u - a[0]) / (b[0] - a[0])) * (b[1] - a[1])); } ys.sort((x, y) => x - y); console.log(u.toFixed(3), ys.map((y) => y.toFixed(3)).join(" ")); }
