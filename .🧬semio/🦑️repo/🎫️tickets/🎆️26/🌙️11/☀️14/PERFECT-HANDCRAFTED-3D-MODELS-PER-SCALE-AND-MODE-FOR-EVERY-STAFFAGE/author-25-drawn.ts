import { drawnSet } from "./volumes-silhouette.ts";
import { openCtx, flag } from "./kit-lib.ts";
import { writeFileSync } from "node:fs";
const a = process.argv.slice(2);
const sc = Number(flag(a, "--scale"));
const c = openCtx(a[0]!, sc);
const view = (flag(a, "--view") ?? "elevation") as any;
const set = drawnSet(c.m, sc, view);
const t = set.tris;
let x0 = 1e9, x1 = -1e9, y0 = 1e9, y1 = -1e9;
for (let i = 0; i < t.length; i += 2) { x0 = Math.min(x0, t[i]!); x1 = Math.max(x1, t[i]!); y0 = Math.min(y0, t[i + 1]!); y1 = Math.max(y1, t[i + 1]!); }
const W = 900, k = W / (x1 - x0), H = Math.ceil((y1 - y0) * k);
let s = `<svg xmlns="http://www.w3.org/2000/svg" width="${W}" height="${H}" viewBox="0 0 ${W} ${H}"><rect width="100%" height="100%" fill="white"/>`;
for (let i = 0; i < t.length; i += 6) s += `<polygon points="${[0, 2, 4].map((j) => `${((t[i + j]! - x0) * k).toFixed(1)},${((y1 - t[i + j + 1]!) * k).toFixed(1)}`).join(" ")}" fill="rgba(0,0,0,0.25)" stroke="none"/>`;
s += "</svg>";
writeFileSync(flag(a, "--out")!, s);
console.log(x0, x1, y0, y1);
