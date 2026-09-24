import { TAXONOMY, PLUGIN_AREAS, PLUGIN_AREAS_STATE } from "./links/plugin-reg/🔎️discovery/🟦️.ts";
import { writeFileSync } from "node:fs";

const out = {
  state: PLUGIN_AREAS_STATE,
  pluginAreas: PLUGIN_AREAS,
  areaValues: Object.fromEntries(PLUGIN_AREAS.map((a) => [a, TAXONOMY.areas[a]])),
};
writeFileSync(new URL("./generated/plugin-areas.json", import.meta.url), JSON.stringify(out, null, 2));
console.log(JSON.stringify(out, null, 2));
