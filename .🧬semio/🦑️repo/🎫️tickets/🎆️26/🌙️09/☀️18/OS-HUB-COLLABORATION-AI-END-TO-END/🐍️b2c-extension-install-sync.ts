/** 🧩️ Slice B2c — republishes every extension whose build output is newer than its install-root copy.
 * `syncBuiltExtensionsToInstallRoot` only walks the registry rows its CALLER passes, and a filtered
 * session (the collab prebuild resolves `space` + `writer` transitively) never names the imperative
 * extensions, so their install-root copy stayed on the build that first published it while the build
 * root moved on. Passing the UNFILTERED registry is the whole repair — no crate is rebuilt here.
 */
import { generatePluginRegistry } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";
import { syncBuiltExtensionsToInstallRoot } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📥️installation/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const registry = generatePluginRegistry(getWorkspaceRoot());
const extensions = registry.filter((entry) => entry.role === "extension");
console.log(`[b2c-extension-sync] extensions in the unfiltered registry: ${extensions.length}`);
syncBuiltExtensionsToInstallRoot(registry);
console.log("[b2c-extension-sync] done");
