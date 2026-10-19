import {readFileSync,writeFileSync,mkdirSync,renameSync,existsSync,statSync,readdirSync,rmdirSync} from "node:fs";
import {dirname,join,relative,resolve,sep} from "node:path";
import {spawnSync} from "node:child_process";
import ts from "typescript";
const repo=process.cwd();
const oldDir=resolve(process.argv[2]!),newDir=resolve(process.argv[3]!),apply=process.argv.includes("--apply");
const listed=spawnSync("git",["ls-files","-co","--exclude-standard","-z"],{cwd:repo,maxBuffer:1<<28}).stdout.toString("utf8").split("\0").filter(Boolean);
const codeExt=/\.(ts|tsx|mts|mjs|js|cjs)$/u;
const within=(p:string,dir:string)=>p===dir||p.startsWith(dir+sep);
const mapPath=(p:string)=>within(p,oldDir)?join(newDir,relative(oldDir,p)):p;
const packages=new Map<string,{directory:string;exports:Record<string,unknown>}>();
for(const f of listed){if(!f.endsWith("package.json")||!f.startsWith("🧰️framework/")||f.includes("node_modules"))continue;try{const j=JSON.parse(readFileSync(f,"utf8"));if(typeof j.name==="string"&&j.exports)packages.set(j.name,{directory:resolve(dirname(f)),exports:typeof j.exports==="string"?{".":j.exports}:j.exports});}catch{}}
const target=(v:unknown):string|undefined=>typeof v==="string"?v:v&&typeof v==="object"?target((v as any).bun??(v as any).import??(v as any).default):undefined;
const resolveBare=(spec:string)=>{const names=[...packages.keys()].filter(n=>spec===n||spec.startsWith(n+"/")).sort((a,b)=>b.length-a.length);for(const n of names){const p=packages.get(n)!,v=target(p.exports["."+spec.slice(n.length)]);if(v)return resolve(p.directory,v);}return undefined;};
const rel=(from:string,to:string)=>{let p=relative(from,to).split(sep).join("/");return p.startsWith(".")?p:"./"+p;};
type Edit={start:number;end:number;text:string};
const specRe=/((?:import|export)\b[^;'"`]*?from\s*|import\s*\(\s*|import\s*(?=["'])|new URL\(\s*)(["'])([^"'\n]+)\2/gm;
const changes:string[]=[],warnings:string[]=[];
const rewrite=(file:string,newFile:string,text:string,moved:boolean):string=>{
 return text.replace(specRe,(whole,head,quote,spec)=>{
  const q=spec.indexOf("?"),clean=q<0?spec:spec.slice(0,q),suffix=q<0?"":spec.slice(q);
  let abs:string|undefined;
  if(clean.startsWith("."))abs=resolve(dirname(file),clean);
  else if(moved&&clean.startsWith("@semio-tech/"))abs=resolveBare(clean);
  else return whole;
  if(!abs)return whole;
  if(!moved&&!within(abs,oldDir))return whole;
  const mapped=mapPath(abs);
  const next=rel(dirname(newFile),mapped)+suffix;
  if(next===spec)return whole;
  changes.push(relative(repo,file)+": "+spec+" -> "+next);
  return head+quote+next+quote;
 });
};
const movedFiles=listed.filter(f=>within(resolve(f),oldDir)&&existsSync(f));
const movedSet=new Set(movedFiles.map(f=>resolve(f)));
const outputs:{path:string;text:string;remove?:string}[]=[];
for(const f of movedFiles){const abs=resolve(f),dest=mapPath(abs);const buf=readFileSync(abs);if(!codeExt.test(abs)){outputs.push({path:dest,text:buf.toString("utf8"),remove:abs});continue;}outputs.push({path:dest,text:rewrite(abs,dest,buf.toString("utf8"),true),remove:abs});}
for(const f of listed){const abs=resolve(f);if(movedSet.has(abs)||!codeExt.test(abs)||!existsSync(abs))continue;let text:string;try{text=readFileSync(abs,"utf8");}catch{continue;}if(!text.includes("."))continue;const out=rewrite(abs,abs,text,false);if(out!==text)outputs.push({path:abs,text:out});}
const tail=oldDir.split(sep).slice(-3).join("/")+"/";
const other=spawnSync("/usr/bin/grep",["-rIl","--exclude-dir=node_modules","--exclude-dir=.git","--exclude-dir=.cache","--exclude-dir=target","-F",tail,"🧰️framework","✏️s",".vscode","📋️project.json","📜️script.ts","package.json"],{cwd:repo,maxBuffer:1<<28}).stdout.toString("utf8").split("\n").filter(Boolean);
console.log(JSON.stringify({moved:movedFiles.length,importersChanged:outputs.filter(o=>!o.remove).length,changes:changes.length},null,0));
for(const c of changes)console.log("  "+c);
console.log("string references to "+tail+":");for(const o of other)console.log("  "+o);
if(!apply){console.log("(dry run)");process.exit(0);}
for(const o of outputs){mkdirSync(dirname(o.path),{recursive:true});writeFileSync(o.path,o.text);if(o.remove&&o.remove!==o.path)(await import("node:fs")).unlinkSync(o.remove);}
const prune=(d:string)=>{if(!existsSync(d)||!statSync(d).isDirectory())return;for(const e of readdirSync(d)){const p=join(d,e);if(statSync(p).isDirectory())prune(p);}if(!readdirSync(d).length)rmdirSync(d);};
prune(oldDir);
console.log("applied");
const stringEdits:string[]=[];
for(const f of movedFiles){
 const oldRel=f,newRel=relative(repo,mapPath(resolve(f)));
 const hits=spawnSync("git",["grep","-l","-I","--untracked","-F",oldRel,"--",".",":!.🧬semio",":!*node_modules*"],{cwd:repo,maxBuffer:1<<28}).stdout.toString("utf8").split("\n").filter(Boolean);
 for(const h of hits){const t=readFileSync(h,"utf8");if(!t.includes(oldRel))continue;writeFileSync(h,t.split(oldRel).join(newRel));stringEdits.push(h+": "+oldRel+" -> "+newRel);}
}
for(const e of stringEdits)console.log("  string: "+e);
const risky=outputs.filter(o=>o.remove).filter(o=>/import\.meta\.(dir|path)|__dirname|process\.cwd\(\)/.test(o.text)).map(o=>relative(repo,o.path));
console.log("moved files with dir/cwd-relative logic to inspect:");for(const r of risky)console.log("  "+r);
