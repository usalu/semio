import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createFrameworkOsDevConfig } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts";
import { DEMONSTRATOR_SHELL_BRANDS } from "../../../🪧️brand.ts";
import contribution from "../../../🐚️shell/🧬️schema/🔣️.json";
const workspace = fileURLToPath(new URL("../../../../../", import.meta.url));
export default createFrameworkOsDevConfig({ brands: DEMONSTRATOR_SHELL_BRANDS, browserEntry: "/@fs/" + resolve(workspace, contribution.browserEntry).replaceAll("\\", "/") });
