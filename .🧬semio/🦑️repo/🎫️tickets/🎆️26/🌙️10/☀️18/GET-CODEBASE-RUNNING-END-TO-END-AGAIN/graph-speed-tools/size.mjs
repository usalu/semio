import {readdirSync,readFileSync} from "node:fs";
import {brotliDecompressSync} from "node:zlib";
const root="/Users/ueli/Documents/semio/.cache/b/g/m";
const dir=root+"/inference/np";
const exp=new Map();
function expBytes(kind,digest){
 const key=kind+digest;let v=exp.get(key);if(v!==undefined)return v;
 try{const j=JSON.parse(readFileSync(root+(kind==="SOURCE_CLOSURE"?"/build-input-closures/":"/build-lock-closures/")+digest+".json","utf8"));
  const list=kind==="SOURCE_CLOSURE"?j.files.map(f=>"{workspaceRoot}/"+f):["{workspaceRoot}/Cargo.lock","{workspaceRoot}/"+j.manifest];
  v=Buffer.byteLength(JSON.stringify(list))-2;}catch{v=0}
 exp.set(key,v);return v;}
let image=0,expanded=0,tokBytes=0,n=0,big=[];
for(const f of readdirSync(dir).filter(f=>f.endsWith(".json"))){
 let text;try{text=brotliDecompressSync(readFileSync(dir+"/"+f)).toString("utf8");}catch{continue}
 const body=text.slice(text.indexOf("\n")+1);const b=Buffer.byteLength(body);image+=b;n++;
 let pe=0;for(const m of body.matchAll(/\{"env":"SEMIO_(SOURCE_CLOSURE|CARGO_LOCK)_([a-f0-9]{64})"\}/g)){tokBytes+=m[0].length;pe+=expBytes(m[1],m[2])+1;}
 expanded+=pe;if(pe>5e6)big.push([f,pe]);
}
console.log(JSON.stringify({records:n,imageBytes:image,tokenBytes:tokBytes,expandedBytes:expanded,estimatedGraphBytes:image-tokBytes+expanded,limit:536870912,big:big.slice(0,5)}));
let distinct=0;for(const v of exp.values())distinct+=v;console.log("distinct expansion bytes",distinct,"distinct tokens",exp.size);
