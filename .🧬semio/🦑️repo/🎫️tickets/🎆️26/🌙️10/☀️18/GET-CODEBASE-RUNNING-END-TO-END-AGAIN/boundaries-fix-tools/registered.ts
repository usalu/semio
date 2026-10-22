import {readFileSync,existsSync} from "node:fs";
import {dirname,relative,resolve} from "node:path";
const R="/Users/ueli/Documents/semio";
const names=new Set(["semio-framework-async-timer","semio-framework-async-sync","semio-framework-async-slots","semio-framework-async-testkit","semio-framework-pack-record-encode-cursor","semio-framework-os-store-presence","semio-framework-os-store-replay-operation","semio-framework-os-store-engine-close","semio-framework-os-store-engine-member-group"]);
const owners=JSON.parse(readFileSync(R+"/📋️build-owners.json","utf8")).owners as {name:string;root:string;manifest:string}[];
const rootToml=Bun.TOML.parse(readFileSync(R+"/Cargo.toml","utf8")) as any;
const fwToml=Bun.TOML.parse(readFileSync(R+"/🧰️framework/Cargo.toml","utf8")) as any;
const rootMembers:string[]=rootToml.workspace.members, fwMembers:string[]=fwToml.workspace.members;
for(const m of readFileSync("/private/tmp/claude-501/today-manifests.txt","utf8").split("\n").filter(Boolean)){
  const t=Bun.TOML.parse(readFileSync(R+"/"+m,"utf8")) as any;
  if(!names.has(t.package.name))continue;
  const d=dirname(m);const pj=existsSync(R+"/"+d+"/📋️project.json");
  const pname=pj?JSON.parse(readFileSync(R+"/"+d+"/📋️project.json","utf8")).name:null;
  const ow=owners.find(o=>o.name===pname);
  const sb=t.package.metadata?.semio?.["rust-source-boundary"];
  console.log(t.package.name.padEnd(46),"sb",JSON.stringify(sb),"| project.json",pj,pname,"| owner",!!ow&&ow.root===d,"| rootMember",rootMembers.includes(d),"fwMember",fwMembers.includes(relative(R+"/🧰️framework",R+"/"+d)));
}
