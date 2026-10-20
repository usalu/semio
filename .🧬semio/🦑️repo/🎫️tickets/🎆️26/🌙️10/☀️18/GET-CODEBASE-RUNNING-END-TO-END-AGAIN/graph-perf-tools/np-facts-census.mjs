// 📊 Counts rows and distinct facts of the persisted per-project inference records (read-only).
import {readdirSync,readFileSync,statSync} from "node:fs";import {brotliDecompressSync} from "node:zlib";import {join} from "node:path";
const dir=process.argv[2],limit=Number(process.argv[3]??"100000"),stride=Number(process.argv[4]??"1");
const files=readdirSync(dir).filter(f=>f.endsWith(".json")).sort().filter((_,i)=>i%stride===0).slice(0,limit);
const started=performance.now();let rows=0,bytes=0,headers=0;const distinct=new Map(),kinds=new Map(),identity=new Map(),paths=new Set();let t_dec=0,t_parse=0;
for(const f of files){
 let t=performance.now();const raw=readFileSync(join(dir,f));bytes+=raw.length;let text;try{text=brotliDecompressSync(raw).toString("utf8");}catch{continue;}t_dec+=performance.now()-t;
 t=performance.now();const nl=text.indexOf("\n"),header=JSON.parse(text.slice(0,nl));t_parse+=performance.now()-t;headers++;
 for(const row of header.observations??[]){rows++;const k=row.kind+"\0"+row.path+"\0"+JSON.stringify(row.fact);distinct.set(k,(distinct.get(k)??0)+1);kinds.set(row.kind,(kinds.get(row.kind)??0)+1);paths.add(row.path);}
}
const dk=new Map();for(const k of distinct.keys()){const kind=k.split("\0")[0];dk.set(kind,(dk.get(kind)??0)+1);}
console.log(JSON.stringify({files:files.length,headers,compressedMB:+(bytes/1e6).toFixed(1),rows,distinctFacts:distinct.size,distinctPaths:paths.size,rowKinds:Object.fromEntries(kinds),distinctByKind:Object.fromEntries(dk),decompressMs:Math.round(t_dec),parseMs:Math.round(t_parse),totalMs:Math.round(performance.now()-started)}));
