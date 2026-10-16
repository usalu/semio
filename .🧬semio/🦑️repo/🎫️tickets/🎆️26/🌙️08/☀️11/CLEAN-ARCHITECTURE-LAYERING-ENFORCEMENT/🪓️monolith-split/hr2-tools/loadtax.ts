/** ✅️ hr2: loads the live taxonomy through the real validator. */
import { loadTaxonomy } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
try { const t = loadTaxonomy(); console.log("loadTaxonomy ok", Object.keys(t.subsetDirectoryOverrides ?? {}).length); } catch (e) { console.log("FAIL", String(e).slice(0, 600)); }
