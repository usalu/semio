import {mkdirSync,writeFileSync,chmodSync} from "node:fs";
import {createHash,randomBytes} from "node:crypto";
import {join} from "node:path";
const [,, base, acN, nxN, casN]=process.argv, ac=Number(acN), nx=Number(nxN), cas=Number(casN);
const dir=join(base,"dir","v1"),host=join(base,"cas","v1");
const hex=()=>randomBytes(32).toString("hex");
const made=new Set<string>();
const put=(path:string,data:string|Uint8Array)=>{const d=path.slice(0,path.lastIndexOf("/"));if(!made.has(d)){mkdirSync(d,{recursive:true});made.add(d);}writeFileSync(path,data);};
const objects:string[]=[];
for(let i=0;i<cas;i++){const body=randomBytes(64+(i%7)*50),h=createHash("sha256").update(body).digest("hex");put(join(host,"o",h.slice(0,2),h.slice(2)),body);objects.push(h);}
const slots=Array.from({length:Math.max(1,Math.floor(ac/4))},hex);
const now=Date.now();
for(let i=0;i<ac;i++){const key=hex(),rel=`main/ac/${key.slice(0,2)}/${key}`;put(join(dir,...rel.split("/")),randomBytes(200));
 const refs=[objects[i%objects.length]!,objects[(i*7+3)%objects.length]!];
 put(join(dir,".meta",...rel.split("/"))+".json",JSON.stringify({version:1,slot:slots[i%slots.length],storedAt:now-(ac-i)*50,refs}));}
for(let i=0;i<nx;i++){const key=hex().slice(0,40),rel=`main/nx/${key.slice(0,2)}/${key}`,h=objects[(i*13)%objects.length]!;put(join(dir,...rel.split("/")),JSON.stringify({version:1,hash:h,size:100}));
 put(join(dir,".meta",...rel.split("/"))+".json",JSON.stringify({version:1,slot:slots[(i*3)%slots.length],storedAt:now-(nx-i)*50,refs:[h]}));}
console.log("filled",ac,nx,cas);
