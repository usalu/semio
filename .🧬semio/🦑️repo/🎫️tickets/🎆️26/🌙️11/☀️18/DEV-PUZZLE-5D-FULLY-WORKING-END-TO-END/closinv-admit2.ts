import{freshAdmissionV1}from"/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🛂️readmission/🟦️.ts";
import{closureReadWorkerScopeV1}from"/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/📡️session/🟦️.ts";
import{relative}from"node:path";
const root=process.cwd(),w="/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🕸️graph/📏️headroom/🧾️closures/👷️closure-read-worker/🟨️.mjs",s=closureReadWorkerScopeV1(root);
const a=await freshAdmissionV1(root,[relative(root,w),...s.roots],new AbortController().signal);
const files=(a.resolutions as any).files,have=new Set(files.map((r:any)=>r.path));
console.log("[DEBUG] closinv total",files.length,"journal admitted",files.filter((r:any)=>r.path.includes("📜️journal")).length);
for(const f of files)for(const[k,v]of Object.entries(f.imports))if(String(k).includes("journal"))console.log("[DEBUG] closinv",f.path,k,v,have.has(v as string));
for(const f of files)if(/funding\/.*(metadata|replacement)|backends\/.*contract\/.*operations/.test(f.path.replaceAll("💰️","funding/").replaceAll("🔌️","backends/")))console.log("[DEBUG] closinv admitted",f.path);
console.log("[DEBUG] closinv ops admitted",files.filter((r:any)=>r.path.endsWith("🔌️backends/📋️contract/🔌️operations/🟦️.ts")).length);
for(const f of files)for(const[k,v]of Object.entries(f.imports))if(String(v).endsWith("🔌️backends/📋️contract/🔌️operations/🟦️.ts")||String(k).includes("operations/🟦️"))console.log("[DEBUG] closinv ops importer",f.path,k);
import{readFileSync}from"node:fs";
for(const f of files){const t=readFileSync(f.path,"utf8");for(const m of t.matchAll(/["'](\.[^"'\n]*📜️journal\/🟦️\.ts)["']/g))if(!Object.hasOwn(f.imports,m[1]))console.log("[DEBUG] closinv undeclared",f.path,m[1]);}
for(const f of files){const t=readFileSync(f.path,"utf8");for(const m of t.matchAll(/^\s*import\s*\{\s*(type\s+[\w$]+(?:\s+as\s+[\w$]+)?\s*,?\s*)+\}\s*from\s*["']([^"']+)["']/gm))if(!Object.hasOwn(f.imports,m[2]))console.log("[DEBUG] closinv alltype",f.path,m[2]);}
const targets=new Set(files.filter((r:any)=>r.path.includes("📋️data/📥️load")).map((r:any)=>r.path));console.log("[DEBUG] closinv load admitted",[...targets].length);
for(const f of files)for(const[k,v]of Object.entries(f.imports))if(targets.has(v as string))console.log("[DEBUG] closinv loadimporter",f.path,k);
let cur=["🧮️digest/📥️read/🟨️.mjs"],seen=new Set<string>();
for(let d=0;d<8;d++){const next:string[]=[];for(const c of cur)for(const f of files)for(const[k,v]of Object.entries(f.imports))if((v as string).endsWith(c)&&!seen.has(f.path)){seen.add(f.path);console.log("[DEBUG] closinv chain",d,f.path.slice(-90),"<-imports",c.slice(-40));next.push(f.path);}cur=next;if(!cur.length)break;}
