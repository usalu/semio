import {readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const g=".cache/b/g/m/generations/",cur=JSON.parse(readFileSync(g+"current.json","utf8")),man=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const [project,target]=process.argv.slice(2);
const row=man.owners[project];if(!row){console.log("no owner",Object.keys(man.owners).filter(k=>k.includes(project)).slice(0,10));process.exit(1);}
const frag=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+row.fragment+".json.br")).toString("utf8"));
const t=frag.node.targets[target];
console.log(JSON.stringify({keys:Object.keys(t),cache:t.cache,inputs:(t.inputs??[]).map(i=>typeof i==="string"?i:JSON.stringify(i)).slice(0,40),count:(t.inputs??[]).length,namedInputKeys:Object.keys(frag.node.namedInputs??{}),graphKeys:Object.keys(frag.node)},null,1));
console.log("rssMB",Math.round(process.memoryUsage().rss/1e6));
