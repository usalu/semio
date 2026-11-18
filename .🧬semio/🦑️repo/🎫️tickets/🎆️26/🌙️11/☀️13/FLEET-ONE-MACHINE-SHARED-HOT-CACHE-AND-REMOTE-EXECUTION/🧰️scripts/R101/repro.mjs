import{readFileSync}from"node:fs";import{hostname}from"node:os";
const R="/Users/ueli/Documents/semio",PN=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📦️project-nodes/";
const E=process.env,wave=E.R101_WAVE,environment={...E};
for(const n of["SEMIO_REMOTE_CACHE_ENDPOINT","SEMIO_REMOTE_CACHE_ACCESS","SEMIO_REMOTE_CACHE_INSTANCE","SEMIO_REMOTE_CACHE_TOKEN","SEMIO_FLEET_CACHE_ENDPOINT"])delete environment[n];
const{deserialize}=await import("node:v8");
const head=JSON.parse(readFileSync(wave+"/head.json","utf8")),units=JSON.parse(readFileSync(wave+"/units.json","utf8")),session=readFileSync(wave+"/session.bin");
const{openOwnerPoolInferenceV1}=await import(PN+"🌐️owner-pool/🔬️inference/🟨️.mjs");
const d=await openOwnerPoolInferenceV1({root:R,environment,session,globals:head.globals,units});
const want=Number(E.R101_COUNT||1)+0;let done=0,tries=0;const states={},lat={};const maxTries=Number(E.R101_TRIES||400);const names=E.R101_ONLY?E.R101_ONLY.split(","):null;
const order=units.map((u,i)=>[u,Math.random()]).sort((a,b)=>a[1]-b[1]).map(x=>x[0]);
for(const u of order){if(names&&!names.includes(u.name))continue;if(done>=want||tries>maxTries)break;tries++;
 const t0=performance.now();const c=await d.claim(u);lat[c.state]=(lat[c.state]||0)+performance.now()-t0;states[c.state]=(states[c.state]||0)+1;if(c.state!=="granted")continue;
 const t=performance.now();let ok=false,err;try{ok=await d.run(u)&&await c.publish();}catch(e){err=String(e?.message??e);await c.abandon().catch(()=>{});}
 console.log(JSON.stringify({t:"owner",name:u.name,ok,err,why:ok?"":c.why(),code:c.code?.(),ms:Math.round(performance.now()-t),rssMb:Math.round(process.memoryUsage().rss/1048576),tries}));done++;}
console.log(JSON.stringify({t:"end",done,tries,states,latMs:Object.fromEntries(Object.entries(lat).map(([k,v])=>[k,Math.round(v/states[k])]))}));d.close?.();process.exit(0);
