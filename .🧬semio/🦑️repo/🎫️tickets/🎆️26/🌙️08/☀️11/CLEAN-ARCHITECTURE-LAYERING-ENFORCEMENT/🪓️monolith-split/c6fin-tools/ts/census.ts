import {toolJobPluginProductionSources} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏭️factory/🟦️.ts";
const m = toolJobPluginProductionSources("/Users/ueli/Documents/semio");
console.log("entries", m.size);
for (const [k, v] of m) if (/animate|mathematical|writer/.test(k) && /✏️editor\//.test(k) && !/\/(🎮️commands|🎭️modes|👥️presence|🫧️transient|📚️examples)\//.test(k)) console.log(k.slice(-90), v.length);
