import{readFileSync,readdirSync,statSync}from"node:fs";import{brotliDecompressSync,gunzipSync}from"node:zlib";
const[,,dir,limit="60",since="0"]=process.argv;
function shape(v){let nodes=0,maxDepth=0;const p=[[v,0]];while(p.length){const[x,d]=p.pop();nodes++;if(d>maxDepth)maxDepth=d;if(x&&typeof x==="object")for(const k of Object.keys(x)){p.push([k,d+1],[x[k],d+1]);}}return{nodes,maxDepth};}
const rows=readdirSync(dir).filter(n=>/\.br$|\.gz$/.test(n)).map(n=>({n,s:statSync(dir+"/"+n)})).filter(r=>r.s.mtimeMs>=+since).sort((a,b)=>b.s.size-a.s.size).slice(0,+limit);
for(const{n,s}of rows){try{const b=readFileSync(dir+"/"+n),t=n.endsWith(".br")?brotliDecompressSync(b):gunzipSync(b),r=shape(JSON.parse(t));console.log(n.slice(0,16),s.size,t.length,r.nodes,r.maxDepth,r.nodes>4194304||r.maxDepth>64?"OVER":"");}catch(e){console.log(n.slice(0,16),"ERR",String(e).slice(0,60));}}
