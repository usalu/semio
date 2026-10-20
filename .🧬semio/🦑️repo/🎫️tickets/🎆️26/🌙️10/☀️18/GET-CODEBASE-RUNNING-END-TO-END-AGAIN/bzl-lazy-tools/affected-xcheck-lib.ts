import {existsSync,mkdirSync,mkdtempSync,readFileSync,writeFileSync,rmSync,realpathSync} from "node:fs";
import {createRequire} from "node:module";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {spawnSync} from "node:child_process";
const root="/Users/ueli/Documents/semio",installed=createRequire(join(root,"package.json"));
export function nx(graph:any,files:string[]):string[]{
 const source=`
const {filterAffected}=require(process.env.NX_AFFECTED),{WholeFileChange,DeletedFileChange,LockFileChange}=require(process.env.NX_FILES),{jsonDiff}=require(process.env.NX_JSONDIFF),fs=require('fs'),path=require('path'),cp=require('child_process');
(async()=>{
 const input=JSON.parse(fs.readFileSync(process.env.NX_INPUT,'utf8')),g=input.graph;
 const nodes=Object.fromEntries(Object.entries(g.projects).map(([name,p])=>[name,{name,type:'lib',data:{name,root:p.root,...(p.namedInputs?{namedInputs:p.namedInputs}:{}),targets:Object.fromEntries(Object.entries(p.targets).map(([t,d])=>[t,{executor:'nx:noop',...(d.inputs?{inputs:d.inputs}:{}),...(d.dependsOn?{dependsOn:d.dependsOn}:{})}]))}}])),
  dependencies=Object.fromEntries(Object.keys(nodes).map(n=>[n,[]]));
 for(const e of g.dependencies)if(nodes[e.source]&&nodes[e.target])dependencies[e.source].push({source:e.source,target:e.target,type:'static'});
 const REPO=process.env.REPO_ROOT,atBase=f=>cp.execFileSync('git',['show','HEAD:'+f],{cwd:REPO,encoding:'utf8',maxBuffer:512*1024*1024,stdio:['ignore','pipe','ignore']}),atHead=f=>fs.readFileSync(path.join(REPO,f),'utf8'),lock=new Set(['yarn.lock','package-lock.json','pnpm-lock.yaml','pnpm-lock.yml','bun.lock','bun.lockb']);
 const touched=input.files.map(file=>({file,hash:'',getChanges:()=>{if(!fs.existsSync(path.join(REPO,file)))return [new DeletedFileChange()];const base=file.split('/').pop();if(lock.has(base)){try{return [new LockFileChange(atBase(file),atHead(file))];}catch{return [new WholeFileChange()];}}if(path.extname(file)==='.json'){try{return jsonDiff(JSON.parse(atBase(file)),JSON.parse(atHead(file)));}catch{return [new WholeFileChange()];}}return [new WholeFileChange()];}}));
 const affected=Object.keys((await filterAffected({nodes,dependencies,externalNodes:{}},touched,{namedInputs:g.namedInputs??{}},{name:'workspace'})).nodes).sort();
 process.stdout.write(JSON.stringify(affected));
})().catch(e=>{process.stderr.write(String(e&&e.stack||e));process.exit(1);});`;
 const scratch=realpathSync(mkdtempSync(join(tmpdir(),"bzl-xcheck-")));writeFileSync(join(scratch,"nx.json"),"{}\n");for(const file of files)if(existsSync(join(root,file))){mkdirSync(dirname(join(scratch,file)),{recursive:true});writeFileSync(join(scratch,file),file.endsWith(".json")?"{}\n":"");}const input=join(scratch,"nx-input.json");writeFileSync(input,JSON.stringify({graph,files}));
 const run=spawnSync("node",["--max-old-space-size=6144","-e",source],{cwd:scratch,encoding:"utf8",maxBuffer:256*1024*1024,env:{...process.env,NX_AFFECTED:installed.resolve("nx/src/project-graph/affected/affected-project-graph"),NX_FILES:installed.resolve("nx/src/project-graph/file-utils"),NX_JSONDIFF:installed.resolve("nx/src/utils/json-diff"),REPO_ROOT:root,NX_INPUT:input,NX_WORKSPACE_ROOT_PATH:scratch,NX_DAEMON:"false"} as NodeJS.ProcessEnv});
 rmSync(scratch,{recursive:true,force:true});
 if(run.status!==0)throw Error("nx: "+run.stderr.slice(0,2000));
 return JSON.parse(run.stdout);
}

