import{readFileSync}from"node:fs";
import{validateJsonSchemaSubset}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{readProcessTestTableV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🟦️.ts";
const dir="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/";
const schema=JSON.parse(readFileSync(dir+"🧬️schema/🔣️.json","utf8"));
for(const path of process.argv.slice(2)){
 const value=JSON.parse(readFileSync(path,"utf8"));
 console.log(path.split("/").slice(-4).join("/"),JSON.stringify(validateJsonSchemaSubset(schema.definitions.Table,value,schema)));
 try{readProcessTestTableV1(value);console.log("  ok");}catch(e){console.log("  refused",String(e));}
}
const R="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const c=await import(R+"⚡️caching/🗂️suites/🟦️.ts"),b=await import(R+"🏗️build/🗂️suites/🟦️.ts");
console.log("caching coverage",JSON.stringify(c.cachingTestCoverageV1()).slice(0,1500));
console.log("build coverage",JSON.stringify(b.buildTestCoverageV1()).slice(0,1500));
