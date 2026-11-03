import { L, evaluateMod } from "./lib.ts";
import { readFileSync } from "node:fs";
const [d, den] = [process.argv[2]!, Number(process.argv[3])];
const a = JSON.parse(readFileSync(`${L}/🗂️catalog/🌳️vegetation/${d}/🔣️.json`, "utf8")).asset;
const g = a.representations[String(den)].geometry; const bad: string[] = [];
for (let k = 0; k < 60; k++) {
  try { evaluateMod.evaluate(structuredClone(a), { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: den } }); break; } catch (e: any) {
    let found: string | undefined;
    for (const v of ["elevation", "front", "plan"]) for (const s of g[v]) if (s.solid) { const keep = g[v]; const b = a.representations[String(den)].geometry; const saved = JSON.stringify(b[v]); b[v] = b[v].filter((q: any) => q !== s); let ok = true; try { evaluateMod.evaluate(structuredClone(a), { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: den } }); } catch (e2: any) { ok = e2.message !== e.message; } if (ok && !found) found = `${v}:${s.id}:${s.solid.kind}`; if (!ok || found) { if (found === `${v}:${s.id}:${s.solid.kind}`) break; b[v] = JSON.parse(saved); } void keep; }
    if (!found) { console.log("cannot isolate", e.message.slice(0, 80)); break; }
    bad.push(found); const [v, id] = found.split(":"); g[v!] = g[v!].filter((q: any) => q.id !== id || !q.solid);
  }
}
console.log(d, den, "bad:", bad.join(" "));
