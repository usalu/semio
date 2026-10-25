import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const root="/Users/ueli/Documents/semio/";
const g=root+".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const rows=[];
for(const [name,row] of Object.entries(man.owners)){
  if(row.kind!=="project")continue;
  let frag;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}
  for(const [t,d] of Object.entries(frag.node?.targets??{})){
    if(d.cache!==true||!(d.inputs??[]).includes("testSourceExclusions"))continue;
    rows.push(name+":"+t+" kind="+d.metadata?.semio?.execution?.kind);
  }
}
console.log(rows.length);console.log(rows.join("\n"));
