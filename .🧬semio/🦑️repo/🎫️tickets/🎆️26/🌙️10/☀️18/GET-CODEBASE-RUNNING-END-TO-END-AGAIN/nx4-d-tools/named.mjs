import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const g=".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const [project,group]=process.argv.slice(2);
const row=man.owners[project];
const frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));
const ni=frag.node.namedInputs;
if(!group){for(const [k,v] of Object.entries(ni))console.log(k,Array.isArray(v)?v.length:typeof v);process.exit(0)}
console.log(JSON.stringify(ni[group],null,0));
