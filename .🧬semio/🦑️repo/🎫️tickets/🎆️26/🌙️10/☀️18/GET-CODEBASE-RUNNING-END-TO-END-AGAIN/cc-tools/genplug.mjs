import {readFileSync,writeFileSync,readdirSync,existsSync,statSync} from "node:fs";
import {join,dirname} from "node:path";
const [dir,file,start,...flags]=process.argv.slice(2);
const names=new Set(readFileSync(file,"utf8").split("\n").map(x=>x.trim()).filter(Boolean));
const map=new Map(),roots=new Set();
const skip=new Set(["node_modules","dist","target",".git","🗑️generated"]);
function walk(d){let es;try{es=readdirSync(d,{withFileTypes:true});}catch{return;}
 for(const e of es){if(e.isDirectory()){if(!skip.has(e.name)&&!e.isSymbolicLink())walk(join(d,e.name));}
  else if(e.name==="Cargo.toml"){const p=join(d,e.name);const t=readFileSync(p,"utf8");const m=t.match(/^\[package\][\s\S]*?^name\s*=\s*"([^"]+)"/m);if(m&&names.has(m[1]))map.set(m[1],d);if(/^\[workspace\]/m.test(t))roots.add(d);}}}
for(const top of ["✏️s","🎓️teaching","🌎️hub"])walk(top);
const by=new Map();const miss=[];
for(const n of names){const d=map.get(n);if(!d){miss.push(n);continue;}let w=d;let found=null;for(;;){if(roots.has(w)){found=w;break;}const up=dirname(w);if(up===w||up==="."){break;}w=up;}if(!found){miss.push(n+"(no ws)");continue;}if(!by.has(found))by.set(found,[]);by.get(found).push(n);}
const batches=JSON.parse(readFileSync(dir+"/cc-tools/batches.json","utf8"));let idx=Number(start);
for(const [w,list] of by){for(let i=0;i<list.length;i+=20){const chunk=list.slice(i,i+20);for(const flag of flags){batches[idx]={section:"plug "+w,cmd:`pl check --manifest-path ${w}/Cargo.toml ${chunk.map(c=>"-p "+c).join(" ")} ${flag} --keep-going --message-format=short`};console.log(idx,w,flag,chunk.length);idx++;}}}
writeFileSync(dir+"/cc-tools/batches.json",JSON.stringify(batches));console.log("missing",miss.join(" "));
