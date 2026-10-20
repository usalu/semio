/** 🦑️ W2c: the repo product tree gets an owner of its own; the framework package stops carrying repo-product code. */
import { Plan } from "./plan.ts";
import { Carver } from "./carve.ts";
import { finish } from "./direct.ts";
const W = "w2c-repo-product";
const plan = new Plan();
const R = "🧰️framework/🛍️products/🦑️repo";
const nested = [
  { name: "@semio-tech/repo-lib", dir: `${R}/🔨️modules/📚️library` },
  { name: "@semio-tech/repo-coordinator", dir: `${R}/🔨️modules/🖥️server/🎛️coordinator` },
  { name: "@semio-tech/repo-test", dir: `${R}/🔨️modules/🧪️test` },
  { name: "@semio-tech/nx-bootstrap-tools", dir: `${R}/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools` },
  { name: "@semio-tech/repo-server-library", dir: `${R}/🔨️modules/🖥️server/📚️library` },
];
new Carver(plan, [{ wave: W, name: "@semio-tech/repo-product", dir: R, description: "repo · product modules (dashboard, client, lint) outside the library, coordinator, test and server-library packages", moveFrom: [{ pkg: "@semio-tech/framework", keys: ["./products/repo/modules/dashboard/installation"] }], anchorDir: "🧰️framework/📦️packages/🟦️typescript" }], nested).run();
finish(plan, "w2c", process.argv);
