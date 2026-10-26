import { spawn } from "node:child_process";
import { readFileSync, existsSync, writeFileSync } from "node:fs";
import { join } from "node:path";
const ticket=import.meta.dir, generated=join(ticket,"🗑️generated"),teaching="C:/git/semio/🎓️teaching",root=join(generated,"g");
const plan=JSON.parse(readFileSync(join(teaching,"video_manifest.json"),"utf8"));
for(const video of plan.videos){const base=join(root,"rendered",video.output+"_480p15");if(!existsSync(base+".mp4")||!existsSync(base+".render.json"))throw new Error("Final movie not ready: "+video.id);}
const env={...process.env,PYTHONUTF8:"1",PYTHONIOENCODING:"utf-8",PYTHONDONTWRITEBYTECODE:"1"};
const child=spawn(join(generated,"v/Scripts/python.exe"),[join(ticket,"make-previews.py"),root,"--ffprobe",join(generated,"f/ffprobe.exe"),"--output",join(generated,"corrected-final-previews"),"--teaching-root",teaching],{env,windowsHide:true});
let log="";for(const stream of [child.stdout,child.stderr])stream.on("data",chunk=>{log+=chunk.toString();process.stdout.write(chunk);});
child.on("error",error=>{console.error("[DEBUG]",error);process.exitCode=1;});
child.on("exit",code=>{writeFileSync(join(generated,"corrected-final-previews.log"),log);process.exitCode=code??1;});
