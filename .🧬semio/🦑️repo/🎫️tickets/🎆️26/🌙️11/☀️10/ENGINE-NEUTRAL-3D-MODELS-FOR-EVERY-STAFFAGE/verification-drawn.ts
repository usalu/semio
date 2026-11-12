import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [path, pose, view, filter] = process.argv.slice(2);
const a = JSON.parse(readFileSync(path!, "utf8")).asset;
const st: any = evaluate(a, { kind: "pose", pose: pose! }, { kind: "scale", scale: { denominator: 50 } });
const rows = (st.shapes[view!] as any[]).map((s) => { const p = s.rings.flat(); return { id: s.shape.id, role: s.shape.role, closed: s.closed, minX: Math.min(...p.map((q: any) => q.x)), maxX: Math.max(...p.map((q: any) => q.x)), minY: Math.min(...p.map((q: any) => q.y)), maxY: Math.max(...p.map((q: any) => q.y)) }; });
rows.sort((p, q) => q.maxX - p.maxX);
console.log("bounds", st.bounds);
for (const r of rows.filter((r) => !filter || r.id.includes(filter)).slice(0, 8)) console.log(r.id, r.role, r.closed, [r.minX, r.maxX, r.minY, r.maxY].map((x) => +x.toFixed(3)).join(" "));
