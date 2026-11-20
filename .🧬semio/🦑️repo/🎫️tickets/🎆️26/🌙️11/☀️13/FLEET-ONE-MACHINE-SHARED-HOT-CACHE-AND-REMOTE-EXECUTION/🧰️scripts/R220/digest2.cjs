const fs=require("node:fs"),crypto=require("node:crypto"),Module=require("node:module"),{dirname}=require("node:path");
const MOD=process.env.MOD||"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️bun/🟨️.cjs",ROOT=process.env.TREE||"/Users/ueli/Documents/semio";
const SRC=process.env.SRC||MOD;const m=new Module(MOD,null);m.filename=MOD;m.paths=Module._nodeModulePaths(dirname(MOD));m._compile(fs.readFileSync(SRC,"utf8"),MOD);const {createBunWorkspaceOwnershipV1}=m.exports;
const own=createBunWorkspaceOwnershipV1({ownershipPath:MOD,ownershipSource:fs.readFileSync(MOD,"utf8")});
const ctl=()=>({signal:new AbortController().signal,frontierBatchPaths:64,advance:async()=>{}});
const sha=v=>crypto.createHash("sha256").update(v).digest("hex");
const run=async sel=>{let r;const t=performance.now();await own.withCurrentBunWorkspaceOwnershipV1(ROOT,sel,ctl(),async c=>{const o=c.observed();const h=crypto.createHash("sha256");let n=0;for(const row of o.paths){h.update(JSON.stringify(row)+"\n");n++;}r={sel:JSON.stringify(sel),scopes:c.scopes.map(s=>s.manifest),scopesSha:sha(JSON.stringify(c.scopes)),paths:n,pathsSha:h.digest("hex"),textsSha:sha(JSON.stringify(o.texts)),root:o.root===ROOT};});r.ms=Math.round(performance.now()-t);return r;};
(async()=>{const first=await run({kind:"all"});console.log(JSON.stringify({...first,scopes:first.scopes.length}));console.log("SCOPES",JSON.stringify(first.scopes));
 const pick=(process.env.SCOPES||"").split("|").filter(Boolean);for(const s of pick)console.log(JSON.stringify({...(await run({kind:"scope",owner:s})),scopes:undefined}));})().catch(e=>console.log("ERR",e.stack));
