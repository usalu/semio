import { readFileSync } from "node:fs";
import { solveRig, apply } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
const files = process.argv.slice(2); const a: any = Object.assign({}, ...files.map((f) => JSON.parse(readFileSync(f, "utf8"))));
const w = solveRig(a.rig, {}, { x: 0, y: 0, angleDeg: 0 });
for (const b of a.rig.bones) { const m = w[b.id]!; const o = apply(m, 0, 0); const t = apply(m, b.lengthM, 0); console.log(`${b.id.padEnd(14)} o=(${o.x.toFixed(2)},${o.y.toFixed(2)}) tip=(${t.x.toFixed(2)},${t.y.toFixed(2)}) ang=${(Math.atan2(m[1], m[0]) * 180 / Math.PI).toFixed(1)}`); }
