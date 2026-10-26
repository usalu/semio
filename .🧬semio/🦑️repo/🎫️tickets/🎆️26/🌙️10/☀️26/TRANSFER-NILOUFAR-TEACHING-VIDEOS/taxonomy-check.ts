import { spawn } from "node:child_process";
import { join } from "node:path";
import { writeFileSync } from "node:fs";
const generated=join(import.meta.dir,"🗑️generated");
const script=process.argv[2]||"test-taxonomy-pipelines.py";
const child=spawn(join(generated,"v/Scripts/python.exe"),[join(import.meta.dir,script),...process.argv.slice(3)],{env:{...process.env,PYTHONUTF8:"1",PYTHONIOENCODING:"utf-8",PYTHONDONTWRITEBYTECODE:"1",SEMIO_TEACHING_OUTPUT_ROOT:join(generated,"pipeline-tests"),SEMIO_TEACHING_CACHE_DIR:join(generated,"fonts"),UV_CACHE_DIR:join(generated,"uv")},windowsHide:true});
let log="";for(const stream of[child.stdout,child.stderr])stream.on("data",c=>{log+=c.toString();process.stdout.write(c);});child.on("error",e=>{console.error("[DEBUG]",e);process.exitCode=1;});child.on("exit",c=>{writeFileSync(join(generated,script.replace('.py','')+'.log'),log);process.exitCode=c??1;});
