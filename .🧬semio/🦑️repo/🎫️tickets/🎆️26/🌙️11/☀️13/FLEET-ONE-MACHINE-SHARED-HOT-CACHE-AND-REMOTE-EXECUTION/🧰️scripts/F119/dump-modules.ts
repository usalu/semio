import { writeFileSync } from "node:fs";
import { inventorySchemaScopes } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🧾️schema-scope-catalog/🟦️.ts";
const inv:any = inventorySchemaScopes("/Users/ueli/Documents/semio");
const have = inv.modules.map((m:any)=>({path:m.modulePath, level:m.level, scope:m.scopeId, id: m.documents.find((d:any)=>d.path===m.modulePath+"/🔣️.json")?.id ?? null}));
const miss = inv.diagnostics.filter((d:any)=>d.code==="schema-module-id-missing").map((d:any)=>({path:d.path,detail:d.detail}));
writeFileSync(process.argv[2], JSON.stringify({have, miss, defs:[...inv.definitionModules]}));
console.log(have.length, miss.length, inv.definitionModules.length);
