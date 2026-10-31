const fs=require("node:fs"),path=require("node:path"),crypto=require("node:crypto"),directory=__dirname,text=fs.readFileSync(path.join(directory,"🗑️generated/schema-owned-metadata-consumers.bin"),"utf8"),files=new Set([...text.matchAll(/([^\0\n]+)\0(\d+)\0([^\n]*)\n/g)].map(m=>m[1]));
for(const row of JSON.parse(fs.readFileSync(path.join(directory,"🗑️generated/schema-definitions-direct-consumers.json"),"utf8")).consumers)if(row.path.startsWith("🧰️framework/🛍️products/📓️print/"))files.add(row.path);
for(const file of["🧰️framework/🛍️products/📓️print/🧬️schema/🟦️.ts","🧰️framework/🛍️products/📓️print/🚪️io/📝️text/💡️inferences/📚️catalogue/🟦️.ts","🧰️framework/🛍️products/📓️print/🚪️io/📝️text/💡️inferences/🦀️.rs"])files.add(file);
for(const match of fs.readFileSync(path.join(directory,"🗑️generated/schema-definitions-owner-metadata.bin"),"utf8").matchAll(/([^\0\n]+)\0(\d+)\0([^\n]*)\n/g))if(match[1].startsWith("🧰️framework/🛍️products/📓️print/"))files.add(match[1]);
const rows=[];
for(const file of files){const source=fs.readFileSync(file,"utf8"),changes=[];function replace(expression,replacement){for(const match of source.matchAll(expression))changes.push({start:match.index,end:match.index+match[0].length,old:match[0],replacement:typeof replacement==="function"?replacement(...match):replacement});}
replace(/\.definitions\.OptionalQuota\.policy\b/g,".definitions.OptionalQuota.definitions.DefaultPolicy.const");
replace(/\.definitions\.OptionalQuota\.optionalPath\b/g,".definitions.OptionalQuota.definitions.OptionalPath.const");
replace(/\.definitions\.OptionalQuota\.protocol\b/g,".definitions.OptionalQuota.definitions.Protocol.const");
replace(/\.definitions\.OptionalLedger\.protocol\b/g,".definitions.OptionalLedger.definitions.Protocol.const");
replace(/\bquotaSchema\.policy\b/g,"quotaSchema.definitions.DefaultPolicy.const");
replace(/\bquotaSchema\.optionalPath\b/g,"quotaSchema.definitions.OptionalPath.const");
replace(/\bquotaSchema\.protocol\b/g,"quotaSchema.definitions.Protocol.const");
replace(/\bledgerSchema\.protocol\b/g,"ledgerSchema.definitions.Protocol.const");
replace(/\boptionalPolicy\.optionalPath\b/g,"optionalPolicy.definitions.OptionalPath.const");
if(file.endsWith("/📒️ledger/🟨️.mjs"))replace(/\bschema\.protocol\b/g,"schema.definitions.Protocol.const");
replace(/\.\$defs\.ChartGuide\["x-semio-guide-defaults"\]/g,".$defs.ChartGuide.definitions.Defaults.const");
replace(/\.\$defs\.ChartGuide\["x-semio-guide-units"\]/g,".$defs.ChartGuide.definitions.Units.const");
replace(/\.\$defs\.ChartGuide\["x-semio-guide-geometry"\]/g,".$defs.ChartGuide.definitions.Geometry.const");
if(file.startsWith("🧰️framework/🛍️products/📓️print/")&&!file.endsWith("/🧬️schema/🟦️.ts"))replace(/\["x-semio-option-syntax"\]/g,".definitions.OptionSyntaxCatalog.const");
if(file.startsWith("🧰️framework/🛍️products/📓️print/")&&file.endsWith(".rs"))replace(/\.get\("x-semio-option-syntax"\)/g,'.get("definitions").and_then(|value|value.get("OptionSyntaxCatalog")).and_then(|value|value.get("const"))');

if(file.endsWith("/📓️print/🧬️schema/🟦️.ts"))replace(/readonly "x-semio-option-syntax": (Readonly<Record<string, Readonly<Record<string, VizOptionSyntaxDescriptor>>>>);/g,(_match,type)=>"readonly definitions: { readonly OptionSyntaxCatalog: { readonly const: "+type+" } };");
if(file.endsWith("/🖼️family-containment/🟦️.ts")){
 replace(/"x-semio-option-syntax":Record<string,Record<string,unknown>>/g,"definitions:{OptionSyntaxCatalog:{const:Record<string,Record<string,unknown>>}}");
 replace(/"x-semio-option-syntax":Record<string,Record<string,Record<string,unknown>>>/g,"definitions:{OptionSyntaxCatalog:{const:Record<string,Record<string,Record<string,unknown>>>}}");
}
if(file.endsWith("/🧪️tests/♻️retirement/🟦️.ts")){
 const old="quotaSchema:{...schema.definitions.OptionalQuota,policy:{...schema.definitions.OptionalQuota.policy,maximumBytes:100}}";
 const start=source.indexOf(old);if(start<0)throw Error("Owned retirement fixture shape required");
 for(let index=changes.length-1;index>=0;index--)if(changes[index].start>=start&&changes[index].end<=start+old.length)changes.splice(index,1);
 changes.push({start,end:start+old.length,old,replacement:"quotaSchema:{...schema.definitions.OptionalQuota,definitions:{...schema.definitions.OptionalQuota.definitions,DefaultPolicy:{const:{...schema.definitions.OptionalQuota.definitions.DefaultPolicy.const,maximumBytes:100}}}}"});
}
if(file.endsWith("/🧪️tests/🪟️retirement-window/🟦️.ts"))replace(/quotaSchema:\{policy,definitions:\{Headroom:\{\}\}\}/g,"quotaSchema:{definitions:{DefaultPolicy:{const:policy},Headroom:{}}}");

if(changes.length)rows.push({path:file,sha256:crypto.createHash("sha256").update(source).digest("hex"),changes});
}
fs.writeFileSync(path.join(directory,"🗑️generated/schema-metadata-reader-plan.json"),JSON.stringify({version:1,rows},null,2)+"\n");console.log(JSON.stringify({files:rows.length,changes:rows.reduce((n,r)=>n+r.changes.length,0),paths:rows.map(r=>r.path)}));
