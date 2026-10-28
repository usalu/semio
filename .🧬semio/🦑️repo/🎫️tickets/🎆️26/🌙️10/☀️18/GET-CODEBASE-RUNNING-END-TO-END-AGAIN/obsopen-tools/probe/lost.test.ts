import{test,expect}from"bun:test";
import{spawn}from"node:child_process";
import{settleOwnedChildV1}from"/private/tmp/sm/oo/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🏁️settlement/🟦️.ts";
test("one sysctl",async()=>{
 const child=spawn("/usr/sbin/sysctl",["-n","kern.bootsessionuuid"],{stdio:["ignore","pipe","pipe"],detached:true});let out="";child.stdout.on("data",c=>out+=c);
 const row=await settleOwnedChildV1(child,"probe-sysctl");console.log("RESULT "+JSON.stringify({row,out:out.trim(),exitCode:child.exitCode}));expect(row.kind).toBe("closed");
});
