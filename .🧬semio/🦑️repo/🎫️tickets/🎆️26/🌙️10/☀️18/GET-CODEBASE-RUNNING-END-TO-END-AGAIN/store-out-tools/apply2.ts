import {readFileSync,writeFileSync,existsSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {spawnSync} from "node:child_process";
import {apply,parse,prop,setProps,type Edit} from "../cache-fw-tools/jsonspan.ts";
const root=process.cwd(),write=process.argv.includes("--write"),{storeReceiptOutputV1}=await import(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️store-receipt/🟨️.mjs"));
const files=spawnSync("git",["ls-files","-z","--","*📋️project.json"],{cwd:root,encoding:"utf8",maxBuffer:1<<28}).stdout.split("\0").filter(Boolean).filter(f=>!f.startsWith(".🧬semio/"));
let changed=0,declared=0;
for(const file of files){
 const path=join(root,file);if(!existsSync(path))continue;const text=readFileSync(path,"utf8");let project:any;try{project=JSON.parse(text);}catch{continue;}
 const targets:string[]=[];
 for(const [name,target] of Object.entries<any>(project.targets??{})){
  const command:string=target.options?.command??"",m=/^bun (\S*📜️script\.ts) (\S+)/u.exec(command);
  const script=m?resolve(root,target.options?.cwd?.replace("{projectRoot}",project.root??dirname(file))??dirname(file),m[1]!):undefined;
  const source=script&&existsSync(script)?readFileSync(script,"utf8"):"";
  const artifactTypeScript=name==="build"&&(project.tags??[]).includes("role:artifact")&&/runArtifactTypeScriptPackageMain/u.test(source);
  const wasm=target.metadata?.semio?.wasmPackageSourceV1!==undefined&&name==="wasm";
  if(artifactTypeScript||wasm)targets.push(name);
 }
 if(!targets.length)continue;
 declared+=targets.length;
 const document=parse(text),targetsNode=prop(document,"targets")!.value,edits:Edit[]=[];
 for(const target of targets)edits.push(...setProps(text,prop(targetsNode,target)!.value,[["outputs",[storeReceiptOutputV1(target)]]],["cache","inputs"]));
 const next=apply(text,edits);JSON.parse(next);if(next!==text){changed+=1;console.log((write?"write ":"plan ")+file+" "+targets.join(","));if(write)writeFileSync(path,next);}
}
console.log("declared "+declared+", manifests changed "+changed);
