import{readFileSync}from"node:fs";import{resolve}from"node:path";import{TOML}from"bun";
const root="/Users/ueli/Documents/semio";const locks=JSON.parse(readFileSync(resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/📋️locks.json"),"utf8"));
const c=(n:string)=>JSON.stringify((TOML.parse(readFileSync(resolve(root,"Cargo.lock"),"utf8")) as any).package.filter((x:any)=>x.name===n).map((x:any)=>x.dependencies));
const canon=c("semio-plugin-ownership"),reg=c("semio-plugin-registration");let n=0;
for(const p of locks){const l:any=TOML.parse(readFileSync(resolve(root,p),"utf8"));const f=(k:string)=>JSON.stringify(l.package.filter((x:any)=>x.name===k).map((x:any)=>x.dependencies));if(f("semio-plugin-ownership")!==canon||f("semio-plugin-registration")!==reg){n++;console.log(p.slice(-70),"own",f("semio-plugin-ownership")===canon,"reg",f("semio-plugin-registration")===reg?"ok":f("semio-plugin-registration").slice(0,40))}}
console.log("stale",n,"of",locks.length);
