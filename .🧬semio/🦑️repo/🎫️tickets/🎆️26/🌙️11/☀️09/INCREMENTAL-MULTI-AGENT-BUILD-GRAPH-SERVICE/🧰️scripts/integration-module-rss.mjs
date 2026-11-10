// RSS over N sequential captured plans that each load an edited provider (worst case: every plan loads a new module generation)
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";
const R="/home/deck/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const {withCapturedPlanV1}=await import(R+"/🧊️revision/🟦️.ts"),{createCasStoreV1}=await import(R+"/🗄️cas/🗃️store/🟦️.ts"),{importObservedModuleV1}=await import(R+"/👁️observation/🟨️.mjs");
const plans=Number(process.argv[2]??500),root=mkdtempSync(join(tmpdir(),"rss-ws-")),cas=createCasStoreV1({root:mkdtempSync(join(tmpdir(),"rss-cas-"))});
mkdirSync(join(root,"plugin"),{recursive:true});
const padding="// "+"x".repeat(Number(process.argv[3]??20000))+"\n";
const rss=()=>Math.round(process.memoryUsage().rss/1048576);
const samples=[];
try{
 for(let i=0;i<plans;i++){
  writeFileSync(join(root,"plugin/dep.ts"),`export const dep:string="d${i}";\n${padding}`);
  writeFileSync(join(root,"plugin/provider.ts"),`import {dep} from "./dep.ts";\nexport const value:string=dep+":${i}";\n`);
  const out=await withCapturedPlanV1({cas,root,workspace:"w",holder:"p",journalCursor:"c"+i},async()=>(await importObservedModuleV1(join(root,"plugin/provider.ts"))).value);
  if(out.result!==`d${i}:${i}`)throw Error("wrong module "+out.result);
  if(i%50===0){Bun.gc(true);samples.push([i,rss()]);}
 }
 Bun.gc(true);samples.push([plans,rss()]);
 console.log(JSON.stringify({plans,samples}));
}finally{rmSync(root,{recursive:true,force:true});}
