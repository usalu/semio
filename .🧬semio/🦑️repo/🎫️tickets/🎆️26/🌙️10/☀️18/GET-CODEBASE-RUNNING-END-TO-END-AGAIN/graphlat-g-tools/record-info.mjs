// [graphlat-g] reads one persisted project record of the LIVE graph store (read-only) and says which of its observation rows hold now. usage: bun record-info.mjs <np-record-path> [limit]
import{plugin}from"bun";
const ROOT="/Users/ueli/Documents/semio",OBS=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/👁️observation/🟨️.mjs";
plugin({name:"rec",setup(b){b.onLoad({filter:/observation\/[^/]*\.mjs$/},async args=>{if(args.path!==OBS)return undefined;const t=await Bun.file(args.path).text();return{contents:t+"\nexport const __rec={readInferencePhysicalRecordV1,current,observationsHold};\n",loader:"js"};});}});
process.chdir(ROOT);
const obs=await import(OBS),path=process.argv[2],limit=Number(process.argv[3]??"20");
const record=obs.__rec.readInferencePhysicalRecordV1(path,true);
if(!record){console.log("no record");process.exit(0);}
const rows=record.header.observations,kinds={};for(const r of rows)kinds[r.kind]=(kinds[r.kind]??0)+1;
console.log(JSON.stringify({digest:record.header.digest,rows:rows.length,kinds,authority:record.header.authorityObservations?.length}));
const bad=[];for(const r of rows){let ok=false;try{ok=obs.__rec.current([r]);}catch(e){ok="throw:"+e.message.slice(0,80);}if(ok!==true)bad.push({kind:r.kind,path:r.path.replace(ROOT+"/",""),ok});}
console.log("failing rows now:",bad.length);for(const r of bad.slice(0,limit))console.log(JSON.stringify(r));
const young=rows.filter(r=>r.kind==="file").map(r=>({p:r.path.replace(ROOT+"/",""),m:Math.max(r.fact.mtimeMs??0,r.fact.ctimeMs??0)})).sort((a,b)=>b.m-a.m).slice(0,5);console.log("youngest file rows",JSON.stringify(young.map(y=>[y.p,new Date(y.m).toISOString()])));
const paths=new Set(rows.map(r=>r.path.replace(ROOT+"/","").split("/").slice(0,6).join("/")));console.log("top path groups:",[...paths].slice(0,25));
