import { load, save, stateAt, three } from "./lib.ts";
const x = load("miscanthus-grass"); const a = x.asset;
for (const s of a.representations["200"].geometry.plan) if (/^(L1|R1)-1-b$/.test(s.id)) s.x1 = Math.round((s.x1 - Math.sign(s.x1) * 0.03) * 10000) / 10000;
const p = three.extentOf(stateAt(a, 200).shapes.plan); console.log((p.maxX - p.minX).toFixed(3), (p.maxY - p.minY).toFixed(3)); save(x);
