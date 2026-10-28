/** 🧮 Exhaustive oracle on a real tree (read-only): for every package of the whole-repository inventory, the declared selection near its own manifest and near its workspace root returns the identical row or defers (undefined); counts mismatches. `bun nearall.ts <module-root> <tree-root>` */
const [moduleRoot,root]=process.argv.slice(2) as [string,string];
const ws=await import(moduleRoot+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts");
const all=ws.cargoRepositoryPackages(root) as {directory:string;manifest:string;name:string;workspace:string}[];
let same=0,deferred=0,wrong=0,ms=0;const deferredBy:Record<string,number>={};
for(const row of all)for(const near of [row.manifest,(row.workspace==="."?"":row.workspace+"/")+"Cargo.toml"]){const a=performance.now(),got=ws.cargoDeclaredPackageSelectionsV1(root,near,[row.name]);ms+=performance.now()-a;
 if(got===undefined){deferred++;deferredBy[row.workspace]=(deferredBy[row.workspace]??0)+1;}else if(JSON.stringify(got)===JSON.stringify([row]))same++;else{wrong++;if(wrong<5)console.log("[DEBUG] WRONG",row.name,near,JSON.stringify(got));}}
console.log(`[DEBUG] packages ${all.length} lookups ${2*all.length} identical ${same} deferred ${deferred} wrong ${wrong} mean ${(ms/(2*all.length)).toFixed(2)} ms`);
console.log("[DEBUG] deferred by workspace",JSON.stringify(Object.entries(deferredBy).sort((a,b)=>b[1]-a[1]).slice(0,8)));
