import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const g=".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const needle=process.argv[2];
for(const [name,row] of Object.entries(man.owners)){
  if(row.kind!=="project")continue;
  let frag;try{frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));}catch{continue}
  const text=JSON.stringify(frag.node?.namedInputs??{});
  if(text.includes(needle)) console.log(name, frag.root);
}
