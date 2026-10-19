import { readFileSync } from "node:fs";
import { toolJobProductionSource, toolJobProductionProtocolSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/📋️inventory/🟦️.ts";
const t = readFileSync(process.argv[2]!, "utf8");
console.log(t.includes("SOURCING_CURATION_SCHEMA: &str"), toolJobProductionSource(t).includes("SOURCING_CURATION_SCHEMA: &str"), toolJobProductionProtocolSource(t).includes("SOURCING_CURATION_SCHEMA: &str"));
