import{readFileSync,writeFileSync,mkdirSync,copyFileSync,renameSync,rmSync}from"node:fs";import{createHash}from"node:crypto";import{dirname}from"node:path";
const R="/Users/ueli/Documents/semio/",SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R119/",BK=SC+"backup/",here=import.meta.dir+"/";
const{all,apply}=await import(here+"patches.mjs");
const sha=p=>createHash("sha256").update(readFileSync(p)).digest("hex").slice(0,12);
const keys=[...new Set(all.map(a=>a.file))];
const out=new Map();for(const k of keys){out.set(k,apply(k,readFileSync(R+k,"utf8"),all));}
const I="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/🧬️read-verdicts/";
const law=readFileSync(here+"test/law.test.ts","utf8").replace('import{join}from"node:path";','import{join}from"node:path";\nimport{fileURLToPath}from"node:url";').replace('const B=process.env.R119_B!,CHILD=process.env.R119_CHILD!;','const B=fileURLToPath(new URL("../../../../",import.meta.url)).replace(/\\/$/u,""),CHILD=join(import.meta.dir,"👷️child","🟦️.ts");process.env.R119_B=B;');
const child=readFileSync(here+"test/child.ts","utf8");
if(process.argv.includes("--dry")){console.log("dry ok",[...out].map(([k,v])=>k.slice(-30)+" "+v.length).join(" | "));process.exit(0);}
rmSync(BK,{recursive:true,force:true});const before={};for(const k of keys){before[k]=sha(R+k);mkdirSync(dirname(BK+k),{recursive:true});copyFileSync(R+k,BK+k);}
for(const[k,text]of out){writeFileSync(R+k+".r119tmp",text);renameSync(R+k+".r119tmp",R+k);}
mkdirSync(R+I+"👷️child",{recursive:true});writeFileSync(R+I+"🟦️.ts",law);writeFileSync(R+I+"👷️child/🟦️.ts",child);
for(const k of keys)console.log(k.slice(-40),before[k],"->",sha(R+k));
