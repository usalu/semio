import{spawn}from"node:child_process";
import{settleOwnedChildV1}from"/private/tmp/sm/oo/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🏁️settlement/🟦️.ts";
const stall=Number(process.argv[2]??0),argv=["-n","kern.bootsessionuuid"];
const child=spawn("/usr/sbin/sysctl",argv,{stdio:["ignore","pipe","pipe"],detached:true});let out="";child.stdout.on("data",c=>out+=c);
if(stall>0){const end=Date.now()+stall;while(Date.now()<end);console.error("[DEBUG] stalled",stall)}
const row=await settleOwnedChildV1(child,"probe");console.log(JSON.stringify({row,out:out.trim()}));
