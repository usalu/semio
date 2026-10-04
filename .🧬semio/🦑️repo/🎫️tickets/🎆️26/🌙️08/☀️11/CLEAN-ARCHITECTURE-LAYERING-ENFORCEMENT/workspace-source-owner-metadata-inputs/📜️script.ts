import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {resolve} from "node:path";
import {createHash} from "node:crypto";
import {parse} from "jsonc-parser";
const root=resolve(import.meta.dir,"../../../../../../../../"),path="bun.lock",input=resolve(import.meta.dir,"lock-predecessor-1.json"),hash=(source:string)=>createHash("sha256").update(source).digest("hex");
const command=process.argv[2],source=readFileSync(resolve(root,path),"utf8");
const lock=parse(source),observations=Object.entries(lock.workspaces as Record<string,{name:string}>).map(([path,value])=>({path,name:value.name}));
if(command==="capture"){
 if(existsSync(input))throw Error("Metadata predecessor already captured");
 writeFileSync(input,JSON.stringify({version:1,path,source,sha256:hash(source),workspaces:observations},null,2)+"\n");console.log(JSON.stringify({capturedWorkspaceEntries:observations.length,sha256:hash(source)}));
}else if(command==="observe"){
 const before=JSON.parse(readFileSync(input,"utf8")),current={version:1,path,before:before.source,current:source,beforeSha256:before.sha256,sha256:hash(source),inverse:{source:before.source},workspaces:observations};
 writeFileSync(resolve(import.meta.dir,"lock-current-inverse-1.json"),JSON.stringify(current,null,2)+"\n");console.log(JSON.stringify({currentWorkspaceEntries:observations.length,sha256:hash(source),added:observations.filter(row=>!before.workspaces.some((old:any)=>old.path===row.path)).map(row=>row.name)}));
}else throw Error("Expected capture or observe");

