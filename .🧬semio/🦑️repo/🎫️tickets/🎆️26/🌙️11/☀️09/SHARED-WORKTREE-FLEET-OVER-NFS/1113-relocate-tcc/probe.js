import {readdirSync,appendFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
const [directory,output,label]=process.argv.slice(2);
appendFileSync(output,`${new Date().toISOString()} ${label} start pid=${process.pid} exec=${process.execPath} ppid=${process.ppid} parent=${(()=>{try{return execFileSync("/bin/ps",["-o","comm=","-p",String(process.ppid)]).toString().trim();}catch{return "?";}})()}\n`);
try{appendFileSync(output,`${label} ok ${JSON.stringify(readdirSync(directory))}\n`);}catch(error){appendFileSync(output,`${label} error ${error.code} ${error.message}\n`);}
