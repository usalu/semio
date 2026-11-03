import { L, evaluateMod } from "./lib.ts";
import { readFileSync } from "node:fs";
const [d, den] = [process.argv[2]!, Number(process.argv[3])];
const a0 = JSON.parse(readFileSync(`${L}/🗂️catalog/🌳️vegetation/${d}/🔣️.json`, "utf8")).asset;
const tryEval = (a: any) => { try { evaluateMod.evaluate(structuredClone(a), { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: den } }); return ""; } catch (e: any) { return e.message.slice(0, 50); } };
const ids = new Set<string>(); for (const v of ["elevation", "front", "plan"]) for (const s of a0.representations[String(den)].geometry[v]) if (s.solid) ids.add(`${v}:${s.id}:${s.solid.kind}:${s.solid.kind === "extruded" ? s.solid.z1 - s.solid.z0 : ""}`);
const bad: string[] = [];
for (const key of ids) { const [v, id] = key.split(":"); const a = structuredClone(a0); const g = a.representations[String(den)].geometry; for (const vv of ["elevation", "front", "plan"]) g[vv] = g[vv].filter((s: any) => !s.solid || (vv === v && s.id === id)); const r = tryEval(a); if (r) bad.push(key); }
console.log(d, den, "solids", ids.size, "bad", bad.length, bad.slice(0, 12).join(" | "));
