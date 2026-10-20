/** 🧪️ Bazel affected: the neutral rule, Bazel's own rdeps and the installed Nx's affected locators agree on every fixture case; a Python graph walk checks the generated package. */
import {test,expect} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {affectedBuildProjectsV1,affectedTouchesV1,selectBuildTasksV1} from "../run/selection.ts";
import {createScriptProcessEnvelope,withScriptProcessEnvelope} from "../../../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {bazelAffectedBuildV1,bazelAffectedExpressionV1,bazelAffectedLabelV1,bazelAffectedProjectsFromOutputV1,bazelAffectedWorkspaceV1} from "./affected.ts";
import schema from "./affected-schema.json" with {type:"json"};
import fixture from "./affected-fixture.json" with {type:"json"};

const findRoot=(from:string):string=>{for(let at=from;;at=dirname(at)){if(existsSync(join(at,"bunfig.toml"))&&existsSync(join(at,"bun.lock")))return at;if(dirname(at)===at)throw Error("repository root not found");}};
const root=findRoot(import.meta.dir),installed=createRequire(join(root,"package.json")),ops=JSON.parse(readFileSync(join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/⚙️operations/🧪️tests/🧫️fixtures/🔣️.json"),"utf8")) as {cases:{name:string;projects:Record<string,{root:string;targets:Record<string,unknown>}>;edges:string[][];files:string[];namedInputs?:Record<string,unknown[]>;expected:{affected:string[]}}[]};
type Row={name:string;projects:Record<string,{root:string;targets:Record<string,unknown>}>;edges:string[][];files:string[];namedInputs?:Record<string,unknown[]>;affected:string[];nx?:boolean};
const rows:Row[]=[...ops.cases.map(row=>({name:row.name,projects:row.projects,edges:row.edges,files:row.files,namedInputs:row.namedInputs,affected:row.expected.affected})),...fixture.cases as Row[]];
const graphOf=(row:Row)=>({version:1 as const,projects:Object.fromEntries(Object.entries(row.projects).map(([name,project])=>[name,{name,root:project.root,targets:project.targets}])),dependencies:row.edges.map(([source,target])=>({source:source!,target:target!,type:"static"})),namedInputs:row.namedInputs??{}}) as never;
const ajv=new Ajv({strict:true,allowUnionTypes:true});

test("the schema is valid and refuses malformed touches and answers",()=>{
 expect(ajv.validateSchema(schema)).toBe(true);
 const touches=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Touches"}),answer=ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/Answer"});
 for(const[value,ok]of [[{a:["x"]},true],[{a:[]},false],[{a:["x","x"]},false],[{a:[""]},false]] as const){expect(touches(value)).toBe(ok);expect(validateJsonSchemaSubset(schema.definitions.Touches,value,schema).length===0).toBe(ok);}
 for(const[value,ok]of [[["//:p_0123456789abcdef"],true],[["//x:p_0123456789abcdef"],false],[["ERROR"],false]] as const){expect(answer(value)).toBe(ok);expect(validateJsonSchemaSubset(schema.definitions.Answer,value,schema).length===0).toBe(ok);}
 expect(()=>bazelAffectedProjectsFromOutputV1({projects:{},dependencies:[]},"WARNING: x\n")).toThrow("Closed affected answer required");
});

test("the neutral rule gives every fixture case its authored answer, and its touches are a subset of the answer",()=>{
 for(const row of rows){
  const graph=graphOf(row);
  expect(affectedBuildProjectsV1(graph,row.files),row.name).toEqual(row.affected);
  for(const name of affectedTouchesV1(graph,row.files).keys())expect(row.affected,row.name).toContain(name);
 }
});

const python=`
import ast,json,sys
rows=json.load(sys.stdin)
out=[]
for row in rows:
    tree=ast.parse(row["build"])
    nodes={}
    for node in tree.body:
        call=node.value
        kw={k.arg:k.value for k in call.keywords}
        nodes[ast.literal_eval(kw["name"])]=[ast.literal_eval(e)[1:] for e in kw["srcs"].elts] if "srcs" in kw else []
    reverse={}
    for name,srcs in nodes.items():
        for s in srcs: reverse.setdefault(s,set()).add(name)
    seen=set(row["files"]); todo=list(seen)
    while todo:
        n=todo.pop()
        for m in reverse.get(n,()):
            if m not in seen: seen.add(m); todo.append(m)
    out.append(sorted(n for n in seen if n.startswith("p_")))
print(json.dumps(out))
`;

