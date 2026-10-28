/** 🕳️ Contract and oracle: the witness of a plan observes every directory below its traversal roots in which git could list a new untracked file, also when the directory carries a name the walk used to skip by name (`target`, `.🧬semio`, `node_modules`, ...) but git does not ignore it (a nested `.🧬semio` a tool left in a package, a `target` below `🧫️fixtures` that `.gitignore` un-ignores), while a directory git ignores as a whole stays unobserved. Oracles: a complete second planner run (byte digest of the workspace) and `git ls-files -co --exclude-standard` of the project. https://git-scm.com/docs/git-check-ignore */
import {expect,test}from"bun:test";
import {createHash}from"node:crypto";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import {tmpdir}from"node:os";
import {dirname,join,relative}from"node:path";
import corpus from"./🧫️fixtures/🔣️.json";
import {readWorldWitnessV1,verifyWorldWitnessV1,worldWitnessEnvironmentV1,worldWitnessPreloadV1}from"../../../../../🔭️witness/🟨️.mjs";
import {bazelIgnoreSourcesDigestV1,bazelTrackedDigestV1}from"../../../📜️script.ts";

process.env.SEMIO_LANGUAGE="en";
const repo=process.cwd(),child=join(import.meta.dirname,"🧫️fixtures/🟦️.ts"),signal=new AbortController().signal;
const git=(root:string,...args:string[])=>{const run=Bun.spawnSync(["git","-c","user.name=t","-c","user.email=t@t","-c","core.fsmonitor=false","-c","commit.gpgsign=false",...args],{cwd:root,stdout:"pipe",stderr:"pipe"});if(run.exitCode!==0)throw Error(args.join(" ")+": "+run.stderr.toString());return run.stdout.toString();};
const write=(root:string,path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
const listed=(root:string)=>git(root,"ls-files","-co","--exclude-standard","-z","--","p").split("\0").filter(Boolean).sort().join("|");
function checkout(){
 const base=realpathSync(mkdtempSync(join(tmpdir(),"excluded-"))),root=join(base,"checkout");mkdirSync(root);
 for(const[path,text]of Object.entries(corpus.files))write(root,path,text);
 git(root,"init","-q");git(root,"add","-A");git(root,"commit","-q","-m","fixture");
 return{base,root};
}
function plan(base:string,root:string,witnessed:boolean){
 writeFileSync(join(base,"preload.mjs"),worldWitnessPreloadV1());
 const workspace=join(base,"ws"),env={...process.env,SEMIO_LANGUAGE:"en",...(witnessed?worldWitnessEnvironmentV1({out:join(base,"witness.json"),root,workspace,writable:[tmpdir()],racyMilliseconds:0}):{})};
 const run=Bun.spawnSync([process.execPath,...(witnessed?["--preload",join(base,"preload.mjs")]:[]),child,root,workspace,repo],{cwd:repo,env,stdout:"pipe",stderr:"pipe"});
 if(run.exitCode!==0)throw Error(run.stderr.toString()+run.stdout.toString());
 return witnessed?readWorldWitnessV1(readFileSync(join(base,"witness.json"),"utf8")):undefined;
}
const ignored=new Set(["runtime.json","snapshot.json","planner-input.json","planner-output.json"]);
function digest(workspace:string):string{
 const hash=createHash("sha256"),walk=(directory:string)=>{for(const entry of readdirSync(directory,{withFileTypes:true}).sort((a,b)=>a.name<b.name?-1:1)){
  const path=join(directory,entry.name),name=relative(workspace,path);
  if(!name.includes("/")&&(ignored.has(entry.name)||entry.name.endsWith(".tmp")||entry.name.startsWith("bazel-")))continue;
  if(entry.isDirectory())walk(path);else hash.update(name+"\0"+(lstatSync(path).mode&0o111)+"\0"+readFileSync(path)+"\0");
 }};
 walk(workspace);return hash.digest("hex");
}
const ports=(root:string)=>({git:async(subject:string,expected:string)=>(await bazelTrackedDigestV1(root,JSON.parse(subject) as string[],signal))===expected,ignore:async(_subject:string,expected:string)=>(await bazelIgnoreSourcesDigestV1(root,signal))===expected});

const relevant:Record<string,(root:string)=>void>={
 "new source file":root=>write(root,"p/src/b.ts","export const b=2;\n"),
 "untracked file in an existing nested .🧬semio directory git does not ignore":root=>write(root,"p/.🧬semio/🦑️repo/n.md","n\n"),
 "untracked file in an existing fixture target directory that .gitignore un-ignores":root=>write(root,"p/🧫️fixtures/f/target/debug/n.txt","n\n"),
};
const unrelated:Record<string,(root:string)=>void>={
 "new file in an existing target directory git ignores":root=>write(root,"p/target/x.rs","// x\n"),
 "content of an unrelated file":root=>write(root,"docs/readme.md","changed\n"),
};

test("the witness of the untouched world holds and observes the nested directories git does not ignore",async()=>{
 const{base,root}=checkout();
 try{
  const witness=plan(base,root,true)!;
  expect(witness.tainted,witness.tainted.join(";")).toEqual([]);expect(witness.unstable).toEqual([]);expect(witness.racy).toBeNull();
  expect(await verifyWorldWitnessV1(witness,{signal,ports:ports(root)})).toMatchObject({holds:true});
  expect(listed(root),"oracle git: the planner can list both nested directories").toContain("p/.🧬semio/🦑️repo/keep.md");
  expect(listed(root)).toContain("p/🧫️fixtures/f/target/debug/keep.txt");
  expect(listed(root),"oracle git: the ignored target directory is no source").not.toContain("p/target/");
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("oracle: whenever the witness holds after an edit the complete plan is identical, a new file git would list breaks it, edits git ignores do not",async()=>{
 const results:string[]=[];
 for(const[group,edits]of[["relevant",relevant],["unrelated",unrelated]]as const)for(const[name,edit]of Object.entries(edits)){
  const{base,root}=checkout();
  try{
   const witness=plan(base,root,true)!,before=digest(join(base,"ws")),files=listed(root);
   expect(witness.tainted,name).toEqual([]);
   edit(root);
   const verdict=await verifyWorldWitnessV1(witness,{signal,ports:ports(root)}),holds=verdict.holds,changed=listed(root)!==files;
   plan(base,root,false);const after=digest(join(base,"ws"));
   results.push(`${group}: ${name}: holds=${holds} same=${after===before} listed-changed=${changed}`);
   if(holds)expect(after,"unsound reuse after: "+name).toBe(before);
   if(group==="relevant"){expect(changed,"oracle git: "+name).toBe(true);expect(holds,"missed: "+name).toBe(false);expect(after,"relevant but no change: "+name).not.toBe(before);}
   else{expect(changed,"oracle git: "+name).toBe(false);expect(holds,"needless replan after: "+name+" "+JSON.stringify(verdict)).toBe(true);}
  }finally{rmSync(base,{recursive:true,force:true});}
 }
 console.log(results.join("\n"));
},600000);
