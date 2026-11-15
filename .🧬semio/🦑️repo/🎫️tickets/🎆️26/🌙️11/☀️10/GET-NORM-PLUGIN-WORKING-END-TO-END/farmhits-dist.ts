import{readdirSync,statSync,readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const CE=process.argv[2],names=readdirSync(CE+"/e").filter((_,i)=>i%20===0);
const by=new Map<string,{n:number,layers:number[],bytes:number[],rows:number[]}>();
for(const n of names){const raw=readFileSync(CE+"/e/"+n),t=brotliDecompressSync(raw).toString("utf8"),r=JSON.parse(t);const fn=r.recipe.fn;let g=by.get(fn);if(!g)by.set(fn,g={n:0,layers:[],bytes:[],rows:[]});g.n++;g.layers.push(r.layers.length);g.bytes.push(t.length);g.rows.push(r.rows.length);}
const q=(a:number[],p:number)=>{const s=[...a].sort((x,y)=>x-y);return s[Math.min(s.length-1,Math.floor(s.length*p))];};
for(const[fn,g]of by)console.log(fn,"n",g.n,"layers p50/p90/max",q(g.layers,.5),q(g.layers,.9),q(g.layers,.999),"textMB p50/p90/max",(q(g.bytes,.5)/1e6).toFixed(2),(q(g.bytes,.9)/1e6).toFixed(2),(q(g.bytes,.999)/1e6).toFixed(2),"rows p50/p90",q(g.rows,.5),q(g.rows,.9));
