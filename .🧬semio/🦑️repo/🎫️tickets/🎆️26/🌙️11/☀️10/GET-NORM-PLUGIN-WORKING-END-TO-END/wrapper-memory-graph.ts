import{readFileSync}from"node:fs";
const snap=JSON.parse(readFileSync(process.argv[2]!,"utf8")),tr=new Bun.Transpiler({loader:"ts"}),by=new Map<string,any>(snap.files.map((f:any)=>[f.path,f]));
const edges=new Map<string,string[]>();
for(const f of snap.files){if(!/\.[cm]?[jt]sx?$/.test(f.path)){edges.set(f.path,[]);continue;}let imps:any[]=[];try{imps=tr.scan(Buffer.from(f.base64,"base64").toString("utf8")).imports;}catch{}
 edges.set(f.path,imps.filter(i=>i.kind==="import-statement"||i.kind==="require-call").map(i=>f.imports[i.path]).filter((t:any)=>typeof t==="string"&&by.has(t)));}
const reach=(skip?:{from:string,to:string}|Set<string>)=>{const seen=new Set<string>([snap.entry]),q=[snap.entry];while(q.length){const p=q.pop()!;for(const t of edges.get(p)!){if(skip instanceof Set&&skip.has(t))continue;if(skip&&!(skip instanceof Set)&&skip.from===p&&skip.to===t)continue;if(!seen.has(t)){seen.add(t);q.push(t);}}}return seen;};
const all=reach();console.log("static reach",all.size,"of",snap.files.length);
const size=(s:Set<string>)=>[...s].reduce((a,p)=>a+Buffer.byteLength(by.get(p).base64)*3/4,0);
console.log("bytes",Math.round(size(all)/1e6));
const rows:any[]=[];
for(const[p,ts]of edges)if(all.has(p))for(const t of new Set(ts)){const r=reach({from:p,to:t});rows.push({from:p.split("/").slice(-4).join("/"),to:t.split("/").slice(-4).join("/"),saved:all.size-r.size,bytes:Math.round((size(all)-size(r))/1e3)});}
rows.sort((a,b)=>b.saved-a.saved);for(const r of rows.slice(0,25))console.log(r.saved,r.bytes+"K",r.from,"->",r.to);
const area=new Map<string,number>();for(const p of all){const k=p.split("/").slice(0,9).join("/").replace(/^🧰️framework\/🛍️products\/🦑️repo\/🔨️modules\/📚️library\//,"");area.set(k,(area.get(k)??0)+1);}
console.log("--areas");for(const[k,v]of[...area].sort((a,b)=>b[1]-a[1]).slice(0,25))console.log(v,k);
