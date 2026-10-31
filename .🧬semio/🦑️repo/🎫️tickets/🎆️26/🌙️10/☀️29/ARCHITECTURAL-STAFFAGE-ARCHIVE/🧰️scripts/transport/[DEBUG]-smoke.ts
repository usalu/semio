import { scenario, SCENARIOS } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/💨️transport/🟦️.ts";
import { solveTransport } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/💨️transport/🟦️.ts";
import { respirationState } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/💨️transport/🫁️respiration/🟦️.ts";
const s = scenario("human");
for (const t of [0, 1, 2.5, 5, 10]) {
  const t0 = performance.now();
  const f = solveTransport(s.asset, { kind: "pose", pose: "rest", timeS: t }, SCENARIOS.environments["calm-cold"], s.options);
  console.log("[DEBUG]", t, "parcels", f.parcels.length, "cond", f.condensation.length, "stream", f.streamlines.length, "ms", (performance.now() - t0).toFixed(0), JSON.stringify(respirationState(s.asset, { kind: "pose", pose: "rest", timeS: t })));
  const far = f.parcels.filter(p=>p.emitter==="nose").slice(0,3).map(p=>[p.ageS.toFixed(2), p.at.x.toFixed(3), p.at.y.toFixed(3), p.radiusM.toFixed(3), p.excessK.toFixed(2), (p.vapourExcessKgPerKg*1000).toFixed(2), (p.condensateKgPerKg*1000).toFixed(3)]);
  console.log("[DEBUG]", JSON.stringify(far));
}
