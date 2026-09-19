import { resolvePlaygroundBoot } from "../../../../../../../🧰️framework/🔨️modules/🎠️kernel/🟦️.ts";
import { PLUGIN_CATALOG } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🟦️.ts";
for (const row of PLUGIN_CATALOG.playgrounds) {
  try { const b = resolvePlaygroundBoot(PLUGIN_CATALOG, row.variant); console.log(row.variant.padEnd(18), String(b.defaultAppId).padEnd(40), b.plugins.length, b.dependencyErrors.length); }
  catch (e) { console.log(row.variant, "ERR", (e as Error).message.slice(0, 120)); }
}
