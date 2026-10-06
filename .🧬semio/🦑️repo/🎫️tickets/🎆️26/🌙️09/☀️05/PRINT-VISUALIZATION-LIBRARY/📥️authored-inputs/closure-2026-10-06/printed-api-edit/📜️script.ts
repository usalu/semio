import {readFileSync,writeFileSync} from "node:fs";
import {createHash} from "node:crypto";
const p="🧰️framework/🛍️products/📓️print/🧬️schema/🔣️.json";
const before=readFileSync(p,"utf8"), schema=JSON.parse(before);
const families=["composition-bar","bar","dot","rank","waterfall","axisplot","timeline","narrative","composition","axis","scale"];
let after=before;
for(const family of families){
 const anchor='\n    "'+family+'": {', start=after.indexOf(anchor,after.indexOf('"x-semio-family-options"')); if(start<0)throw Error(family);
 const open=after.indexOf('"options": {',start)+'"options": {'.length;
 const spec={type:["number","string","boolean","null"],description:{en:"Result for an unknown scale input; omitted uses the undefined result sentinel.",de:"Ergebnis für eine unbekannte Skaleneingabe; ohne Angabe gilt der undefinierte Ergebniswert."}};
 if(schema["x-semio-family-options"][family].options.unknown)throw Error("existing "+family);
 after=after.slice(0,open)+'\n        "unknown": '+JSON.stringify(spec,null,2).replaceAll("\n","\n        ")+','+after.slice(open);
}
writeFileSync(p,after);
console.log(JSON.stringify({families,before:createHash("sha256").update(before).digest("hex"),after:createHash("sha256").update(after).digest("hex")}));

