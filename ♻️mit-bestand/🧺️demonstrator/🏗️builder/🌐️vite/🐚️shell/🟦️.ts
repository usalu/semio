import { specificOsDevBuildInventoryV1 } from "@semio-tech/s-dev-catalog-ts/builder";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createFrameworkOsDevConfig } from "@semio-tech/framework-os-dev/builder/vite";
import { DEMONSTRATOR_SHELL_BRANDS } from "../../../🪧️brand.ts";
import contribution from "../../../🐚️shell/🧬️schema/🔣️.json";
const workspace = fileURLToPath(new URL("../../../../../", import.meta.url));
export default createFrameworkOsDevConfig({ inventory: specificOsDevBuildInventoryV1(), brands: DEMONSTRATOR_SHELL_BRANDS, browserEntry: "/@fs/" + resolve(workspace, contribution.browserEntry).replaceAll("\\", "/") });
