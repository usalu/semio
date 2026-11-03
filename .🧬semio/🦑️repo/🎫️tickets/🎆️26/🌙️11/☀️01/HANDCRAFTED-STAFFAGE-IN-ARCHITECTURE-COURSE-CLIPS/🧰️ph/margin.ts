/** 🫕️ ph: error margins of the solver against the phase-change oracle (cwd = 🧍️staffage/📦️packages/🟦️typescript) */
import fixture from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧫️fixtures/🫕️phase-change/🔣️.json" with { type: "json" };
import { pathToFileURL } from "node:url";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { thermalState } = await import(pathToFileURL(`${S}/🧫️fixtures/🌡️thermal/🟦️.ts`).href);
const { solveThermal } = await import(pathToFileURL(`${S}/🔨️modules/🌡️thermal/🟦️.ts`).href);
for (const c of fixture.cases as any[]) {
  const inputs = (t: number) => ({ activityMet: 0, externalWorkW: 0, operating: { power: t > 0 ? 1 : 0 }, regions: { [c.id]: { exposedFraction: 1, sunlitFraction: 0 } }, emitters: {}, breathing: null, passengers: {} });
  c.timesS.forEach((t: number, i: number) => {
    const r = solveThermal(thermalState({ asset: c.asset, state: { timeS: t, thermal: inputs(t), anchors: {}, bones: {} } }), c.asset.thermal.environment, { history: (x: number) => ({ inputs: inputs(x) }) }).regions[c.id];
    const e = c.expected.rows[i];
    console.log(c.id, t, `dT=${(r.surfaceTempC - e.tempC).toFixed(3)} dm=${((r.phaseChangedKg - e.phaseKg) * 1000).toFixed(3)}g dP=${(r.phaseChangeW - e.phaseW).toFixed(1)}W (${(100 * (r.phaseChangeW - c.expected.plateauW) / c.expected.plateauW).toFixed(2)}% of plateau)`);
  });
}
