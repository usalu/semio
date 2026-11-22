import{readFileSync,writeFileSync}from"node:fs";import{resolve}from"node:path";import{TOML}from"bun";
const root="/Users/ueli/Documents/semio";const jf=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️feature-consumers.json"),rec=JSON.parse(readFileSync(jf,"utf8"));
const l:any=TOML.parse(readFileSync(resolve(root,"Cargo.lock"),"utf8")),dep=l.package.find((x:any)=>x.name===rec.consumer).dependencies as string[];
console.log("added",dep.filter(x=>!rec.dependencies.includes(x)),"removed",rec.dependencies.filter((x:string)=>!dep.includes(x)));
const roots=rec.roots.map((r:any)=>{const k:any=TOML.parse(readFileSync(resolve(root,r.path),"utf8"));return{...r,now:k.package.some((x:any)=>x.name===rec.consumer)}});console.log("presence changed",roots.filter((r:any)=>r.now!==r.present).map((r:any)=>r.path.slice(-50)));
if(process.argv.includes("--write")){rec.dependencies=dep;writeFileSync(jf,JSON.stringify(rec,null,2)+"\n")}
