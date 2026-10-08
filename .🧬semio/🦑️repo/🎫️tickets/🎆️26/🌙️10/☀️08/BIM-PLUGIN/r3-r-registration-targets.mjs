import { join } from "node:path";
import { pathToFileURL } from "node:url";
const workspace = process.cwd();
const { cacheInternals } = await import(pathToFileURL(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs")).href);
const targets = await cacheInternals.componentTargets("🌎️hub/🧩️compositions/🏙️bim/📦️packages/🦀️rust", workspace, []);
console.log(JSON.stringify(targets, null, 1));
