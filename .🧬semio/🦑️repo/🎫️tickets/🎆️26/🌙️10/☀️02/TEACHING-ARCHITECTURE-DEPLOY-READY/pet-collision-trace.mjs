import {createRequire} from "node:module";
import {readFileSync,writeFileSync} from "node:fs";
import {resolve} from "node:path";
const require=createRequire(resolve("package.json")),{unzipSync}=require("fflate"),[input,output]=process.argv.slice(2),archive=unzipSync(readFileSync(input)),events=[];
for(const [name,bytes] of Object.entries(archive))if(name.endsWith(".trace"))for(const line of Buffer.from(bytes).toString("utf8").split("\n")){if(!line)continue;try{events.push({...JSON.parse(line),entry:name});}catch{}}
const calls=new Map();for(const row of events){if(row.type==="before")calls.set(row.callId,{id:row.callId,start:row.startTime,title:row.apiName??row.method,params:row.params,stack:row.stack});if(row.type==="after"){const call=calls.get(row.callId);if(call){call.end=row.endTime;call.duration=row.endTime-call.start;call.error=row.error;call.result=row.result;}}}
const summary={entries:Object.keys(archive).filter(name=>/trace|stack/.test(name)),calls:[...calls.values()].filter(row=>row.duration>200||/mouse|evaluate|poll|expect/.test(row.title??"")),annotations:events.filter(row=>row.type==="annotation"||row.type==="error"||row.method==="addAnnotations")};
writeFileSync(output,JSON.stringify(summary,null,2),{flag:"wx"});console.log(JSON.stringify({entries:summary.entries,calls:summary.calls.length,annotations:summary.annotations}));
