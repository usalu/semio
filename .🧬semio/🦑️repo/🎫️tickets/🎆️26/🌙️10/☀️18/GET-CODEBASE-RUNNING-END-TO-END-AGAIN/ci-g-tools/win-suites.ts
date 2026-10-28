import{readFileSync}from"node:fs";import{relative,sep}from"node:path";
import{discoverProcessTestFilesV1,routeProcessTestFilesV1,readProcessTestTableV1}from"/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🟦️.ts";
const R="/Users/ueli/Documents/semio/",L=R+"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/",P=R+"🧰️framework/🔨️modules/🏃️process/";
const roots:Record<string,[string,string]>={process:[P,P+"🧪️testing/🗂️suites/🔣️.json"],build:[L+"🏗️build",L+"🏗️build/🗂️suites/🔣️.json"],caching:[L+"⚡️caching",L+"⚡️caching/🗂️suites/🔣️.json"]};
const out=new Map<string,string[]>();
for(const[k,[root,tp]]of Object.entries(roots)){
 const table=readProcessTestTableV1(JSON.parse(readFileSync(tp,"utf8"))),found=discoverProcessTestFilesV1(root);
 const win=found.filter(f=>/🪟️|windows|win32/iu.test(f));
 for(const f of win){const owners=routeProcessTestFilesV1([f],table).get(f);const key=k+":"+(owners?.map(s=>s.id).join("+")??"(none)");(out.get(key)??out.set(key,[]).get(key)!).push(f);}
}
for(const[k,v]of out)console.log(k,v.length,v.map(f=>f).join(" ; "));
