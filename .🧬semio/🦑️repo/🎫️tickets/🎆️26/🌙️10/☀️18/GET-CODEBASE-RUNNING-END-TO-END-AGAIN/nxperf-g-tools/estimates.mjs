import {pathToFileURL} from "node:url";
const F=process.cwd()+"/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/";
const gate=await import(pathToFileURL(F+"🎫️gate/🟦️.ts").href),est=await import(pathToFileURL(F+"📐️estimates/🟦️.ts").href);
const dir=gate.hostCapacityDirectoryV1(process.env);console.log("dir",dir);
const store=est.readHostEstimateStoreV1(dir);const keys=Object.keys(store.classes);console.log("classes",keys.length);
const pat=new RegExp(process.argv[2]??"nx");
for(const k of keys.filter(k=>pat.test(k)).slice(0,40)){const s=store.classes[k].samples;const r=s.map(x=>x.ratioBasisPoints).sort((a,b)=>a-b);console.log(k.slice(0,110).padEnd(110),"n",s.length,"ratio% min",r[0]/100,"med",r[r.length>>1]/100,"max",r[r.length-1]/100);}
console.log("rss",Math.round(process.memoryUsage().rss/1e6));
