import { PLUGIN_CATALOG } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/📇️registry/🟦️catalog.ts";
import { resolvePlaygroundBoot, pluginGraphErrorMessage } from "../../../../../../../🧰️framework/🔨️modules/🎠️kernel/🟦️.ts";

const variants = ["koordinator", "aggregator", "aussuchen", "bearbeiten", "verfolgen", "procedural3d"];
for (const variant of variants) {
  const boot = resolvePlaygroundBoot(PLUGIN_CATALOG, variant);
  const ids = boot.plugins.map((p) => p.pluginId);
  const errors = boot.dependencyErrors.map((e) => pluginGraphErrorMessage(e, "en"));
  console.log(JSON.stringify({
    variant,
    pluginCount: ids.length,
    hasDemonstrator: ids.includes("demonstrator"),
    hasStdio: ids.includes("stdio"),
    hasFlow: ids.includes("flow"),
    hasCad: ids.includes("cad"),
    errorCount: errors.length,
    errors: errors.slice(0, 8),
    sample: ids.slice(0, 15),
  }, null, 2));
}
