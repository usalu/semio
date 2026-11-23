import {readFileSync} from "node:fs";
import {dirname} from "node:path";
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const rows=(memo.observations as {kind:string;path:string;fact:unknown}[]).filter(r=>r.kind==="resolve");
const keys=new Set<string>(),dirs=new Set<string>(),requests=new Map<string,number>();
for(const r of rows){const [from,request]=JSON.parse(r.path) as [string,string];keys.add(dirname(from)+"\0"+request);dirs.add(dirname(from));requests.set(request,(requests.get(request)??0)+1);}
console.log({rows:rows.length,distinctDirRequest:keys.size,distinctDirs:dirs.size,distinctRequests:requests.size});
console.log([...requests].sort((a,b)=>b[1]-a[1]).slice(0,12));