test("a Python walk over the generated package returns the authored answer for every case",()=>{
 const inputs=rows.map(row=>{const graph=graphOf(row),touches=affectedTouchesV1(graph,row.files);return{build:bazelAffectedBuildV1(graph,touches),files:[...new Set([...touches.values()].flat())].map(file=>bazelAffectedLabelV1("f",file)),graph,row};});
 const run=spawnSync("python3",["-I","-c",python],{input:JSON.stringify(inputs.map(input=>({build:input.build,files:input.files}))),encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const answers=JSON.parse(run.stdout) as string[][];
 for(const[index,input]of inputs.entries()){
  const names=new Map(Object.keys(input.graph.projects).map(name=>[bazelAffectedLabelV1("p",name),name]));
  expect(answers[index]!.map(label=>names.get(label)!).sort(),input.row.name).toEqual(input.row.affected);
 }
},60000);

function nxAffected(selected:readonly Row[]):Record<string,string[]>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"bzl-aff-")));
 try{
  writeFileSync(join(directory,"nx.json"),"{}\n");
  const source=`
const {filterAffected}=require(process.env.NX_AFFECTED),{WholeFileChange}=require(process.env.NX_FILES);
(async()=>{
 const input=JSON.parse(process.env.NX_CASES),result={};
 for(const row of input){
  const nodes=Object.fromEntries(Object.entries(row.projects).map(([name,project])=>[name,{name,type:"lib",data:{name,root:project.root,targets:Object.fromEntries(Object.entries(project.targets).map(([target,definition])=>[target,{executor:"nx:noop",...definition}]))}}])),
   dependencies=Object.fromEntries(Object.keys(nodes).map(name=>[name,row.edges.filter(([source])=>source===name).map(([source,target])=>({source,target,type:"static"}))])),
   graph={nodes,dependencies,externalNodes:{}},
   touched=row.files.map(file=>({file,hash:"",getChanges:()=>[new WholeFileChange()]}));
  result[row.name]=Object.keys((await filterAffected(graph,touched,{namedInputs:row.namedInputs??{}},{name:"workspace"})).nodes).sort();
 }
 process.stdout.write(JSON.stringify(result));
})().catch(error=>{process.stderr.write(String(error&&error.stack||error));process.exit(1);});`;
  const run=spawnSync("node",["-e",source],{cwd:directory,encoding:"utf8",maxBuffer:64*1024*1024,env:{...process.env,NX_AFFECTED:installed.resolve("nx/src/project-graph/affected/affected-project-graph"),NX_FILES:installed.resolve("nx/src/project-graph/file-utils"),NX_CASES:JSON.stringify(selected),NX_WORKSPACE_ROOT_PATH:directory,NX_DAEMON:"false"} as NodeJS.ProcessEnv});
  if(run.status!==0)throw Error("nx oracle: "+run.stderr);
  return JSON.parse(run.stdout);
 }finally{rmSync(directory,{recursive:true,force:true});}
}

test("the installed Nx's affected locators give every fixture case the same answer",()=>{
 const selected=rows.filter(row=>row.nx!==false),answers=nxAffected(selected);
 for(const row of selected)expect(answers[row.name],row.name).toEqual(row.affected);
},120000);

test("the selection asks the injected tool for the affected projects and keeps the neutral rule as the default",async()=>{
 const row=rows.find(candidate=>candidate.name==="dependents follow a changed library")!,graph=graphOf(row),seen:string[][]=[];
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"bazel-affected-model",maximumElapsedMilliseconds:60000},{},Date.now()),async original=>{
  const args=["affected","--targets=build",`--files=${row.files.join(",")}`];
  const neutral=await selectBuildTasksV1(root,graph,args,original),injected=await selectBuildTasksV1(root,graph,args,original,async files=>{seen.push([...files]);return ["tool"];});
  expect(neutral.tasks).toEqual(["app:build","lib:build"]);
  expect(injected.tasks).toEqual(["tool:build"]);
  expect(seen).toEqual([row.files]);
  const all=await selectBuildTasksV1(root,graph,["affected","--all","--targets=build"],original,async()=>{throw Error("not consulted");});
  expect(all.tasks).toEqual(["app:build","lib:build","tool:build"]);
 });
});

const binary=process.env.SEMIO_TEST_BAZEL_BINARY,real=binary?test:test.skip;
real("Bazel's rdeps over the generated package returns the same projects as the neutral rule and Nx",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"bzl-aff-q-"))),output=join(directory,"ob"),fixed=bazelAffectedWorkspaceV1();
 try{
  writeFileSync(join(directory,"MODULE.bazel"),fixed.module);
  for(const row of rows){
   const graph=graphOf(row),touches=affectedTouchesV1(graph,row.files);
   writeFileSync(join(directory,"BUILD.bazel"),bazelAffectedBuildV1(graph,touches));
   if(!touches.size){expect(row.affected,row.name).toEqual([]);continue;}
   const run=spawnSync(binary!,[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","--max_idle_secs=30","query",...fixed.options,bazelAffectedExpressionV1(touches)],{cwd:directory,encoding:"utf8",env:{...process.env,LANG:"en_US.UTF-8",LC_ALL:"en_US.UTF-8"}});
   expect(run.status,row.name+"\n"+run.stderr).toBe(0);
   expect(bazelAffectedProjectsFromOutputV1(graph,run.stdout),row.name).toEqual(row.affected);
  }
 }finally{
  spawnSync(binary!,[`--output_base=${output}`,"--nosystem_rc","--nohome_rc","shutdown"],{cwd:directory});
  spawnSync("chmod",["-R","u+w",directory]);
  rmSync(directory,{recursive:true,force:true});
 }
},300000);
