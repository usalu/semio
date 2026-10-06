import{readFileSync,writeFileSync,renameSync}from"node:fs";
const ticket=process.env.SEMIO_TICKET_DIR!,path="C:/git/semio/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧫️fixtures/🔓️api-freshness.json",before=readFileSync(path,"utf8"),vectors=JSON.parse(before).printed.packageRows.slice(0,2);
if(vectors[0].id!=="bare-empty-default-marker"||vectors[1].id!=="literal-dash-default-value")throw Error("neutral order guard");
const marker='"packageRows": [',prefix=marker+'\n'+vectors.map((vector:any)=>JSON.stringify(vector,null,2).split('\n').map(line=>'      '+line).join('\n')+',').join('\n');if(before.split(prefix).length!==2)throw Error("neutral exact prefix");
let after=before.replace(prefix,marker),start=after.indexOf(marker)+marker.length-1,depth=0,quoted=false,escaped=false,end=-1;
for(let index=start;index<after.length;index++){const c=after[index];if(quoted){if(escaped)escaped=false;else if(c==='\\')escaped=true;else if(c==='"')quoted=false;}else if(c==='"')quoted=true;else if(c==='[')depth++;else if(c===']'&&!--depth){end=index;break;}}
if(end<0)throw Error("neutral array boundary");const tail=after.slice(0,end).trimEnd(),added=vectors.map((vector:any)=>JSON.stringify(vector,null,2).split('\n').map(line=>'      '+line).join('\n')).join(',\n');after=tail+',\n'+added+'\n    '+after.slice(end);JSON.parse(after);
const candidate=ticket+"/🗑️generated/gallery-empty-default-order/fixture-candidate.json";writeFileSync(candidate,after);if(readFileSync(path,"utf8")!==before)throw Error("concurrent fixture write");renameSync(candidate,path);
const{verifyPrintApiFreshness}=await import("C:/git/semio/🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🟦️.ts");verifyPrintApiFreshness();
