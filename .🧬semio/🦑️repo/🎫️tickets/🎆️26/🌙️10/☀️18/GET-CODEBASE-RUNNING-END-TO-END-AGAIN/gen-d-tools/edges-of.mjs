import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const g=".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
for(const [name,row] of Object.entries(man.owners)){
  if(row.kind!=="project"||name!==process.argv[2])continue;
  const frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));
  console.log(JSON.stringify(frag.edges,null,1)); console.log(frag.root);
  if(process.argv[3])console.log(JSON.stringify(frag.node,null,1));
}
