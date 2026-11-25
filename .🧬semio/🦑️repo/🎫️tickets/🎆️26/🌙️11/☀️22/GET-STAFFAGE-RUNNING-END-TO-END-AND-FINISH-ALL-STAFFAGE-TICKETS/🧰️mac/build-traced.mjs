/** 🏗️ Builds the Staffage site with its own Vite configuration and reports every catalog asset module as its check ends, so a long catalog check shows where it is. Usage: bun build-traced.mjs <release|prototype>
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🌐️vite/🟦️.ts — the plugin that checks each asset before it serves the module */
import { resolve } from "node:path";
import { build } from "vite";

const mode = process.argv[2] ?? "release";
const site = resolve(import.meta.dirname, "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🌐️site");
const started = Date.now();
let assets = 0;
process.chdir(site);
await build({
  configFile: resolve(site, "🏗️builder/🌐️vite/🟦️.ts"),
  mode,
  plugins: [{
    name: "s1-asset-trace",
    moduleParsed(info) {
      const at = info.id.indexOf("staffage-asset:");
      if (at >= 0) console.log(`[DEBUG] ${((Date.now() - started) / 1000).toFixed(1)} s asset ${++assets} ${info.id.slice(at + 15).split("🗂️catalog/")[1] ?? info.id}`);
    },
  }],
});
console.log(`[DEBUG] built ${mode} in ${((Date.now() - started) / 1000).toFixed(1)} s, ${assets} asset modules`);
