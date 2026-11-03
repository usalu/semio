import { readFileSync } from "node:fs";
import { measureSuitability, DEFAULT_PRINT_PROFILE } from "C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🖨️representation/🟦️.ts";
const a = JSON.parse(readFileSync("C:/git/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/thermostatic-radiator-valve/🔣️.json", "utf8")).asset;
const s: any = measureSuitability(a, a.representations["50"], 50, DEFAULT_PRINT_PROFILE);
console.log(JSON.stringify(s.violations));
for (const v of ["elevation","front","plan"]) { const r = { ...a.representations["50"], geometry: { elevation: [], plan: [], front: [], [v]: a.representations["50"].geometry[v] } }; try { const x: any = measureSuitability({ ...a, representations: { ...a.representations, "50": r } }, r, 50, DEFAULT_PRINT_PROFILE); console.log(v, JSON.stringify(x.violations)); } catch (e) { console.log(v, String(e)); } }
