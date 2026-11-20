import { existsSync } from "node:fs";
import { join } from "node:path";
import { readGeneratedCatalogProjection } from "@semio-tech/plugin-registry/catalog-view";
import { buildPlaygroundSession } from "@semio-tech/plugin-registry/playground/session";
import { browserProductRootsV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/📂️paths/🟦️.ts";
import { ACTIVATION_RECEIPT_FILE, developmentRuntimeRoot, playgroundSessionStagedOutputPath, readActivationReceipt } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/🟦️.ts";

const workspace = process.cwd();
const variant = "process3d";
const mode = "prototype";
const products = browserProductRootsV1(workspace, mode);
const runtime = developmentRuntimeRoot(workspace, variant, mode, "react");
const receiptRoot = join(runtime, "activation");
const activated = existsSync(join(receiptRoot, ACTIVATION_RECEIPT_FILE))
  ? readActivationReceipt(receiptRoot).plugins.map(({ pluginId }) => pluginId)
  : [];
const projection = readGeneratedCatalogProjection("✏️s/🧑‍💻dev/🧩️catalog/🤖️generated");
const staged = buildPlaygroundSession(variant, projection).plugins.filter(({ moduleUrl }) => {
  const [, directory, bridge] = moduleUrl.split("/").slice(1);
  return existsSync(join(products.modules, directory, bridge));
}).map(({ pluginId }) => pluginId);
console.log("[DEBUG] Process browser activation", {
  sessionStaged: existsSync(playgroundSessionStagedOutputPath(products.sessions, variant)),
  staged,
  activated,
});
