// Extracts every bash `run` body of the workflow and both composite actions, replaces `${{ ... }}` by a placeholder and runs `bash -n`
// (syntax only; shellcheck is not installed on this host). Usage: bun lint-scripts.ts <root>
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {join} from "node:path";
const root=process.argv[2]!;
type Step={name?:string;run?:string;shell?:string};
const files=[".github/workflows/build-system.yml",".github/actions/bun-repo/action.yml",".github/actions/cache-proof/action.yml"];
let checked=0,failed=0;
for(const file of files){
 const doc=Bun.YAML.parse(readFileSync(join(root,file),"utf8")) as {jobs?:Record<string,{steps:Step[]}>;runs?:{steps:Step[]}};
 const steps=[...Object.values(doc.jobs??{}).flatMap(job=>job.steps),...doc.runs?.steps??[]];
 for(const step of steps){
  if(step.run===undefined||step.shell==="pwsh")continue;
  const script=step.run.replace(/\$\{\{[\s\S]*?\}\}/gu,"EXPR");
  const result=spawnSync("bash",["-n"],{input:script,encoding:"utf8"});
  checked++;
  if(result.status!==0){failed++;console.log(`${file} :: ${step.name}\n${result.stderr}`);}
 }
}
console.log(`bash -n: ${checked} scripts checked, ${failed} failed`);
process.exit(failed===0?0:1);
