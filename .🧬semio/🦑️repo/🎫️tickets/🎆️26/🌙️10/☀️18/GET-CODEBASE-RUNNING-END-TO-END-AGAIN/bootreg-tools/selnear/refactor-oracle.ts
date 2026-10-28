/** ⚖️ Old-vs-new oracle for the membersPlan refactor on a real tree (read-only): the base module's whole-repository inventory and per-workspace member walks equal the batch module's. `bun refactor-oracle.ts <base-module-root> <batch-module-root> <tree-root>` */
const [baseRoot,batchRoot,root]=process.argv.slice(2) as [string,string,string],path="/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";
const base=await import(baseRoot+path),batch=await import(batchRoot+path);
const a=performance.now(),left=JSON.stringify(base.cargoRepositoryPackages(root)),b=performance.now(),right=JSON.stringify(batch.cargoRepositoryPackages(root)),c=performance.now();
console.log(`[DEBUG] inventory equal ${left===right} packages ${JSON.parse(left).length} base ${(b-a).toFixed(0)} ms batch ${(c-b).toFixed(0)} ms`);
let same=0,diff=0;for(const owner of base.discoverCargoWorkspaces(root)){const x=JSON.stringify(base.cargoWorkspaceMembers(root,owner)),y=JSON.stringify(batch.cargoWorkspaceMembers(root,owner));if(x===y)same++;else{diff++;console.log("[DEBUG] differs",owner.manifest);}}
console.log(`[DEBUG] workspaces equal ${same} differ ${diff}`);
