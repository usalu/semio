/** 🗄️ W2b: the repo server library gets its own package (it left the framework package: framework-no-products) and owns the ports it needs. */
import { Plan } from "./plan.ts";
import { Carver } from "./carve.ts";
import { finish } from "./direct.ts";
import { model } from "./lib.ts";
const S = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server";
const W = "w2b-server-library";
const plan = new Plan();
const keys = ["./products/repo/modules/server/library/event", "./products/repo/modules/server/library/source-index", "./products/repo/modules/server/library/authentication", "./products/repo/modules/server/library/persistence"];
new Carver(plan, [{ wave: W, name: "@semio-tech/repo-server-library", dir: `${S}/📚️library`, description: "repo · server library (persistence, authentication, events, source index, worker execution) and its ports", moveFrom: [{ pkg: "@semio-tech/framework", keys }, { pkg: "@semio-tech/repo-coordinator", keys: ["./ports"] }], anchorDir: "🧰️framework/📦️packages/🟦️typescript", extraThirdParty: ["next", "pg", "pg-boss"] }]).run();
plan.edit({ file: `${S}/📚️library/🔌️ports/🟦️.ts`, from: `new URL("../../🎛️coordinator/📦️packages/🟦️typescript/package.json", import.meta.url)`, to: `new URL("../package.json", import.meta.url)`, mode: "text", wave: W, why: "the library package declares the drivers it adapts (pg, pg-boss, next)" });
const fwPkg = model.byName.get("@semio-tech/framework")!;
if ("@semio-tech/repo-coordinator" in fwPkg.deps) plan.manifest({ op: "remove", packageJson: fwPkg.packageJson, package: fwPkg.name, field: "dependencies", key: "@semio-tech/repo-coordinator", value: fwPkg.deps["@semio-tech/repo-coordinator"]!, wave: W, why: "framework no longer imports the coordinator: the server library moved out and owns its ports" });
finish(plan, "w2b", process.argv);
