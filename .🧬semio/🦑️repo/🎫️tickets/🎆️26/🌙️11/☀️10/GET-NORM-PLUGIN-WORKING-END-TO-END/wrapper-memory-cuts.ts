import{readFileSync}from"node:fs";
const snap=JSON.parse(readFileSync(process.argv[2]!,"utf8")),cuts=process.argv.slice(3).map(c=>c.split("=>")),tr=new Bun.Transpiler({loader:"ts"}),by=new Map<string,any>(snap.files.map((f:any)=>[f.path,f]));
const edges=new Map<string,string[]>();
for(const f of snap.files){if(!/\.[cm]?[jt]sx?$/.test(f.path)){edges.set(f.path,[]);continue;}let imps:any[]=[];try{imps=tr.scan(Buffer.from(f.base64,"base64").toString("utf8")).imports;}catch{}
 edges.set(f.path,imps.filter(i=>i.kind==="import-statement"||i.kind==="require-call").map(i=>f.imports[i.path]).filter((t:any)=>typeof t==="string"&&by.has(t)));}
const cut=(p:string,t:string)=>cuts.some(([a,b])=>p.includes(a!)&&t.includes(b!));
const reach=(skip?:{from:string,to:string})=>{const seen=new Set<string>([snap.entry]),q=[snap.entry];while(q.length){const p=q.pop()!;for(const t of edges.get(p)!){if(cut(p,t)||skip&&skip.from===p&&skip.to===t)continue;if(!seen.has(t)){seen.add(t);q.push(t);}}}return seen;};
const all=reach();console.log("reach",all.size);const rows:any[]=[];
for(const[p,ts]of edges)if(all.has(p))for(const t of new Set(ts))rows.push({p,t,saved:all.size-reach({from:p,to:t}).size});
rows.sort((a,b)=>b.saved-a.saved);const s=(x:string)=>x.replace(/^🧰️framework\/(🛍️products\/🦑️repo\/)?🔨️modules\//,"").split("/").slice(-5).join("/");for(const r of rows.slice(0,14))console.log(r.saved,s(r.p),"=>",s(r.t));
