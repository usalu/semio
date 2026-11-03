import { readFileSync } from "node:fs";
const gesture = "/home/deck/semio/🧰️framework/🔨️modules/🕹️interaction/👆️gesture";
const { gripHitEdgePx, gripHitRadiusPx, pickGrip, lineParameter, gripParameter } = await import(`${gesture}/🟦️.ts`);
const f = JSON.parse(readFileSync(`${gesture}/🧪️tests/🧫️fixtures/🎯️grip-pick.json`, "utf8"));
let bad = 0;
const check = (cond, msg) => { if (!cond) { bad++; console.log("BAD", msg); } };
for (const r of f.hitEdge) check(gripHitEdgePx(r.device, r.profileMin) === r.expect, JSON.stringify(r));
for (const r of f.pick) check(pickGrip(r.points, r.at, gripHitRadiusPx(r.device, r.profileMin, r.drawn)) === r.expect, r.name);
for (const r of f.line) { const g = lineParameter(r.rayOrigin, r.rayDir, r.origin, r.axis); check(r.expect === null ? g === null : Math.abs(g - r.expect) < 1e-9, `${r.name} ${g} ${r.expect}`); }
for (const r of f.drag) check(Math.abs(gripParameter(r.anchorS, r.pressT, r.cursorT) - r.expect) < 1e-12, JSON.stringify(r));
console.log("bad", bad);
