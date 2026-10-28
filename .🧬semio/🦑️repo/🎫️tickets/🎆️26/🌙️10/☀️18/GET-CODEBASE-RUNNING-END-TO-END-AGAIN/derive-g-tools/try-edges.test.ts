import{expect,test}from"bun:test";
import{mkdtempSync,mkdirSync,writeFileSync,realpathSync}from"node:fs";import{tmpdir}from"node:os";import{join,dirname}from"node:path";
const inference=await import("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🟨️.mjs"),{cacheInternals}=inference as any;
await (inference as any).libraryBootstrap;
test("bare",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"edges-")));
 for(const p of ["a","b"]){mkdirSync(join(root,p),{recursive:true});}
 writeFileSync(join(root,"a/x.ts"),'import "../b/y.ts";\nexport const a=1;\n');writeFileSync(join(root,"b/y.ts"),"export const b=1;\n");
 const edges:any[]=[],publication:any={};
 await cacheInternals.collectImportEdges(root,{a:[{file:"a/x.ts"}],b:[{file:"b/y.ts"}]},{a:{root:"a"},b:{root:"b"}},new Map(),undefined,(s:string,t:string,f:string)=>edges.push([s,t,f]),publication,undefined,{moduleGraphProjects:new Set(["a"])});
 console.log(JSON.stringify(edges),Object.keys(publication));
});
