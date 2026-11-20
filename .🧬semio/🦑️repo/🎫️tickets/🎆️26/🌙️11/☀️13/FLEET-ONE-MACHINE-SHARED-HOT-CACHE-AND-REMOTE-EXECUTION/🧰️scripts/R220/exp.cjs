const fs=require("node:fs"),crypto=require("node:crypto"),Module=require("node:module"),{dirname}=require("node:path");
const MOD="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs",ROOT="/Users/ueli/Documents/semio",SEL={kind:"scope",owner:"🎓️teaching"};
const load=()=>{const m=new Module(MOD,null);m.filename=MOD;m.paths=Module._nodeModulePaths(dirname(MOD));m._compile(fs.readFileSync(process.env.SRC||MOD,"utf8"),MOD);return m.exports.createBunWorkspaceOwnershipV1({ownershipPath:MOD,ownershipSource:fs.readFileSync(MOD,"utf8")});};
const ctl=()=>({signal:new AbortController().signal,advance:async()=>{}});
const calls={};["opendirSync"].forEach(n=>{const o=fs[n];fs[n]=function(...a){calls[n]=(calls[n]||0)+1;return o.apply(this,a);};});
const sha=v=>crypto.createHash("sha256").update(JSON.stringify(v)).digest("hex");
(async()=>{
 const a=load();let rows1;
 await a.withCurrentBunWorkspaceOwnershipV1(ROOT,SEL,ctl(),async c=>{rows1=sha([...c.observed().paths]);});
 await new Promise(r=>setTimeout(r,7000));
 const index=a.exportOwnershipIndexV1(ROOT);console.log("export rows",index.length,"sha",sha(index).slice(0,16),"obs",rows1.slice(0,16));
 const b=load();b.seedOwnershipIndexV1(ROOT,JSON.parse(JSON.stringify(index)));calls.opendirSync=0;let rows2;
 await b.withCurrentBunWorkspaceOwnershipV1(ROOT,SEL,ctl(),async c=>{rows2=sha([...c.observed().paths]);});
 console.log("seeded walk opendir calls",calls.opendirSync,"rows equal",rows1===rows2,"re-export equal",sha(b.exportOwnershipIndexV1(ROOT))===sha(index));
})().catch(e=>console.log("ERR",e.stack));
