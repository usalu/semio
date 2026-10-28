import { spawn } from "node:child_process";
import { readFileSync,mkdirSync,createWriteStream } from "node:fs";
import { resolve,join } from "node:path";
const root=process.cwd(), ticket=import.meta.dirname, out=join(ticket,"🗑️generated","silhouettes");
mkdirSync(out,{recursive:true});
const binary=readFileSync(resolve(root,".🧬semio/🦑️repo/native-repo-path"),"utf8").trim();
const args=process.argv.slice(2), name=args.shift();
const env={...process.env,PYTHONUTF8:"1",PYTHONIOENCODING:"utf-8",PYTHONDONTWRITEBYTECODE:"1",SEMIO_TEST_MANIM:"1",
 UV_PROJECT_ENVIRONMENT:resolve(root,".🧬semio/🎓️teaching/⚡️cache/python"),SEMIO_TEACHING_ENV:resolve(root,".🧬semio/🎓️teaching/⚡️cache/python"),
 SEMIO_TEACHING_CACHE_DIR:join(out,"fonts"),SEMIO_TEACHING_OUTPUT_ROOT:out};
const project=resolve(root,"🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python");
const command=name==="aggregate" ? ["tools","exec","bun","--","🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts","test"] : ["tools","exec","uv","--","run","--project",project,"--python","3.12","--group","test","python","-B",...args];
const child=spawn(binary,command,{cwd:root,env,windowsHide:true,stdio:["ignore","pipe","pipe"]});
const log=createWriteStream(join(out,name+".log"));let tail="";
for(const stream of [child.stdout,child.stderr])stream.on("data",chunk=>{log.write(chunk);tail=(tail+chunk).slice(-5000);});
const timer=setInterval(()=>console.log("[DEBUG] "+name+" is running"),30000);
child.on("exit",code=>{clearInterval(timer);log.end();console.log(tail);console.log("[DEBUG] "+name+" exit "+code);process.exitCode=code??1;});
process.on("SIGINT",()=>child.kill("SIGINT"));
