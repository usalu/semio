import gagge from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🫀️two-node-gagge/🔣️.json" with { type: "json" };
import leaf from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🍃️leaf-transpiration/🔣️.json" with { type: "json" };
import trans from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/⏳️thermal-transient/🔣️.json" with { type: "json" };
import bb from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🌡️blackbody-exchange/🔣️.json" with { type: "json" };
import { thermalState } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🌡️thermal/🟦️.ts";
import { simulateBody, solveThermal } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";

const max = (xs: number[]) => Math.max(...xs.map(Math.abs)).toFixed(3);
const g: Record<string, number[]> = { core: [], skin: [], w: [], bl: [], Qs: [], E: [], Qres: [] };
for (const c of (gagge as any).cases) {
  const r = simulateBody(thermalState(c).asset, c.state.thermal, c.asset.thermal.environment, 3600);
  g.core!.push(r.field.coreTempC - c.expected.t_core); g.skin!.push(r.field.meanSkinTempC - c.expected.t_skin); g.w!.push(r.field.skinWettedness - c.expected.w);
  g.bl!.push(r.field.skinBloodFlowLM2H - c.expected.m_bl); g.Qs!.push(r.diag.regions[0]!.dryWM2 - c.expected.q_sensible); g.E!.push(r.diag.regions[0]!.evapWM2 - c.expected.e_skin); g.Qres!.push(r.field.respiratoryLatentW / 1.8258 - c.expected.q_res);
}
console.log("[DEBUG] gagge max |dev|", Object.fromEntries(Object.entries(g).map(([k, v]) => [k, max(v)])));
const l: Record<string, number[]> = { T: [], rad: [], sens: [], lat: [], pm: [] };
for (const c of (leaf as any).cases) {
  const regs = (leaf as any).asset.thermal.regions;
  const st = thermalState({ asset: (leaf as any).asset, state: { timeS: 0, thermal: { activityMet: 0, externalWorkW: 0, operating: {}, regions: Object.fromEntries(regs.map((r: any) => [r.id, { exposedFraction: 1, sunlitFraction: c.sunlit }])), emitters: {}, breathing: null, passengers: {} }, anchors: {}, bones: {} } as any });
  const f = solveThermal(st, c.environment);
  for (const [id, e] of Object.entries<any>(c.expected)) { const r = f.regions[id]!; l.T!.push(r.surfaceTempC - e.surfaceTempC); l.rad!.push(r.netRadiativeWM2 - e.netRadiativeWM2); l.sens!.push(r.sensibleWM2 - e.sensibleWM2); l.lat!.push(r.latentWM2 - e.latentWM2); if (e.pmLinearised !== null) l.pm!.push((r.latentWM2 - e.pmLinearised) / Math.max(1, Math.abs(e.pmLinearised))); }
}
console.log("[DEBUG] leaf max |dev| (K, W/m2; pm relative)", Object.fromEntries(Object.entries(l).map(([k, v]) => [k, max(v)])));
const t: number[] = [];
const inputsAt = (x: number) => ({ activityMet: 0, externalWorkW: 0, operating: { power: x > 0 ? 1 : 0 }, regions: { "sunlit-plate": { exposedFraction: 1, sunlitFraction: x > 0 ? 1 : 0 }, "heated-plate": { exposedFraction: 1, sunlitFraction: 0 } }, emitters: {}, breathing: null, passengers: {} });
for (const [id, e] of Object.entries<any>((trans as any).expected)) (trans as any).timesS.forEach((x: number, i: number) => { const f = solveThermal(thermalState({ asset: (trans as any).asset, state: { timeS: x, thermal: inputsAt(x), anchors: {}, bones: {} } as any }), (trans as any).asset.thermal.environment, { history: (tt) => ({ inputs: inputsAt(tt) as any }) }); t.push(f.regions[id]!.surfaceTempC - e.responseC[i]); });
console.log("[DEBUG] transient max |dev| K", max(t));
const b: number[] = [];
const bf = solveThermal(thermalState(bb as any));
for (const [id, e] of Object.entries<any>((bb as any).expected)) b.push((bf.regions[id]!.netRadiativeWM2 - e.netWM2) / Math.abs(e.netWM2 || 1));
console.log("[DEBUG] blackbody max relative net dev", max(b));
