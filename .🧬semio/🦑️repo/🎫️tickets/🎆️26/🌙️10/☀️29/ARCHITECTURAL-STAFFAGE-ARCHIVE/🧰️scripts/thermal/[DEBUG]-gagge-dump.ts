import fixture from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🫀️two-node-gagge/🔣️.json" with { type: "json" };
import { simulateBody } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

for (const c of (fixture as any).cases) {
  const r = simulateBody(c.asset, c.state.thermal, c.asset.thermal.environment, 3600, "neutral", 60);
  const a = 1.8258;
  const d = r.diag.regions[0]!;
  const row = { core: r.field.coreTempC, skin: r.field.meanSkinTempC, w: r.field.skinWettedness, bl: r.field.skinBloodFlowLM2H, e_skin: d.evapWM2, q_sens: d.dryWM2, q_res: r.field.respiratoryLatentW / a, c_res: r.field.respiratorySensibleW / a };
  const e = c.expected;
  console.log("[DEBUG]", c.id.padEnd(14), "core", row.core.toFixed(2), e.t_core.toFixed(2), "| skin", row.skin.toFixed(2), e.t_skin.toFixed(2), "| w", row.w.toFixed(2), e.w.toFixed(2), "| bl", row.bl.toFixed(1), e.m_bl.toFixed(1), "| E", row.e_skin.toFixed(1), e.e_skin.toFixed(1), "| Qs", row.q_sens.toFixed(1), e.q_sensible.toFixed(1), "| Qres", row.q_res.toFixed(2), e.q_res.toFixed(2), "| hc", d.exposure.hc.toFixed(2), "hr", d.exposure.hr.toFixed(2));
}
