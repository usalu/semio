import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const g=".cache/b/g/m/generations/";
const cur=JSON.parse(readFileSync(g+"current.json","utf8"));
const man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const name=process.argv[2];
const row=man.owners[name];
if(!row){console.log("no owner",name);process.exit(1)}
const frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));
process.stdout.write(JSON.stringify(frag));
