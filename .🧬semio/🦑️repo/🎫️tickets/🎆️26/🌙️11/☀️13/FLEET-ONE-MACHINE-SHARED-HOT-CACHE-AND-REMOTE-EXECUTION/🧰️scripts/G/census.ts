import {execSync} from "node:child_process";
import {readFileSync,writeFileSync} from "node:fs";
import {join,dirname} from "node:path";
import * as c from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/📍️coordinates/🟨️.mjs";
import * as tp from "/Users/Shared/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎯️target-policy/🟨️.mjs";
tp.wire({coordinates:c});
const root="/Users/Shared/semio";
const files=execSync("git ls-files -z",{cwd:root,maxBuffer:1<<29}).toString().split("\0").filter(f=>/(^|\/)(📋️)?project\.json$/u.test(f)&&!f.startsWith(".🧬semio/"));
const projects:Record<string,any>={};let bad=0;
for(const f of files){let j:any;try{j=JSON.parse(readFileSync(join(root,f),"utf8"));}catch{bad++;continue;}
 const name=j.name??f;const targets:Record<string,any>={};
 for(const [t,def] of Object.entries<any>(j.targets??{})){
  let d=tp.targetWithDefaults(def,dirname(f)||".",true) as any;
  d=tp.targetPolicy(t,d,c.POLICY,name+":"+t);
  d=tp.cachedOutputsAdmittedV1(d);
  const cmd=d.options?.command??d.command??(d.options?.commands?JSON.stringify(d.options.commands):"");
  targets[t]={...d,execution:{kind:d.executor==="nx:noop"||d.executor==="repo:noop"?"noop":"command",command:cmd}};
 }
 projects[name]={name,root:dirname(f)||".",targets,namedInputs:j.namedInputs??{}};
}
writeFileSync("/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/G/census-graph.json",JSON.stringify({version:1,projects,dependencies:[],namedInputs:JSON.parse(readFileSync(join(root,"nx.json"),"utf8")).namedInputs}));
console.log(files.length,"project files,",bad,"unparsable,",Object.keys(projects).length,"projects");
