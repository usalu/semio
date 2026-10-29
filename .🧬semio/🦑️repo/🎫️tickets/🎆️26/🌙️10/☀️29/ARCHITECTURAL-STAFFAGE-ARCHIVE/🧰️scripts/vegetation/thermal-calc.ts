import { readFileSync } from "node:fs";
import { evaluate } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveThermal } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🌡️thermal/🟦️.ts";
const doc = JSON.parse(readFileSync(process.argv[2]!, "utf8")); const a = doc.assets?.[0] ?? doc;
const st = evaluate(a, { kind: "pose", pose: process.argv[3] ?? "calm" });
const out: Record<string, number[]> = {}; const lat: number[] = [];
const solar = [0, 50, 300, 600, 800, 650, 350, 80];
for (const s of solar) { const f = solveThermal(st, { ...a.thermal.environment, solarWM2: s }); lat.push(Math.round(Object.values(f.regions).reduce((n: number, r: any) => n + r.latentW, 0)));
  for (const r of Object.values(f.regions) as any[]) (out[r.region] ??= []).push(Math.round(r.surfaceTempC * 100) / 100); }
console.log("JSON" + JSON.stringify({ solar, temps: out, latentW: lat }));
