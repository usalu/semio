import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const g=".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const rows=[];
for(const [name,row] of Object.entries(man.owners)){
  if(row.kind!=="project")continue;
  let frag;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}
  for(const [t,def] of Object.entries(frag.node?.targets??{})){
    rows.push({project:name,target:t,cache:def.cache===true,cls:def.metadata?.semio?.cache?.class,reason:def.metadata?.semio?.cache?.reason,kind:def.metadata?.semio?.execution?.kind,inputs:def.inputs});
  }
}
process.stdout.write(JSON.stringify(rows));
