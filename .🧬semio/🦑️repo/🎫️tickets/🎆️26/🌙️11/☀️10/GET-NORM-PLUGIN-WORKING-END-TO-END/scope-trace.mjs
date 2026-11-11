import{readFileSync}from"node:fs";import{brotliDecompressSync}from"node:zlib";
const root="/home/deck/semio",L=await import(root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔌️load/🟨️.mjs");
const g=root+"/.cache/b/g/m/generations",cur=JSON.parse(readFileSync(g+"/current.json","utf8")),man=JSON.parse(readFileSync(g+"/m/"+cur.manifest+".json","utf8"));
const name=process.argv[2],target=process.argv[3]||"check";
const frag=n=>{const row=man.owners[n];const f=row.fragment;for(const d of["a","c","f","o","s","x"]){try{return JSON.parse(brotliDecompressSync(readFileSync(g+"/"+d+"/"+f+".json.br")));}catch{}}throw Error("nofrag "+n+" "+f);};
const cacheF=new Map(),get=n=>{if(!cacheF.has(n))cacheF.set(n,frag(n));return cacheF.get(n);};
const owners=Object.fromEntries(Object.entries(man.owners).map(([n,r])=>[n,r.root]));
const accessed=[],ext=new Set();for(const n of Object.keys(owners))for(const e of get(n).edges)if(!owners[e.target])ext.add(e.target);
const out=await L.sliceClosureV1({owners,external:[...ext],fragment:async n=>{accessed.push(n);return L.sliceViewV1(get(n));},selection:{kind:"tasks",tasks:[{project:name,target}]},roots:[name],providers:JSON.parse(readFileSync(root+"/📋️build-owners.json","utf8")).catalogProviders});
console.log("owners",Object.keys(owners).length,"slice",out.size,"gismap",out.has("@semio-tech/gis-gismap-js"));
const edgesOnly=new Set(),q=[name];while(q.length){const n=q.pop();if(edgesOnly.has(n)||!owners[n])continue;edgesOnly.add(n);for(const e of get(n).edges)q.push(e.target);}
console.log("edge closure",edgesOnly.size,"gismap",edgesOnly.has("@semio-tech/gis-gismap-js"));
const tg=get(name).node?.targets?.[target];console.log(JSON.stringify(tg?.dependsOn),JSON.stringify(tg?.inputs).slice(0,600));

const prov=JSON.parse(readFileSync(root+"/📋️build-owners.json","utf8")).catalogProviders[0];let best;for(const[n,r]of Object.entries(owners))if((r==="."||prov.sourcePath===r||prov.sourcePath.startsWith(r+"/"))&&(best===undefined||r.length>owners[best].length))best=n;
console.log("provider owner",best,owners[best],"edges",get(best).edges.length,"has gismap edge",get(best).edges.some(e=>e.target==="@semio-tech/gis-gismap-js"));
