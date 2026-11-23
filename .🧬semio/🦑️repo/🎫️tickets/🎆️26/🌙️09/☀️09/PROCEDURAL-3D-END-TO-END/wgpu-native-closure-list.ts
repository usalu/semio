import { playgroundComponentClosureV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts";
import { runtimeCatalogBuildInventoryV1 } from "@semio-tech/plugin-registry/catalog-view";
/** 🧭️ [DEBUG] Lists the component closure `generation3d` loads. */
const inv = runtimeCatalogBuildInventoryV1();
for (const scope of ["boot", "full"] as const) {
  const ids = [...playgroundComponentClosureV1("generation3d", scope)];
  console.log(scope, ids.length, JSON.stringify(ids));
}
