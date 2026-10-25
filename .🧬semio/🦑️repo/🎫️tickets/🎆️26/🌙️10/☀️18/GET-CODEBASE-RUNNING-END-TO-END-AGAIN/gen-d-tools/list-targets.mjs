import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const root="/Users/ueli/Documents/semio/";
const g=root+".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const re=new RegExp(process.argv[2]);
for(const [name,row] of Object.entries(man.owners)){
  if(row.kind!=="project")continue;
  let frag;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}
  for(const [t,d] of Object.entries(frag.node?.targets??{})){
    if(!re.test(t)||d.cache!==true)continue;
    const s=d.metadata?.semio;
    console.log([name,t,"kind="+s?.execution?.kind,"prec="+(s?.inputPrecision?.rule??"-"),"inputs="+(d.inputs?.length),"cmd="+String(d.options?.command??"").slice(0,90)].join(" | "));
  }
}
