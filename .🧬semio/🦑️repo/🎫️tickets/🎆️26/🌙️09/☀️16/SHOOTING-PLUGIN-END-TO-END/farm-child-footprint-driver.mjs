import{spawn}from"node:child_process";
import{readProcessFootprintBytesV1}from"/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧠️memory/🦶️footprint/🟨️.mjs";
const[,,probe,...owners]=process.argv,MiB=1048576;
for(const owner of owners){
 const child=spawn(process.execPath,[probe,owner,"cold"],{stdio:["ignore","pipe","inherit"]});
 let peak=0,peakAt=0,text="",stopped=false;const began=performance.now();
 child.stdout.on("data",data=>{text+=data;for(const line of String(data).split("\n"))if(line.includes("] OWNER "))console.log(line.replace(/ before.*afterRun/," afterRun"));});
 const poll=(async()=>{while(!stopped){const bytes=await readProcessFootprintBytesV1(child.pid);if(Number.isFinite(bytes)&&bytes>peak){if(bytes>peak+200*MiB)console.log("[DEBUG] SAMPLE",Math.round(bytes/MiB),"MB at",Math.round((performance.now()-began)/1000),"s");peak=bytes;peakAt=performance.now()-began;}await new Promise(done=>setTimeout(done,300));}})();
 await new Promise(done=>child.once("exit",done));stopped=true;await poll;
 const result=text.split("\n").find(line=>line.includes("RESULT"))??text.split("\n").slice(-3).join(" | ");
 console.log("[DEBUG] FOOTPRINT owner",owner,"peakFootprintMB",Math.round(peak/MiB),"atMs",Math.round(peakAt),"totalMs",Math.round(performance.now()-began),"|",result.replace("[DEBUG] ",""));
}
