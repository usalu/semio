// keys3-g: prints the full input list of project:target from the sealed generation. usage: bun dump-inputs.mjs <project> <target>...
import {readFileSync} from "node:fs"; import {brotliDecompressSync} from "node:zlib";
const g="/Users/ueli/Documents/semio/.cache/b/g/m/generations/"; const cur=JSON.parse(readFileSync(g+"current.json","utf8")); const m=JSON.parse(readFileSync(g+"m/"+cur.manifest+".json","utf8"));
const [p,...ts]=process.argv.slice(2);
const f=JSON.parse(brotliDecompressSync(readFileSync(g+"o/"+m.owners[p].fragment+".json.br")).toString("utf8"));
for (const t of ts) { const d=f.node.targets[t]; console.log("==",p,t,JSON.stringify(d.options), JSON.stringify(d.metadata));
 for (const i of d.inputs??[]) console.log("  ", typeof i==="string"? i.replace("{workspaceRoot}/","").slice(0,170): JSON.stringify(i).slice(0,170)); }
