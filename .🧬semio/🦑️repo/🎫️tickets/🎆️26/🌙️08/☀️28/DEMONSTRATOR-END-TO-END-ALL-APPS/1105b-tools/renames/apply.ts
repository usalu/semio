import {readdirSync,readFileSync,writeFileSync,renameSync} from "node:fs";
import {join} from "node:path";
const R="/Users/ueli/Documents/semio",APPLY=process.argv[2]==="apply",SKIP=new Set(["node_modules","target",".git","🗑️generated","dist",".🧬semio"]);
const names=new Set<string>(),pkgFiles:string[]=[],all:string[]=[];
const walk=(d:string)=>{for(const e of readdirSync(d,{withFileTypes:true})){if(e.isDirectory()){if(SKIP.has(e.name)||e.name.startsWith(".cache"))continue;walk(join(d,e.name));}else if(e.name==="Cargo.toml"){const p=join(d,e.name);all.push(p);pkgFiles.push(p);}else if(e.name.endsWith(".rs"))all.push(join(d,e.name));}};
walk(R);
const FW=/^(\[package\][^\[]*?^name\s*=\s*")semio-framework-((?:os|plugin|ui)(?:-[a-z0-9]+)*)(")/ms;
const fromTemp=(t:string)=>t.match(/^\[package\][^\[]*?^name\s*=\s*"([^"]+)"/ms)?.[1];
for(const p of pkgFiles){const n=fromTemp(readFileSync(p,"utf8"));if(n)names.add(n);}
const renamed:string[]=[];
for(const n of [...names])if(/^semio-framework-(os|plugin|ui)(-|$)/.test(n)){const s=n.replace("semio-framework-","semio-");if(names.has(s)){console.log("COLLISION",n,s);process.exit(1);}renamed.push(n);}
for(const n of renamed){names.delete(n);names.add(n.replace("semio-framework-","semio-"));}
const reT=/(?<![\w-])semio-framework-((?:os|plugin|ui)(?:-[a-z0-9]+)*)(?![\w-])/g,reU=/(?<![\w-])semio_framework_((?:os|plugin|ui)(?:_[a-z0-9]+)*)(?![\w-])/g;
const reR=/(?<![\w"'-])semio_framework_((?:os|plugin|ui)(?:_[a-z0-9]+)*)(?=\s*::|\s*;|\s+as\s)/g;
let changed=0,hits=0;const unk=new Map<string,number>();
for(const p of all){
 const old=readFileSync(p,"utf8");if(!old.includes("semio-framework-")&&!old.includes("semio_framework_"))continue;
 let n=0;const rep=(re:RegExp,sep:string)=>(s:string)=>s.replace(re,(m,r)=>{const h=("semio-"+r.split(sep).join("-")).replace(/^semio-os-renderer-/,"semio-renderer-");if(names.has(h)){n++;return sep==="_"?h.replaceAll("-","_"):h;}unk.set(m,(unk.get(m)??0)+1);return m;});
 const out=p.endsWith("Cargo.toml")?rep(reU,"_")(rep(reT,"-")(old)):rep(reR,"_")(old);
 if(out===old)continue;hits+=n;changed++;
 if(APPLY){if(readFileSync(p,"utf8")!==old){console.log("RACE skip",p);continue;}const t=p+".renames.tmp";writeFileSync(t,out);renameSync(t,p);}
}
console.log(APPLY?"applied":"dry",{renamedPackages:renamed.length,files:changed,tokens:hits});
console.log([...unk].sort((a,b)=>b[1]-a[1]).slice(0,30));
