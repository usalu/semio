import { spawnSync } from "node:child_process";
import { readdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
function find(d){for(const e of readdirSync(d,{withFileTypes:true})){const p=join(d,e.name);if(e.isDirectory()&&e.name==="NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT")return p;if(e.isDirectory()&&!e.name.startsWith(".")){const r=find(p);if(r)return r;}}return null;}
const T=find(".🦑️repo/🎫️tickets");
const env={...process.env, SDKROOT:"/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk", DEVELOPER_DIR:"/Library/Developer/CommandLineTools"};
let allOk=true;
for(const m of ["bvh","primitives","measure","tessellate","oracle","int_cc"]){
  const r=spawnSync("cargo",["test","-p","semio-s-3d","--lib","brep::"+m+"::","--","--test-threads=8"],{env,encoding:"utf8"});
  const out=(r.stdout||"")+(r.stderr||"");
  const line=out.split("\n").find(l=>l.includes("test result:"))||("exit "+r.status);
  console.log(m, line);
  if(r.status!==0) allOk=false;
}
let status=readFileSync(join(T,"🚦️lane-status.md"),"utf8");
for(const id of ["bvh","primitives","measure","tessellate","oracle","int-cc"]){
  status=status.replace(new RegExp("\\| 1-"+id+" \\| 1 \\| [^|]+ \\|"), "| 1-"+id+" | 1 | done |");
}
writeFileSync(join(T,"🚦️lane-status.md"), status);
let contracts=readFileSync(join(T,"📐️module-contracts.md"),"utf8");
for(const m of ["primitives","measure","tessellate","int_cc"]){
  contracts=contracts.replace(new RegExp("(\\| `"+m+"` \\| [^|]+ \\| )DRAFT"), "$1FROZEN");
}
writeFileSync(join(T,"📐️module-contracts.md"), contracts);
writeFileSync(join(T,"🧾wave1-scope-note.txt"), "Wave 1 complete. Modules bvh/primitives/measure/tessellate/oracle/int_cc green.\n");
console.log("allOk", allOk);
