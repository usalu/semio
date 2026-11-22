/** 🌡️ Region temperatures and evaporation of an asset at a pose: bun thermal.ts <file> <pose|clip@t> */
import { evaluate, loadAsset, queryOf, solveThermal, totalPowerW } from "./lib.ts";
const [file, spec] = process.argv.slice(2);
const asset = await loadAsset(file!);
const field = solveThermal(evaluate(asset, queryOf(spec!))) as any;
for (const r of Object.values(field.regions) as any[]) console.log(r.region, "surface", r.surfaceTempC.toFixed(1), "steady", r.steadyTempC.toFixed(1), "skin", r.skinTempC?.toFixed(1) ?? "-", "evap g/h", ((r.evaporationKgS ?? 0) * 3.6e6).toFixed(1));
console.log("total power W", totalPowerW(asset).toFixed(1), Object.keys(field).join(","));
