import {spawn} from "node:child_process";
import {resolve} from "node:path";
const fixture=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/🧪️tests/🏭️executor/🏗️fixture/🟨️.mjs");
for(const color of ["0","1"]){
 const child=spawn(process.execPath,[fixture],{env:{...process.env,FORCE_COLOR:color},stdio:["ignore","pipe","pipe","pipe"]});
 let out="",err="",frames="";
 child.stdout.on("data",chunk=>out+=chunk);child.stderr.on("data",chunk=>err+=chunk);
 const timer=setTimeout(()=>child.kill("SIGTERM"),3000);
 child.stdio[3].on("data",chunk=>{frames+=chunk; if(String(chunk).includes('"ready"'))child.stdio[3].write(JSON.stringify({id:1,argv:["echo","probe"]})+"\n"); if(String(chunk).includes('"done"'))child.stdio[3].write('{"t":"stop"}\n');});
 await new Promise(resolve=>child.once("close",resolve));clearTimeout(timer);
 console.log("[DEBUG] "+JSON.stringify({color,out,err,frames}));
}