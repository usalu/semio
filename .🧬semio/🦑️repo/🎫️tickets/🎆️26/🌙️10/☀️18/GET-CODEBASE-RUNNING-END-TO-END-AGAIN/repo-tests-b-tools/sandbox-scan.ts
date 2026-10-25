import{readFileSync}from"node:fs";import{join}from"node:path";
const Z="/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🏖️sandbox";
const{compileLawV1,scanTestsV1}=await import(Z+"/🟦️.ts");const protocol=JSON.parse(readFileSync(Z+"/🔣️.json","utf8"));
const law=compileLawV1(protocol),root="/Users/ueli/Documents/semio";
const files=process.argv.slice(2);
const r=scanTestsV1(law,files,(p:string)=>{try{return readFileSync(join(root,p),"utf8");}catch{return undefined;}});
for(const f of r.findings)console.log(f.rule,f.file,f.line,JSON.stringify(f).slice(0,260));
