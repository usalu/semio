import { loadAsset, resolveAsset, restState, simplifyRing } from "./volume-lib.ts";
const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const view = (process.argv[4] ?? "elevation") as "elevation" | "front" | "plan";
const st = restState(asset, 50);
for (const e of st[view]) if (e.shape.id === process.argv[3]) for (const ring of e.rings) console.log(simplifyRing(ring, 0.004).map((p) => `(${Math.round(p.x * 1000) / 1000},${Math.round(p.y * 1000) / 1000})`).join(" "));
