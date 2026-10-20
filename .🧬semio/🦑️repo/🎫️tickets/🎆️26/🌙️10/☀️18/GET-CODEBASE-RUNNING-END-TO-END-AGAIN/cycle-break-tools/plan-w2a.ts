/** 📣️ W2a: the plugin catalog publication contract becomes a leaf package, so the repo library no longer imports the registry. */
import { Plan } from "./plan.ts";
import { Carver } from "./carve.ts";
import { finish } from "./direct.ts";
const R = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry";
const plan = new Plan();
new Carver(plan, [{ wave: "w2a-publication", name: "@semio-tech/plugin-registry-publication", dir: `${R}/📣️publication`, description: "plugin catalog · publication contract and build contribution (leaf)", moveFrom: [{ pkg: "@semio-tech/plugin-registry", keys: ["./publication", "./publication/build"] }] }]).run();
finish(plan, "w2a", process.argv);
