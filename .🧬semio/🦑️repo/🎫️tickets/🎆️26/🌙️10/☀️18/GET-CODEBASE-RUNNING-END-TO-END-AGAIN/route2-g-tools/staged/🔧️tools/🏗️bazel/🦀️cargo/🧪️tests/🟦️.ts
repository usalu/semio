/** 🏷️ Contract: typed Cargo actions render the exact ingress argv, a closed host-independent manifest, deterministic BUILD text and a spawn-free toolchain identity; native Cargo tasks are the only exclusive actions. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {spawnSync} from "node:child_process";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import * as cargo from "../🟦️.ts";
import {selectCargoArtifactsV1} from "../🏃️action/🟦️.ts";

const api=await import("../../📜️script.ts"),python=testPythonV1(),oracle=(source:string,input:unknown):any=>{const run=spawnSync(python!,["-I","-c",source],{input:JSON.stringify(input),encoding:"utf8"});if(run.status!==0)throw new Error(run.stderr);return JSON.parse(run.stdout);};

test("native Cargo commands classify by their authored command and execution kind",()=>{
 for(const row of corpus.classes)expect<unknown>(cargo.bazelCargoClassV1({definition:{execution:{command:row.command},metadata:{semio:{execution:{kind:row.kind}}}}}) ?? null,row.name).toEqual(row.expected);
});

test("actions render the ingress argument vector exactly like an independent implementation and refuse everything outside the closed schema",()=>{
 const ok=corpus.arguments.filter(row=>row.expected!==null),viaPython=oracle("import json,sys\nrows=json.load(sys.stdin)\ndef argv(a):\n v=['cargo','--store=shared','--configuration='+a['configuration'],'--parallelism=1','--',a['subcommand'],'--manifest-path',a['manifest'],'-p',a['package'],a['selection']]\n if a.get('target'): v+=['--target',a['target']]\n if a.get('features'): v+=['--features',','.join(sorted(a['features']))]\n return v+['--message-format=json-render-diagnostics']\nprint(json.dumps([argv(r['action']) for r in rows]))",ok);
 ok.forEach((row,index)=>{expect(cargo.bazelCargoArgumentsV1(row.action as any),row.action.id).toEqual(row.expected as string[]);expect(viaPython[index]).toEqual(row.expected);});
 for(const row of corpus.arguments.filter(row=>row.expected===null))expect(()=>cargo.bazelCargoArgumentsV1(row.action as any),row.action.id).toThrow();
});

test("manifests are closed, carry the toolchain and name no host path",()=>{
 const validate=new Ajv({strict:false}).compile({...schema.definitions.Manifest,definitions:schema.definitions});
 for(const row of corpus.arguments.filter(row=>row.expected!==null)){const manifest=cargo.bazelCargoManifestV1(row.action as any);expect(validate(manifest),JSON.stringify(validate.errors)).toBe(true);expect(manifest.toolchain).toBe(row.action.toolchain);expect(JSON.stringify(manifest)).not.toContain(process.cwd());}
});

test("artifacts are the deduplicated, name-sorted files of one package taken from Cargo's own JSON messages",()=>{
 const viaPython=oracle("import json,sys\nrows=json.load(sys.stdin)\ndef pick(r):\n found={}\n for line in r['lines']:\n  try: m=json.loads(line)\n  except ValueError: continue\n  if m.get('reason')!='compiler-artifact' or ('#'+r['package']+'@') not in m['package_id']: continue\n  for f in m['filenames']:\n   if any(f.endswith(s) for s in r['suffixes']): found[f.rsplit('/',1)[-1]]=f\n return [found[k] for k in sorted(found)]\nprint(json.dumps([pick(r) for r in rows]))",corpus.artifacts);
 corpus.artifacts.forEach((row,index)=>{expect(cargo.bazelCargoArtifactsV1(row.lines,row.package,row.suffixes),row.name).toEqual(row.expected);expect(viaPython[index],row.name).toEqual(row.expected);});
});

test("the toolchain identity needs no compiler process and changes with the pinned file and the installed channel manifest",async()=>{
 const digests:string[]=[];
 for(const row of corpus.toolchain){
  const root=mkdtempSync(join(tmpdir(),"bzl-cargo-")),home=join(root,"rustup");
  try{
   if(row.file!==null)writeFileSync(join(root,"rust-toolchain.toml"),row.file);
   if(row.installed!==null){const directory=join(home,"toolchains",row.expected_parts.channel+"-"+row.expected_parts.host,"lib","rustlib");mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"multirust-channel-manifest.toml"),row.installed);}
   const actual=await cargo.bazelCargoToolchainDigestV1(root,{RUSTUP_HOME:home},row.platform),expected=oracle("import hashlib,json,re,sys\nr=json.load(sys.stdin)\nh=lambda b:hashlib.sha256(b).hexdigest()\nf=r['file']\nc=re.search(r'^\\s*channel\\s*=\\s*\"([^\"]+)\"',f or '',re.M)\nd={'file':h(f.encode()) if f is not None else None,'channel':c.group(1) if c else None,'host':r['expected_parts']['host'],'installed':h(r['installed'].encode()) if r['installed'] is not None and c else None}\nprint(json.dumps(h(json.dumps(d,separators=(',',':'),ensure_ascii=False).encode())))",row);
   expect(actual,row.name).toBe(expected);digests.push(actual);
  }finally{rmSync(root,{recursive:true,force:true});}
 }
 expect(new Set(digests).size).toBe(digests.length);
});

test("rendered BUILD text equals an independent rendering and the Starlark rule serializes one Cargo action at a time",()=>{
 for(const row of corpus.render){
  const text=cargo.renderBazelCargoBuildV1(row as any),expected=oracle("import json,sys\nr=json.load(sys.stdin)\nq=lambda v:json.dumps(v,ensure_ascii=False)\nL=['load(%s, \"repo_cargo\")'%q(r['load']),'']\nfor a in sorted(r['actions'],key=lambda a:a['label']):\n L+=['repo_cargo(','    name = %s,'%q(a['label']),'    manifest = %s,'%q(a['manifest']),'    runner = %s,'%q(r['runner']),'    srcs = ['+''.join('\\n        %s,'%q(s) for s in sorted(set(a['srcs'])))+'\\n    ],','    bun = %s,'%q(r['bun']),'    identity = %s,'%q(a['id']),')','']\nprint(json.dumps('\\n'.join(L)))",row);
  expect(text,row.name).toBe(expected);
 }
 expect(cargo.BAZEL_CARGO_RULE_V1).toContain("resource_set = cargo_resources");
 expect(cargo.BAZEL_CARGO_RULE_V1).toContain('return {"cpu": '+cargo.BAZEL_CARGO_EXCLUSIVE_V1.cpu+'}');
});

test("only native Cargo task actions are exclusive and the generated rule carries the flag",()=>{
 const graph=corpus.adapter.graph as any,plan=api.planBazelTasksV1(graph,corpus.adapter.request),emitted=api.bazelActionsV1(graph,plan,false);
 expect(Object.fromEntries([...emitted.values()].map(action=>[action.id,action.exclusive===true]))).toEqual(corpus.adapter.expected);
 const text=api.renderBazelBuildV1({actions:[...emitted.values()].map(action=>({...action,sources:undefined})),groups:new Map(),bun:"/bin/bun",sourceIdentity:"x",extension:""});
 expect(text.match(/exclusive = True,/gu)?.length).toBe(1);
 const block=text.split("repo_task(").find(part=>part.includes('identity = "p:check"'))!;
 expect(block).toContain("exclusive = True,");
});

test.skipIf(!process.env.SEMIO_TEST_BAZEL_BINARY)("the real Bazel loads the rendered Cargo rule",()=>{
 const root=mkdtempSync(join(tmpdir(),"bzl-cargo-"));
 try{
  writeFileSync(join(root,"MODULE.bazel"),'module(name = "t", version = "0.0.0")\n');writeFileSync(join(root,"tasks.bzl"),cargo.BAZEL_CARGO_RULE_V1);writeFileSync(join(root,"runner.ts"),"");writeFileSync(join(root,"a.json"),"{}");
  writeFileSync(join(root,"BUILD.bazel"),cargo.renderBazelCargoBuildV1({load:":tasks.bzl",bun:"/bin/bun",runner:"runner.ts",actions:[{label:"a",id:"a",manifest:"a.json",srcs:[]}]}));
  const run=spawnSync(process.env.SEMIO_TEST_BAZEL_BINARY!,[`--output_base=${join(root,"ob")}`,"--nosystem_rc","--nohome_rc","--noworkspace_rc","query","//:all","--lockfile_mode=off","--incompatible_autoload_externally="],{cwd:root,encoding:"utf8"});
  expect(run.status,run.stderr).toBe(0);expect(run.stdout.trim()).toContain("//:a");
 }finally{spawnSync(process.env.SEMIO_TEST_BAZEL_BINARY!,[`--output_base=${join(root,"ob")}`,"--nosystem_rc","shutdown"],{cwd:root});rmSync(root,{recursive:true,force:true});}
},120000);
