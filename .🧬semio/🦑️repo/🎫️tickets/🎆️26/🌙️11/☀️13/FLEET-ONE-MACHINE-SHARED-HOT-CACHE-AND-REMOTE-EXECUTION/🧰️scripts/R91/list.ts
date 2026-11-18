import{readFileSync,existsSync}from"node:fs";import{join}from"node:path";import{brotliDecompressSync}from"node:zlib";import{createHash}from"node:crypto";
const R="/Users/ueli/Documents/semio",B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",PN=B+"🕸️graph/📥️inference/📦️project-nodes/";
const dir=process.env.R91_WAVE!;const units=JSON.parse(readFileSync(dir+"/units.json","utf8"));
const dataDir=R+"/.cache/b/g/m/inference/selections/8d8f21d74af11ed12bed7540aa42908f48a1a023e5437e26666ef5722ed50089",store=dataDir+"/ce";
const{libraryBootstrap}=await import(B+"🕸️graph/📥️inference/🟨️.mjs");await libraryBootstrap;
const SI:any=await import(B+"🕸️graph/📥️inference/🧭️script-inputs/🟨️.mjs");
const{ownerRecordKeyV1}=await import(PN+"📋️present/🟨️.mjs") as any;
const rec=(key:string)=>{try{const t=brotliDecompressSync(readFileSync(join(dataDir,"np",key+".json"))).toString("utf8");return JSON.parse(t.slice(t.indexOf("\n")+1));}catch{return undefined;}};
const stored=(r:any)=>existsSync(store+"/e/"+createHash("sha256").update(R+"\0"+SI.closureRecipeKeyV1(r)).digest("hex")+".json.br");
let n=0,out:any[]=[];
for(const [i,u] of units.entries()){const v=rec(ownerRecordKeyV1(u.json.root));if(!v)continue;const rs=Object.values(v.closureRecipes??{}) as any[];const st=rs.filter(stored).length;out.push([i,u.name,u.kind,rs.length,st,Math.round(JSON.stringify(v).length/1024)]);if(out.length>=int(process.env.N||"30"))break;}
function int(s:string){return parseInt(s)}
for(const o of out)console.log(o.join("\t"));
process.exit(0);
