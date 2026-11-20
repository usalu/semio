import {readFileSync,writeFileSync} from "node:fs";
const root="C:/semio/",ticket=root+".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️18/DEV-FEM-3D-END-TO-END/🗑️generated/";
const owners=JSON.parse(readFileSync(ticket+"owners.json","utf8")) as Record<string,{lib:string[];dev:string[];manifest:string}>;
const uses=(files:string[],needle:string)=>files.some(f=>readFileSync(root+f,"utf8").includes(needle));
const crates=[["semio-s-artifact-fem-3d-diff","fem_3d_diff"],["semio-s-artifact-fem-3d-guards","fem_3d_guards"]];
for(const[name,o]of Object.entries(owners)){const path=root+o.manifest;let text=readFileSync(path,"utf8");
 for(const[section,files]of[["dependencies",o.lib],["dev-dependencies",o.dev]]as const)for(const[dep,needle]of crates){if(!uses(files,needle))continue;
  const line=dep+" = { workspace = true }",header="["+section+"]",at=text.indexOf(header+"\n");
  if(at<0){text=text.trimEnd()+"\n\n"+header+"\n"+line+"\n";continue;}
  const bodyStart=at+header.length+1,next=text.slice(bodyStart).search(/^\[/m),end=next<0?text.length:bodyStart+next;
  const body=text.slice(bodyStart,end),trimmed=body.replace(/\s+$/,""),tail=body.slice(trimmed.length);
  text=text.slice(0,bodyStart)+trimmed+"\n"+line+(tail||"\n")+text.slice(end);console.log(name,section,dep);}
 writeFileSync(path,text);}
