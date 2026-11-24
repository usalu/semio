const root="/Users/ueli/Documents/semio",L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",X=L+"🕸️graph/📥️load/🤝️selected-exchange/";
const ex=await import(X+"🟦️.ts"),at=await import(X+"🔌️attempt/🟦️.ts"),port=await import(L+"🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts"),ws=await import(L+"🕸️graph/📁️workspace/🟦️.ts");
const scope=process.argv[2],client=at.selectedFleetClientV1(root);
if(!client){console.log("no client");process.exit(0);}
const r=await ex.lookSelectedGenerationV1({client,scope,platform:"darwin-arm64",host:port.hostOfRootV1(root),lookMaximumMilliseconds:60000});
if(typeof r==="string"){console.log(r);process.exit(0);}
console.log("head created",new Date(r.head.createdMs).toISOString(),"owners",Object.keys(r.head.owners).length);
const stale=new Map<string,string[]>(),nameOf=new Map<unknown,string>([...r.rowsByOwner].map(([o,rows])=>[rows,o]));
const verdict=await ex.acceptSelectedOwnersV1({fetched:r,holds:async rows=>{const mine:string[]=[];const ok=await ws.buildGraphSourceFactsHoldV1(rows as never,undefined,row=>{mine.push(row.kind+" "+row.path);});stale.set(nameOf.get(rows)??"?",mine);return ok;}}).catch(e=>{console.log("err",e);return undefined;});
console.log("differing owners",[...stale].filter(([,m])=>m.length).length);if(verdict)console.log(JSON.stringify({matching:verdict.matching.length,differing:verdict.differing.length}));
const byDir=new Map<string,number>();for(const [o,rows] of stale)for(const row of rows.slice(0,3)){const k=row.split("/").slice(0,6).join("/");byDir.set(k,(byDir.get(k)??0)+1);}
console.log([...byDir].sort((a,b)=>b[1]-a[1]).slice(0,15));
const uniq=new Map<string,number>();for(const [,rows] of stale)for(const row of rows){uniq.set(row,(uniq.get(row)??0)+1);}
console.log("distinct stale rows",uniq.size);
console.log([...uniq].sort((a,b)=>b[1]-a[1]).slice(0,12).map(([r,n])=>n+" x "+r.slice(0,200)));
