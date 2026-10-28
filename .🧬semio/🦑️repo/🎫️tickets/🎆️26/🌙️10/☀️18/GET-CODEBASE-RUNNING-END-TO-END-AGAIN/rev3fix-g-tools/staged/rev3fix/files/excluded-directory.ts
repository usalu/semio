/** 🙈️ Contract and oracle: the witness of a planner run may only hold while the complete plan would be identical. A directory with an excluded name (`dist`, `build`, `target`, `coverage`, ...) that git does not ignore can gain files that git lists and the planner reads; the witness walk enters exactly those directories, so a file that appears below one makes the witness differ, while a directory that git ignores holds nothing of the universe and the witness keeps holding. The oracle is `git ls-files -co --exclude-standard` before and after the change. Run from the repository root with real git checkouts and the real planner, like the oracle of the plan reuse. */
import {expect,test}from"bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import {tmpdir}from"node:os";
import {dirname,join}from"node:path";
import corpus from"../../../🎬️scopes/🧪️tests/🧱️workspace/🔣️.json";
import {readWorldWitnessV1,verifyWorldWitnessV1,worldWitnessEnvironmentV1,worldWitnessGitDigestV1,worldWitnessPreloadV1}from"../../../../../🔭️witness/🟨️.mjs";
import {bazelIgnoreSourcesDigestV1}from"../../../📜️script.ts";

process.env.SEMIO_LANGUAGE="en";
const repo=process.cwd(),child=join(import.meta.dirname,"../🧫️fixtures/🟦️.ts"),signal=new AbortController().signal;
const git=(root:string,...args:string[])=>{const run=Bun.spawnSync(["git","-c","user.name=t","-c","user.email=t@t","-c","core.fsmonitor=false","-c","commit.gpgsign=false",...args],{cwd:root,stdout:"pipe",stderr:"pipe"});if(run.exitCode!==0)throw Error(args.join(" ")+": "+run.stderr.toString());return run.stdout.toString();};
const write=(root:string,path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
function checkout(prepare:(root:string)=>void){
 const base=realpathSync(mkdtempSync(join(tmpdir(),"excluded-"))),root=join(base,"checkout");mkdirSync(root);
 for(const[path,text]of Object.entries(corpus.files))write(root,path,text);
 write(root,"other/x.txt","x\n");
 git(root,"init","-q");git(root,"add","-A");git(root,"commit","-q","-m","fixture");
 prepare(root);
 return{base,root};
}
function plan(base:string,root:string){
 writeFileSync(join(base,"preload.mjs"),worldWitnessPreloadV1());
 const env={...process.env,SEMIO_LANGUAGE:"en",...worldWitnessEnvironmentV1({out:join(base,"witness.json"),root,workspace:join(base,"ws"),writable:[tmpdir()],racyMilliseconds:0})};
 const run=Bun.spawnSync([process.execPath,"--preload",join(base,"preload.mjs"),child,root,join(base,"ws"),"{}",repo],{cwd:repo,env,stdout:"pipe",stderr:"pipe"});
 if(run.exitCode!==0)throw Error(run.stderr.toString()+run.stdout.toString());
 return readWorldWitnessV1(readFileSync(join(base,"witness.json"),"utf8"));
}
const ports=(root:string)=>({git:async(subject:string,expected:string)=>{const specs=JSON.parse(subject) as string[];let listing:string;try{listing=git(root,"ls-files","-c","-z",...(specs.length?["--",...specs]:[]));}catch{return expected==="!failed";}return worldWitnessGitDigestV1(listing)===expected;},ignore:async(_subject:string,expected:string)=>(await bazelIgnoreSourcesDigestV1(root,signal))===expected});

const cases:Record<string,{prepare:(root:string)=>void;later?:string;listed?:boolean}>={
 "an untracked file below a dist directory that git does not ignore":{prepare:root=>write(root,"course/dist/out.txt","o\n"),later:"course/dist/later.txt"},
 "an untracked file below a build directory that git does not ignore":{prepare:root=>write(root,"course/build/out.txt","o\n"),later:"course/build/later.txt"},
 "an untracked file below a nested target directory that git does not ignore":{prepare:root=>write(root,"course/src/target/out.txt","o\n"),later:"course/src/target/later.txt"},
 "an untracked file below a coverage directory of a dependency that git does not ignore":{prepare:root=>write(root,"gen/src/coverage/out.txt","o\n"),later:"gen/src/coverage/later.txt"},
 "an empty dist directory that git does not ignore":{prepare:root=>mkdirSync(join(root,"course/dist")),later:"course/dist/later.txt"},
 "a nested dist directory below a dist directory that git does not ignore":{prepare:root=>mkdirSync(join(root,"course/dist/dist"),{recursive:true}),later:"course/dist/dist/later.txt"},
 "a dist directory that git ignores by a directory rule":{prepare:root=>{write(root,".gitignore","dist/\n");write(root,"course/dist/out.txt","o\n");},later:"course/dist/later.txt",listed:false},
 "a target directory that git ignores by a rule of the project":{prepare:root=>{write(root,"course/.gitignore","target\n");write(root,"course/target/out.txt","o\n");},later:"course/target/later.txt",listed:false},
 "a .nx directory that git does not ignore as a whole but whose children it ignores":{prepare:root=>{write(root,".gitignore",".nx/cache\n.nx/workspace-data\n");write(root,".nx/cache/x","x\n");write(root,".nx/workspace-data/y","y\n");},later:".nx/cache/later.txt",listed:false},
 "a checkout without any directory of an excluded name":{prepare:()=>undefined},
};
const universe=(root:string)=>git(root,"ls-files","-co","--exclude-standard","-z");

for(const[name,item]of Object.entries(cases))test("planner run with "+name+": the witness holds exactly while git lists the same universe",async()=>{
 const{base,root}=checkout(item.prepare);
 try{
  const witness=plan(base,root);expect(witness.tainted,witness.tainted.join(";")).toEqual([]);
  const before=universe(root);if(item.later)write(root,item.later,"l\n");
  const relevant=universe(root)!==before;
  expect(relevant,"oracle: git ls-files").toBe(item.later!==undefined&&item.listed!==false);
  expect(await verifyWorldWitnessV1(witness,{signal,ports:ports(root)}),"a file git lists that the witness cannot see would be a stale plan").toMatchObject({holds:!relevant});
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("a file that appears later below an ignored dist directory does not make the world differ for the planner: the complete plan is identical",async()=>{
 const{base,root}=checkout(root=>{write(root,".gitignore","dist/\n");write(root,"course/dist/out.txt","o\n");});
 try{
  const witness=plan(base,root);expect(witness.tainted).toEqual([]);
  write(root,"course/dist/later.txt","l\n");
  expect(await verifyWorldWitnessV1(witness,{signal,ports:ports(root)})).toMatchObject({holds:true});
  expect(git(root,"ls-files","-co","--exclude-standard","course").includes("later.txt")).toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);
