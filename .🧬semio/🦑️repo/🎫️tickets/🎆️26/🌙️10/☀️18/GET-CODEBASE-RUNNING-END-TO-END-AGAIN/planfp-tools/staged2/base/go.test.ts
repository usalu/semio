/** 🏷️ Contract: a task of the Go family is keyed by a spawn-free toolchain identity (version file, binary, host, compile variables) that is independent of where the toolchain is installed and of the checkout path. */
import {expect,test} from "bun:test";
import {runtimeIdentityV1} from "../../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {mkdirSync,mkdtempSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {createHash,randomUUID} from "node:crypto";
import {readFile} from "node:fs/promises";
import ts from "typescript";
import {spawnSync} from "node:child_process";
import Ajv from "ajv";
import corpus from "./🔣️.json";
import schema from "../🧬️schema/🔣️.json";
import * as go from "../🟦️.ts";

const python=testPythonV1(),oracle=(source:string,input:unknown):any=>{const run=spawnSync(python!,["-I","-c",source],{input:JSON.stringify(input),encoding:"utf8"});if(run.status!==0)throw new Error(run.stderr);return JSON.parse(run.stdout);};
const materialize=(row:(typeof corpus.digests)[number],root:string)=>{
 for(const [path,text] of Object.entries(row.files)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}
 for(const [path,target] of Object.entries((row as {links?:Record<string,string>}).links??{}))symlinkSync(join(root,target),join(root,path));
 mkdirSync(join(root,"empty"),{recursive:true});
 return Object.fromEntries(Object.entries(row.env).map(([name,value])=>[name,value.replaceAll("@root",root)]));
};

test("a task is Go exactly when the neutral family rule table says so",()=>{
 for(const row of corpus.classes)expect(go.bazelGoClassV1({project:row.project,target:row.target,root:row.root,definition:{execution:{kind:row.kind,...("command" in row?{command:row.command}:{})}}}),row.name).toBe(row.expected);
});

test("the identity is closed and the policy lists exactly the variables the corpus relies on",()=>{
 const policy=schema.definitions.Policy.const;
 expect(policy.environment).toEqual(corpus.policyEnvironment);
 expect(policy.hosts).toEqual(corpus.hosts);
 const validate=new Ajv({strict:false}).compile({...schema.definitions.Identity,definitions:schema.definitions});
 return Promise.all(corpus.digests.map(async row=>{
  const root=mkdtempSync(join(tmpdir(),"bzl-go-"));
  try{const identity=await go.bazelGoIdentityV1(materialize(row,root),row.platform);expect(validate(identity),JSON.stringify(validate.errors)).toBe(true);expect(JSON.stringify(identity),row.name).not.toContain(root);}finally{rmSync(root,{recursive:true,force:true});}
 }));
});

test("the digest equals an independent implementation, ignores the install path and irrelevant variables, and moves with every compile-relevant input",async()=>{
 const roots=[mkdtempSync(join(tmpdir(),"bzl-go-")),mkdtempSync(join(tmpdir(),"bzl-go-"))],actual:Record<string,string>={};
 try{
  const expected=oracle("import hashlib,json,posixpath,sys\nd=json.load(sys.stdin)\nh=lambda b:hashlib.sha256(b.encode()).hexdigest()\ndef digest(r):\n files=r['files']; links=r.get('links',{}); env=r['env']\n def real(p):\n  for a,b in links.items():\n   if p==a or p.startswith(a+'/'): return b+p[len(a):]\n  return p\n names=[]\n if env.get('GOROOT'): names.append(env['GOROOT'].replace('@root','')+'/bin/go')\n names+=[e.replace('@root','')+'/go' for e in env.get('PATH','').split(':') if e]\n binary=None\n for n in names:\n  q=real(n)\n  if q in files: binary=q; break\n ident={'version':h(files[posixpath.dirname(posixpath.dirname(binary))+'/VERSION']) if binary and posixpath.dirname(posixpath.dirname(binary))+'/VERSION' in files else None,'binary':h(files[binary]) if binary else None,'host':d['hosts'].get(r['platform']),'environment':{k:env[k] for k in d['env'] if k in env}}\n return hashlib.sha256(json.dumps(ident,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()\nprint(json.dumps({r['name']:digest(r) for r in d['rows']}))",{rows:corpus.digests,hosts:corpus.hosts,env:corpus.policyEnvironment});
  for(const row of corpus.digests){
   const digests=await Promise.all(roots.map(root=>go.bazelGoToolchainDigestV1(materialize(row,root),row.platform)));
   expect(digests[0],row.name+" install path").toBe(digests[1]);
   expect(digests[0],row.name).toBe(expected[row.name]);
   actual[row.name]=digests[0]!;
  }
  for(const row of corpus.digests){
   const same=(row as {same_as?:string}).same_as;
   if(same)expect(actual[row.name],row.name).toBe(actual[same]);
  }
  const distinct=corpus.digests.filter(row=>!(row as {same_as?:string}).same_as).map(row=>actual[row.name]);
  expect(new Set(distinct).size).toBe(distinct.length);
 }finally{for(const root of roots)rmSync(root,{recursive:true,force:true});}
});

test("the adapter's action key takes the Go toolchain digest for Go tasks only",async()=>{
 const source=await readFile(new URL("../../📜️script.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("bazel.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="taskFingerprint");
 if(!node)throw Error("Bazel fingerprint implementation required");
 const code=ts.transpileModule(node.getText(tree).replace(/^export /u,"")+";return taskFingerprint;",{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const fingerprint=async(task:{target:string;root:string},digest:string,cargo=false)=>{
  const bindings={runtimeIdentityV1,bazelNativeToolchainClassV1:()=>undefined,createHash,randomUUID,join,bazelCargoClassV1:()=>cargo?{subcommand:"test",manifest:"Cargo.toml"}:undefined,bazelCargoToolchainDigestV1:async()=>"cargo",bazelGoClassV1:go.bazelGoClassV1,bazelGoToolchainDigestV1:async()=>digest,process:{...process,env:{}},bazelTaskInputsV1:()=>[],issueBazelTaskEnvironmentV1:()=>({SEMIO_BUILD_MODE:"prototype"}),bazelTaskBaseEnvironmentV1:()=>({}),checkScriptInvocation:()=>undefined,bazelSourceIdentityV1:async()=>"source",captureOwnedCommandV1:async()=>""};
  const run=Function(...Object.keys(bindings),code)(...Object.values(bindings)),definition={execution:{kind:"command",command:"repo ./📜️script.ts test"}};
  return run("/root",{},{id:"p:"+task.target,project:"p",target:task.target,root:task.root,args:[],definition},["a.go"],new Map(),{control:{signal:new AbortController().signal}},{},true,"implementation",undefined) as Promise<string>;
 };
 const goRow=corpus.classes.find(row=>row.expected)!,otherRow=corpus.classes.find(row=>!row.expected&&row.kind==="command")!;
 const goTask={target:goRow.target,root:goRow.root},otherTask={target:otherRow.target,root:otherRow.root};
 expect(await fingerprint(goTask,"one")).not.toBe(await fingerprint(goTask,"two"));
 expect(await fingerprint(goTask,"one")).toBe(await fingerprint(goTask,"one"));
 expect(await fingerprint(otherTask,"one")).toBe(await fingerprint(otherTask,"two"));
});
