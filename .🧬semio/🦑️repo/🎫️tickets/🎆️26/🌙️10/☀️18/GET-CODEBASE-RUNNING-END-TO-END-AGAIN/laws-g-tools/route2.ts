import{readFileSync}from"node:fs";import{join,resolve,relative,sep}from"node:path";
import{discoverProcessTestFilesV1,routeProcessTestFilesV1,readProcessTestTableV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🟦️.ts";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",P=R+"🧰️framework/🔨️modules/🏃️process/";
const roots:Record<string,[string,string]>={process:[P,P+"🧪️testing/🗂️suites/🔣️.json"],build:[L+"🏗️build",L+"🏗️build/🗂️suites/🔣️.json"],caching:[L+"⚡️caching",L+"⚡️caching/🗂️suites/🔣️.json"]};
const tables=Object.fromEntries(Object.entries(roots).map(([k,[root,tp]])=>[k,{root,table:readProcessTestTableV1(JSON.parse(readFileSync(tp,"utf8"))),found:discoverProcessTestFilesV1(root)}]));
const probe=process.argv.slice(2);
for(const f of probe){
 const out:string[]=[];
 for(const[k,{root,table,found}]of Object.entries(tables)){
  const rel=relative(root,f).split(sep).join("/");
  const owners=routeProcessTestFilesV1(found.includes(rel)?[rel]:[rel],table).get(rel);
  if(owners)out.push(k+":"+owners.map(s=>s.id).join("+"));
 }
 console.log(f.slice(R.length),"=>",out.join(" | ")||"(none)");
}
