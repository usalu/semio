/** 🙈️ Contract and oracle: the witness of a planner run may only hold while the complete plan would be identical. A directory that the witness walk never enters (`dist`, `build`, `target`, `coverage`, ...) and that git does not ignore can gain files that change no observed identity, so a planner run that saw one is not reusable (the witness is tainted); one that git ignores holds nothing of the universe and stays reusable. Run from the repository root with real git checkouts and the real planner, like the oracle of the plan reuse. */
import {expect,test}from"bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import {tmpdir}from"node:os";
import {dirname,join}from"node:path";
import corpus from"../../../🎬️scopes/🧪️tests/🧱️workspace/🔣️.json";
import {readWorldWitnessV1,verifyWorldWitnessV1,worldWitnessEnvironmentV1,worldWitnessGitDigestV1,worldWitnessPreloadV1}from"../../../../../🔭️witness/🟨️.mjs";

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
const ports=(root:string)=>({git:async(subject:string,expected:string)=>{const specs=JSON.parse(subject) as string[];let listing:string;try{listing=git(root,"ls-files","-c","-z",...(specs.length?["--",...specs]:[]));}catch{return expected==="!failed";}return worldWitnessGitDigestV1(listing)===expected;}});

const open=(root:string)=>({base:"",root});
test("[rev4] dist ignored by .git/info/exclude at plan time, exclude edited later: the witness must differ",async()=>{
 const{base,root}=checkout(root=>{write(root,".git/info/exclude","dist/\n");write(root,"course/dist/out.txt","o\n");});
 try{
  const witness=plan(base,root);expect(witness.tainted).toEqual([]);
  write(root,".git/info/exclude","");
  const verdict=await verifyWorldWitnessV1(witness,{signal,ports:ports(root)});
  console.log("[rev4] info/exclude edit ->",JSON.stringify(verdict));
  expect(verdict.holds).toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);
test("[rev4] untracked file ignored through core.excludesFile (the usual ~/.gitignore_global) at plan time, the file edited later to un-ignore: the witness must differ",async()=>{
 const{base,root}=checkout(root=>{write(root,"course/notes.tmp","n\n");});
 try{
  const global=join(base,"gitignore_global");writeFileSync(global,"*.tmp\n");
  git(root,"config","core.excludesFile",global);
  const witness=plan(base,root);
  const listed=git(root,"ls-files","-co","--exclude-standard","course").includes("notes.tmp");
  writeFileSync(global,"");
  const after=git(root,"ls-files","-co","--exclude-standard","course").includes("notes.tmp");
  const verdict=await verifyWorldWitnessV1(witness,{signal,ports:ports(root)});
  console.log("[rev4] core.excludesFile: universe listed before",listed,"after",after,"-> witness",JSON.stringify(verdict),"tainted",JSON.stringify(witness.tainted));
  expect(listed).toBe(false);expect(after).toBe(true);
  expect(witness.tainted.length>0||verdict.holds===false,"universe changed but the witness still holds").toBe(true);
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);
