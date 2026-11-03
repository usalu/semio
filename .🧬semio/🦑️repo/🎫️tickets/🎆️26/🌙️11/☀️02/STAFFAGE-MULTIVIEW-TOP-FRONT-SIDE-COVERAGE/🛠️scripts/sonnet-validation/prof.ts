import { loadCatalog, scaleEntries, subjects, scaleSelector } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🔭️multiview-coverage/🧰️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { defaultDepictionOptions, depict } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎨️depiction/🟦️.ts";
const a = loadCatalog().find((x) => x.id === "cyclist")!;
let t = performance.now(); const lap = (n: string) => { console.log(n, (performance.now() - t).toFixed(0)); t = performance.now(); };
const subs = subjects(a, 100); lap("subjects " + subs.length);
const states = subs.map((s) => evaluate(a, s.query, scaleSelector(100))); lap("evaluate");
depict(states[0]!, defaultDepictionOptions("light-line", "en", 100, "plan")); lap("first depict (suitability)");
for (const m of ["light-line", "thermal", "forces"] as const) { for (const s of states.slice(0, 20)) depict(s, defaultDepictionOptions(m, "en", 100, "plan")); lap(m + " x20"); }
