const mode=process.argv[2],N=4_000_000;
const shared=Array.from({length:50000},(_,i)=>({kind:"file",path:"/x/y/"+i,fact:{size:i,mtimeMs:i*2.5,sha256:"a".repeat(64)}}));
Bun.gc(true);const base=process.memoryUsage().rss;
const keep=new Array(N);
if(mode==="refs")for(let i=0;i<N;i++)keep[i]=shared[i%50000];
else for(let i=0;i<N;i++){const s=shared[i%50000];keep[i]={kind:s.kind,path:s.path,fact:s.fact};}
Bun.gc(true);const used=process.memoryUsage().rss-base;
console.log(JSON.stringify({mode,N,rssMB:Math.round(used/1048576),bytesEach:Math.round(used/N),projectionAt154MGB:+(154e6*used/N/1e9).toFixed(1),keep:keep.length}));
