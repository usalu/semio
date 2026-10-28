import{spawn}from"node:child_process";
import{settleOwnedChildV1}from"/private/tmp/sm/oo/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🏁️settlement/🟦️.ts";
const total=Number(process.argv[2]??200),width=Number(process.argv[3]??4),detached=process.argv[4]!=="attached",kinds:Record<string,number>={},bad:string[]=[];let started=0;
const one=async()=>{const child=spawn("/usr/sbin/sysctl",["-n","kern.bootsessionuuid"],{stdio:["ignore","pipe","pipe"],detached});let out="";child.stdout.on("data",c=>out+=c);const t0=Date.now(),row=await settleOwnedChildV1(child,"probe");kinds[row.kind]=(kinds[row.kind]??0)+1;if(row.kind!=="closed")bad.push(row.kind+" pid "+child.pid+" "+(Date.now()-t0)+"ms out="+JSON.stringify(out.trim()));};
await Promise.all(Array.from({length:width},async()=>{while(started++<total)await one();}));
console.log(JSON.stringify({total,width,detached,kinds,bad}));
