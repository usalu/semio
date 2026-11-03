import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
import { resolve } from "node:path";
const S = "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage";
const { thermalState } = await import(pathToFileURL(resolve(S, "🧫️fixtures/🌡️thermal/🟦️.ts")).href);
const { solveThermal } = await import(pathToFileURL(resolve(S, "🔨️modules/🌡️thermal/🟦️.ts")).href);
const fx = JSON.parse(readFileSync(process.argv[2]!, "utf8"));
const c = fx.coupled;
const inp = (l: number) => ({ activityMet: 0, externalWorkW: 0, operating: { power: l }, regions: { water: { exposedFraction: 1, sunlitFraction: 0 }, wall: { exposedFraction: 1, sunlitFraction: 0 } }, emitters: {}, breathing: null, passengers: {} });
c.timesS.forEach((t: number, i: number) => {
  const f = solveThermal(thermalState({ asset: c.asset, state: { timeS: t, thermal: inp(1), anchors: {}, bones: {} } }), c.asset.thermal.environment, { history: (x: number) => ({ inputs: inp(x > 0 ? 1 : 0) }) });
  console.log(t, f.regions.water.surfaceTempC.toFixed(2), c.expected.rows[i].waterC.toFixed(2), f.regions.wall.surfaceTempC.toFixed(2), c.expected.rows[i].wallC.toFixed(2));
});
