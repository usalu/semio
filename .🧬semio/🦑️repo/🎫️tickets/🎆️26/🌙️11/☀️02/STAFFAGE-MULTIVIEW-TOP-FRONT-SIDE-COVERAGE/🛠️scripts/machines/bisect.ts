import { readFileSync } from "node:fs";
import { fileOf } from "./lib.ts";
import { evaluate } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [id, pose, scale] = process.argv.slice(2);
const base = JSON.parse(readFileSync(fileOf(id), "utf8"));
const ok = (a: any) => { try { evaluate(a, { kind: "pose", pose }, { kind: "scale", scale: { denominator: Number(scale) } } as any); return true; } catch { return false; } };
const G = base.asset.representations[scale].geometry;
const all: [string, number][] = [];
for (const v of ["elevation", "front", "plan"]) G[v].forEach((s: any, i: number) => all.push([v, i]));
const cul: string[] = [];
for (const [v, i] of all) { const a = structuredClone(base); const s = a.asset.representations[scale].geometry[v][i]; const keep = s; a.asset.representations[scale].geometry[v].splice(i, 1); if (ok(a)) cul.push(`${v}/${keep.id}(${keep.bone}${keep.solid ? ",solid" : ""})`); }
console.log(id, pose, scale, "removing alone fixes:", cul.join(" "));
