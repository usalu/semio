import{censusNxAuthorizerSourcesV1}from"../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
const root=process.cwd(),entry=process.argv[2]!,depth=Number(process.argv[3]??12);
const c=await censusNxAuthorizerSourcesV1(root,entry,AbortSignal.timeout(300000),{installed:true}).catch((e:any)=>{console.log("ERR",e.message);return e.sourceCensus?{files:e.sourceCensus.length,sourceBytes:0,sources:e.sourceCensus}:null});
if(c){console.log("files",c.files,"bytes",c.sourceBytes);const g=new Map<string,[number,number]>();for(const s of c.sources){const p=s.path.split("/").slice(0,depth);const k=p.join("/");const v=g.get(k)??[0,0];v[0]++;v[1]+=s.bytes;g.set(k,v);}
for(const[k,v]of[...g].sort((a,b)=>b[1][0]-a[1][0]).slice(0,60))console.log(v[0],v[1],k);}
if(c){await Bun.write(process.argv[4]!,JSON.stringify(c.sources.map((s:any)=>[s.path,s.bytes,s.origin,s.importer])));}
