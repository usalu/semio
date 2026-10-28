// [derive-g] explains the edge differences of owners between two scratch generations by the base generation: an extra edge is explained when its source file is not part of the base fragment's module facts or was modified after the base generation was verified (the quiet scratch ignores tree changes, the direct discovery reads them). usage: bun explain-diff.mjs <baseDir> <dirA> <dirB>
import{readFileSync,existsSync,statSync}from"node:fs";import{join}from"node:path";import{brotliDecompressSync}from"node:zlib";
const load=d=>{const p=JSON.parse(readFileSync(join(d,"current.json"),"utf8")),m=JSON.parse(readFileSync(join(d,"m",p.manifest+".json"),"utf8"));return{d,m};};
const[base,A,B]=process.argv.slice(2).map(load);
const frag=(g,n)=>JSON.parse(brotliDecompressSync(readFileSync(join(g.d,"o",g.m.owners[n].fragment+".json.br"))).toString("utf8"));
const BASE_MS=JSON.parse(readFileSync(join(process.argv[2],'current.json'),'utf8')).verified?.atMs??0;let owners=0,extra=0,explained=0,missing=0;const bad=[];
for(const n of Object.keys(A.m.owners)){
 if(A.m.owners[n].fragment===B.m.owners[n]?.fragment)continue;
 const a=frag(A,n),b=frag(B,n),baseFiles=new Set((frag(base,n).node?.metadata?.semio?.moduleGraph?.files??[]).map(f=>f.file)),key=e=>JSON.stringify([e.source,e.target,e.sourceFile,e.type]);
 const ka=new Set(a.edges.map(key)),kb=new Set(b.edges.map(key));owners++;
 for(const e of b.edges)if(!ka.has(key(e))){extra++;if(existsSync("/Users/ueli/Documents/semio/"+e.sourceFile)&&(!baseFiles.has(e.sourceFile)||statSync("/Users/ueli/Documents/semio/"+e.sourceFile).mtimeMs>BASE_MS))explained++;else bad.push(["extra",n,e.sourceFile]);}
 for(const e of a.edges)if(!kb.has(key(e))){missing++;bad.push(["only-stream",n,e.sourceFile]);}
 const rest=JSON.stringify({...a,edges:0,node:0}),restB=JSON.stringify({...b,edges:0,node:0});if(rest.replace(/SEMIO_[A-Z_]+_[a-f0-9]{64}/g,"X")!==restB.replace(/SEMIO_[A-Z_]+_[a-f0-9]{64}/g,"X"))bad.push(["other-fields",n]);
}
console.log(JSON.stringify({owners,extraEdgesInDirect:extra,explainedByNewFiles:explained,onlyInStream:missing,unexplained:bad.length}));for(const x of bad.slice(0,10))console.log(JSON.stringify(x).slice(0,220));
