/** 🧪️ Runs bounded read-only command-source queries through registered tools. */
import fs from "node:fs";import path from "node:path";import {spawn} from "node:child_process";import {fileURLToPath} from "node:url";
const ticket=path.dirname(fileURLToPath(import.meta.url)),rows=[];
for(const root of ["🧰️framework/📦️packages/🦀️rust","🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust"]){
 const project=JSON.parse(fs.readFileSync(path.join(root,"📋️project.json"),"utf8"));
 for(const command of Object.keys(project.targets)){
  const started=performance.now(),result=await new Promise(resolve=>{
   const child=spawn("repo",["tools","exec","--light","node","--",path.join(ticket,"graph-command-probe.mjs"),root,command,"--observed","--authored"],{env:{...process.env,SEMIO_LANGUAGE:"en"},stdio:["ignore","pipe","pipe"]});let output="";
   child.stdout.on("data",data=>{output+=data});child.stderr.on("data",data=>{output+=data});
   const timer=setTimeout(()=>child.kill("SIGTERM"),12000);
   child.on("close",(code,signal)=>{clearTimeout(timer);resolve({code,signal,output:output.slice(-2000)})});
  });
  const row={root,command,wallMilliseconds:Math.round(performance.now()-started),...result};rows.push(row);console.log(JSON.stringify(row));fs.writeFileSync(path.join(ticket,"🗑️generated/graph-command-probes.json"),JSON.stringify(rows,null,2));
 }
}
