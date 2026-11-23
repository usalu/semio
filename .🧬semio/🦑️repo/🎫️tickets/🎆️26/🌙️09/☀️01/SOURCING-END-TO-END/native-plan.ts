import { componentPassPlanV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts";
const root = process.cwd(), ids = new Set(["sourcing", "sourcing-module-beams", "sourcing-module-slabs", "sourcing-module-windows"]);
console.log(JSON.stringify(componentPassPlanV1(root, ids, "prototype"), null, 1));
