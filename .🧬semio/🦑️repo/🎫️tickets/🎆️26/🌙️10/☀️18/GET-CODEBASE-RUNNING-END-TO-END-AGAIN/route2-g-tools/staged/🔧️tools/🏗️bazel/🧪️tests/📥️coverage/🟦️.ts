/** 🧪️ Contract: a selected, unconsumed, cached task named test that declares no outputs is a Bazel test target; everything else keeps its action kind. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {readFile} from "node:fs/promises";
import ts from "typescript";
import Ajv from "ajv";
import coverageSchema from "../../../../🏃️execution/🧬️schema/🎯️coverage.json";
import component from "../../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema.json";

const runner="\u{1F4DC}\uFE0Fscript.ts",api=await import("../../📜️script.ts"),graph=fixture.graph as any,python=testPythonV1(),ajv=()=>new Ajv({strict:true}).addSchema(coverageSchema,"coverage");

test("test disposition fixture is closed and its expectations are admitted by the neutral schema",()=>{
 const validate=ajv().compile(fixtureSchema);
 expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 expect(validate({...fixture,requests:[{...fixture.requests[0],kinds:{"lib:test":"suite"}}]})).toBe(false);
});

test("a selected pure test becomes a Bazel test only when no other action consumes its result",()=>{
 for(const request of fixture.requests){
  const plan=api.planBazelTasksV1(graph,{tasks:request.tasks,args:[]}),emitted=api.bazelActionsV1(graph,plan,"skipCache" in request&&request.skipCache===true);
  expect(Object.fromEntries([...emitted.entries()].map(([id,action])=>[id,action.kind as string])),request.name).toEqual(request.kinds as unknown as Record<string,string>);
  const consumed=new Set([...emitted.values()].flatMap(action=>action.restore));
  for(const action of emitted.values())if(action.kind==="test"){expect(consumed.has(action.label),request.name).toBe(false);expect(action.cache.cache).toBe(true);expect(plan.selected).toContain(action.id);expect(plan.tasks[action.id]!.target).toMatch(/^test(-|$)/u);expect(plan.tasks[action.id]!.definition.outputs).toEqual([]);}
 }
});

test("the adapter reports the neutral family and disposition of every planned task",()=>{
 const plan=api.planBazelTasksV1(graph,{tasks:["lib:test","app:test-live","app:test-snapshot","app:check","app:serve"],args:[]}),emitted=api.bazelActionsV1(graph,plan,false),admit=ajv().compile({$ref:"coverage#"});
 for(const[id,expected]of Object.entries(fixture.coverage)){
  const actual=api.bazelTaskCoverageV1(plan.tasks[id]!,emitted.get(id)!.cache);
  expect(actual,id).toEqual(expected);expect(admit(actual),id).toBe(true);
 }
});

test("the adapter counts a planned selection by family and by how Bazel executes it",()=>{
 const admit=ajv().compile({$ref:"coverage#/definitions/Report"});
 for(const report of fixture.reports){
  const plan=api.planBazelTasksV1(graph,{tasks:report.tasks,args:[]}),emitted=api.bazelActionsV1(graph,plan,false),actual=api.bazelCoverageReportV1(plan,emitted);
  expect(actual,report.name).toEqual(report.expected as never);expect(admit(actual),report.name).toBe(true);
 }
});

test.skipIf(!python)("generated BUILD declares one repo_test per Bazel test and an independent Python parse agrees on its arguments",async()=>{
 const plan=api.planBazelTasksV1(graph,{tasks:["lib:test","app:test","app:build"],args:[]}),emitted=api.bazelActionsV1(graph,plan,false),identity="a".repeat(64);
 const build=api.renderBazelBuildV1({actions:[...emitted.values()],groups:new Map(),bun:"/opt/bun",sourceIdentity:identity,extension:""});
 expect(build.split("\n")[0]).toContain('"repo_test"');
 const program="import ast,json,sys\nsource=json.loads(sys.stdin.read())\ntree=ast.parse(source)\nout=[]\nfor node in tree.body:\n    if isinstance(node,ast.Expr) and isinstance(node.value,ast.Call):\n        call=node.value\n        row={'rule':call.func.id}\n        for keyword in call.keywords:\n            row[keyword.arg]=ast.literal_eval(keyword.value)\n        out.append(row)\nprint(json.dumps(out,ensure_ascii=False))",
  result=Bun.spawnSync([python!,"-I","-c",program],{stdin:new TextEncoder().encode(JSON.stringify(build.replace(/^load\(.*$/mu,""))),stdout:"pipe",stderr:"pipe"});
expect(result.exitCode,result.stderr.toString()).toBe(0);
 const rows=JSON.parse(result.stdout.toString()) as Record<string,unknown>[],tests=rows.filter(row=>row.rule==="repo_test"),expectedTests=[...emitted.values()].filter(action=>action.kind==="test");
 expect(tests.map(row=>row.name).sort()).toEqual(expectedTests.map(action=>action.label).sort());
 for(const row of tests){
  const action=expectedTests.find(value=>value.label===row.name)!;
  expect(row.args).toEqual([runner,"implementation.json",identity,"birth-test",action.label+".json",...action.restore.map(label=>label+".snapshot")]);
  expect(row.timeout).toBe("eternal");expect(row.cache).toBe(true);expect(row.manifest).toBe(action.label+".json");expect(row.fingerprint).toBe(action.label+".input");expect(row.identity).toBe(action.id);expect("uncached_reason" in row).toBe(false);
 }
 expect(rows.filter(row=>row.rule==="repo_task").length).toBe([...emitted.values()].filter(action=>action.kind==="task").length);
 const rules=api.bazelStarlarkRulesV1();
 const parsed=Bun.spawnSync([python!,"-I","-c","import ast,json,sys\ntree=ast.parse(json.loads(sys.stdin.read()))\nrow={}\nfor node in tree.body:\n    if isinstance(node,ast.Assign) and isinstance(node.value,ast.Call) and getattr(node.value.func,'id','')=='rule':\n        name=node.targets[0].id\n        row[name]={k.arg:ast.unparse(k.value) for k in node.value.keywords}\nprint(json.dumps(row))"],{stdin:new TextEncoder().encode(JSON.stringify(rules)),stdout:"pipe",stderr:"pipe"});
 expect(parsed.exitCode,parsed.stderr.toString()).toBe(0);
 const declared=JSON.parse(parsed.stdout.toString()) as Record<string,Record<string,string>>;
 expect(Object.keys(declared).sort()).toEqual(["repo_inputs_witness","repo_launch","repo_noop","repo_task","repo_test"]);expect(declared.repo_inputs_witness!.test).toBeUndefined();
 expect(declared.repo_test!.test).toBe("True");expect(declared.repo_task!.test).toBeUndefined();expect(declared.repo_launch!.executable).toBe("True");
 for(const name of Object.keys(declared).filter(name=>declared[name]!.test==="True"))expect(name.endsWith("_test")).toBe(true);
});

test("the Bazel birth protocol admits the test birth kind and nothing else new",()=>{
 const birth=ajv().addSchema(component,"component").compile({$ref:"component#/definitions/ActionBirth"}),row={task:"lib:test",manifestSha256:"a".repeat(64),parameters:["/x/t_a.snapshot"]};
 for(const kind of ["action","launcher","launch","test"])expect(birth({...row,kind}),kind).toBe(true);
 expect(birth({...row,kind:"tests"})).toBe(false);
});
