// 🕸 Writes a synthetic BuildGraphV1: independent projects below the ticket's generated folder, each with one task of a worker action kind.
import {mkdirSync,writeFileSync} from "node:fs";
import {join,relative} from "node:path";
const root=process.cwd(),dir=process.argv[2]!,plan=(process.argv[3]??"test:8,check:8,generate:8").split(",").map(row=>row.split(":")),work=process.argv[4]??"120";
const projects:Record<string,unknown>={};
for(const[kind,count]of plan)for(let index=0;index<Number(count);index++){
 const name=`synthetic-${kind}-${index}`,project=join(dir,"p",name);mkdirSync(project,{recursive:true});
 writeFileSync(join(project,"📜️script.ts"),`const [kind="x",ms="0"]=process.argv.slice(2),end=performance.now()+Number(ms);let n=0;while(performance.now()<end)n++;console.log("[bw] "+kind+" done "+(n>0));\n`);
 projects[name]={name,root:relative(root,project),targets:{[`${kind}-${index}`]:{execution:{kind:"command",command:`bun ./📜️script.ts ${kind} ${work}`,cwd:"{projectRoot}"},inputs:[],outputs:[],cache:false}}};
}
writeFileSync(join(dir,"graph.json"),JSON.stringify({version:1,projects,dependencies:[],namedInputs:{default:[],sharedGlobals:[]}}));
console.log("graph",Object.keys(projects).length,"projects",join(dir,"graph.json"));
