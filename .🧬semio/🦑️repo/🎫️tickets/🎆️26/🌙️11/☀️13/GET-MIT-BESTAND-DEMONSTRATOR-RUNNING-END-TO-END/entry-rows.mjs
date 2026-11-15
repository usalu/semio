import {readdirSync,readFileSync,statSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
import {join} from "node:path";
const d=process.argv[2]+"/e",names=readdirSync(d).filter(n=>n.endsWith(".json.br")).slice(0,Number(process.argv[3]??5));
for(const n of names){const r=JSON.parse(brotliDecompressSync(readFileSync(join(d,n)),{maxOutputLength:1<<29}).toString());
 const kinds={};for(const row of r.rows){kinds[row.kind]=(kinds[row.kind]??0)+1;}
 console.log(n.slice(0,8),JSON.stringify(r.recipe).slice(0,120),kinds);
 for(const row of r.rows)if(row.kind==="analysis"||row.kind==="digest")console.log("  ",row.kind,row.path.slice(-80),JSON.stringify(row.fact).slice(0,260));
}
