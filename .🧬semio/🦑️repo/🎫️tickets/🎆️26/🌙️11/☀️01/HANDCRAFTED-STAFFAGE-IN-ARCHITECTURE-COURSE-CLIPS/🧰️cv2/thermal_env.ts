import { resolve } from "node:path";

const S = resolve(import.meta.dirname, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { loadCatalogFromFiles, loadEnvironments } = await import(`${S}/🔨️modules/🗂️catalog/📥️files/🟦️.ts`);
const { solveTransport } = await import(`${S}/🔨️modules/💨️transport/🟦️.ts`);
const { solveThermalAt } = await import(`${S}/🔨️modules/🌡️thermal/🟦️.ts`);

const catalog = await loadCatalogFromFiles(`${S}/🗂️catalog`);
const presets = new Map(((await loadEnvironments(`${S}/🗂️catalog`))?.presets ?? []).map((p: any) => [p.id, p.environment]));
const asset = catalog.assets.find((a: any) => a.id === "electric-kettle")!;
const constant = presets.get("office-neutral");
const timeline = { keys: [{ t: 0, environment: presets.get("office-neutral"), ease: "linear" }, { t: 200, environment: presets.get("summer-hot-sun"), ease: "linear" }] };
const sessionFree = (state: any, environment: any) => solveThermalAt(state.asset, state.query, environment);
const at = (timeS: number) => ({ kind: "clip", clip: "boil", timeS });
const digest = (field: any) => JSON.stringify(field.parcels.map((p: any) => [p.at, p.radiusM, p.condensateKgPerKg]));
const settings = { maxAgeS: 2, pathlines: false, streamlines: false };

for (const [name, environment] of [["constant", constant], ["timeline", timeline]] as const) {
  const withSession = [60, 120, 180].map((t) => digest(solveTransport(asset, at(t), environment, settings)));
  const without = [60, 120, 180].map((t) => digest(solveTransport(asset, at(t), environment, { ...settings, thermal: sessionFree })));
  console.log(name, "default (shared session) equals session-free at each time:", withSession.map((s, i) => s === without[i]));
}

const reversed = [180, 120, 60].map((t) => digest(solveTransport(asset, at(t), timeline, { ...settings, seed: 7 })));
const ascending = [60, 120, 180].map((t) => digest(solveTransport(asset, at(t), timeline, { ...settings, seed: 7 })));
console.log("timeline, seed 7, query order independence (reverse vs ascending, fresh tracks per seed):", reversed.slice().reverse().map((s, i) => s === ascending[i]));
