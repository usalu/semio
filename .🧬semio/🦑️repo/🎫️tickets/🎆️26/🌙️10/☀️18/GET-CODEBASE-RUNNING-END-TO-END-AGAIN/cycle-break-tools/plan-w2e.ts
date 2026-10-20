/** 🎛️ W2e: the dashboard installer is library bootstrap (the library loads it lazily and it uses the library's storage and cargo layers); it moves into the library's bootstrap so repo-lib and repo-product stop depending on each other. */
import { Plan } from "./plan.ts";
import { planMoves } from "./moves.ts";
import { finish } from "./direct.ts";
import { model } from "./lib.ts";
const W = "w2e-dashboard-installer";
const R = "🧰️framework/🛍️products/🦑️repo/🔨️modules";
const plan = new Plan();
planMoves(plan, W, "dashboard installation is library bootstrap", [{ from: `${R}/🎛️dashboard/📦️installation`, to: `${R}/📚️library/⚡️caching/🚀️bootstrap/🎛️dashboard/📦️installation` }]);
const fw = model.byName.get("@semio-tech/framework")!;
const key = "./products/repo/modules/dashboard/installation";
const value = fw.exports[key] as string;
plan.manifest({ op: "remove", packageJson: fw.packageJson, package: fw.name, field: "exports", key, value, wave: W, why: "no importer uses the key; the module moved into the library" });
plan.del(`${fw.dir}/${value.slice(2)}`, W, "dead glue");
finish(plan, "w2e", process.argv);
