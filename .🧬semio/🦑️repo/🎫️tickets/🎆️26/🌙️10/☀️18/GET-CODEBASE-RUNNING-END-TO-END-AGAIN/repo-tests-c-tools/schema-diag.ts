import {schemaContractDiagnostics,TEST_DOMAIN_REL_PATH,repoRootFromHere} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
const root=repoRootFromHere();
for(const d of schemaContractDiagnostics(root,TEST_DOMAIN_REL_PATH))console.log("[prof]",JSON.stringify(d).slice(0,800));
