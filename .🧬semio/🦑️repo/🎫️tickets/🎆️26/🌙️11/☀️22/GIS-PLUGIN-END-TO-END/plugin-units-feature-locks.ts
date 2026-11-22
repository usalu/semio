import{readFileSync,writeFileSync}from"node:fs";import{resolve}from"node:path";import{createHash}from"node:crypto";import{TOML}from"bun";
const root=resolve(import.meta.dir,"../../../../../../..");const d=resolve(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📦️units/");
const fl=JSON.parse(readFileSync(resolve(d,"📋️feature-locks.json"),"utf8"));const bad:string[]=[];
for(const r of fl){const l:any=TOML.parse(readFileSync(resolve(root,r.path),"utf8"));const pins=l.package.filter((x:any)=>typeof x.source==="string").map((x:any)=>[x.name,x.version,x.source,x.checksum??null]);const h=createHash("sha256").update(JSON.stringify(pins)).digest("hex");if(h!==r.thirdpartyPinsSha256){bad.push(r.path);r.thirdpartyPinsSha256=h}}
if(process.argv.includes("--write"))writeFileSync(resolve(d,"📋️feature-locks.json"),JSON.stringify(fl,null,2)+"\n");
console.log(fl.length,"bad",bad.length,bad.slice(0,5));
