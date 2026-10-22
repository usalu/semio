/** 🕸 Contract: the project dependency edges of a module (relative imports to the deepest owner, bare imports to the workspace package, the lock file or nowhere, core modules to nothing) and the tracked source file choice equal the authored cases in `./🔣️.json` and an independent Python derivation; the authored specifier lists equal what the TypeScript compiler's own pre-processor reads from the source, and the real graph code asks the lock file exactly the packages the corpus lists. */
import {afterAll,beforeAll,expect,setDefaultTimeout,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {requireTestToolV1} from "@FW/🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import corpus from "@CORPUS";
import corpusSchema from "@SCHEMA";

setDefaultTimeout(60000);

const inference=await import("@LIB/🏗️build/🕸️graph/📥️inference/🟨️.mjs"),{cacheInternals}=inference as any;
await (inference as any).libraryBootstrap;
const python=requireTestToolV1("python"),oracleFile=join(import.meta.dir,"@ORACLE");
const ask=(requests:unknown[]):any[]=>{const run=spawnSync(python,["-I",oracleFile],{input:JSON.stringify({requests}),encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout);};
let workspace="";
beforeAll(()=>{workspace=realpathSync(mkdtempSync(join(tmpdir(),"dep-edges-")));});
afterAll(()=>{rmSync(workspace,{recursive:true,force:true});});

test("the corpus is closed by its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(corpusSchema,corpus),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate(corpusSchema,{...corpus,workspaceRoot:"ws"})).toBe(false);
 expect(ajv.validate(corpusSchema,{...corpus,rows:[{...corpus.rows[0],queried:[["a"]]}]})).toBe(false);
});

test("the authored specifier lists are what the TypeScript pre-processor reads from the sources",()=>{
 for(const row of corpus.rows){
  if(row.file.endsWith(".json"))continue;
  const read=ts.preProcessFile(row.text,true,true).importedFiles.map(entry=>entry.fileName);
  expect([...new Set(read)].sort(),row.name).toEqual([...new Set(row.imports)].sort());
 }
});

test("a module's imports become project edges exactly as the authored rows and the oracle say, and the lock file is asked only about the remaining packages",()=>{
 const answers=ask(corpus.rows.map(row=>({kind:"targets",file:row.file,imports:row.imports,projects:corpus.projects,byPackage:corpus.byPackage,locked:corpus.locked,workspaceRoot:corpus.workspaceRoot})));
 for(const[index,row]of corpus.rows.entries()){
  const asked:[string,string][]=[],locked={resolveImport:(file:string,name:string)=>{asked.push([file,name]);return (corpus.locked.keys as Record<string,string>)[name];},externalNodes:Object.fromEntries(corpus.locked.external.map(key=>[key,{}]))};
  const actual=cacheInternals.importTargetsFromSource(row.text,row.file,corpus.workspaceRoot,structuredClone(corpus.projects),new Map(Object.entries(corpus.byPackage)),locked);
  expect(actual,row.name).toEqual(row.expected);
  expect(answers[index].targets,row.name).toEqual(row.expected);
  expect(answers[index].queried.length>=0).toBe(true);
  if("queried" in row)expect(asked,row.name).toEqual(row.queried);
  expect(asked,row.name).toEqual(answers[index].queried);
 }
 expect(cacheInternals.importTargetsFromSource("import a from 'left-pad';","x.ts",corpus.workspaceRoot,{},new Map(),undefined)).toEqual([]);
 expect(cacheInternals.importTargetsFromSource("import a from 'left-pad';","x.ts",corpus.workspaceRoot,{},new Map(),{externalNodes:{}})).toEqual([]);
});

test("a moved dependency source file is found by its base name only when exactly one candidate exists",()=>{
 const answers=ask(corpus.tracked.map(row=>({kind:"tracked",files:row.files,map:row.map,project:row.project,preferred:row.preferred,hint:(row as any).hint})));
 for(const[index,row]of corpus.tracked.entries()){
  const root=join(workspace,String(index));mkdirSync(root,{recursive:true});
  for(const file of row.files){mkdirSync(dirname(join(root,file)),{recursive:true});writeFileSync(join(root,file),"");}
  const context={workspaceRoot:root,...(row.map?{fileMap:{projectFileMap:Object.fromEntries(Object.entries(row.map).map(([project,files])=>[project,files.map(file=>({file}))]))}}:{})};
  expect(cacheInternals.buildTrackedSourceFile(context,row.project,row.preferred,(row as any).hint),row.name).toBe(row.expected);
  expect(answers[index],row.name).toBe(row.expected);
 }
});
