// 🔎 Runs the admission's own selected-source-input closure (buildSourceContractGroupsV1) over every authored manifest and reports refusals plus the projects nearest to the reference bound; also counts references the way the function does.
import {readFileSync} from "node:fs";
import {join,dirname} from "node:path";
import {spawnSync} from "node:child_process";
import {buildSourceContractGroupsV1} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🚪️admission/🟦️.ts";
const W="/Users/ueli/Documents/semio",L=join(W,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library");
const policy=JSON.parse(readFileSync(join(L,"🏗️build/🔣️policy.json"),"utf8")),defaults=policy.namedInputs;
const bound=65536;
const rows:{name:string;root:string;targets:number;refs:number;error?:string}[]=[];
const count=(json:any)=>{const named={...defaults,...json.namedInputs},complete=new Set<string>();let refs=0;
 const visit=(values:any[],depth=0)=>{for(const v of values){refs++;if(typeof v==="object"&&v&&typeof v.input==="string"&&(v.projects?.includes?.(json.name)||(v.projects===undefined&&!v.dependencies)))visit([v.input],depth+1);if(typeof v!=="string"||v.startsWith("^")||v.includes("{"))continue;if(complete.has(v))continue;complete.add(v);if(named[v])visit(named[v],depth+1);}};
 for(const t of Object.values<any>(json.targets??{}))visit(t.inputs??["default"]);return refs;};
for(const f of spawnSync("git",["-C",W,"-c","core.quotepath=false","ls-files","-z","--","*📋️project.json"],{maxBuffer:1<<28}).stdout.toString().split("\0")){
 if(!f||f.startsWith(".🧬semio/"))continue;let json:any;try{json=JSON.parse(readFileSync(join(W,f),"utf8"));}catch{continue;}
 const row:any={name:json.name,root:dirname(f),targets:Object.keys(json.targets??{}).length,refs:count(json)};
 try{buildSourceContractGroupsV1(json,defaults,[]);}catch(error:any){row.error=String(error.message);}
 rows.push(row);
}
rows.sort((a,b)=>b.refs-a.refs);
console.log(JSON.stringify({projects:rows.length,errors:rows.filter(r=>r.error).map(r=>({name:r.name,root:r.root,refs:r.refs,error:r.error})),top:rows.slice(0,8).map(r=>({name:r.name,targets:r.targets,refs:r.refs})),overHalf:rows.filter(r=>r.refs>bound/2).length},null,1));
