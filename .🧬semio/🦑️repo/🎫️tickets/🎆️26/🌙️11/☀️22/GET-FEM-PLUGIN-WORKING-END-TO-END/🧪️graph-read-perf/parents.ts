import {readFileSync} from "node:fs";
const memo=JSON.parse(readFileSync(process.argv[2],"utf8"));
const rows=memo.observations as {kind:string;path:string;fact:any}[];
const listed=new Set(rows.filter(r=>r.kind==="readdir"||r.kind==="readdirNames").map(r=>r.path));
const probe=new Set(["lstat","exists","fileExists","directoryExists","stat"]);
const parentOf=(p:string)=>p.slice(0,Math.max(p.lastIndexOf("\\"),p.lastIndexOf("/")));
let n=0,covered=0;const parents=new Set<string>(),unlisted=new Set<string>();
const ancestors=new Set<string>();
for(const r of rows){if(!probe.has(r.kind))continue;n++;const p=parentOf(r.path);parents.add(p);if(listed.has(p))covered++;else unlisted.add(p);}
console.log({probeRows:n,distinctParents:parents.size,rowsWithListedParent:covered,unlistedParents:unlisted.size,readdirRows:listed.size});
const kinds=new Map<string,string>();
for(const r of rows)if(r.kind==="lstat")kinds.set(String(r.fact),"");
console.log([...kinds.keys()].slice(0,10));
const lstatDirs=rows.filter(r=>r.kind==="lstat"&&r.fact==="directory").length;
console.log({lstatDirectory:lstatDirs});
