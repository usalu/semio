import {readFileSync,existsSync} from "node:fs";
import {join,dirname,resolve} from "node:path";
import {spawnSync} from "node:child_process";
const root=process.cwd();
const files=spawnSync("git",["ls-files","-z","--","*📋️project.json"],{cwd:root,encoding:"utf8",maxBuffer:1<<28}).stdout.split("\0").filter(Boolean).filter(f=>!f.startsWith(".🧬semio/"));
const markers:Record<string,RegExp>={cargo:/buildAdmittedCargoArtifactsV1|buildRepositoryCargoArtifacts|runDeclaredRustBuildV1|runArtifactRustPackageMain/u,vite:/buildViteArtifact|runViteBuild/u,stage:/stageRepositoryArtifacts|withRepositoryArtifactDestinationV1|withRepositoryPublishedCargoExecutableV1/u};
const rows:any[]=[];
for(const file of files){let project:any;try{project=JSON.parse(readFileSync(join(root,file),"utf8"));}catch{continue;}
 for(const[name,target]of Object.entries<any>(project.targets??{})){const command:string=target.options?.command??"";const m=/^bun (\S*📜️script\.ts) (\S+)/u.exec(command);if(!m)continue;
  const cwd=resolve(root,target.options?.cwd??dirname(file)),script=resolve(cwd,m[1]);if(!existsSync(script))continue;const text=readFileSync(script,"utf8");const kinds=Object.entries(markers).filter(([,re])=>re.test(text)).map(([k])=>k);if(!kinds.length)continue;
  rows.push({project:project.name,file,target:name,verb:m[2],kinds:kinds.join("+"),cache:target.cache,outputs:target.outputs});}}
console.log(JSON.stringify(rows,null,1));
