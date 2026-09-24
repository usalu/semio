import { existsSync } from "node:fs";
import { loadOracleRegistry } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts";
const r = loadOracleRegistry("/Users/ueli/Documents/semio");
for (const o of r.oracles as any[]) if (o.hostPath !== undefined) console.log(o.kind, o.ecosystem, o.id, existsSync(`/Users/ueli/Documents/semio/${o.hostPath}`) ? "EXISTS" : "MISSING", o.hostPath);
