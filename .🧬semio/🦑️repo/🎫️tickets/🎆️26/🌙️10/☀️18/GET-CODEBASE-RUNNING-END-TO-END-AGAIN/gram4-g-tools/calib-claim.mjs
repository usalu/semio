// [gram4-g] heap growth of shapeComputeResultV1 for N owners sharing a block of S rows, each with F found rows. usage: bun [--preload ..] calib-claim.mjs N S F
import{heapStats}from"bun:jsc";
const [N,S,F]=process.argv.slice(2).map(Number),L=process.env.LIB+"/🕸️graph/🕰️generations/";
const{shapeComputeResultV1}=await import(L+"🔌️load/🟨️.mjs");
const shared=Array.from({length:S},(_,i)=>({kind:"exists",path:"/w/lib/f"+i,fact:"v"+(i%7)}));
const capture=[...shared],part={owners:[],edgeOnly:[],edges:[],rows:capture,env:{digest:"x",rows:[]},externalNodes:{},parseUnavailable:[],replaceTags:[]};
for(let o=0;o<N;o++){const own=Array.from({length:F},(_,i)=>({kind:"file",path:"/w/p"+o+"/f"+i,fact:"o"+o}));capture.push(...own);part.owners.push({name:"o"+o,root:"p"+o,node:{targets:{}},nodeRows:[...shared,...own]});}
const live=()=>{Bun.gc(true);const h=heapStats();return h.heapSize+h.extraMemorySize;};
const before=live();const t=performance.now();
const shaped=shapeComputeResultV1({workspaceRoot:"/w",part,stubs:{}});
const ms=performance.now()-t;const after=live();
console.log(JSON.stringify({N,S,F,growthMB:Math.round((after-before)/1048576),ms:Math.round(ms),owners:shaped.owners.length}));
