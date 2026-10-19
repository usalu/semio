import{readFileSync,readdirSync,statSync,existsSync}from"node:fs";import{dirname,join,relative,resolve,extname}from"node:path";import{isBuiltin}from"node:module";
const root=process.cwd(),entry=process.argv[2];const par=new Map<string,string>(),seen=new Set<string>(),q=[entry],sd=new Set<string>();
const sch=(d:string)=>{if(sd.has(d))return;sd.add(d);if(!existsSync(join(root,d)))return;for(const n of readdirSync(join(root,d))){const p=d+"/"+n;if(statSync(join(root,p)).isDirectory())sch(p);else if(extname(n)===".json")q.push(p);}};
par.set(entry,"");
while(q.length){const p=q.shift()!;if(seen.has(p))continue;seen.add(p);if(extname(p)===".json")continue;const text=readFileSync(join(root,p),"utf8");sch((dirname(p)==="."?"":dirname(p)+"/")+"🧬️schema");
for(const m of text.matchAll(/(["'])(\.{1,2}\/[^"'\\\r\n]+\.(?:[cm]?[jt]sx?|json))\1/gu)){const s=m[2];if(/\.d\.[cm]?ts$/u.test(s))continue;const c=relative(root,resolve(root,dirname(p),s));if(c.split("/").some(x=>x==="node_modules"||x===".cache"))continue;if(existsSync(join(root,c))){if(!par.has(c))par.set(c,p);q.push(c);}}
const imports=new Bun.Transpiler({loader:/\.[cm]?tsx?$/u.test(p)?"ts":"js",target:"bun",define:{"import.meta.vitest":"undefined"}}).scan(text.replace(/^#![^\r\n]*/u,"")).imports;
for(const row of imports){let s=row.path;if(/[\u0080-ÿ]/u.test(s)&&!/[Ā-￿]/u.test(s)){const d=Buffer.from(s,"latin1").toString("utf8");if(Buffer.from(d).toString("latin1")===s)s=d;}if(s.startsWith("node:")||s.startsWith("bun:")||isBuiltin(s))continue;const r=relative(root,Bun.resolveSync(s,dirname(join(root,p))));if(r.split("/").some(x=>x==="node_modules"||x===".cache")){console.log("EXTERNAL",s,"<-",p);continue;}if(!par.has(r))par.set(r,p);q.push(r);}}
console.log("files",seen.size);
let t=process.argv[3];while(t){console.log("  ",t);t=par.get(t)!;}
